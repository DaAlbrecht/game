use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    events::TurnOver, get_single_mut, grid::Collider, ldtk::LevelWalls, ActionTimer,
    AnimationTimer, AppState, Health, IdleAnimationTimer, IndeciesIter, ACTION_DELAY,
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
            (
                update_player_walking_animation,
                update_player_idle_animation,
                update_idle_player_atlas,
                update_player_position,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .register_type::<Direction>()
        .register_type::<PlayerAction>()
        .register_type::<Health>();
    }
}

#[derive(Default, Component, Reflect)]
pub struct Player;

#[derive(Event, Default)]
pub struct PlayerMove(pub GridCoords);

#[derive(Component, Default, PartialEq, Debug, Reflect)]
enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl TryFrom<GridCoords> for Direction {
    type Error = ();

    fn try_from(value: GridCoords) -> Result<Self, Self::Error> {
        match value {
            GridCoords { x: 0, y: 1 } => Ok(Direction::Up),
            GridCoords { x: 0, y: -1 } => Ok(Direction::Down),
            GridCoords { x: -1, y: 0 } => Ok(Direction::Left),
            GridCoords { x: 1, y: 0 } => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Component, Default, PartialEq, Reflect, Debug)]
pub enum PlayerAction {
    #[default]
    Idle,
    Walking,
    Combat,
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
        tile_size_x = 16,
        tile_size_y = 16,
        columns = 24,
        rows = 8,
        padding_x = 16,
        padding_y = 16,
        offset_x = 8,
        offset_y = 8
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
            ActionTimer(Timer::from_seconds(ACTION_DELAY, TimerMode::Repeating)),
            IdleAnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            player_animation_indices,
            PlayerAction::default(),
            Direction::default(),
            Health::default(),
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

#[allow(clippy::type_complexity)]
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

fn update_player_position(
    mut players: Query<(&mut GridCoords, &mut Direction, &mut PlayerAction), With<Player>>,
    mut move_direction_er: EventReader<PlayerMove>,
    mut action_timer: Query<&mut ActionTimer, With<Player>>,
    colliders: Query<&Collider, Without<Player>>,
    mut turn_over_ew: EventWriter<TurnOver>,
    level_walls: Res<LevelWalls>,
    time: Res<Time>,
) {
    let (mut player_pos, mut player_direction, mut player_action) = get_single_mut!(players);

    let mut action_timer = get_single_mut!(action_timer);

    action_timer.tick(time.delta());

    let event = move_direction_er.read().next();
    let move_direction = match event {
        Some(PlayerMove(direction)) => *direction,
        None => return,
    };

    if *player_action == PlayerAction::Combat {
        return;
    }

    //If the player was idling, we want to start walking immediately and not wait for the action timer to finish
    if *player_action == PlayerAction::Idle {
        *player_direction = if let Ok(direction) = Direction::try_from(move_direction) {
            direction
        } else {
            return;
        };

        *player_action = PlayerAction::Walking;
        turn_over_ew.send(TurnOver(PlayerAction::Walking));

        let destination = *player_pos + move_direction;

        for collider in colliders.iter() {
            if collider
                .get_occupied_coords()
                .iter()
                .any(|&coords| coords == destination)
            {
                return;
            }
        }

        if !level_walls.in_wall(&destination) {
            *player_pos = destination;
        }

        //reset the action timer to prevent the player from moving twice, if the action timer would
        //finish right after the player started moving from idle
        action_timer.reset();
    }

    if action_timer.finished() {
        match Direction::try_from(move_direction) {
            Ok(direction) => {
                *player_direction = direction;

                *player_action = PlayerAction::Walking;
                turn_over_ew.send(TurnOver(PlayerAction::Walking));

                let destination = *player_pos + move_direction;

                for collider in colliders.iter() {
                    if collider
                        .get_occupied_coords()
                        .iter()
                        .any(|&coords| coords == destination)
                    {
                        return;
                    }
                }

                if !level_walls.in_wall(&destination) {
                    *player_pos = destination;
                }
            }
            Err(_) => {
                if *player_action != PlayerAction::Idle {
                    *player_action = PlayerAction::Idle;
                    turn_over_ew.send(TurnOver(PlayerAction::Idle));
                }
            }
        }
    }
}
