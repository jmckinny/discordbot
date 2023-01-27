use std::time::Duration;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::builder::CreateButton;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::interaction::InteractionResponseType::UpdateMessage;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::TokenCounter;

const API_URL: &str = "https://opentdb.com/api.php?amount=1&type=multiple";

const EASY_REWARD: u64 = 1;
const MEDIUM_REWARD: u64 = 3;
const HARD_REWARD: u64 = 5;

#[command]
pub async fn trivia(ctx: &Context, msg: &Message) -> CommandResult {
    let response: Response = serde_json::from_str(&reqwest::get(API_URL).await?.text().await?)?;
    let trivia_question = response.to_message();

    let m = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.content(&trivia_question.formated_message);
            m.components(|c| {
                c.create_action_row(|row| {
                    row.add_button(create_index_button("one", "1️⃣".parse().unwrap()));
                    row.add_button(create_index_button("two", "2️⃣".parse().unwrap()));
                    row.add_button(create_index_button("three", "3️⃣".parse().unwrap()));
                    row.add_button(create_index_button("four", "4️⃣".parse().unwrap()))
                })
            })
        })
        .await?;
    let user = msg.author.id;
    let interaction = match m
        .await_component_interaction(ctx)
        .filter(move |f| f.user.id == user)
        .timeout(Duration::from_secs(10))
        .await
    {
        Some(x) => x,
        None => {
            let timeout_msg = MessageBuilder::new()
                .push_line("Timed out!")
                .push("Correct answer was: ")
                .push(trivia_question.correct_answer)
                .build();
            msg.reply(&ctx, timeout_msg).await?;
            return Ok(());
        }
    };

    let answer = &interaction.data.custom_id;

    interaction
        .create_interaction_response(&ctx, |r| {
            r.kind(UpdateMessage).interaction_response_data(|d| {
                d.content(
                    MessageBuilder::new()
                        .push_line(trivia_question.formated_message)
                        .push_line("")
                        .push(&msg.author.name)
                        .push(" answered: ")
                        .push_bold(&trivia_question.all_answers[string_to_index(answer) - 1])
                        .build(),
                );
                d.components(|c| c)
            })
        })
        .await?;

    if answer == &index_to_string(trivia_question.correct_index) {
        let reward = match trivia_question.difficulty {
            Difficulty::Easy => EASY_REWARD,
            Difficulty::Medium => MEDIUM_REWARD,
            Difficulty::Hard => HARD_REWARD,
        };

        let mut data = ctx.data.write().await;
        let token_counter = data
            .get_mut::<TokenCounter>()
            .expect("Expected TokenCounter in TypeMap");

        if let Some(v) = token_counter.get_mut(&msg.author.id) {
            *v += reward;
        } else {
            token_counter.insert(msg.author.id, reward);
        }
        let reply = MessageBuilder::new()
            .push_line("That's correct!")
            .push_line(format!("You recieved {reward} tokens"))
            .build();
        msg.reply(ctx, reply).await?;
    } else {
        let response = MessageBuilder::new()
            .push_line("Wrong.")
            .push("The correct answer was: ")
            .push(trivia_question.correct_answer)
            .build();
        msg.reply(ctx, response).await?;
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

        let mut rng = rand::thread_rng();
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
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.label("");
    b.style(ButtonStyle::Primary);
    b.emoji(emoji);
    b
}

fn index_to_string(index: usize) -> String {
    match index {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        _ => panic!("Invalid answer index"),
    }
}

fn string_to_index(input: &str) -> usize {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        _ => panic!("Invalid index"),
    }
}
