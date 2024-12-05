use bevy::prelude::{ButtonInput, KeyCode, Res};

pub enum GameControl {
    Space,
}

impl GameControl {
    pub fn just_pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            GameControl::Space => keyboard_input.just_pressed(KeyCode::Space),
        }
    }
}
