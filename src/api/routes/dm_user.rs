use crate::api::ApiState;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use serenity::all::{CreateMessage, UserId};
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
    let user_id = UserId::new(dm_request.user);
    let user = match user_id.to_user(state.discord.clone()).await {
        Ok(user) => user,
        Err(_) => return StatusCode::NOT_FOUND,
    };
    let agent = headers
        .get("user-agent")
        .map_or("Unkown", |h| h.to_str().unwrap_or("Unkown"));
    let from_agent = format!("# Service: {}", agent);
    let msg_str = format!("{}\n{}", from_agent, dm_request.msg);
    let discord_msg = CreateMessage::new().content(msg_str);
    let status_result = match user_id.direct_message(state.discord, discord_msg).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    info!("Service {:?} sent DM to {:?}", agent, user.name);
    status_result
}
