use std::time::Duration;

use crate::utils::tokens::add_tokens;
use crate::utils::wordle;
use rand::seq::IteratorRandom;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn wordle(ctx: &Context, msg: &Message) -> CommandResult {
    let solution = choose_word();
    let mut game_state = wordle::Game::new(&solution);

    while !game_state.is_game_over() {
        let guess_left_mssg = format!("Input guess {} of 6", 6 - game_state.guesses_left() + 1);
        msg.reply(&ctx, guess_left_mssg).await?;

        if let Some(response) = collect_response(ctx, msg).await? {
            match game_state.guess(&response).await {
                Ok(data) => {
                    msg.reply(ctx, format!("{}", data)).await?;
                }
                Err(_) => {
                    msg.reply(ctx, "Invalid guess!").await?;
                    continue;
                }
            }
        } else {
            msg.reply(ctx, "Wordle game timed out!").await?;
            return Ok(());
        }
    }

    if game_state.game_won() {
        let tokens_won = game_state.guesses_left() * 5;

        let game_won_mssg = format!(
            "You won on try {}\nYou win {} tokens!",
            6 - game_state.guesses_left() + 1,
            tokens_won
        );
        msg.reply(ctx, game_won_mssg).await?;
        add_tokens(ctx, msg.author.id, tokens_won as u64).await?;
    } else {
        msg.reply(
            ctx,
            format!("You lost!  The correct word was '{}'", solution),
        )
        .await?;
    }

    Ok(())
}

type ResponseResult = Result<Option<String>, CommandError>;

async fn collect_response(ctx: &Context, msg: &Message) -> ResponseResult {
    if let Some(answer) = &msg
        .author
        .await_reply(ctx)
        .timeout(Duration::from_secs(60))
        .await
    {
        return Ok(Some(answer.content.to_lowercase()));
    }
    Ok(None)
}

fn choose_word() -> String {
    let mut rng = rand::thread_rng();
    let wordlist = std::fs::read_to_string("worldlist.txt").expect("Failed to load word list");
    let solution = wordlist.lines().choose(&mut rng).unwrap();
    solution.to_string()
}
