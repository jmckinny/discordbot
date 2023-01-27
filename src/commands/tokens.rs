use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::TokenCounter;

#[command]
#[aliases("token", "t")]
pub async fn tokens(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let token_counter = data
        .get::<TokenCounter>()
        .expect("Failed to find TokenCounter in TypeMap");
    if let Some(v) = token_counter.get(&msg.author.id) {
        msg.reply(&ctx, format!("You have {v} tokens")).await?;
    } else {
        msg.reply(&ctx, "You have no tokens").await?;
    }

    Ok(())
}
