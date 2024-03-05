use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
    index: i32,
    finish_reason: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_url = "http://localhost:1234/v1/chat/completions";
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "Answer concise and precise".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "Tell me a joke".to_string(),
        },
    ];

    let response = client
        .post(api_url)
        .json(&json!({
            "model": "local-model",
            "messages": messages,
            "tempature": 0.7,
        }))
        .header("Autorization", "Bearer not-needed")
        .send()
        .await?;

    let completion: CompletionResponse = response.json().await?;

    if let Some(choice) = completion.choices.first() {
        println!("Completion: {}", choice.message.content);
    }

    Ok(())
}
