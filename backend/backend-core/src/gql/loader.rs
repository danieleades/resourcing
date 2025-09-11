use async_graphql::dataloader::Loader;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{domain, repo::Repo};

#[derive(Clone)]
pub struct ProjectLoader {
    repo: Repo,
}
#[derive(Clone)]
pub struct ResourceLoader {
    repo: Repo,
}

impl ProjectLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}
impl ResourceLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl Loader<Uuid> for ProjectLoader {
    type Value = domain::Project;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        // batch read
        let items = self
            .repo
            .projects(keys)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(items.into_iter().map(|p| (p.id, p)).collect())
    }
}

impl Loader<Uuid> for ResourceLoader {
    type Value = domain::Resource;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let items = self
            .repo
            .resources(keys)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(items.into_iter().map(|r| (r.id, r)).collect())
    }
}
