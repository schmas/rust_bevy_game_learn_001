use crate::GameState;
use bevy::app::App;
use bevy::prelude::{Camera2d, Commands, Msaa, OnEnter, Plugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}
