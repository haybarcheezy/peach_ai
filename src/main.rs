use arboard;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime; // Import the Tokio runtime
use winit::event_loop::{ControlFlow, EventLoopBuilder}; // Import the runtime for executing async code
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

// Separate async function for API call
async fn make_api_call() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_url = "http://localhost:1234/v1/chat/completions";
    let clipboard_content =
        get_clipboard_content().unwrap_or_else(|_| "Error accessing clipboard".to_string());
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "Always answer in rhymes.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: clipboard_content, // Get clipboard content
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
    }

    Ok(())
}

// function to access clipboard and return content as string
fn get_clipboard_content() -> Result<String, Box<dyn std::error::Error>> {
    let clipboard = clipboard::ClipboardContext::new()?;
    Ok(clipboard.get_contents()?.to_string())
}

fn main() {
    // Create a new Tokio runtime
    let rt = Runtime::new().unwrap();

    let event_loop = EventLoopBuilder::new().build().unwrap();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    // Define your hotkeys here
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Digit9);

    hotkeys_manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            // Process global hotkey events
            if let Ok(event) = global_hotkey_channel.try_recv() {
                println!("Hotkey pressed: {:?}", event);

                // Check for specific hotkey and its state
                if event.id == hotkey.id() && event.state == HotKeyState::Pressed {
                    // Execute the API call asynchronously
                    rt.spawn(async {
                        make_api_call().await.expect("Failed to make API call");
                    });
                }
            }
        })
        .unwrap();
}
