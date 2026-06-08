#[cfg(test)]
mod items_api {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::{json, Value};

    use axum_template::{config::AppConfig, routes::build_router, state::AppState};

    fn test_server() -> TestServer {
        let cfg = AppConfig::load().expect("config");
        let app = build_router(AppState::new(cfg));
        TestServer::new(app).expect("test server")
    }

    // ── Health ────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn health_check_should_return_200() {
        let server = test_server();
        let response = server.get("/health").await;
        response.assert_status_ok();
    }

    // ── Create ────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn create_item_should_return_201_with_valid_payload() {
        let server = test_server();
        let response = server
            .post("/api/v1/items")
            .json(&json!({ "name": "Widget" }))
            .await;
        response.assert_status(StatusCode::CREATED);
    }

    #[tokio::test]
    async fn create_item_should_return_422_when_name_is_empty() {
        let server = test_server();
        let response = server
            .post("/api/v1/items")
            .json(&json!({ "name": "" }))
            .await;
        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    // ── Read ──────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_item_should_return_404_for_unknown_id() {
        let server = test_server();
        let response = server
            .get("/api/v1/items/00000000-0000-0000-0000-000000000000")
            .await;
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_item_should_return_200_after_create() {
        let server = test_server();

        let created: Value = server
            .post("/api/v1/items")
            .json(&json!({ "name": "Sprocket", "description": "A fine sprocket" }))
            .await
            .json();

        let id = created["id"].as_str().expect("id field");

        let fetched: Value = server
            .get(&format!("/api/v1/items/{id}"))
            .await
            .json();

        assert_eq!(fetched["id"], created["id"]);
        assert_eq!(fetched["name"], "Sprocket");
    }

    #[tokio::test]
    async fn list_items_should_return_empty_data_initially() {
        let server = test_server();
        let response: Value = server.get("/api/v1/items").await.json();
        assert_eq!(response["total"], 0);
    }

    #[tokio::test]
    async fn update_item_should_return_200_with_new_name() {
        let server = test_server();

        let created: Value = server
            .post("/api/v1/items")
            .json(&json!({ "name": "Old Name" }))
            .await
            .json();

        let id = created["id"].as_str().expect("id field");

        let updated: Value = server
            .put(&format!("/api/v1/items/{id}"))
            .json(&json!({ "name": "New Name" }))
            .await
            .json();

        assert_eq!(updated["name"], "New Name");
    }

    #[tokio::test]
    async fn delete_item_should_return_204_then_404_on_refetch() {
        let server = test_server();

        let created: Value = server
            .post("/api/v1/items")
            .json(&json!({ "name": "Ephemeral" }))
            .await
            .json();

        let id = created["id"].as_str().expect("id field");

        server
            .delete(&format!("/api/v1/items/{id}"))
            .await
            .assert_status(StatusCode::NO_CONTENT);

        server
            .get(&format!("/api/v1/items/{id}"))
            .await
            .assert_status(StatusCode::NOT_FOUND);
    }
}