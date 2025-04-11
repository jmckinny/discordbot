use axum::routing::get;
use axum::{Router, routing::post};
use serenity::all::Http;
use std::sync::Arc;

use super::routes::{dm_user, health_check};

#[derive(Debug, Clone)]
pub struct ApiState {
    pub discord: Arc<Http>,
}

pub async fn create_app(state: ApiState) -> Router<()> {
    let api_routes = Router::new()
        .route("/health_check", get(health_check))
        .route("/dm_user", post(dm_user));
    Router::new().nest("/api/v1/", api_routes).with_state(state)
}
