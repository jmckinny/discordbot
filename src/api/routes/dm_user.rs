use std::sync::Arc;

use crate::api::ApiState;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use serenity::all::{CreateMessage, Http, UserId};
use tracing::info;

#[derive(Deserialize)]
pub struct DmRequest {
    user: u64,
    msg: String,
}

pub async fn dm_user(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(dm_request): Json<DmRequest>,
) -> impl IntoResponse {
    send_user_message(
        state.discord,
        dm_request.user,
        Arc::new(headers),
        Arc::new(dm_request.msg),
    )
    .await
}

pub async fn send_user_message(
    discord: Arc<Http>,
    user_id: u64,
    headers: Arc<HeaderMap>,
    msg: Arc<String>,
) -> StatusCode {
    let user_id = UserId::new(user_id);
    let user = match user_id.to_user(discord.clone()).await {
        Ok(user) => user,
        Err(_) => return StatusCode::NOT_FOUND,
    };
    let agent_name = headers
        .get("user-agent")
        .map_or("Unkown", |h| h.to_str().unwrap_or("Unkown"));
    let from_agent = format!("# Service: {}", agent_name);
    let msg_str = format!("{}\n{}", from_agent, msg);
    let discord_msg = CreateMessage::new().content(msg_str);
    let status_result = match user.direct_message(discord, discord_msg).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    info!("Service {:?} sent DM to {:?}", agent_name, user.name);
    status_result
}
