use crate::commands::types::{Context, Error};
use reqwest::header::{ACCEPT, USER_AGENT};

const JOKE_API_URL: &str = "https://icanhazdadjoke.com/";
const USER_AGENT_STRING: &str = "frothybot (https://github.com/jmckinny/frothybot)";

/// Tell the user a joke
#[poise::command(slash_command, prefix_command, category = "Games")]
pub async fn joke(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(JOKE_API_URL)
        .header(USER_AGENT, USER_AGENT_STRING)
        .header(ACCEPT, "text/plain")
        .send()
        .await?;
    let mssg = response.text().await?;
    ctx.reply(mssg).await?;
    Ok(())
}
