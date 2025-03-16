use crate::commands::types::{Context, Error};
use std::collections::HashSet;
use std::time::Duration;

use crate::utils::tokens::add_tokens;
use crate::utils::wordle;
use rand::seq::IteratorRandom;
use serenity::all::MessageBuilder;
use serenity::collector::MessageCollector;

#[poise::command(slash_command, prefix_command)]
pub async fn wordle(ctx: Context<'_>) -> Result<(), Error> {
    let solution = choose_solution().await;
    let mut game_state = wordle::Game::new(&solution);
    let mut letters_left = HashSet::new();

    let guess_left_mssg = format!("Input guess {} of 6", 6 - game_state.guesses_left() + 1);
    ctx.reply(guess_left_mssg).await?;
    let author_id = ctx.author().id;

    loop {
        // Message listen loop
        let msg_collector = MessageCollector::new(ctx)
            .filter(move |m| m.author.id == author_id)
            .timeout(Duration::from_secs(60 * 2));
        let response = msg_collector.next().await;

        if response.is_none() {
            let timeout_mssg = MessageBuilder::new()
                .push("Wordle game timed out!  Word was ")
                .push_bold(solution)
                .build();
            ctx.reply(timeout_mssg).await?;
            return Ok(());
        }

        let response_msg = response.expect("Unreachable");
        match game_state.guess(&response_msg.content).await {
            Ok(score) => {
                let lower_letters = response_msg.content.to_uppercase();
                let chars_used = lower_letters.chars();
                letters_left.extend(chars_used);
                let msg = format!(
                    "{}\n{}",
                    score,
                    format_keyboard_left(&letters_left, &solution)
                );
                response_msg.reply(&ctx, msg).await?;
            }
            Err(_) => {
                ctx.reply("Invalid guess!").await?;
                continue;
            }
        }

        if game_state.is_game_over() {
            break;
        }
    }

    if game_state.game_won() {
        let tokens_won = (game_state.guesses_left() * 5) + 5;

        let game_won_mssg = format!(
            "You won on try {}\nYou win {} tokens!",
            6 - game_state.guesses_left(),
            tokens_won
        );
        ctx.reply(game_won_mssg).await?;
        add_tokens(ctx, ctx.author(), tokens_won as u64)?;
    } else {
        let lose_mssg = MessageBuilder::new()
            .push("You lost!  The correct word was ")
            .push_bold(solution)
            .build();
        ctx.reply(lose_mssg).await?;
    }

    Ok(())
}

async fn choose_solution() -> String {
    let wordlist = include_str!("../../data/solutions.txt");
    let mut rng = rand::rng();
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

        if letters_used.contains(letter) && solution.contains(&lower) {
            output.push_str(&format!(" **{letter}** "));
        } else if letters_used.contains(letter) {
            output.push_str(&format!(" ~~{letter}~~ "));
        } else {
            output.push_str(&format!(" {letter} "));
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
