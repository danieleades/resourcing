//! SQLite-backed repository.

use crate::domain;
use async_trait::async_trait;
use sqlx::{
    QueryBuilder, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

mod dto;

pub type Repo = Arc<dyn Repository>;

// Domain-level filtering inputs for assignment queries
#[derive(Clone, Debug, Default)]
pub struct AssignmentFilter {
    pub resource_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub year: Option<i32>,
    pub month: Option<i32>, // 1..=12
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn list_projects(&self) -> sqlx::Result<Vec<domain::Project>>;
    async fn list_resources(&self) -> sqlx::Result<Vec<domain::Resource>>;
    async fn list_assignments(
        &self,
        filter: &AssignmentFilter,
    ) -> sqlx::Result<Vec<domain::Assignment>>;

    async fn projects(&self, ids: &[Uuid]) -> sqlx::Result<Vec<domain::Project>>;

    async fn resources(&self, ids: &[Uuid]) -> sqlx::Result<Vec<domain::Resource>>;

    async fn upsert_project(&self, project: &domain::Project) -> sqlx::Result<()>;
    async fn upsert_resource(&self, resource: &domain::Resource) -> sqlx::Result<()>;

    // Simplified API operations
    async fn create_project(&self, name: &str) -> sqlx::Result<domain::Project>;
    async fn create_resource(&self, name: &str) -> sqlx::Result<domain::Resource>;
    async fn delete_project(&self, id: Uuid) -> sqlx::Result<u64>;
    async fn delete_resource(&self, id: Uuid) -> sqlx::Result<u64>;

    async fn assign(&self, assignment: &domain::Assignment) -> sqlx::Result<()>;
    async fn unassign(&self, assignment: &domain::Assignment) -> sqlx::Result<u64>;
}

#[derive(Clone)]
pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    /// Connects to the `SQLite` database at `url`, running pending migrations.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be established or migrations fail.
    pub async fn connect(url: &str) -> sqlx::Result<Self> {
        // Use connect options to ensure pragma foreign_keys=ON for every connection.
        let connect_opts = SqliteConnectOptions::from_str(url)
            .map_err(|e| sqlx::Error::Configuration(Box::new(e)))?
            .create_if_missing(true)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .after_connect(|conn, _meta| {
                Box::pin(async move {
                    // Ensure FKs are enforced on every pooled connection.
                    sqlx::query("PRAGMA foreign_keys = ON")
                        .execute(conn)
                        .await
                        .map(|_| ())
                })
            })
            .connect_with(connect_opts)
            .await?;

        // Run database migrations on startup for the provided DATABASE_URL.
        // This ensures dynamically created databases (e.g. for tests) are initialized.
        sqlx::migrate!().run(&pool).await?;

        let repo = Self { pool };
        Ok(repo)
    }
}

#[async_trait]
impl Repository for SqliteRepository {
    async fn list_projects(&self) -> sqlx::Result<Vec<domain::Project>> {
        let rows: Vec<dto::Project> =
            sqlx::query_as::<_, dto::Project>("SELECT id, name FROM projects ORDER BY name")
                .fetch_all(&self.pool)
                .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_resources(&self) -> sqlx::Result<Vec<domain::Resource>> {
        let rows: Vec<dto::Resource> =
            sqlx::query_as::<_, dto::Resource>("SELECT id, name FROM resources ORDER BY name")
                .fetch_all(&self.pool)
                .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_assignments(
        &self,
        filter: &AssignmentFilter,
    ) -> sqlx::Result<Vec<domain::Assignment>> {
        let mut qb = QueryBuilder::<Sqlite>::new(
            "SELECT resource_id, project_id, year, month FROM assignments WHERE 1=1",
        );

        if let Some(ref v) = filter.resource_id {
            qb.push(" AND resource_id = ").push_bind(v);
        }
        if let Some(ref v) = filter.project_id {
            qb.push(" AND project_id = ").push_bind(v);
        }
        if let Some(v) = filter.year {
            qb.push(" AND year = ").push_bind(v);
        }
        if let Some(v) = filter.month {
            qb.push(" AND month = ").push_bind(v);
        }

        qb.push(" ORDER BY year, month, project_id, resource_id");

        let rows: Vec<dto::Assignment> = qb.build_query_as().fetch_all(&self.pool).await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn upsert_project(&self, project: &domain::Project) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO projects (id, name) VALUES (?, ?) \
             ON CONFLICT(id) DO UPDATE SET name=excluded.name",
        )
        .bind(project.id)
        .bind(&project.name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert_resource(&self, resource: &domain::Resource) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO resources (id, name) VALUES (?, ?) \
             ON CONFLICT(id) DO UPDATE SET name=excluded.name",
        )
        .bind(resource.id)
        .bind(&resource.name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_project(&self, name: &str) -> sqlx::Result<domain::Project> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO projects (id, name) VALUES (?, ?)")
            .bind(id)
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(domain::Project {
            id,
            name: name.to_string(),
        })
    }

    async fn create_resource(&self, name: &str) -> sqlx::Result<domain::Resource> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO resources (id, name) VALUES (?, ?)")
            .bind(id)
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(domain::Resource {
            id,
            name: name.to_string(),
        })
    }

    async fn delete_project(&self, id: Uuid) -> sqlx::Result<u64> {
        let res = sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn delete_resource(&self, id: Uuid) -> sqlx::Result<u64> {
        let res = sqlx::query("DELETE FROM resources WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn assign(&self, assignment: &domain::Assignment) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO assignments (resource_id, project_id, year, month) VALUES (?, ?, ?, ?) \
             ON CONFLICT(resource_id, project_id, year, month) DO NOTHING",
        )
        .bind(assignment.resource_id)
        .bind(assignment.project_id)
        .bind(assignment.year)
        .bind(assignment.month)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn unassign(&self, assignment: &domain::Assignment) -> sqlx::Result<u64> {
        let res = sqlx::query(
            "DELETE FROM assignments WHERE resource_id=? AND project_id=? AND year=? AND month=?",
        )
        .bind(assignment.resource_id)
        .bind(assignment.project_id)
        .bind(assignment.year)
        .bind(assignment.month)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    async fn projects(&self, ids: &[Uuid]) -> sqlx::Result<Vec<domain::Project>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = vec!["?"; ids.len()].join(", ");
        let sql = format!("SELECT id, name FROM projects WHERE id IN ({placeholders})");

        let mut q = sqlx::query_as::<_, (Uuid, String)>(&sql);
        for id in ids {
            q = q.bind(id);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .map(|(id, name)| domain::Project { id, name })
            .collect())
    }

    async fn resources(&self, ids: &[Uuid]) -> sqlx::Result<Vec<domain::Resource>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = vec!["?"; ids.len()].join(", ");
        let sql = format!("SELECT id, name FROM resources WHERE id IN ({placeholders})");

        let mut q = sqlx::query_as::<_, (Uuid, String)>(&sql);
        for id in ids {
            q = q.bind(id);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .map(|(id, name)| domain::Resource { id, name })
            .collect())
    }
}
