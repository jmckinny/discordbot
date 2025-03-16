use crate::{
    Token,
    commands::types::{Context, Error},
};
use poise::serenity_prelude as serenity;

pub fn get_tokens(ctx: Context<'_>, user: &serenity::User) -> Token {
    let data = ctx.data();
    let token_count = data.tokens.get(&user.id);
    *token_count.unwrap_or(&0u64)
}
pub async fn add_tokens(ctx: Context<'_>, user: Option<serenity::User>) {}
