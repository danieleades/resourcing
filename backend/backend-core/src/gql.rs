mod loader;
pub mod mutation;
pub mod query;
pub mod types;

use crate::repo::Repo;

use async_graphql::{EmptySubscription, Schema, dataloader::DataLoader};

pub type AppSchema = Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;

pub fn build_schema(repo: Repo) -> AppSchema {
    let project_loader = DataLoader::new(loader::ProjectLoader::new(repo.clone()), tokio::spawn);
    let resource_loader = DataLoader::new(loader::ResourceLoader::new(repo.clone()), tokio::spawn);

    Schema::build(query::QueryRoot, mutation::MutationRoot, EmptySubscription)
        .data(repo)
        .data(project_loader)
        .data(resource_loader)
        .finish()
}
