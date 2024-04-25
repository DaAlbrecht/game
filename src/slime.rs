use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use rand::Rng;

use crate::{
    assets::LevelWalls,
    turn::{FreeWalkEvents, WalkingState},
    AnimationTimer, AppState, IdleAnimationTimer, IndeciesIter, ACTION_DELAY,
};

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::InGame)
                .load_collection::<SlimeAnimation>(),
        )
        .add_systems(OnEnter(AppState::InGame), patch_slime)
        .add_systems(
            FixedUpdate,
            (
                update_slime_idle_animation,
                update_slime_walking_animation,
                update_slime_atlas_index,
                move_slime,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .register_type::<SlimeAnimationState>();
    }
}

#[derive(Default, Component, Reflect)]
pub struct Slime;

#[derive(Component, Reflect, Default, PartialEq, Debug)]
pub enum SlimeAnimationState {
    #[default]
    Idle,
    Walking,
}

#[derive(Component)]
struct SlimeAnimationIndecies {
    idle: IndeciesIter,
    walking: IndeciesIter,
}

#[derive(Component, Deref, DerefMut)]
struct SlimeIdleAnimationTimer(Timer);

#[derive(AssetCollection, Resource)]
struct SlimeAnimation {
    #[asset(texture_atlas_layout(
        tile_size_x = 18.,
        tile_size_y = 17.,
        columns = 15,
        rows = 1,
        padding_x = 14.,
        padding_y = 8.,
        offset_x = 8.,
        offset_y = 4.
    ))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "puny_characters/slime.png")]
    texture: Handle<Image>,
}

fn patch_slime(
    mut commands: Commands,
    asset: Res<SlimeAnimation>,
    mut slime_query: Query<(Entity, &mut TextureAtlas, &mut Handle<Image>), With<Slime>>,
) {
    for (entity, mut atlas, mut texture) in &mut slime_query {
        let slime_animation_indices = SlimeAnimationIndecies {
            idle: vec![0, 1].into(),
            walking: vec![2, 3, 4, 5].into(),
        };

        atlas.layout = asset.layout.clone();
        *texture = asset.texture.clone();
        commands.entity(entity).insert((
            AnimationTimer(Timer::from_seconds(
                ACTION_DELAY / 4.0,
                TimerMode::Repeating,
            )),
            IdleAnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            slime_animation_indices,
            SlimeAnimationState::default(),
        ));
    }
}

fn update_slime_walking_animation(
    mut query: Query<
        (
            &mut SlimeAnimationIndecies,
            &mut AnimationTimer,
            &mut TextureAtlas,
            &SlimeAnimationState,
        ),
        With<Slime>,
    >,
    time: Res<Time>,
) {
    for (mut slime_indices, mut timer, mut atlas, slime_state) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() && *slime_state == SlimeAnimationState::Walking {
            atlas.index = slime_indices.walking.next().expect("looping iterator");
        }
    }
}

fn update_slime_idle_animation(
    mut query: Query<
        (
            &mut SlimeAnimationIndecies,
            &mut IdleAnimationTimer,
            &mut TextureAtlas,
            &SlimeAnimationState,
        ),
        With<Slime>,
    >,
    time: Res<Time>,
) {
    for (mut slime_indices, mut timer, mut atlas, slime_state) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() && *slime_state == SlimeAnimationState::Idle {
            atlas.index = slime_indices.idle.next().expect("looping iterator");
        }
    }
}

fn update_slime_atlas_index(
    mut query: Query<
        (
            &mut SlimeAnimationIndecies,
            &mut TextureAtlas,
            &SlimeAnimationState,
        ),
        (Changed<SlimeAnimationState>, With<Slime>),
    >,
) {
    for (mut slime_indices, mut atlas, slime_state) in &mut query {
        if *slime_state == SlimeAnimationState::Idle {
            atlas.index = slime_indices.idle.next().expect("looping iterator");
        }
    }
}

fn move_slime(
    mut query: Query<(&mut GridCoords, &mut SlimeAnimationState), With<Slime>>,
    level_walls: Res<LevelWalls>,
    mut event: EventReader<FreeWalkEvents>,
) {
    if let Some(free_walking_event) = event.read().next() {
        match free_walking_event.walking_state {
            WalkingState::Walking => {
                for (mut coords, mut slime_animation) in query.iter_mut() {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(-1..=1);
                    let y = rng.gen_range(-1..=1);
                    let direction = GridCoords::new(x, y);

                    if direction != GridCoords::new(0, 0) {
                        *slime_animation = SlimeAnimationState::Walking;
                        let destination = *coords + direction;
                        if !level_walls.in_wall(&destination) {
                            *coords = destination;
                        }
                    }
                }
            }
            WalkingState::Idle => {
                for (_, mut slime_animation) in query.iter_mut() {
                    if *slime_animation != SlimeAnimationState::Idle {
                        *slime_animation = SlimeAnimationState::Idle;
                    }
                }
            }
        }
    }
}
