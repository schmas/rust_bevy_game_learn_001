use crate::loading::{GameManager, TextureAssets};
use crate::{GameState, PIXEL_RATIO};
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

pub const OBSTACLE_AMOUNT: usize = 5;
pub const OBSTACLE_WIDTH: f32 = 32.;
pub const OBSTACLE_HEIGHT: f32 = 144.;
pub const OBSTACLE_VERTICAL_OFFSET: f32 = 30.;
pub const OBSTACLE_GAP_SIZE: f32 = 15.;
pub const OBSTACLE_SPACING: f32 = 60.;
pub const OBSTACLE_SCROLL_SPEED: f32 = 150.;

pub struct ObstaclePlugin;

#[derive(Component)]
pub struct Obstacle {
    pipe_direction: f32,
}

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_obstacles_system)
            .add_systems(
                Update,
                update_obstacles.run_if(in_state(GameState::Playing)),
            );
    }
}

pub fn spawn_obstacles_system(
    mut commands: Commands,
    game_manager: Res<GameManager>,
    textures: Res<TextureAssets>,
) {
    spawn_obstacles(&mut commands, game_manager, textures);
}

pub fn spawn_obstacles(
    commands: &mut Commands,
    game_manager: Res<GameManager>,
    textures: Res<TextureAssets>,
) {
    // let commands = &mut commands;
    let window_width = game_manager.window_dimensions.x;
    let rand: &mut ThreadRng = &mut thread_rng();

    for i in 0..OBSTACLE_AMOUNT {
        let y_offset = generate_offset(rand);
        let x_pos = window_width / 2. + (OBSTACLE_SPACING * PIXEL_RATIO * i as f32);
        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (get_centered_pipe_position() + y_offset),
            1.,
            commands,
            &textures,
        );

        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (-get_centered_pipe_position() + y_offset),
            -1.,
            commands,
            &textures,
        );
    }
}

fn get_centered_pipe_position() -> f32 {
    (OBSTACLE_HEIGHT / 2. + OBSTACLE_GAP_SIZE) * PIXEL_RATIO
}

fn generate_offset(rand: &mut ThreadRng) -> f32 {
    rand.gen_range(-OBSTACLE_VERTICAL_OFFSET..OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO
}

fn spawn_obstacle(
    translation: Vec3,
    //bottom or top of screen
    pipe_direction: f32,
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
) {
    commands.spawn((
        Sprite::from_image(textures.pipe.clone()),
        Transform::from_translation(translation).with_scale(Vec3::new(
            PIXEL_RATIO,
            PIXEL_RATIO * -pipe_direction,
            PIXEL_RATIO,
        )),
        Obstacle { pipe_direction },
    ));
}

fn update_obstacles(
    time: Res<Time>,
    game_manager: Res<GameManager>,
    mut obstacle_query: Query<(&mut Obstacle, &mut Transform)>,
) {
    let mut rand = thread_rng();
    let y_offset = generate_offset(&mut rand);
    for (obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= time.delta_secs() * OBSTACLE_SCROLL_SPEED;

        if transform.translation.x + OBSTACLE_WIDTH * PIXEL_RATIO / 2.
            < -game_manager.window_dimensions.x / 2.
        {
            transform.translation.x += OBSTACLE_AMOUNT as f32 * OBSTACLE_SPACING * PIXEL_RATIO;
            transform.translation.y =
                get_centered_pipe_position() * obstacle.pipe_direction + y_offset;
        }
    }
}
