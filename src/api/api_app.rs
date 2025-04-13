use axum::extract::MatchedPath;
use axum::routing::post;
use axum::{
    Router,
    body::Body,
    http::{Request, Response},
    routing::get,
};
use serenity::all::{Cache, Http};
use serenity::prelude::TypeMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{Span, info, info_span};

use super::routes::{dm_group, dm_user, health_check};

#[derive(Clone)]
pub struct ApiState {
    pub discord: Arc<Http>,
    pub discord_cache: Arc<Cache>,
    pub state: Arc<RwLock<TypeMap>>,
}

async fn create_app(state: ApiState) -> Router<()> {
    let api_routes = Router::new()
        .route("/health_check", get(health_check))
        .route("/dm_user", post(dm_user))
        .route("/dm_group", post(dm_group))
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

pub async fn start_api(discord: Arc<Http>, cache: Arc<Cache>, state: Arc<RwLock<TypeMap>>) {
    start_api_on_socket(discord, cache, state, "127.0.0.1:5000").await;
}

pub async fn start_api_on_socket(
    discord: Arc<Http>,
    cache: Arc<Cache>,
    state: Arc<RwLock<TypeMap>>,
    socket: &str,
) {
    let api_state = ApiState {
        discord,
        discord_cache: cache,
        state,
    };
    let api = create_app(api_state).await;
    let listener = tokio::net::TcpListener::bind(socket)
        .await
        .expect("Failed to start listener for API");
    info!("API Listening on http://{}", socket);
    axum::serve(listener, api)
        .await
        .expect("Failed to start API service");
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!("{} in {:?}", response.status(), latency)
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("Request failed: {:?} after {:?}", error, latency)
}
