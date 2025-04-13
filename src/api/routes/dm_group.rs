use std::sync::Arc;

use crate::api::ApiState;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use tokio::task::JoinSet;

use super::dm_user::send_user_message;

#[derive(Deserialize)]
pub struct DmGroupRequest {
    users: Vec<u64>,
    msg: String,
}

pub async fn dm_group(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(dm_request): Json<DmGroupRequest>,
) -> impl IntoResponse {
    let mut set = JoinSet::new();

    let header_wrapper = Arc::new(headers);
    let msg_wrapper = Arc::new(dm_request.msg);
    for user in dm_request.users {
        set.spawn(send_user_message(
            state.discord.clone(),
            user,
            header_wrapper.clone(),
            msg_wrapper.clone(),
        ));
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(code) => {
                if code != StatusCode::OK {
                    return code;
                }
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
    StatusCode::OK
}
