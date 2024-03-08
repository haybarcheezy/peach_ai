use crate::keyboard::simulate_keyboard_input;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
    index: i32,
    finish_reason: String,
}

pub async fn simple_api_call(text: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_url = "http://localhost:1234/v1/chat/completions";

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "Be concise and accurate.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: text.to_string(),
        },
    ];

    let response = client
        .post(api_url)
        .json(&json!({
            "model": "local-model",
            "messages": messages,
            "temperature": 0.7,
        }))
        .header("Authorization", "Bearer not-needed")
        .send()
        .await?;

    let completion_response: CompletionResponse = response.json().await?;

    if let Some(choice) = completion_response.choices.first() {
        println!("{}", choice.message.content);

        simulate_keyboard_input(&choice.message.content);
    }

    Ok(())
}

pub async fn context_api_call(command: &str, context: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_url = "http://localhost:1234/v1/chat/completions";
    let command_and_context = format!("{}, use this as context: {}", command, context);
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "Be concise and accurate.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: command_and_context.to_string(),
        },
    ];

    let response = client
        .post(api_url)
        .json(&json!({
            "model": "local-model",
            "messages": messages,
            "temperature": 0.7,
        }))
        .header("Authorization", "Bearer not-needed")
        .send()
        .await?;

    let completion_response: CompletionResponse = response.json().await?;

    if let Some(choice) = completion_response.choices.first() {
        println!("{}", choice.message.content);

        simulate_keyboard_input(&choice.message.content);
    }

    Ok(())
}
