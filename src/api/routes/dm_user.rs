use crate::api::ApiState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
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
    Json(dm_request): Json<DmRequest>,
) -> impl IntoResponse {
    let user_id = UserId::new(dm_request.user);
    let discord_msg = CreateMessage::new().content(dm_request.msg);
    match user_id.direct_message(state.discord, discord_msg).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
}
