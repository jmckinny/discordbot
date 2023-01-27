use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::TokenCounter;

#[command]
pub async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let token_counter = data
        .get::<TokenCounter>()
        .expect("Failed to find TokenCounter in TypeMap");
    let mut sorted: Vec<(&UserId, &u64)> = token_counter.into_iter().collect();
    sorted.sort_by(|(_, tokens1), (_, tokens2)| tokens2.cmp(tokens1));

    let mut leaderboard_fields = Vec::new();
    for (index, (id, tokens)) in sorted.iter().enumerate() {
        let user = ctx.http.get_user(*id.as_u64()).await?;
        leaderboard_fields.push(format!("{}. {} : {} tokens", index + 1, user.name, tokens));
    }

    let leaderboard_fields: Vec<(String, String, bool)> = leaderboard_fields
        .into_iter()
        .map(|x| (x, "".to_string(), false))
        .collect();

    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.title("Leaderboard").fields(leaderboard_fields))
        })
        .await?;

    Ok(())
}
