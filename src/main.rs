use enigo::{Enigo, KeyboardControllable};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};

use std::{sync::mpsc, thread, time::Duration};
use tokio::runtime::Runtime;
use tray_item::{IconSource, TrayItem};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
mod clipboard_ops;
use clipboard_ops::{get_clipboard_content, set_clipboard_content};
mod api;
use api::{context_api_call, image_api_call, simple_api_call};
mod keyboard;

enum Message {
    Quit,
}

async fn handle_highlighted_text() -> Result<(), Box<dyn std::error::Error>> {
    let original_clipboard_content = get_clipboard_content()?;

    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{CTRL}c");

    thread::sleep(Duration::from_secs(1));

    let clipboard_content = get_clipboard_content()?;

    simple_api_call(&clipboard_content).await?;
    set_clipboard_content(&original_clipboard_content)?;

    Ok(())
}

async fn handle_highlighted_text_with_context() -> Result<(), Box<dyn std::error::Error>> {
    let context = get_clipboard_content()?;

    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{CTRL}c");

    thread::sleep(Duration::from_secs(1));
    let command = get_clipboard_content()?;
    context_api_call(&command, &context).await?;
    set_clipboard_content(&context)?;

    Ok(())
}

async fn handle_highlighted_text_with_image() -> Result<(), Box<dyn std::error::Error>> {
    let image_path = get_clipboard_content()?;

    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{CTRL}c");

    thread::sleep(Duration::from_secs(1));
    let command = get_clipboard_content()?;
    image_api_call(&command, &image_path).await?;
    set_clipboard_content(&image_path)?;

    Ok(())
}

fn main() {
    let mut tray = TrayItem::new("PeachAI", IconSource::Resource("peach_icon")).unwrap();

    let (tx, rx) = mpsc::channel();

    // Quit
    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    thread::spawn(move || {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit action triggered");
                std::process::exit(0);
            }
            Err(e) => {
                println!("Error receiving message: {}", e);
                // Handle error or ignore
            }
        }
    });
    // Create a new Tokio runtime
    let rt = Runtime::new().unwrap();

    let event_loop = EventLoopBuilder::new().build().unwrap();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    // Define your hotkeys here
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Digit9);
    let context_hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Digit8);
    let image_hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Digit7);

    hotkeys_manager.register(hotkey).unwrap();
    hotkeys_manager.register(context_hotkey).unwrap();
    hotkeys_manager.register(image_hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            // global hotkey events
            while let Ok(event) = global_hotkey_channel.try_recv() {
                println!("Hotkey pressed: {:?}", event);

                // hotkey matching
                match event {
                    _ if event.id == hotkey.id() && event.state == HotKeyState::Pressed => {
                        rt.spawn(async {
                            handle_highlighted_text()
                                .await
                                .expect("Failed to make API call");
                        });
                    }
                    _ if event.id == context_hotkey.id() && event.state == HotKeyState::Pressed => {
                        rt.spawn(async {
                            handle_highlighted_text_with_context()
                                .await
                                .expect("Failed to make API call");
                        });
                    }
                    _ if event.id == image_hotkey.id() && event.state == HotKeyState::Pressed => {
                        rt.spawn(async {
                            handle_highlighted_text_with_image()
                                .await
                                .expect("Failed to make API call");
                        });
                    }
                    _ => {} // Default case
                }
            }
        })
        .unwrap();
}
