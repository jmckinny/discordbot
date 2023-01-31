use serenity::{framework::standard::CommandResult, model::prelude::UserId, prelude::Context};

use crate::TokenCounter;

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
