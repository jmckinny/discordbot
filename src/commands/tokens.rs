use crate::commands::types::{Context, Error};
use crate::utils::tokens::get_tokens;
use poise::serenity_prelude as serenity;

/// Query the number of tokens a user has
#[poise::command(slash_command, prefix_command)]
pub async fn tokens(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let tokens = get_tokens(ctx, u);
    let response = format!("You have {tokens} tokens");
    ctx.say(response).await?;
    Ok(())
}
