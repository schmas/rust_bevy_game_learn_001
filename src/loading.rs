use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<TextureAssets>(),
        )
        .add_systems(Startup, setup_game_manager);
    }
}

fn setup_game_manager(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let game_manager = GameManager {
        window_dimensions: Vec2::new(window.width(), window.height()),
    };
    commands.insert_resource(game_manager);
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bird.png")]
    pub bird: Handle<Image>,

    #[asset(path = "textures/pipe.png")]
    pub pipe: Handle<Image>,
}

#[derive(Resource)]
pub struct GameManager {
    pub window_dimensions: Vec2,
}
