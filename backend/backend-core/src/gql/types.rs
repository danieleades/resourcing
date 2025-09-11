use crate::domain;
use crate::gql::loader::ProjectLoader;
use crate::repo::{AssignmentFilter as RepoAssignmentFilter, Repo};
use async_graphql::dataloader::DataLoader;
use async_graphql::{
    ComplexObject, Context, InputObject, InputValueError, InputValueResult, Scalar, ScalarType,
    SimpleObject, Value,
};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

// Scalar: Month in format "YYYY-MM"
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MonthScalar {
    pub year: i32,
    pub month: u8, // 1..=12
}

impl MonthScalar {
    pub fn as_str(&self) -> String {
        format!("{:04}-{:02}", self.year, self.month)
    }
}

impl Display for MonthScalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[Scalar(name = "Month")]
impl ScalarType for MonthScalar {
    fn parse(value: async_graphql::Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                // Expect YYYY-MM
                let parts: Vec<&str> = s.split('-').collect();
                if parts.len() != 2 {
                    return Err(InputValueError::custom("Month must be in YYYY-MM format"));
                }
                let year: i32 = parts[0]
                    .parse()
                    .map_err(|_| InputValueError::custom("Invalid year in Month"))?;
                let month: u8 = parts[1]
                    .parse()
                    .map_err(|_| InputValueError::custom("Invalid month in Month"))?;
                if !(1..=12).contains(&month) {
                    return Err(InputValueError::custom("Month must be between 01 and 12"));
                }
                Ok(Self { year, month })
            }
            other => Err(InputValueError::custom(format!(
                "Expected string for Month, got {other:?}"
            ))),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::ScalarType;

    #[test]
    fn month_scalar_parse_valid() {
        let m = MonthScalar::parse(Value::String("2024-03".into())).unwrap();
        assert_eq!(m.year, 2024);
        assert_eq!(m.month, 3);
        assert_eq!(m.to_value(), Value::String("2024-03".into()));
    }

    #[test]
    fn month_scalar_parse_invalid() {
        assert!(MonthScalar::parse(Value::String("2024".into())).is_err());
        assert!(MonthScalar::parse(Value::String("2024-13".into())).is_err());
        assert!(MonthScalar::parse(Value::String("abcd-ef".into())).is_err());
    }
}

// Health removed per simplified API

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
}

impl From<domain::Project> for Project {
    fn from(v: domain::Project) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

#[ComplexObject]
impl Project {
    async fn assignments(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Assignment>> {
        let repo = ctx.data_unchecked::<Repo>();
        let f = RepoAssignmentFilter {
            project_id: Some(self.id),
            ..Default::default()
        };
        let out = repo.list_assignments(&f).await?;
        Ok(out.into_iter().map(Into::into).collect())
    }
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
}

#[ComplexObject]
impl Resource {
    async fn assignments(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Assignment>> {
        let repo = ctx.data_unchecked::<Repo>();
        let f = RepoAssignmentFilter {
            resource_id: Some(self.id),
            ..Default::default()
        };
        let out = repo.list_assignments(&f).await?;
        Ok(out.into_iter().map(Into::into).collect())
    }
}

impl From<domain::Resource> for Resource {
    fn from(resource: domain::Resource) -> Self {
        Self {
            id: resource.id,
            name: resource.name,
        }
    }
}

// No extra fields on Resource per simplified API

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Assignment {
    // Composite ID encoded as string (resource:project:YYYY-MM)
    pub id: String,
    pub month: MonthScalar,
    // Backing fields (not exposed) for resolvers
    #[graphql(skip)]
    pub resource_id: Uuid,
    #[graphql(skip)]
    pub project_id: Uuid,
}

impl From<domain::Assignment> for Assignment {
    fn from(v: domain::Assignment) -> Self {
        let month = MonthScalar {
            year: v.year,
            // domain::Assignment guarantees 1..=12
            month: u8::try_from(v.month).expect("Assignment.month must be 1..=12"),
        };
        let id = format!("{}:{}:{}", v.resource_id, v.project_id, month.as_str());
        Self {
            id,
            month,
            resource_id: v.resource_id,
            project_id: v.project_id,
        }
    }
}

#[ComplexObject]
impl Assignment {
    async fn project(&self, ctx: &Context<'_>) -> async_graphql::Result<Project> {
        let dl = ctx.data_unchecked::<DataLoader<ProjectLoader>>();
        let p = dl
            .load_one(self.project_id)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Project not found"))?;
        Ok(p.into())
    }

    async fn resource(&self, ctx: &Context<'_>) -> async_graphql::Result<Resource> {
        let dataloader = ctx.data_unchecked::<DataLoader<crate::gql::loader::ResourceLoader>>();
        let r = dataloader
            .load_one(self.resource_id)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Resource not found"))?;
        Ok(r.into())
    }
}

// AssignmentFilter removed from public API

// Inputs per simplified API
#[derive(InputObject, Clone, Debug)]
pub struct CreateResourceInput {
    pub name: String,
}

#[derive(InputObject, Clone, Debug)]
pub struct CreateProjectInput {
    pub name: String,
}

#[derive(InputObject, Clone, Debug)]
pub struct AssignInput {
    #[graphql(name = "resourceId")]
    pub resource_id: Uuid,
    #[graphql(name = "projectId")]
    pub project_id: Uuid,
    pub month: MonthScalar,
}

#[derive(InputObject, Clone, Debug)]
pub struct UnassignInput {
    #[graphql(name = "resourceId")]
    pub resource_id: Uuid,
    #[graphql(name = "projectId")]
    pub project_id: Uuid,
    pub month: MonthScalar,
}

// Table-optimized structures (legacy flat shapes) removed in favor of matrix types

// ----------------------------
// Matrix (pivoted) structures per API_DESIGN.graphql
// ----------------------------

#[derive(SimpleObject, Clone)]
pub struct ProjectMonthCell {
    pub resources: Vec<Resource>,
}

#[derive(SimpleObject, Clone)]
pub struct ProjectMonthMatrixRow {
    pub project: Project,
    pub cells: Vec<ProjectMonthCell>,
}

#[derive(SimpleObject, Clone)]
pub struct ProjectMonthMatrix {
    pub months: Vec<MonthScalar>,
    pub rows: Vec<ProjectMonthMatrixRow>,
}

#[derive(SimpleObject, Clone)]
pub struct ResourceMonthCell {
    pub projects: Vec<Project>,
}

#[derive(SimpleObject, Clone)]
pub struct ResourceMonthMatrixRow {
    pub resource: Resource,
    pub cells: Vec<ResourceMonthCell>,
}

#[derive(SimpleObject, Clone)]
pub struct ResourceMonthMatrix {
    pub months: Vec<MonthScalar>,
    pub rows: Vec<ResourceMonthMatrixRow>,
}
