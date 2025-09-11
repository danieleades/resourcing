use uuid::Uuid;

use crate::domain;

// Internal SQL DTOs (not exposed outside this module)
#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub(super) struct Project {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub(super) struct Resource {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub(super) struct Assignment {
    pub resource_id: Uuid,
    pub project_id: Uuid,
    pub year: i32,
    pub month: i32,
}

impl From<Project> for domain::Project {
    fn from(v: Project) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}
impl From<Resource> for domain::Resource {
    fn from(v: Resource) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}
impl From<Assignment> for domain::Assignment {
    fn from(v: Assignment) -> Self {
        Self {
            resource_id: v.resource_id,
            project_id: v.project_id,
            year: v.year,
            month: v.month,
        }
    }
}
