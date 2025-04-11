use crate::api::ApiState;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use serenity::all::{CreateMessage, UserId};

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
    let agent = headers
        .get("user-agent")
        .map_or("Unkown", |h| h.to_str().unwrap_or("Unkown"));
    let from_agent = format!("# Service: {}", agent);
    let msg_str = format!("{}\n{}", from_agent, dm_request.msg);
    let discord_msg = CreateMessage::new().content(msg_str);
    match user_id.direct_message(state.discord, discord_msg).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
}
