use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let sub = args.next().unwrap_or_else(|| "help".to_string());

    match sub.as_str() {
        "generate-demo" | "gen-demo" => {
            let db_path = args.next().unwrap_or_else(|| "./demo.db".to_string());
            generate_demo(&db_path).await?;
        }
        "help" | "-h" | "--help" => {
            eprintln!(
                "xtask commands:\n  generate-demo [path]     Create and seed a demo SQLite DB (default ./demo.db)"
            );
        }
        other => {
            eprintln!("Unknown command: {other}\nRun with --help for usage.");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn generate_demo(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use time::{Month, OffsetDateTime};

    // Helper: add n months to (year, month)
    fn add_months(mut year: i32, month: Month, n: i32) -> (i32, u8) {
        let total = i32::from(u8::from(month)) + n - 1; // Month is 1..=12
        year += total.div_euclid(12);
        let m0 = total.rem_euclid(12) + 1; // back to 1..=12
        // safe: m0 is 1..=12
        (year, u8::try_from(m0).expect("month in 1..=12"))
    }

    let db_url = format!("sqlite:{db_path}");

    if let Some(parent) = PathBuf::from(db_path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    let connect_opts = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect_with(connect_opts)
        .await?;

    // Use backend-core migrations folder
    sqlx::migrate!("../backend-core/migrations")
        .run(&pool)
        .await?;

    let (project_count, resource_count): (i64, i64) = sqlx::query_as(
        "SELECT \
            (SELECT COUNT(*) FROM projects) AS project_count, \
            (SELECT COUNT(*) FROM resources) AS resource_count",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or((0, 0));

    if project_count == 0 && resource_count == 0 {
        // Stable UUIDv5s for reproducible demo seeds
        // Namespace choice is arbitrary; DNS is convenient for deterministic names.
        let p_alpha = Uuid::new_v4();
        let p_beta = Uuid::new_v4();
        let r_alice = Uuid::new_v4();
        let r_bob = Uuid::new_v4();

        // Projects
        sqlx::query("INSERT OR IGNORE INTO projects (id, name) VALUES (?, ?)")
            .bind(p_alpha.as_bytes().to_vec()) // BLOB(16)
            .bind("Project Alpha")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT OR IGNORE INTO projects (id, name) VALUES (?, ?)")
            .bind(p_beta.as_bytes().to_vec())
            .bind("Project Beta")
            .execute(&pool)
            .await?;

        // Resources
        sqlx::query("INSERT OR IGNORE INTO resources (id, name) VALUES (?, ?)")
            .bind(r_alice.as_bytes().to_vec())
            .bind("Alice")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT OR IGNORE INTO resources (id, name) VALUES (?, ?)")
            .bind(r_bob.as_bytes().to_vec())
            .bind("Bob")
            .execute(&pool)
            .await?;

        // Assignments relative to current month
        let now = OffsetDateTime::now_utc().date();
        let base_year = now.year();
        let base_month = now.month();

        // (resource_id, project_id, offset months)
        let assignments: Vec<([u8; 16], [u8; 16], i32)> = vec![
            (*r_alice.as_bytes(), *p_alpha.as_bytes(), 0),
            (*r_alice.as_bytes(), *p_beta.as_bytes(), 1),
            (*r_bob.as_bytes(), *p_alpha.as_bytes(), 0),
            (*r_bob.as_bytes(), *p_beta.as_bytes(), 2),
        ];

        for (res_bytes, proj_bytes, offset) in assignments {
            let (y, m) = add_months(base_year, base_month, offset);
            sqlx::query(
                "INSERT OR IGNORE INTO assignments (resource_id, project_id, year, month) \
                 VALUES (?, ?, ?, ?)",
            )
            .bind(res_bytes.to_vec()) // BLOB(16)
            .bind(proj_bytes.to_vec()) // BLOB(16)
            .bind(y)
            .bind(i32::from(m))
            .execute(&pool)
            .await?;
        }
    }

    println!("Demo database ready at: {db_path}");
    Ok(())
}
