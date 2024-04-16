use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{assets::LevelWalls, AnimationIndices, AnimationTimer, AppState, GameplaySet};



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
            Update,
            (update_slime_animation, update_slime_atlas_index).run_if(in_state(AppState::InGame)),
        )
        .register_type::<SlimeAnimationIndecies>()
        .register_type::<SlimeAnimationState>();
    }
}

#[derive(Default, Component, Reflect)]
pub struct Slime;

#[derive(Component, Reflect, Default)]
pub enum SlimeAnimationState {
    #[default]
    Idle,
    Walking    
}

#[derive(Component, Reflect)]
struct SlimeAnimationIndecies {
    idle: AnimationIndices,
    walking: AnimationIndices,    
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
) {
    for (entity, mut atlas, mut texture) in &mut slime_query {
        let slime_animation_indices = SlimeAnimationIndecies {
            idle: AnimationIndices {first: 0, last: 1 }, 
            walking: AnimationIndices{first: 2, last: 5 },          
        };

        atlas.layout = asset.layout.clone();
        *texture = asset.texture.clone();
        commands.entity(entity).insert((
            AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            slime_animation_indices,
            SlimeAnimationState::default(),
        ));
    }
}

fn update_slime_animation(
    mut query: Query<
        (
            &SlimeAnimationIndecies,
            &mut AnimationTimer,
            &mut TextureAtlas,
        ),
        With<Slime>,
    >,
    slime_states: Query<&SlimeAnimationState, With<Slime>>,
    time: Res<Time>,
) {
    let slime_state = slime_states.iter().next().unwrap();
    for (slime_indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match slime_state {
                SlimeAnimationState::Idle => {                    
                    atlas.index = if atlas.index == slime_indices.idle.last {
                        slime_indices.idle.first
                    } else {
                        atlas.index + 1
                    };
                }
                SlimeAnimationState::Walking => {
                    atlas.index = if atlas.index == slime_indices.walking.last {
                        slime_indices.walking.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}

fn update_slime_atlas_index(
    mut query: Query<
        (
            &SlimeAnimationIndecies,
            &mut TextureAtlas,
            &SlimeAnimationState,
        ),
        Changed<SlimeAnimationState>,
    >,
) {
    for (slime_indices, mut atlas, slime_state) in &mut query {
        match slime_state {
            SlimeAnimationState::Idle => {
                atlas.index = slime_indices.idle.first;
            }
            SlimeAnimationState::Walking => {
                atlas.index = slime_indices.walking.first;
            }            
        }
    }
}