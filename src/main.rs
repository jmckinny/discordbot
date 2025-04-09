mod api;
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
use crate::commands::types::Error;
use crate::commands::weather::weather;
use crate::commands::wordle::wordle;
use crate::utils::database::connect_to_db;
use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};

use ::serenity::all::{ActivityData, UserId};
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
            on_error: |error| Box::pin(on_error(error)),
            pre_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "User {:?} is executing {:?}",
                        ctx.author().name,
                        ctx.command().qualified_name
                    );
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "User {:?} executed command {:?}",
                        ctx.author().name,
                        ctx.command().qualified_name
                    );
                })
            },
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
        .activity(ActivityData::custom("By your command"))
        .framework(framework)
        .await
        .expect("Failed to create client");

    tokio::spawn(async move {
        let api = api::create_app().await;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
            .await
            .expect("Failed to start listener for API");
        axum::serve(listener, api)
            .await
            .expect("Failed to start API service");
    });

    info!("Starting client!");
    if let Err(why) = client.start().await {
        error!("ERROR: start failed due to {:?}", why);
    }
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}
