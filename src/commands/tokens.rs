use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::tokens::get_tokens;

#[command]
#[aliases("token", "t")]
pub async fn tokens(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(tokens) = get_tokens(ctx, msg.author.id).await?{
        msg.reply(&ctx, format!("You have {tokens} tokens")).await?;
    }else{
        msg.reply(&ctx, "You have no tokens").await?;
    }
    Ok(())
}
