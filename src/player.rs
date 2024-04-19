use bevy::{input::common_conditions::input_just_released, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    assets::LevelWalls,
    turn::{Turn, TurnEvent, TurnMode},
    AnimationIndices, AnimationTimer, AppState, GameplaySet,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::InGame)
                .load_collection::<PlayerAnimation>(),
        )
        .add_systems(OnEnter(AppState::InGame), patch_players)
        .add_systems(
            Update,
            (move_player_from_input.run_if(in_state(AppState::InGame)))
                .in_set(GameplaySet::InputSet),
        )
        .add_systems(
            Update,
            (update_player_animation, update_player_atlas_index).run_if(in_state(AppState::InGame)),
        )
        .register_type::<PlayerAnimationIndecies>()
        .register_type::<PlayerAnimationState>();
    }
}

#[derive(Default, Component, Reflect)]
pub struct Player;

#[derive(Component, Reflect, Default, PartialEq)]
pub enum PlayerAnimationState {
    #[default]
    IdleDown,
    IdleLeft,
    IdleRight,
    IdleUp,
    WalkingLeft,
    WalkingRight,
    WalkingUp,
    WalkingDown,
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
            AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            player_animation_indices,
            PlayerAnimationState::default(),
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

fn update_player_animation(
    mut query: Query<
        (
            &PlayerAnimationIndecies,
            &mut AnimationTimer,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
    player_states: Query<&PlayerAnimationState, With<Player>>,
    time: Res<Time>,
) {
    let player_state = player_states.iter().next().unwrap();
    for (player_indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match player_state {
                PlayerAnimationState::IdleDown => {
                    atlas.index = if atlas.index == player_indices.idle_down.last {
                        player_indices.idle_down.first
                    } else {
                        atlas.index + 1
                    };
                }
                PlayerAnimationState::IdleLeft => {
                    atlas.index = if atlas.index == player_indices.idle_left.last {
                        player_indices.idle_left.first
                    } else {
                        atlas.index + 1
                    };
                }
                PlayerAnimationState::IdleRight => {
                    atlas.index = if atlas.index == player_indices.idle_right.last {
                        player_indices.idle_right.first
                    } else {
                        atlas.index + 1
                    };
                }
                PlayerAnimationState::IdleUp => {
                    atlas.index = if atlas.index == player_indices.idle_up.last {
                        player_indices.idle_up.first
                    } else {
                        atlas.index + 1
                    };
                }
                PlayerAnimationState::WalkingUp => {
                    atlas.index = if atlas.index == player_indices.up.last {
                        player_indices.up.first
                    } else {
                        atlas.index + 1
                    };
                }
                PlayerAnimationState::WalkingLeft => {
                    atlas.index = if atlas.index == player_indices.left.last {
                        player_indices.left.first
                    } else {
                        atlas.index + 1
                    };
                }
                PlayerAnimationState::WalkingRight => {
                    atlas.index = if atlas.index == player_indices.right.last {
                        player_indices.right.first
                    } else {
                        atlas.index + 1
                    };
                }
                PlayerAnimationState::WalkingDown => {
                    atlas.index = if atlas.index == player_indices.down.last {
                        player_indices.down.first
                    } else {
                        atlas.index + 1
                    };
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
            &PlayerAnimationState,
        ),
        Changed<PlayerAnimationState>,
    >,
) {
    for (player_indices, mut atlas, player_state) in &mut query {
        match player_state {
            PlayerAnimationState::IdleDown => {
                atlas.index = player_indices.idle_down.first;
            }
            PlayerAnimationState::IdleLeft => {
                atlas.index = player_indices.idle_left.first;
            }
            PlayerAnimationState::IdleRight => {
                atlas.index = player_indices.idle_right.first;
            }
            PlayerAnimationState::IdleUp => {
                atlas.index = player_indices.idle_up.first;
            }
            PlayerAnimationState::WalkingUp => {
                atlas.index = player_indices.up.first;
            }
            PlayerAnimationState::WalkingLeft => {
                atlas.index = player_indices.left.first;
            }
            PlayerAnimationState::WalkingRight => {
                atlas.index = player_indices.right.first;
            }
            PlayerAnimationState::WalkingDown => {
                atlas.index = player_indices.down.first;
            }
        }
    }
}

fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_state: Query<&mut PlayerAnimationState, With<Player>>,
    level_walls: Res<LevelWalls>,
    mut ev_turn: EventWriter<TurnEvent>,
    turn: Res<Turn>,
) {
    let mut player_state = player_state.get_single_mut().expect("Player should exist");
    let turn_state = &turn.mode;
    let movement_direction =
        if input.pressed(KeyCode::KeyW) && turn_state != &TurnMode::PlayerAction {
            if *player_state != PlayerAnimationState::WalkingUp {
                *player_state = PlayerAnimationState::WalkingUp;
            }
            ev_turn.send(TurnEvent(TurnMode::PlayerAction));
            GridCoords::new(0, 1)
        } else if input.pressed(KeyCode::KeyA) && turn_state != &TurnMode::PlayerAction {
            if *player_state != PlayerAnimationState::WalkingLeft {
                *player_state = PlayerAnimationState::WalkingLeft;
            }
            ev_turn.send(TurnEvent(TurnMode::PlayerAction));
            GridCoords::new(-1, 0)
        } else if input.pressed(KeyCode::KeyS) && turn_state != &TurnMode::PlayerAction {
            if *player_state != PlayerAnimationState::WalkingDown {
                *player_state = PlayerAnimationState::WalkingDown;
            }
            ev_turn.send(TurnEvent(TurnMode::PlayerAction));
            GridCoords::new(0, -1)
        } else if input.pressed(KeyCode::KeyD) && turn_state != &TurnMode::PlayerAction {
            if *player_state != PlayerAnimationState::WalkingRight {
                *player_state = PlayerAnimationState::WalkingRight;
            }
            ev_turn.send(TurnEvent(TurnMode::PlayerAction));
            GridCoords::new(1, 0)
        } else {
            if input.just_released(KeyCode::KeyW) {
                if *player_state != PlayerAnimationState::IdleUp {
                    *player_state = PlayerAnimationState::IdleUp;
                }
            } else if input.just_released(KeyCode::KeyA) {
                if *player_state != PlayerAnimationState::IdleLeft {
                    *player_state = PlayerAnimationState::IdleLeft;
                }
            } else if input.just_released(KeyCode::KeyD) {
                if *player_state != PlayerAnimationState::IdleRight {
                    *player_state = PlayerAnimationState::IdleRight;
                }
            } else if input.just_released(KeyCode::KeyS) {
                if *player_state != PlayerAnimationState::IdleDown {
                    *player_state = PlayerAnimationState::IdleDown;
                }
            }
            ev_turn.send(TurnEvent(TurnMode::Idle));
            return;
        };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}
