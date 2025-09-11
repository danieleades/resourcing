use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::http::header;
use axum::{Router, routing::get};
use backend_core::{SqliteRepository, build_schema};
use tower_http::cors::{Any, CorsLayer};

async fn graphiql() -> impl axum::response::IntoResponse {
    axum::response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read DATABASE_URL
    let db_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL env var is required, e.g. sqlite:../backend/dev.db")?;

    // Init repo
    let repo = SqliteRepository::connect(&db_url).await?;

    // Build GraphQL schema with repo in context
    let schema = build_schema(Arc::new(repo));

    let app = Router::new()
        .route("/", get(graphiql))
        .route_service("/graphql", GraphQL::new(schema.clone()))
        .layer(
            CorsLayer::new()
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_origin(Any),
        );

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("GraphQL IDE: http://{addr}/");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
