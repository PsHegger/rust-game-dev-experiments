use piston::Button::Keyboard;
use piston::{ButtonArgs, ButtonState, Key, Motion};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct GameInput {
    pressed_keys: HashSet<Key>,
}

#[allow(dead_code)]
impl GameInput {
    pub fn new() -> GameInput {
        GameInput {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn on_key_event(&mut self, event: ButtonArgs) {
        match event.button {
            Keyboard(key) => {
                if event.state == ButtonState::Press {
                    self.pressed_keys.insert(key);
                } else {
                    self.pressed_keys.remove(&key);
                }
            }
            _ => {}
        }
    }

    pub fn on_move_event(&mut self, _event: Motion) {
        // todo
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }
}

pub trait InputHandler {
    fn on_button_event(&mut self, args: ButtonArgs);
}
