use axum::Router;
use axum::routing::get;

pub async fn create_app() -> Router<()> {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
