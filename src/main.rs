mod commands;
mod utils;
use crate::commands::info::*;
use crate::commands::joke::*;
use crate::commands::leaderboard::*;
use crate::commands::ping::*;
use crate::commands::slots::*;
use crate::commands::tokens::*;
use crate::commands::trivia::*;
use crate::commands::weather::*;
use crate::commands::wordle::*;
use crate::utils::database;

use dotenv::dotenv;
use serenity::framework::standard::macros::help;
use serenity::model::prelude::UserId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::sync::Arc;

use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::macros::hook;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        if Path::new("database.json").exists() {
            info!("Loading database");
            let mut gaurd = ctx.data.write().await;
            let tokens = gaurd
                .get_mut::<TokenCounter>()
                .expect("Failed to load token count from context");
            if let Ok(data) = database::load_data() {
                tokens.extend(data.iter());
            } else {
                info!("Failed to load database");
            }
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await?;
    Ok(())
}

#[group]
#[commands(ping, info, trivia, joke, weather, tokens, leaderboard, slots, wordle)]
struct General;

struct TokenCounter;

impl TypeMapKey for TokenCounter {
    type Value = HashMap<UserId, u64>;
}

//Hooks
#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );
    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    info!("Could not find command named '{}'", unknown_command_name);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("ERROR: Failed to load token");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {why:?}"),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!").case_insensitivity(true))
        .after(after)
        .before(before)
        .help(&MY_HELP)
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .type_map_insert::<TokenCounter>(HashMap::default())
        .await
        .expect("ERROR: failed to create client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();
    {
        let guard = client.data.clone();

        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Could not register ctrl+c handler");
            shard_manager.lock().await.shutdown_all().await;
            let lock = guard.read().await;
            let token_count = lock
                .get::<TokenCounter>()
                .expect("Failed to get TokenCounter");

            database::save_data(token_count.clone()).expect("Failed to save database on shutdown");
        });
    }

    if let Err(why) = client.start().await {
        error!("ERROR: start failed due to {:?}", why);
    }
}
