use crate::commands::types::{Context, Error};
use crate::utils::database;
use poise::serenity_prelude as serenity;

/// Query the number of tokens a user has
#[poise::command(slash_command, prefix_command, category = "Utility")]
pub async fn tokens(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let tokens = database::get_tokens(&ctx.data().db, u.id).await?;
    let response = format!("You have {tokens} tokens");
    ctx.say(response).await?;
    Ok(())
}
