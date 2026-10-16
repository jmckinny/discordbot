use crate::commands::types::{Context, Error};
use crate::utils::database;
use poise::CreateReply;
use poise::serenity_prelude::CreateEmbed;

/// Show token leaderboard
#[poise::command(slash_command, prefix_command, category = "Tokens")]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let leaderboard = database::list_leadboard(&ctx.data().db).await?;

    let mut lines = Vec::with_capacity(leaderboard.len());
    for (index, (id, tokens)) in leaderboard.iter().enumerate() {
        let name = match ctx.http().get_user(*id).await {
            Ok(user) => user.name.to_string(),
            Err(_) => format!("Unknown user ({id})"),
        };
        lines.push(format!("{}. {} : {} :coin:", index + 1, name, tokens));
    }

    let description = if lines.is_empty() {
        "No one has any tokens yet.".to_string()
    } else {
        lines.join("\n")
    };

    let embed = CreateEmbed::new()
        .title("Leaderboard")
        .description(description);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
