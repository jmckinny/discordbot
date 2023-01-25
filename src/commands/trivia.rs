use serde_json::{Result, Value};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

const API_URL: &str = "https://opentdb.com/api.php?amount=1&type=multiple";

#[command]
pub async fn trivia(ctx: &Context, msg: &Message) -> CommandResult {
    let response: Value = serde_json::from_str(&reqwest::get(API_URL).await?.text().await?)?;
    let question = &response["results"][0]["question"];
    let wrong_answers = &response["results"][0]["incorrect_answers"];
    let correct_answer = &response["results"][0]["correct_answer"];
    let difficulty = &response["results"][0]["difficulty"];
    let category = &response["results"][0]["category"];
    msg.channel_id.say(&ctx.http, question).await?;
    Ok(())
}
