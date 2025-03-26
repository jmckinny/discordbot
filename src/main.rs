mod commands;
mod utils;

use std::collections::HashMap;

use crate::commands::age::age;
use crate::commands::help::help;
use crate::commands::info::info;
use crate::commands::joke::joke;
use crate::commands::leaderboard::leaderboard;
use crate::commands::slots::slots;
use crate::commands::tokens::tokens;
use crate::commands::trivia::trivia;
use crate::commands::weather::weather;
use crate::commands::wordle::wordle;
use crate::utils::database::connect_to_db;
use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};

use ::serenity::all::UserId;
use dotenvy::dotenv;
use sqlx::SqlitePool;
use tracing::{error, info};

pub type Token = u64;
pub type TokenCounter = HashMap<UserId, Token>;

pub struct Data {
    db: SqlitePool,
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

    let sqlite_db = connect_to_db()
        .await
        .expect("Failed to open database connection");
    sqlx::migrate!("./migrations")
        .run(&sqlite_db)
        .await
        .expect("Failed to migrate DB");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                help(),
                age(),
                info(),
                tokens(),
                joke(),
                leaderboard(),
                slots(),
                weather(),
                wordle(),
                trivia(),
            ],
            prefix_options,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let data = Data { db: sqlite_db };
                Ok(data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    info!("Starting client!");
    if let Err(why) = client.start().await {
        error!("ERROR: start failed due to {:?}", why);
    }
}
