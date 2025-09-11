use super::types::{
    Assignment, MonthScalar, Project, ProjectMonthCell, ProjectMonthMatrix, ProjectMonthMatrixRow,
    Resource, ResourceMonthCell, ResourceMonthMatrix, ResourceMonthMatrixRow,
};
use crate::repo::Repo;
use async_graphql::{Context, Object, Result};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[graphql(name = "projects")]
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let repo = ctx.data_unchecked::<Repo>();
        let out = repo.list_projects().await?;
        Ok(out.into_iter().map(Into::into).collect())
    }

    #[graphql(name = "resources")]
    async fn resources(&self, ctx: &Context<'_>) -> Result<Vec<Resource>> {
        let repo = ctx.data_unchecked::<Repo>();
        let out = repo.list_resources().await?;
        Ok(out.into_iter().map(Into::into).collect())
    }

    #[graphql(name = "assignments")]
    async fn assignments(&self, ctx: &Context<'_>) -> Result<Vec<Assignment>> {
        let repo = ctx.data_unchecked::<Repo>();
        let out = repo
            .list_assignments(&crate::repo::AssignmentFilter::default())
            .await?;
        Ok(out.into_iter().map(Into::into).collect())
    }

    #[graphql(name = "resource")]
    async fn resource(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Resource>> {
        let repo = ctx.data_unchecked::<Repo>();
        let items = repo.resources(&[id]).await?;
        Ok(items.into_iter().next().map(Into::into))
    }

    #[graphql(name = "project")]
    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Project>> {
        let repo = ctx.data_unchecked::<Repo>();
        let items = repo.projects(&[id]).await?;
        Ok(items.into_iter().next().map(Into::into))
    }

    #[graphql(name = "projectMonthMatrix")]
    async fn project_month_matrix(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "months")] months: Vec<MonthScalar>,
    ) -> Result<ProjectMonthMatrix> {
        let repo = ctx.data_unchecked::<Repo>();
        let projects = repo.list_projects().await?;

        // Preload all assignments for requested months
        let mut by_project_month: HashMap<(Uuid, i32, i32), Vec<Uuid>> = HashMap::new();
        for m in &months {
            let f = crate::repo::AssignmentFilter {
                year: Some(m.year),
                month: Some(i32::from(m.month)),
                ..Default::default()
            };
            let rows = repo.list_assignments(&f).await?;
            for a in rows {
                by_project_month
                    .entry((a.project_id, a.year, a.month))
                    .or_default()
                    .push(a.resource_id);
            }
        }

        // Build rows: all projects; cells length equals months length; empty cells have []
        let mut rows_out: Vec<ProjectMonthMatrixRow> = Vec::new();
        for p in projects {
            let mut cells: Vec<ProjectMonthCell> = Vec::with_capacity(months.len());
            for m in &months {
                let key = (p.id, m.year, i32::from(m.month));
                if let Some(resource_ids) = by_project_month.get(&key) {
                    let items = repo.resources(resource_ids).await?;
                    let resources: Vec<Resource> = items.into_iter().map(Into::into).collect();
                    cells.push(ProjectMonthCell { resources });
                } else {
                    cells.push(ProjectMonthCell { resources: vec![] });
                }
            }
            rows_out.push(ProjectMonthMatrixRow {
                project: p.into(),
                cells,
            });
        }
        Ok(ProjectMonthMatrix {
            months,
            rows: rows_out,
        })
    }

    #[graphql(name = "resourceMonthMatrix")]
    async fn resource_month_matrix(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "months")] months: Vec<MonthScalar>,
    ) -> Result<ResourceMonthMatrix> {
        let repo = ctx.data_unchecked::<Repo>();
        let resources = repo.list_resources().await?;

        let mut by_resource_month: HashMap<(Uuid, i32, i32), Vec<Uuid>> = HashMap::new();
        for m in &months {
            let f = crate::repo::AssignmentFilter {
                year: Some(m.year),
                month: Some(i32::from(m.month)),
                ..Default::default()
            };
            let rows = repo.list_assignments(&f).await?;
            for a in rows {
                by_resource_month
                    .entry((a.resource_id, a.year, a.month))
                    .or_default()
                    .push(a.project_id);
            }
        }

        let mut rows_out: Vec<ResourceMonthMatrixRow> = Vec::new();
        for r in resources {
            let mut cells: Vec<ResourceMonthCell> = Vec::with_capacity(months.len());
            for m in &months {
                let key = (r.id, m.year, i32::from(m.month));
                if let Some(project_ids) = by_resource_month.get(&key) {
                    let items = repo.projects(project_ids).await?;
                    let projects: Vec<Project> = items.into_iter().map(Into::into).collect();
                    cells.push(ResourceMonthCell { projects });
                } else {
                    cells.push(ResourceMonthCell { projects: vec![] });
                }
            }
            rows_out.push(ResourceMonthMatrixRow {
                resource: r.into(),
                cells,
            });
        }
        Ok(ResourceMonthMatrix {
            months,
            rows: rows_out,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain,
        repo::{AssignmentFilter, Repository},
    };
    use async_graphql::{EmptySubscription, Schema, dataloader::DataLoader};
    use async_trait::async_trait;
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Default)]
    struct MemRepo {
        projects: Arc<Mutex<HashMap<Uuid, domain::Project>>>,
        resources: Arc<Mutex<HashMap<Uuid, domain::Resource>>>,
        assignments: Arc<Mutex<Vec<domain::Assignment>>>,
    }

    #[async_trait]
    impl Repository for MemRepo {
        async fn list_projects(&self) -> sqlx::Result<Vec<domain::Project>> {
            Ok(self.projects.lock().unwrap().values().cloned().collect())
        }

        async fn list_resources(&self) -> sqlx::Result<Vec<domain::Resource>> {
            Ok(self.resources.lock().unwrap().values().cloned().collect())
        }

        async fn list_assignments(
            &self,
            filter: &AssignmentFilter,
        ) -> sqlx::Result<Vec<domain::Assignment>> {
            let rows = self.assignments.lock().unwrap().clone();
            let out = rows
                .into_iter()
                .filter(|a| {
                    (filter.resource_id.is_none_or(|id| a.resource_id == id))
                        && (filter.project_id.is_none_or(|id| a.project_id == id))
                        && (filter.year.is_none_or(|y| a.year == y))
                        && (filter.month.is_none_or(|m| a.month == m))
                })
                .collect();
            Ok(out)
        }

        async fn projects(&self, ids: &[Uuid]) -> sqlx::Result<Vec<domain::Project>> {
            let set: HashSet<Uuid> = ids.iter().copied().collect();
            Ok(self
                .projects
                .lock()
                .unwrap()
                .values()
                .filter(|p| set.contains(&p.id))
                .cloned()
                .collect())
        }

        async fn resources(&self, ids: &[Uuid]) -> sqlx::Result<Vec<domain::Resource>> {
            let set: HashSet<Uuid> = ids.iter().copied().collect();
            Ok(self
                .resources
                .lock()
                .unwrap()
                .values()
                .filter(|r| set.contains(&r.id))
                .cloned()
                .collect())
        }

        async fn upsert_project(&self, project: &domain::Project) -> sqlx::Result<()> {
            self.projects
                .lock()
                .unwrap()
                .insert(project.id, project.clone());
            Ok(())
        }

        async fn upsert_resource(&self, resource: &domain::Resource) -> sqlx::Result<()> {
            self.resources
                .lock()
                .unwrap()
                .insert(resource.id, resource.clone());
            Ok(())
        }

        async fn create_project(&self, name: &str) -> sqlx::Result<domain::Project> {
            let p = domain::Project {
                id: Uuid::new_v4(),
                name: name.to_string(),
            };
            self.upsert_project(&p).await?;
            Ok(p)
        }

        async fn create_resource(&self, name: &str) -> sqlx::Result<domain::Resource> {
            let r = domain::Resource {
                id: Uuid::new_v4(),
                name: name.to_string(),
            };
            self.upsert_resource(&r).await?;
            Ok(r)
        }

        async fn delete_project(&self, id: Uuid) -> sqlx::Result<u64> {
            let removed = self.projects.lock().unwrap().remove(&id).is_some();
            // cascade assignments
            self.assignments
                .lock()
                .unwrap()
                .retain(|a| a.project_id != id);
            Ok(u64::from(removed))
        }

        async fn delete_resource(&self, id: Uuid) -> sqlx::Result<u64> {
            let removed = self.resources.lock().unwrap().remove(&id).is_some();
            self.assignments
                .lock()
                .unwrap()
                .retain(|a| a.resource_id != id);
            Ok(u64::from(removed))
        }

        async fn assign(&self, assignment: &domain::Assignment) -> sqlx::Result<()> {
            let mut rows = self.assignments.lock().unwrap();
            if !rows.iter().any(|a| a == assignment) {
                rows.push(assignment.clone());
            }
            drop(rows);
            Ok(())
        }

        async fn unassign(&self, assignment: &domain::Assignment) -> sqlx::Result<u64> {
            let mut rows = self.assignments.lock().unwrap();
            let before = rows.len();
            rows.retain(|a| a != assignment);
            Ok((before - rows.len()) as u64)
        }
    }

    fn schema_with(
        repo: MemRepo,
    ) -> Schema<QueryRoot, crate::gql::mutation::MutationRoot, EmptySubscription> {
        let project_loader = DataLoader::new(
            crate::gql::loader::ProjectLoader::new(Arc::new(repo.clone())),
            tokio::spawn,
        );
        let resource_loader = DataLoader::new(
            crate::gql::loader::ResourceLoader::new(Arc::new(repo.clone())),
            tokio::spawn,
        );
        Schema::build(
            QueryRoot,
            crate::gql::mutation::MutationRoot,
            EmptySubscription,
        )
        .data(Arc::new(repo) as Repo)
        .data(project_loader)
        .data(resource_loader)
        .finish()
    }

    #[tokio::test]
    async fn project_and_resource_month_matrix_includes_all_rows_and_cells() {
        let repo = MemRepo::default();
        let p1 = domain::Project {
            id: Uuid::new_v4(),
            name: "P1".into(),
        };
        let p2 = domain::Project {
            id: Uuid::new_v4(),
            name: "P2".into(),
        };
        let r1 = domain::Resource {
            id: Uuid::new_v4(),
            name: "R1".into(),
        };
        let r2 = domain::Resource {
            id: Uuid::new_v4(),
            name: "R2".into(),
        };
        repo.upsert_project(&p1).await.unwrap();
        repo.upsert_project(&p2).await.unwrap();
        repo.upsert_resource(&r1).await.unwrap();
        repo.upsert_resource(&r2).await.unwrap();
        repo.assign(&domain::Assignment {
            resource_id: r1.id,
            project_id: p1.id,
            year: 2024,
            month: 3,
        })
        .await
        .unwrap();
        repo.assign(&domain::Assignment {
            resource_id: r2.id,
            project_id: p2.id,
            year: 2024,
            month: 4,
        })
        .await
        .unwrap();

        let schema = schema_with(repo.clone());

        // projectMonthMatrix for two months includes all projects and two cells each
        let q = r"
            query($ms: [Month!]!) {
              projectMonthMatrix(months: $ms) {
                months
                rows { project { id } cells { resources { id } } }
              }
            }
        ";
        let resp = schema
            .execute(
                async_graphql::Request::new(q).variables(async_graphql::Variables::from_json(
                    serde_json::json!({"ms":["2024-03","2024-04"]}),
                )),
            )
            .await;
        assert!(resp.errors.is_empty(), "{:?}", resp.errors);
        let data = serde_json::to_value(resp.data).unwrap();
        // months echo input order
        assert_eq!(
            data["projectMonthMatrix"]["months"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        let rows = data["projectMonthMatrix"]["rows"].as_array().unwrap();
        // Expect 2 projects
        assert_eq!(rows.len(), 2);
        // Each row should have exactly two cells
        for row in rows {
            assert_eq!(row["cells"].as_array().unwrap().len(), 2);
        }
        // There should be exactly one cell containing R1 for P1 at 2024-03 and one containing R2 for P2 at 2024-04
        let p1_row = rows
            .iter()
            .find(|row| row["project"]["id"].as_str().unwrap() == p1.id.to_string())
            .unwrap();
        let p1_cells = p1_row["cells"].as_array().unwrap();
        // 2024-03 is first month
        let r_list = p1_cells[0]["resources"].as_array().unwrap();
        assert_eq!(r_list.len(), 1);
        assert_eq!(r_list[0]["id"].as_str().unwrap(), r1.id.to_string());

        // resourceMonthMatrix similar shape
        let q3 = r"
            query($ms: [Month!]!) {
              resourceMonthMatrix(months: $ms) {
                months
                rows { resource { id } cells { projects { id } } }
              }
            }
        ";
        let resp = schema
            .execute(async_graphql::Request::new(q3).variables(
                async_graphql::Variables::from_json(
                    serde_json::json!({"ms":["2024-03","2024-04"]}),
                ),
            ))
            .await;
        assert!(resp.errors.is_empty());
        let data = serde_json::to_value(resp.data).unwrap();
        let rows = data["resourceMonthMatrix"]["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 2);
        for row in rows {
            assert_eq!(row["cells"].as_array().unwrap().len(), 2);
        }
    }
}
