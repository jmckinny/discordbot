mod commands;
mod utils;

use std::collections::HashMap;

use crate::commands::info::info;
use crate::commands::joke::joke;
use crate::commands::leaderboard::leaderboard;
use crate::commands::slots::slots;
use crate::commands::tokens::tokens;
use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};

use ::serenity::all::UserId;
use dotenvy::dotenv;
use std::sync::{Arc, RwLock};
use tracing::{error, info};
use utils::database;

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

    let token_data = database::load_data().unwrap_or_default();
    let token_ref = Arc::new(RwLock::new(token_data));
    let token_ref_setup = token_ref.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![info(), tokens(), joke(), leaderboard(), slots()],
            prefix_options,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let data = Data {
                    tokens: token_ref_setup,
                };
                Ok(data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
        let token_data = token_ref
            .clone()
            .read()
            .expect("Failed to aquire token read lock")
            .clone();
        database::save_data(token_data).expect("Failed to save database");
    });

    info!("Starting client!");
    if let Err(why) = client.start().await {
        error!("ERROR: start failed due to {:?}", why);
    }
}
