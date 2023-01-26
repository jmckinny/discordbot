use std::time::Duration;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

const API_URL: &str = "https://opentdb.com/api.php?amount=1&type=multiple";

#[command]
pub async fn trivia(ctx: &Context, msg: &Message) -> CommandResult {
    let response: Response = serde_json::from_str(&reqwest::get(API_URL).await?.text().await?)?;
    let trivia_question = response.to_message();
    msg.channel_id
        .say(&ctx.http, trivia_question.formated_message)
        .await?;

    if let Some(answer) = &msg
        .author
        .await_reply(ctx)
        .timeout(Duration::from_secs(10))
        .await
    {
        if let Ok(n) = answer.content.to_lowercase().parse::<usize>() {
            if n == trivia_question.correct_index {
                answer.reply(ctx, "That's correct!").await?;
            } else {
                let response = MessageBuilder::new()
                    .push_line("Wrong.")
                    .push("The correct answer was: ")
                    .push(trivia_question.correct_answer)
                    .build();
                answer.reply(ctx, response).await?;
            }
        } else {
            let response = MessageBuilder::new()
                .push_line("Invalid Response.")
                .push("The correct answer was: ")
                .push(trivia_question.correct_answer)
                .build();
            answer.reply(ctx, response).await?;
        }
    } else {
        let response = MessageBuilder::new()
            .push_line("No answer within 10 seconds.")
            .push("The correct answer was: ")
            .push(trivia_question.correct_answer)
            .build();
        msg.reply(ctx, response).await?;
    };
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

struct TriviaQuestion {
    formated_message: String,
    correct_index: usize,
    correct_answer: String,
}

impl Response {
    fn to_message(&self) -> TriviaQuestion {
        let question = html_escape::decode_html_entities(&self.results[0].question);
        let category = html_escape::decode_html_entities(&self.results[0].category);
        let correct_answer = html_escape::decode_html_entities(&self.results[0].correct_answer);
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
            .push(choices)
            .build();

        TriviaQuestion {
            formated_message,
            correct_index,
            correct_answer: correct_answer.to_string(),
        }
    }
}
