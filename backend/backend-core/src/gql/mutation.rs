use super::types::{
    AssignInput, Assignment, CreateProjectInput, CreateResourceInput, Project, Resource,
    UnassignInput,
};
use crate::{domain, repo::Repo};
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    #[graphql(name = "createProject")]
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<Project> {
        let repo = ctx.data_unchecked::<Repo>();
        let p = repo.create_project(&input.name).await?;
        Ok(p.into())
    }

    #[graphql(name = "deleteProject")]
    async fn delete_project(&self, ctx: &Context<'_>, id: uuid::Uuid) -> Result<bool> {
        let repo = ctx.data_unchecked::<Repo>();
        Ok(repo.delete_project(id).await? > 0)
    }

    #[graphql(name = "createResource")]
    async fn create_resource(
        &self,
        ctx: &Context<'_>,
        input: CreateResourceInput,
    ) -> Result<Resource> {
        let repo = ctx.data_unchecked::<Repo>();
        let r = repo.create_resource(&input.name).await?;
        Ok(r.into())
    }

    #[graphql(name = "deleteResource")]
    async fn delete_resource(&self, ctx: &Context<'_>, id: uuid::Uuid) -> Result<bool> {
        let repo = ctx.data_unchecked::<Repo>();
        Ok(repo.delete_resource(id).await? > 0)
    }

    #[graphql(name = "assign")]
    async fn assign(&self, ctx: &Context<'_>, input: AssignInput) -> Result<Assignment> {
        let repo = ctx.data_unchecked::<Repo>();
        let d = domain::Assignment {
            resource_id: input.resource_id,
            project_id: input.project_id,
            year: input.month.year,
            month: i32::from(input.month.month),
        };
        repo.assign(&d).await?;
        Ok(d.into())
    }

    #[graphql(name = "unassign")]
    async fn unassign(&self, ctx: &Context<'_>, input: UnassignInput) -> Result<bool> {
        let repo = ctx.data_unchecked::<Repo>();
        let d = domain::Assignment {
            resource_id: input.resource_id,
            project_id: input.project_id,
            year: input.month.year,
            month: i32::from(input.month.month),
        };
        Ok(repo.unassign(&d).await? > 0)
    }
}
