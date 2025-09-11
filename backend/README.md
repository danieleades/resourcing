# backend (GraphQL)

Rust reimplementation of the Python FastAPI backend, using Axum + async-graphql, backed by SQLite via SQLx. It exposes GraphQL equivalents of the prior REST endpoints.

## Run

- Reuse the existing SQLite DB created by the Python service:

  - The Python app’s dev DB lives at `../backend/dev.db`.
  - Point this service at it (sqlx expects `sqlite:<path>`):

    ```bash
    DATABASE_URL="sqlite:../backend/dev.db" cargo run
    ```

- Or use another SQLite file/URL (created separately):

  ```bash
  DATABASE_URL="sqlite:./dev.db" cargo run
  ```

The server listens on `http://localhost:8000/` (GraphiQL UI) and `POST /graphql` for queries.

## Database & Migrations

- Migrations: This service runs SQLx migrations automatically on startup from `./migrations`. If the `DATABASE_URL` points to a new SQLite file, the schema is created on first run.
- `DATABASE_URL`: Provide a SQLite URL like `sqlite:./dev.db` or `sqlite::memory:`. Using different URLs enables isolated databases (e.g., for parallel E2E runs).
- Foreign keys: Enabled at connection time for all SQLite connections.
- Demo data (optional): Use the helper to generate a pre-populated demo database file, then run the server against it:

  ```bash
  # create demo.db with sample data (via xtask)
  cargo run -p xtask -- generate-demo ./demo.db
  # or via just: just gen-demo db=./demo.db

  # run the server against it
  DATABASE_URL="sqlite:./demo.db" cargo run
  # or via just: just demo
  ```

### Parallel E2E / Isolated DBs

- Spawn the backend with a unique `DATABASE_URL` per test worker, e.g. `sqlite:/tmp/backend-rs-e2e-$WORKER.db` or an in-memory `sqlite::memory:` if the server lifecycle matches a single test.
- The backend will create and migrate the database automatically, avoiding cross-test interference.

## Schema Overview

- Query
  - `health: Health!` — returns `{ status: "ok" }`.
  - `projects: [Project!]!` — list of projects.
  - `resources: [Resource!]!` — list of resources.
  - `assignments(resourceId, projectId, year, month): [Assignment!]!` — optional filters; if `month` is provided, `year` is required.
  - `tableProjects(from: String!, to: String!): TableProjects!` — monthly window, inclusive, e.g. `from: "2025-01", to: "2025-03"`.

- Types
  - `Project { id: String!, name: String! }`
  - `Resource { id: String!, name: String! }`
  - `Assignment { resourceId: String!, projectId: String!, year: Int!, month: Int! }`
  - `TableProjects { months: [String!]!, rows: [TableProjectsRow!]! }`
  - `TableProjectsRow { projectId: String!, projectName: String!, cells: [[TableProjectsAssignment!]!]! }`
  - `TableProjectsAssignment { resourceId: String!, resourceName: String! }`

## Example Queries

Open `http://localhost:8000/` for GraphiQL and try:

```graphql
query Health { health { status } }
```

```graphql
query Projects { projects { id name } }
```

```graphql
query Resources { resources { id name } }
```

```graphql
query Assignments {
  assignments(projectId: "proj-orion", year: 2025) {
    resourceId projectId year month
  }
}
```

```graphql
query TableProjects {
  tableProjects(from: "2025-01", to: "2025-03") {
    months
    rows {
      projectId
      projectName
      cells { resourceId resourceName }
    }
  }
}
```

## Notes

- CORS is enabled for all origins and the `GET`/`POST` methods to match the dev-friendly posture of the Python service.
- The schema mirrors the semantics of the REST endpoints but with GraphQL types and queries.
- The service relies on the existing SQLite schema (same tables and constraints as the Python backend).
