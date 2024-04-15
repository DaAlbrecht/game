use std::time::Duration;

use bevy::{core::FrameCount, prelude::*, time::common_conditions::on_timer};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{assets::LevelWalls, AppState, GameplaySet};

const INDEX_IDLE :usize = 0;
const INDEX_UP : usize = 98;
const INDEX_LEFT : usize = 146;
const INDEX_REIGHT : usize = 51;
const INDEX_DOWN : usize = 3;
const FRAME_IDLE : usize = 2;
const FRAME_WALKING : usize = 3;


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
            (move_player_from_input.run_if(on_timer(std::time::Duration::from_millis(100))))
                .in_set(GameplaySet::InputSet),
        )
        .add_systems(
            Update,
            (update_player_animation).run_if(in_state(AppState::InGame)),
        )
        .register_type::<PlayerWalkingState>();
    }
}

#[derive(Default, Component, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
pub enum PlayerWalkingState {    
    Idle{index: usize, frame_count: usize},
    WalkingLeft{index: usize, frame_count: usize},
    WalkingRight{index: usize, frame_count: usize},
    WalkingUp{index: usize, frame_count: usize},
    WalkingDown{index: usize, frame_count: usize},
}


#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}

#[derive(AssetCollection, Resource)]
struct PlayerAnimation {
    #[asset(texture_atlas_layout(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 24,
        rows = 8,
        padding_x = 16.,
        padding_y = 8.,
        offset_x = 8.,
        offset_y = 8.
    ))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "puny_characters/human_worker_red.png")]
    sprite: Handle<Image>,
}

fn patch_players(
    mut commands: Commands,
    asset: Res<PlayerAnimation>,
    mut player_query: Query<(Entity, &mut TextureAtlas, &mut Handle<Image>), With<Player>>,
) {
    for (entity, mut atlas, mut texture) in &mut player_query {
        atlas.layout = asset.layout.clone();
        *texture = asset.sprite.clone();
        commands.entity(entity).insert((
            AnimationTimer {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                frame_count: 2,
            },
            PlayerWalkingState::Idle{index: INDEX_IDLE, frame_count:FRAME_IDLE},
        ));
    }
}

fn update_player_animation(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    player_states: Query<&PlayerWalkingState, With<Player>>,
    time: Res<Time>,
) {
    let player_state = player_states.iter().next().unwrap();
    for (mut sprite, mut animation) in &mut sprites {
        match player_state {
            PlayerWalkingState::Idle{index, frame_count} => {                
                animation.timer.tick(time.delta());
                if sprite.index != *index {
                    sprite.index = *index;
                }
                if animation.timer.just_finished() {
                    sprite.index += 1;
                    if sprite.index >= sprite.index + *frame_count {
                        sprite.index = *index;
                    }
                }
            }
            PlayerWalkingState::WalkingUp{index, frame_count} => {
                animation.timer.tick(time.delta());
                if sprite.index != *index {
                    sprite.index = *index;
                }
                if animation.timer.just_finished() {
                    sprite.index += 1;
                    if sprite.index >= sprite.index + *frame_count {
                        sprite.index = *index;
                    }
                }
            }        
            PlayerWalkingState::WalkingLeft{index, frame_count} => {
                animation.timer.tick(time.delta());
                if sprite.index != *index {
                    sprite.index = *index;
                }
                if animation.timer.just_finished() {
                    sprite.index += 1;
                    if sprite.index >= sprite.index + *frame_count {
                        sprite.index = *index;
                    }
                }
            },
            PlayerWalkingState::WalkingRight{index, frame_count} => {
                animation.timer.tick(time.delta());
                if sprite.index != *index {
                    sprite.index = *index;
                }
                if animation.timer.just_finished() {
                    sprite.index += 1;
                    if sprite.index >= sprite.index + *frame_count {
                        sprite.index = *index;
                    }
                }
            },
            PlayerWalkingState::WalkingDown{index, frame_count} => {
                animation.timer.tick(time.delta());
                if sprite.index != *index {
                    sprite.index = *index;
                }
                if animation.timer.just_finished() {
                    sprite.index += 1;
                    if sprite.index >= sprite.index + *frame_count {
                        sprite.index = *index;
                    }
                }
            }
        }            
    }
}


fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
    mut player_state: Query<&mut PlayerWalkingState, With<Player>>,
) {
    let mut player_state = player_state.get_single_mut().expect("Player should exist");
    let movement_direction = if input.pressed(KeyCode::KeyW) {
        *player_state = PlayerWalkingState::WalkingUp{index: INDEX_IDLE, frame_count:FRAME_IDLE};
        GridCoords::new(0, 1)
    } else if input.pressed(KeyCode::KeyA) {
        *player_state = PlayerWalkingState::WalkingLeft{index: INDEX_IDLE, frame_count:FRAME_IDLE};
        GridCoords::new(-1, 0)
    } else if input.pressed(KeyCode::KeyS) {
        *player_state = PlayerWalkingState::WalkingDown{index: INDEX_IDLE, frame_count:FRAME_IDLE};
        GridCoords::new(0, -1)
    } else if input.pressed(KeyCode::KeyD) {
        *player_state = PlayerWalkingState::WalkingRight{index: INDEX_IDLE, frame_count:FRAME_IDLE};
        GridCoords::new(1, 0)
    } else {
        *player_state = PlayerWalkingState::Idle{index: INDEX_IDLE, frame_count:FRAME_IDLE};
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}
