use serenity::{
    framework::standard::{CommandError, CommandResult},
    model::prelude::UserId,
    prelude::Context,
};

use crate::TokenCounter;
type TokenResult<T = Option<u64>> = std::result::Result<T, CommandError>;

pub async fn add_tokens(ctx: &Context, user: UserId, amount: u64) -> CommandResult {
    let mut data = ctx.data.write().await;
    let token_counter = data
        .get_mut::<TokenCounter>()
        .expect("Expected TokenCounter in TypeMap");

    if let Some(v) = token_counter.get_mut(&user) {
        *v += amount;
    } else {
        token_counter.insert(user, amount);
    }
    Ok(())
}
/// Returns the amount of tokens removed or None if not enough tokens
pub async fn remove_tokens(ctx: &Context, user: UserId, amount: u64) -> TokenResult {
    let mut data = ctx.data.write().await;
    let token_counter = data
        .get_mut::<TokenCounter>()
        .expect("Expected TokenCounter in TypeMap");

    if let Some(v) = token_counter.get_mut(&user) {
        if *v < amount {
            return Ok(None);
        }
        *v -= amount;
        Ok(Some(*v))
    } else {
        Ok(None)
    }
}

pub async fn get_tokens(ctx: &Context, user: UserId) -> TokenResult {
    let data = ctx.data.read().await;
    let token_counter = data
        .get::<TokenCounter>()
        .expect("Failed to find TokenCounter in TypeMap");
    let tokens = token_counter.get(&user);
    Ok(tokens.copied())
}
