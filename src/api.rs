use crate::keyboard::simulate_keyboard_input;
use base64::encode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

pub async fn image_api_call(command: &str, image_path: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_url = "http://localhost:1234/v1/chat/completions";
    let trimmed_path = image_path.trim_matches('"');
    let path = Path::new(trimmed_path);
    println!("Path: {:?}", path);

    // Read and encode the image to base64
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("Couldn't open the file.");
            return Ok(());
        }
    };
    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).is_err() {
        println!("Failed to read the file.");
        return Ok(());
    }
    let base64_image = encode(buffer);

    // Construct the JSON payload with the command and the encoded image
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "This is a chat between a user and an assistant. The assistant is helping the user with a task with the image as context.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: json!([
                {"type": "text", "text": command},
                {"type": "image_url", "image_url": {"url": format!("data:image/jpeg;base64,{}", base64_image)}},
            ]).to_string(),
        },
    ];

    // Send the request to the API
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

    // Process the response
    if let Some(choice) = completion_response.choices.first() {
        println!("{}", choice.message.content);
        simulate_keyboard_input(&choice.message.content);
    }

    Ok(())
}
