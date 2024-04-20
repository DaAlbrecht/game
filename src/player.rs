use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    assets::LevelWalls, turn::FreeWalkEvents, AnimationIndices, AnimationTimer, AppState,
    GameplaySet,
};

pub struct PlayerPlugin;

const PLAYER_ACTION_DELAY: f32 = 0.15;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::InGame)
                .load_collection::<PlayerAnimation>(),
        )
        .add_systems(OnEnter(AppState::InGame), patch_players)
        .add_systems(
            FixedUpdate,
            (move_player_from_input.run_if(
                in_state(AppState::InGame)
                    .and_then(on_timer(Duration::from_secs_f32(PLAYER_ACTION_DELAY))),
            ))
            .in_set(GameplaySet::InputSet),
        )
        .add_systems(
            Update,
            (
                update_player_animation.after(GameplaySet::InputSet),
                update_player_atlas_index.after(GameplaySet::InputSet),
            )
                .run_if(in_state(AppState::InGame)),
        )
        .register_type::<PlayerAnimationIndecies>()
        .register_type::<PlayerAction>();
    }
}

#[derive(Default, Component, Reflect)]
pub struct Player;

#[derive(Component, Default, PartialEq, Debug)]
enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Component, Default, PartialEq, Reflect, Debug)]
enum PlayerAction {
    #[default]
    Idle,
    Walking,
}

#[derive(Component, Reflect)]
struct PlayerAnimationIndecies {
    idle_down: AnimationIndices,
    idle_left: AnimationIndices,
    idle_right: AnimationIndices,
    idle_up: AnimationIndices,
    up: AnimationIndices,
    left: AnimationIndices,
    right: AnimationIndices,
    down: AnimationIndices,
}

#[derive(Component, Deref, DerefMut)]
struct PlayerIdleAnimationTimer(Timer);

#[derive(AssetCollection, Resource)]
struct PlayerAnimation {
    #[asset(texture_atlas_layout(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 24,
        rows = 8,
        padding_x = 16.,
        padding_y = 16.,
        offset_x = 8.,
        offset_y = 8.
    ))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "puny_characters/human_worker_red.png")]
    texture: Handle<Image>,
}

fn patch_players(
    mut commands: Commands,
    asset: Res<PlayerAnimation>,
    mut player_query: Query<(Entity, &mut TextureAtlas, &mut Handle<Image>), With<Player>>,
) {
    for (entity, mut atlas, mut texture) in &mut player_query {
        let player_animation_indices = patch_player_animation();
        atlas.layout = asset.layout.clone();
        *texture = asset.texture.clone();
        commands.entity(entity).insert((
            AnimationTimer(Timer::from_seconds(
                PLAYER_ACTION_DELAY / 3.0,
                TimerMode::Repeating,
            )),
            PlayerIdleAnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            player_animation_indices,
            PlayerAction::default(),
            Direction::default(),
        ));
    }
}

fn patch_player_animation() -> PlayerAnimationIndecies {
    PlayerAnimationIndecies {
        idle_down: AnimationIndices { first: 0, last: 1 },
        idle_left: AnimationIndices {
            first: 144,
            last: 145,
        },
        idle_right: AnimationIndices {
            first: 48,
            last: 49,
        },
        idle_up: AnimationIndices {
            first: 96,
            last: 97,
        },
        up: AnimationIndices {
            first: 98,
            last: 100,
        },
        left: AnimationIndices {
            first: 146,
            last: 149,
        },
        right: AnimationIndices {
            first: 50,
            last: 52,
        },
        down: AnimationIndices { first: 2, last: 4 },
    }
}

#[allow(clippy::type_complexity)]
fn update_player_animation(
    mut query: Query<
        (
            &PlayerAnimationIndecies,
            &mut AnimationTimer,
            &mut PlayerIdleAnimationTimer,
            &mut TextureAtlas,
            &Direction,
            &PlayerAction,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    for (player_indices, mut timer, mut idle_timer, mut atlas, player_direction, player_action) in
        &mut query
    {
        timer.tick(time.delta());
        idle_timer.tick(time.delta());
        match player_action {
            PlayerAction::Idle => {
                if idle_timer.finished() {
                    match player_direction {
                        Direction::Up => {
                            atlas.index = if atlas.index == player_indices.idle_up.last {
                                player_indices.idle_up.first
                            } else {
                                atlas.index + 1
                            };
                        }
                        Direction::Down => {
                            atlas.index = if atlas.index == player_indices.idle_down.last {
                                player_indices.idle_down.first
                            } else {
                                atlas.index + 1
                            };
                        }
                        Direction::Left => {
                            atlas.index = if atlas.index == player_indices.idle_left.last {
                                player_indices.idle_left.first
                            } else {
                                atlas.index + 1
                            };
                        }
                        Direction::Right => {
                            atlas.index = if atlas.index == player_indices.idle_right.last {
                                player_indices.idle_right.first
                            } else {
                                atlas.index + 1
                            };
                        }
                    }
                }
            }
            PlayerAction::Walking => {
                if timer.just_finished() {
                    match player_direction {
                        Direction::Up => {
                            atlas.index = if atlas.index == player_indices.up.last {
                                player_indices.up.first
                            } else {
                                atlas.index + 1
                            };
                        }
                        Direction::Down => {
                            atlas.index = if atlas.index == player_indices.down.last {
                                player_indices.down.first
                            } else {
                                atlas.index + 1
                            };
                        }
                        Direction::Left => {
                            atlas.index = if atlas.index == player_indices.left.last {
                                player_indices.left.first
                            } else {
                                atlas.index + 1
                            };
                        }
                        Direction::Right => {
                            atlas.index = if atlas.index == player_indices.right.last {
                                player_indices.right.first
                            } else {
                                atlas.index + 1
                            };
                        }
                    }
                }
            }
        }
    }
}

fn update_player_atlas_index(
    mut query: Query<
        (
            &PlayerAnimationIndecies,
            &mut TextureAtlas,
            &Direction,
            &PlayerAction,
        ),
        Changed<PlayerAction>,
    >,
) {
    for (player_indices, mut atlas, player_direction, player_action) in &mut query {
        match player_action {
            PlayerAction::Idle => match player_direction {
                Direction::Up => {
                    atlas.index = player_indices.idle_up.first;
                }
                Direction::Down => {
                    atlas.index = player_indices.idle_down.first;
                }
                Direction::Left => {
                    atlas.index = player_indices.idle_left.first;
                }
                Direction::Right => {
                    atlas.index = player_indices.idle_right.first;
                }
            },
            PlayerAction::Walking => match player_direction {
                Direction::Up => {
                    atlas.index = player_indices.up.first;
                }
                Direction::Down => {
                    atlas.index = player_indices.down.first;
                }
                Direction::Left => {
                    atlas.index = player_indices.left.first;
                }
                Direction::Right => {
                    atlas.index = player_indices.right.first;
                }
            },
        }
    }
}

#[allow(clippy::type_complexity)]
fn move_player_from_input(
    mut players: Query<(&mut GridCoords, &mut Direction, &mut PlayerAction), With<Player>>,
    mut free_walk_ev: EventWriter<FreeWalkEvents>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    for (mut player_grid_coords, mut player_direction, mut player_action) in players.iter_mut() {
        let movement_direction = if input.pressed(KeyCode::KeyW) {
            *player_direction = Direction::Up;
            GridCoords::new(0, 1)
        } else if input.pressed(KeyCode::KeyA) {
            *player_direction = Direction::Left;
            GridCoords::new(-1, 0)
        } else if input.pressed(KeyCode::KeyS) {
            *player_direction = Direction::Down;
            GridCoords::new(0, -1)
        } else if input.pressed(KeyCode::KeyD) {
            *player_direction = Direction::Right;
            GridCoords::new(1, 0)
        } else {
            if *player_action != PlayerAction::Idle {
                *player_action = PlayerAction::Idle;
            }
            return;
        };

        if movement_direction != GridCoords::new(0, 0) {
            *player_action = PlayerAction::Walking;
            free_walk_ev.send(FreeWalkEvents);
            let destination = *player_grid_coords + movement_direction;
            if !level_walls.in_wall(&destination) {
                *player_grid_coords = destination;
            }
        }
    }
}
