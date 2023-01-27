use reqwest::header::USER_AGENT;
use serde_json::Value;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

const COLLEGE_PARK_WEATHER_API: &str = "https://api.weather.gov/gridpoints/LWX/99,76/forecast";
const BOT_USER_AGENT: &str = "frothybot (https://github.com/jmckinny/frothybot)";

#[command]
pub async fn weather(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let response = client
        .get(COLLEGE_PARK_WEATHER_API)
        .header(USER_AGENT, BOT_USER_AGENT)
        .send()
        .await?;
    let json_response: Value = response.json().await?;

    let current_period = &json_response["properties"]["periods"][0];

    let temprature = &current_period["temperature"];
    let temperature_unit = &current_period["temperatureUnit"]
        .to_string()
        .replace('"', "");
    let forecast = &current_period["shortForecast"].to_string().replace('"', "");
    let icon_url = &current_period["icon"].to_string().replace('"', "");

    let weather_message = MessageBuilder::new()
        .push_line("Weather for College Park MD")
        .push_bold_line(format!("{temprature}°{temperature_unit} {forecast}"))
        .build();

    msg.channel_id
        .send_message(&ctx, |m| {
            m.content(weather_message).add_embed(|e| e.image(icon_url))
        })
        .await?;
    Ok(())
}
