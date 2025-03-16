use crate::commands::types::{Context, Error};
use poise::CreateReply;
use reqwest::header::USER_AGENT;
use serde_json::Value;
use serenity::all::{CreateEmbed, MessageBuilder};

const COLLEGE_PARK_WEATHER_API: &str = "https://api.weather.gov/gridpoints/LWX/99,76/forecast";
const BOT_USER_AGENT: &str = "frothybot (https://github.com/jmckinny/frothybot)";

#[poise::command(slash_command, prefix_command)]
pub async fn weather(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(COLLEGE_PARK_WEATHER_API)
        .header(USER_AGENT, BOT_USER_AGENT)
        .send()
        .await?;
    let response_text = response.text().await?;
    let json_response: Value = serde_json::from_str(&response_text)?;

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
    let embed = CreateEmbed::default()
        .title("Weather")
        .image(icon_url)
        .field(weather_message, "", false);
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
