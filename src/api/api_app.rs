use axum::extract::MatchedPath;
use axum::routing::post;
use axum::{
    Router,
    body::Body,
    http::{Request, Response},
    routing::get,
};
use serenity::all::Http;
use std::{sync::Arc, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{Span, info_span};

use super::routes::{dm_user, health_check};

#[derive(Debug, Clone)]
pub struct ApiState {
    pub discord: Arc<Http>,
}

pub async fn create_app(state: ApiState) -> Router<()> {
    let api_routes = Router::new()
        .route("/health_check", get(health_check))
        .route("/dm_user", post(dm_user))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                    info_span!(
                        "api",
                        method = ?request.method(),
                        matched_path
                    )
                })
                .on_response(on_response)
                .on_failure(on_failure),
        );
    Router::new().nest("/api/v1/", api_routes).with_state(state)
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!("{} in {:?}", response.status(), latency)
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("Request failed: {:?} after {:?}", error, latency)
}
