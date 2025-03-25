use crate::commands::types::{Context, Error};
use crate::utils::database;
use ::serenity::all::CreateEmbed;
use poise::CreateReply;

/// Show token leaderboard
#[poise::command(slash_command, prefix_command, category = "Utility")]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let leaderboard = database::list_leadboard(&ctx.data().db).await?;

    let mut leaderboard_fields = Vec::new();
    for (index, (id, tokens)) in leaderboard.iter().enumerate() {
        let user = ctx
            .http()
            .get_user(*id)
            .await
            .expect("Failed to lookup user");
        leaderboard_fields.push(format!("{}. {} : {} :coin:", index + 1, user.name, tokens));
    }

    let leaderboard_fields: Vec<(String, String, bool)> = leaderboard_fields
        .into_iter()
        .map(|x| (x, "".to_string(), false))
        .collect();

    let embed = CreateEmbed::new()
        .title("Leaderboard")
        .fields(leaderboard_fields);

    let reply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };
    ctx.send(reply).await?;
    Ok(())
}
