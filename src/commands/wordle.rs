use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use crate::utils::tokens::add_tokens;
use crate::utils::wordle::{self};
use rand::seq::IteratorRandom;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
pub async fn wordle(ctx: &Context, msg: &Message) -> CommandResult {
    let solution = choose_solution().await;
    let mut game_state = wordle::Game::new(&solution);
    let mut letters_left = HashSet::new();

    while !game_state.is_game_over() {
        let guess_left_mssg = format!("Input guess {} of 6", 6 - game_state.guesses_left() + 1);
        msg.reply(&ctx, guess_left_mssg).await?;

        if let Some(response) = collect_response(ctx, msg).await? {
            match game_state.guess(&response.content).await {
                Ok(data) => {
                    let lower_letters = response.content.to_uppercase();
                    let chars_used = lower_letters.chars();
                    letters_left.extend(chars_used);
                    response
                        .reply(
                            ctx,
                            format!(
                                "{}\n{}",
                                data,
                                format_keyboard_left(&letters_left, &solution)
                            ),
                        )
                        .await?;
                }
                Err(_) => {
                    response.reply(ctx, "Invalid guess!").await?;
                    continue;
                }
            }
        } else {
            let timeout_mssg = MessageBuilder::new()
                .push("Wordle game timed out!  Word was ")
                .push_bold(solution)
                .build();
            msg.reply(ctx, timeout_mssg).await?;
            return Ok(());
        }
    }

    if game_state.game_won() {
        let tokens_won = (game_state.guesses_left() * 5) + 5;

        let game_won_mssg = format!(
            "You won on try {}\nYou win {} tokens!",
            6 - game_state.guesses_left(),
            tokens_won
        );
        msg.reply(ctx, game_won_mssg).await?;
        add_tokens(ctx, msg.author.id, tokens_won as u64).await?;
    } else {
        let lose_mssg = MessageBuilder::new()
            .push("You lost!  The correct word was ")
            .push_bold(solution)
            .build();
        msg.reply(ctx, lose_mssg).await?;
    }

    Ok(())
}

type ResponseResult = Result<Option<Arc<Message>>, CommandError>;

async fn collect_response(ctx: &Context, msg: &Message) -> ResponseResult {
    if let Some(answer) = &msg
        .author
        .await_reply(ctx)
        .timeout(Duration::from_secs(60 * 2))
        .await
    {
        return Ok(Some(answer.clone()));
    }
    Ok(None)
}

async fn choose_solution() -> String {
    let wordlist = include_str!("../../data/solutions.txt");
    let mut rng = rand::thread_rng();
    let solution = wordlist.lines().choose(&mut rng).unwrap();
    solution.to_string()
}

fn format_keyboard_left(letters_used: &HashSet<char>, solution: &str) -> String {
    const LETTERS: [char; 26] = [
        'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K',
        'L', 'Z', 'X', 'C', 'V', 'B', 'N', 'M',
    ];
    let mut output = String::new();
    for (i, letter) in LETTERS.iter().enumerate() {
        let lower: String = letter.to_lowercase().collect();
        if letters_used.contains(&letter) && solution.contains(&lower) {
            output.push_str(&format!(" {letter} "));
        } else if letters_used.contains(&letter) {
            output.push_str(&format!(" ~~{letter}~~ "));
        } else {
            output.push_str(&format!(" **{letter}** "));
        }
        match i {
            // Querty top layer is 10 keys long (10-1 == 9)
            9 => output.push('\n'),
            // Querty second layer is 9 (9 + 9 == 18)
            18 => output.push_str("\n   "),
            _ => continue,
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_koala() {
        let guess = "koala";
        let solution = "vodka";
        let mut game = wordle::Game::new(solution);

        let response = tokio_test::block_on(game.guess(guess)).unwrap();
        assert_eq!(response.to_string(), "🟨🟩🟥🟥🟩")
    }

    #[test]
    fn test_generic() {
        let guess = "crane";
        let solution = "thorn";
        let mut game = wordle::Game::new(solution);

        let response = tokio_test::block_on(game.guess(guess)).unwrap();
        assert_eq!(response.to_string(), "🟥🟨🟥🟨🟥")
    }

    #[test]
    fn test_generic2() {
        let guess = "apple";
        let solution = "pains";
        let mut game = wordle::Game::new(solution);

        let response = tokio_test::block_on(game.guess(guess)).unwrap();
        assert_eq!(response.to_string(), "🟨🟨🟥🟥🟥")
    }

    #[test]
    fn test_generic3() {
        let guess = "koala";
        let solution = "raise";
        let mut game = wordle::Game::new(solution);

        let response = tokio_test::block_on(game.guess(guess)).unwrap();
        assert_eq!(response.to_string(), "🟥🟥🟨🟥🟥")
    }

    #[test]
    fn test_generic4() {
        let guess = "leeks";
        let solution = "still";
        let mut game = wordle::Game::new(solution);

        let response = tokio_test::block_on(game.guess(guess)).unwrap();
        assert_eq!(response.to_string(), "🟨🟥🟥🟥🟨")
    }
}
