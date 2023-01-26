use reqwest::header::{ACCEPT, USER_AGENT};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

const JOKE_API_URL: &str = "https://icanhazdadjoke.com/";
const USER_AGENT_STRING: &str = "frothybot (https://github.com/jmckinny/frothybot)";
#[command]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let response = client
        .get(JOKE_API_URL)
        .header(USER_AGENT, USER_AGENT_STRING)
        .header(ACCEPT, "text/plain")
        .send()
        .await?;
    msg.reply(&ctx.http, response.text().await?).await?;
    Ok(())
}
