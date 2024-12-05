use crate::actions::game_control::GameControl;
use crate::loading::{GameManager, TextureAssets};
use crate::obstacle::{spawn_obstacles, OBSTACLE_HEIGHT, OBSTACLE_WIDTH};
use crate::{obstacle::Obstacle, GameState, PIXEL_RATIO};
use bevy::prelude::*;

const FLAP_FORCE: f32 = 500.;
const GRAVITY: f32 = 2000.;
const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub velocity: f32,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, update_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.bird.clone()),
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Player { velocity: 0. },
    ));
}

fn update_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Obstacle>>,
    mut obstacle_query: Query<(&Transform, Entity), With<Obstacle>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
    textures: Res<TextureAssets>,
) {
    if let Ok((mut player, mut player_transform)) = player_query.get_single_mut() {
        if GameControl::Space.just_pressed(&keys) {
            player.velocity = FLAP_FORCE;
        }

        player.velocity -= time.delta_secs() * GRAVITY;
        player_transform.translation.y += player.velocity * time.delta_secs();

        player_transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(player.velocity / VELOCITY_TO_ROTATION_RATIO, -90., 90.).to_radians(),
        );

        let mut dead = false;
        if player_transform.translation.y <= -game_manager.window_dimensions.y / 2. {
            dead = true;
        } else {
            for (pipe_transform, _entity) in obstacle_query.iter() {
                //collision check
                if (pipe_transform.translation.y - player_transform.translation.y).abs()
                    < OBSTACLE_HEIGHT * PIXEL_RATIO / 2.
                    && (pipe_transform.translation.x - player_transform.translation.x).abs()
                        < OBSTACLE_WIDTH * PIXEL_RATIO / 2.
                {
                    dead = true;
                    break;
                }
            }
        }
        if dead {
            player_transform.translation = Vec3::ZERO;
            player.velocity = 0.;
            for (_pipe_transform, entity) in obstacle_query.iter_mut() {
                commands.entity(entity).despawn();
            }

            spawn_obstacles(&mut commands, game_manager, textures);
        }
    }
}
