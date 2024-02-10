use reqwest::header::USER_AGENT;
use serde_json::Value;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

const WTTR_WEATHER_API: &str = "https://wttr.in/";
const BOT_USER_AGENT: &str = "frothybot (https://github.com/jmckinny/frothybot)";

#[command]
pub async fn weather(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let req_url = format!("{}Silver+Spring", WTTR_WEATHER_API);
    let response = client
        .get(req_url)
        .header(USER_AGENT, BOT_USER_AGENT)
        .send()
        .await?;
    let content = response.text().await?;

    msg.channel_id
        .send_message(&ctx, |m| m.content(content))
        .await?;
    Ok(())
}
