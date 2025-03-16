mod commands;
mod utils;

use std::collections::HashMap;

use crate::commands::info::info;
use crate::commands::joke::joke;
use crate::commands::tokens::tokens;
use crate::utils::database::load_data;
use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};

use ::serenity::all::UserId;
use dotenv::dotenv;
use std::sync::{Arc, RwLock};
use tracing::info;

pub type Token = u64;
pub type TokenCounter = HashMap<UserId, Token>;

pub struct Data {
    tokens: Arc<RwLock<TokenCounter>>,
} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let prefix_options = PrefixFrameworkOptions {
        prefix: Some(String::from("!")),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![info(), tokens(), joke()],
            prefix_options,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let token_data = load_data().unwrap_or_default();
                let tokens = Arc::new(RwLock::new(token_data));
                let data = Data { tokens };
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    info!("Starting client!");
    client.unwrap().start().await.unwrap();
}
