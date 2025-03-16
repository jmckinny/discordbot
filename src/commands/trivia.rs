use crate::commands::types::{Context, Error};
use std::time::Duration;

use crate::utils::tokens::*;
use ::serenity::all::EditMessage;
use poise::{CreateReply, serenity_prelude as serenity};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::all::{ButtonStyle, CreateActionRow, ReactionType};
use serenity::builder::CreateButton;
use serenity::utils::MessageBuilder;

const API_URL: &str = "https://opentdb.com/api.php?amount=1&type=multiple";

const EASY_REWARD: u64 = 1;
const MEDIUM_REWARD: u64 = 3;
const HARD_REWARD: u64 = 5;

/// Start a trivia game
#[poise::command(slash_command, prefix_command)]
pub async fn trivia(ctx: Context<'_>) -> Result<(), Error> {
    let response: Response = serde_json::from_str(&reqwest::get(API_URL).await?.text().await?)?;
    let trivia_question = response.to_message();

    let buttons = vec![
        (create_index_button("1", "1️⃣".parse().unwrap())),
        (create_index_button("2", "2️⃣".parse().unwrap())),
        (create_index_button("3", "3️⃣".parse().unwrap())),
        (create_index_button("4", "4️⃣".parse().unwrap())),
    ];
    let row = CreateActionRow::Buttons(buttons);
    let components = vec![row];
    let trivia_message = ctx
        .send(
            CreateReply::default()
                .content(&trivia_question.formated_message)
                .components(components),
        )
        .await?;
    let resp = serenity::ComponentInteractionCollector::new(ctx)
        .timeout(Duration::from_secs(10))
        .channel_id(ctx.channel_id())
        .await;

    if resp.is_none() {
        let timeout_msg = MessageBuilder::new()
            .push_line("Timed out!")
            .push("Correct answer was: ")
            .push(trivia_question.correct_answer)
            .build();
        ctx.reply(timeout_msg).await?;
        trivia_message
            .edit(
                ctx,
                CreateReply::default()
                    .content(&trivia_question.formated_message)
                    .components(vec![]),
            )
            .await?;
        return Ok(());
    }

    let mci = resp.unwrap();
    let answer = &mci.data.custom_id;

    let mut original_msg = mci.message.clone();
    let msg_data = MessageBuilder::new()
        .push_line(trivia_question.formated_message)
        .push_line("")
        .push(&ctx.author().name)
        .push(" answered: ")
        .push_bold(
            &trivia_question.all_answers[answer
                .parse::<usize>()
                .expect("Attempted to parse invalid index")
                - 1],
        )
        .build();
    original_msg
        .edit(ctx, EditMessage::new().content(msg_data).components(vec![]))
        .await?;

    mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
        .await?;

    if answer == &trivia_question.correct_index.to_string() {
        let reward = match trivia_question.difficulty {
            Difficulty::Easy => EASY_REWARD,
            Difficulty::Medium => MEDIUM_REWARD,
            Difficulty::Hard => HARD_REWARD,
        };

        add_tokens(ctx, ctx.author(), reward)?;

        let reply = MessageBuilder::new()
            .push_line("That's correct!")
            .push_line(format!("You recieved {reward} tokens"))
            .build();
        ctx.reply(reply).await?;
    } else {
        let response = MessageBuilder::new()
            .push_line("Wrong.")
            .push("The correct answer was: ")
            .push(trivia_question.correct_answer)
            .build();
        ctx.reply(response).await?;
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    response_code: i32,
    results: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    category: String,
    #[serde(alias = "type")]
    question_type: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}
enum Difficulty {
    Easy,
    Medium,
    Hard,
}
struct TriviaQuestion {
    formated_message: String,
    correct_index: usize,
    correct_answer: String,
    all_answers: Vec<String>,
    difficulty: Difficulty,
}

impl Response {
    fn to_message(&self) -> TriviaQuestion {
        let question = html_escape::decode_html_entities(&self.results[0].question);
        let category = html_escape::decode_html_entities(&self.results[0].category);
        let correct_answer = html_escape::decode_html_entities(&self.results[0].correct_answer);
        let difficulty = html_escape::decode_html_entities(&self.results[0].difficulty);
        let mut incorret_answers = self.results[0]
            .incorrect_answers
            .iter()
            .map(|text| html_escape::decode_html_entities(text).to_string())
            .collect::<Vec<String>>();
        let mut answers = vec![correct_answer.to_string()];
        answers.append(&mut incorret_answers);

        let mut rng = rand::rng();
        answers.shuffle(&mut rng);

        let choices = answers
            .iter()
            .enumerate()
            .map(|(i, x)| format!("{}. {}", i + 1, x))
            .collect::<Vec<String>>()
            .join("\n");

        let mut correct_index = 0;
        for (i, s) in answers.iter().enumerate() {
            if s == &correct_answer {
                correct_index = i + 1;
                break;
            }
        }

        let formated_message = MessageBuilder::new()
            .push("Question: ")
            .push_bold_line(question)
            .push("Category: ")
            .push_bold_line(category)
            .push("Difficulty: ")
            .push_bold_line(difficulty.as_ref())
            .push(choices)
            .build();

        let difficulty = match difficulty.as_ref() {
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            _ => {
                panic!("Invalid difficulty")
            }
        };

        TriviaQuestion {
            formated_message,
            correct_index,
            correct_answer: correct_answer.to_string(),
            all_answers: answers,
            difficulty,
        }
    }
}

fn create_index_button(name: &str, emoji: ReactionType) -> CreateButton {
    CreateButton::new(name)
        .custom_id(name)
        .label("")
        .style(ButtonStyle::Primary)
        .emoji(emoji)
}
