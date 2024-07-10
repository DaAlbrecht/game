use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use rand::Rng;

use crate::{
    events::TurnOver,
    ldtk::LevelWalls,
    player::{Player, PlayerAction},
    AnimationTimer, AppState, IdleAnimationTimer, IndeciesIter, ACTION_DELAY,
};

use super::{Enemy, EnemyBehaviorState, HealthBar};

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            Enemy::default(),
        ));

        let healt_bar = commands
            .spawn((
                HealthBar,
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 1.0))),
                    material: materials.add(Color::RED),
                    transform: Transform::from_xyz(0., 8., 0.),
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ))
            .id();

        commands.entity(entity).add_child(healt_bar);
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

#[allow(clippy::type_complexity)]
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

#[allow(clippy::type_complexity)]
fn move_slime(
    mut query: Query<
        (&mut GridCoords, &mut SlimeAnimationState, &Enemy),
        (With<Slime>, Without<Player>),
    >,
    player_pos: Query<&GridCoords, With<Player>>,
    level_walls: Res<LevelWalls>,
    mut turn_over_er: EventReader<TurnOver>,
) {
    let event = turn_over_er.read().next();
    if event.is_none() {
        return;
    }
    match event.unwrap().0 {
        PlayerAction::Idle => {
            for (_, mut slime_animation, _) in query.iter_mut() {
                if *slime_animation != SlimeAnimationState::Idle {
                    *slime_animation = SlimeAnimationState::Idle;
                }
            }
        }
        PlayerAction::Walking => {
            let player_pos = if let Ok(player_pos) = player_pos.get_single() {
                *player_pos
            } else {
                return;
            };

            for (mut coords, mut slime_animation, enemy) in query.iter_mut() {
                let direction = match enemy.behavior_state {
                    EnemyBehaviorState::Idle => {
                        let mut rng = rand::thread_rng();
                        let x = rng.gen_range(-1..=1);
                        let y = rng.gen_range(-1..=1);
                        GridCoords::new(x, y)
                    }
                    EnemyBehaviorState::Fleeing => todo!(),
                    EnemyBehaviorState::Pursuing => {
                        let direction =
                            enemy.move_towards_player(&player_pos, &coords, &level_walls);
                        GridCoords::new(direction.x as i32, direction.y as i32)
                    }
                    EnemyBehaviorState::Patrolling => todo!(),
                };
                info!("direction: {:?}", direction);

                if direction != GridCoords::new(0, 0) {
                    *slime_animation = SlimeAnimationState::Walking;
                    let destination = *coords + direction;
                    if !level_walls.in_wall(&destination) {
                        *coords = destination;
                    }
                }
            }
        }
    };
}
