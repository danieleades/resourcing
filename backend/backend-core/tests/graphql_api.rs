use async_graphql::{Request, Variables};
use backend_core::{SqliteRepository, build_schema};

#[tokio::test]
#[allow(clippy::too_many_lines)]
async fn graphql_create_assign_and_query_tables() {
    // Repo + schema
    let repo = SqliteRepository::connect("sqlite::memory:").await.unwrap();
    let schema = build_schema(std::sync::Arc::new(repo));

    // Create a resource
    let resp = schema
        .execute(Request::new(
            r#"mutation { createResource(input: { name: "Alice" }) { id name } }"#,
        ))
        .await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let data = serde_json::to_value(resp.data).unwrap();
    let r_id = data["createResource"]["id"].as_str().unwrap().to_string();

    // Create a project
    let resp = schema
        .execute(Request::new(
            r#"mutation { createProject(input: { name: "Alpha" }) { id name } }"#,
        ))
        .await;
    assert!(resp.errors.is_empty());
    let data = serde_json::to_value(resp.data).unwrap();
    let p_id = data["createProject"]["id"].as_str().unwrap().to_string();

    // Assign Alice to Alpha in 2024-03
    let assign_mut = format!(
        r#"mutation {{ assign(input: {{ resourceId: "{r_id}", projectId: "{p_id}", month: "2024-03" }}) {{ id month }} }}"#
    );
    let resp = schema.execute(Request::new(assign_mut)).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);

    // Query raw assignments to exercise DataLoaders via nested fields
    let query = r"
        query {
            assignments {
                id
                month
                project { id name }
                resource { id name }
            }
        }
    ";
    let resp = schema.execute(Request::new(query)).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let data = serde_json::to_value(resp.data).unwrap();
    let assigns = data["assignments"].as_array().unwrap();
    assert_eq!(assigns.len(), 1);
    assert_eq!(assigns[0]["project"]["id"].as_str().unwrap(), p_id);
    assert_eq!(assigns[0]["resource"]["id"].as_str().unwrap(), r_id);

    // Query resource with nested assignments
    let q = format!(
        r#"query {{ resource(id: "{r_id}") {{ id name assignments {{ id project {{ id }} month }} }} }}"#
    );
    let resp = schema.execute(Request::new(q)).await;
    assert!(resp.errors.is_empty());
    let data = serde_json::to_value(resp.data).unwrap();
    assert_eq!(data["resource"]["assignments"].as_array().unwrap().len(), 1);

    // Query project with nested assignments
    let q = format!(
        r#"query {{ project(id: "{p_id}") {{ id name assignments {{ id resource {{ id }} month }} }} }}"#
    );
    let resp = schema.execute(Request::new(q)).await;
    assert!(resp.errors.is_empty());
    let data = serde_json::to_value(resp.data).unwrap();
    assert_eq!(data["project"]["assignments"].as_array().unwrap().len(), 1);

    // Query projectMonthMatrix for [2024-03]
    let query = r"
        query($months: [Month!]!) {
            projectMonthMatrix(months: $months) {
                months
                rows { project { id name } cells { resources { id name } } }
            }
        }
    ";
    let vars = Variables::from_json(serde_json::json!({ "months": ["2024-03"] }));
    let resp = schema.execute(Request::new(query).variables(vars)).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let data = serde_json::to_value(resp.data).unwrap();
    let months = data["projectMonthMatrix"]["months"].as_array().unwrap();
    assert_eq!(months.len(), 1);
    assert_eq!(months[0].as_str().unwrap(), "2024-03");
    let rows = data["projectMonthMatrix"]["rows"].as_array().unwrap();
    // Expect a row for every project, and exactly one cell per month
    assert!(!rows.is_empty());
    let mut found = false;
    for row in rows {
        let project_id = row["project"]["id"].as_str().unwrap();
        let cells = row["cells"].as_array().unwrap();
        assert_eq!(cells.len(), 1);
        if project_id == p_id {
            let res_list = cells[0]["resources"].as_array().unwrap();
            // Should include Alice in 2024-03
            assert_eq!(res_list.len(), 1);
            let resource_id = res_list[0]["id"].as_str().unwrap();
            assert_eq!(resource_id, r_id);
            found = true;
        }
    }
    assert!(found, "expected row for project not found");

    // resourceMonthMatrix should include a row for Alice with Alpha in the cell for 2024-03
    let query = r"
        query($months: [Month!]!) {
            resourceMonthMatrix(months: $months) {
                months
                rows { resource { id name } cells { projects { id name } } }
            }
        }
    ";
    let vars = Variables::from_json(serde_json::json!({ "months": ["2024-03"] }));
    let resp = schema.execute(Request::new(query).variables(vars)).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let data = serde_json::to_value(resp.data).unwrap();
    let months = data["resourceMonthMatrix"]["months"].as_array().unwrap();
    assert_eq!(months.len(), 1);
    assert_eq!(months[0].as_str().unwrap(), "2024-03");
    let rows = data["resourceMonthMatrix"]["rows"].as_array().unwrap();
    assert!(!rows.is_empty());
    let mut found = false;
    for row in rows {
        let resource_id = row["resource"]["id"].as_str().unwrap();
        let cells = row["cells"].as_array().unwrap();
        assert_eq!(cells.len(), 1);
        if resource_id == r_id {
            let proj_list = cells[0]["projects"].as_array().unwrap();
            assert_eq!(proj_list.len(), 1);
            let project_id = proj_list[0]["id"].as_str().unwrap();
            assert_eq!(project_id, p_id);
            found = true;
        }
    }
    assert!(found, "expected row for resource not found");

    // Test deleteResource mutation and verify cascading effects
    let del_res = format!(r#"mutation {{ deleteResource(id: "{r_id}") }}"#);
    let resp = schema.execute(Request::new(del_res)).await;
    assert!(resp.errors.is_empty());
    let data = serde_json::to_value(resp.data).unwrap();
    assert!(data["deleteResource"].as_bool().unwrap());

    // The resource query should now return null
    let get_res = format!(r#"query {{ resource(id: "{r_id}") {{ id }} }}"#);
    let resp = schema.execute(Request::new(get_res)).await;
    assert!(resp.errors.is_empty());
    let data = serde_json::to_value(resp.data).unwrap();
    assert!(data.get("resource").unwrap().is_null());

    // Unassign on the same tuple should now return false
    let unassign_mut = format!(
        r#"mutation {{ unassign(input: {{ resourceId: "{r_id}", projectId: "{p_id}", month: "2024-03" }}) }}"#
    );
    let resp = schema.execute(Request::new(unassign_mut)).await;
    assert!(resp.errors.is_empty());
    let data = serde_json::to_value(resp.data).unwrap();
    assert!(!data["unassign"].as_bool().unwrap());

    // Delete project as well
    let del_proj = format!(r#"mutation {{ deleteProject(id: "{p_id}") }}"#);
    let resp = schema.execute(Request::new(del_proj)).await;
    assert!(resp.errors.is_empty());
    let data = serde_json::to_value(resp.data).unwrap();
    assert!(data["deleteProject"].as_bool().unwrap());
}
