use enigo::{Enigo, Key, KeyboardControllable};

pub fn simulate_keyboard_input(text: &str) {
    let mut enigo = Enigo::new();
    let lines: Vec<&str> = text.split('\n').collect();
    let last_index = lines.len() - 1;

    for (i, line) in lines.iter().enumerate() {
        enigo.key_sequence(line);
        if i != last_index {
            enigo.key_click(Key::Return);
        }
    }
}
