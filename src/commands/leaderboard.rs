use crate::commands::types::{Context, Error};
use ::serenity::all::CreateEmbed;
use poise::CreateReply;
use poise::serenity_prelude::UserId;

/// Show token leaderboard
#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let token_counter = ctx
        .data()
        .tokens
        .read()
        .expect("Failed to get token read lock")
        .clone();

    let mut sorted: Vec<(&UserId, &u64)> = token_counter.iter().collect();
    sorted.sort_by(|(_, tokens1), (_, tokens2)| tokens2.cmp(tokens1));

    let mut leaderboard_fields = Vec::new();
    for (index, (id, tokens)) in sorted.iter().enumerate() {
        let user = ctx
            .http()
            .get_user(**id)
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
