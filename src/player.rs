use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    assets::LevelWalls,
    turn::{FreeWalkEvents, WalkingState},
    AnimationTimer, AppState, GameplaySet, IdleAnimationTimer, IndeciesIter, ACTION_DELAY,
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
            FixedUpdate,
            (move_player_from_input.run_if(
                in_state(AppState::InGame)
                    .and_then(on_timer(Duration::from_secs_f32(ACTION_DELAY))),
            ))
            .in_set(GameplaySet::InputSet),
        )
        .add_systems(
            FixedUpdate,
            (
                update_player_walking_animation,
                update_player_idle_animation,
                update_idle_player_atlas,
            )
                .run_if(in_state(AppState::InGame)),
        )
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

#[derive(Component)]
struct PlayerAnimationIndecies {
    idle_down: IndeciesIter,
    idle_left: IndeciesIter,
    idle_right: IndeciesIter,
    idle_up: IndeciesIter,
    up: IndeciesIter,
    left: IndeciesIter,
    right: IndeciesIter,
    down: IndeciesIter,
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
            AnimationTimer(Timer::from_seconds(
                ACTION_DELAY / 2.0,
                TimerMode::Repeating,
            )),
            IdleAnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            player_animation_indices,
            PlayerAction::default(),
            Direction::default(),
        ));
    }
}

fn patch_player_animation() -> PlayerAnimationIndecies {
    PlayerAnimationIndecies {
        idle_down: vec![0, 1].into(),
        idle_left: vec![144, 145].into(),
        idle_right: vec![48, 49].into(),
        idle_up: vec![96, 97].into(),
        up: vec![98, 99, 100, 99].into(),
        left: vec![146, 147, 148, 147].into(),
        right: vec![50, 51, 52, 51].into(),
        down: vec![2, 3, 4, 3].into(),
    }
}

#[allow(clippy::type_complexity)]
fn update_player_walking_animation(
    mut query: Query<
        (
            &mut PlayerAnimationIndecies,
            &mut AnimationTimer,
            &mut TextureAtlas,
            &Direction,
            &PlayerAction,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    for (mut player_indices, mut timer, mut atlas, player_direction, player_action) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() && PlayerAction::Walking == *player_action {
            match player_direction {
                Direction::Up => {
                    atlas.index = player_indices.up.next().expect("looping iterator");
                }
                Direction::Down => {
                    atlas.index = player_indices.down.next().expect("looping iterator");
                }
                Direction::Left => {
                    atlas.index = player_indices.left.next().expect("looping iterator");
                }
                Direction::Right => {
                    atlas.index = player_indices.right.next().expect("looping iterator");
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_player_idle_animation(
    mut query: Query<
        (
            &mut PlayerAnimationIndecies,
            &mut IdleAnimationTimer,
            &mut TextureAtlas,
            &Direction,
            &PlayerAction,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    for (mut player_indices, mut timer, mut atlas, player_direction, player_action) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() && PlayerAction::Idle == *player_action {
            match player_direction {
                Direction::Up => {
                    atlas.index = player_indices.idle_up.next().expect("looping iterator");
                }
                Direction::Down => {
                    atlas.index = player_indices.idle_down.next().expect("looping iterator");
                }
                Direction::Left => {
                    atlas.index = player_indices.idle_left.next().expect("looping iterator");
                }
                Direction::Right => {
                    atlas.index = player_indices.idle_right.next().expect("looping iterator");
                }
            }
        }
    }
}

fn update_idle_player_atlas(
    mut query: Query<
        (
            &mut PlayerAnimationIndecies,
            &mut TextureAtlas,
            &Direction,
            &PlayerAction,
        ),
        (Changed<PlayerAction>, With<Player>),
    >,
) {
    for (mut player_indices, mut atlas, player_direction, player_action) in &mut query {
        if *player_action == PlayerAction::Idle {
            match player_direction {
                Direction::Up => {
                    atlas.index = player_indices.idle_up.next().expect("looping iterator");
                }
                Direction::Down => {
                    atlas.index = player_indices.idle_down.next().expect("looping iterator");
                }
                Direction::Left => {
                    atlas.index = player_indices.idle_left.next().expect("looping iterator");
                }
                Direction::Right => {
                    atlas.index = player_indices.idle_right.next().expect("looping iterator");
                }
            }
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
                free_walk_ev.send(FreeWalkEvents {
                    walking_state: WalkingState::Idle,
                });
            }
            return;
        };

        if movement_direction != GridCoords::new(0, 0) {
            *player_action = PlayerAction::Walking;
            free_walk_ev.send(FreeWalkEvents {
                walking_state: WalkingState::Walking,
            });
            let destination = *player_grid_coords + movement_direction;
            if !level_walls.in_wall(&destination) {
                *player_grid_coords = destination;
            }
        }
    }
}
