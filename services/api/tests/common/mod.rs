use axum::Router;
use flowmason_db::connection::create_pool;

pub async fn create_test_app() -> Router {
    let database_url = "sqlite::memory:";
    let pool = create_pool(database_url).await.expect("Failed to create test database");
    flowmason_api::routes::create_router(pool).await
}

pub async fn create_test_app() -> Router {
    let database_url = "sqlite::memory:";
    let pool = create_pool(database_url).await.expect("Failed to create test database");
    create_router(pool).await
}

pub async fn create_test_user_and_token() -> (String, String) {
    // This would create a test user and return (user_id, token)
    // For now, return placeholder
    ("test_user_id".to_string(), "test_token".to_string())
}
