use crate::commands::types::{Context, Error};
use ::serenity::all::CreateEmbed;
use poise::CreateReply;
use poise::serenity_prelude::UserId;

/// Show token leaderboard
#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}
