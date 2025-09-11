use backend_core::{AssignmentFilter, DomainAssignment, Repository, SqliteRepository};

#[tokio::test]
async fn repo_create_assign_delete_flow() {
    let repo = SqliteRepository::connect("sqlite::memory:").await.unwrap();

    // Create project and resource
    let p = repo.create_project("Test Project").await.unwrap();
    let r = repo.create_resource("Test Resource").await.unwrap();

    // Assign in March 2024
    let a = DomainAssignment {
        resource_id: r.id,
        project_id: p.id,
        year: 2024,
        month: 3,
    };
    repo.assign(&a).await.unwrap();

    // List assignments
    let rows = repo
        .list_assignments(&AssignmentFilter {
            year: Some(2024),
            month: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], a);

    // Unassign
    let removed = repo.unassign(&a).await.unwrap();
    assert_eq!(removed, 1);

    // Delete resource and project
    assert_eq!(repo.delete_resource(r.id).await.unwrap(), 1);
    assert_eq!(repo.delete_project(p.id).await.unwrap(), 1);
}
