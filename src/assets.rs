use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

use std::collections::HashSet;
use crate::AppState;


pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame).load_collection::<Playeranimation>())      
            .add_systems(OnEnter(AppState::InGame),patch_player)
            .add_systems(Update, update_player_animation.run_if(in_state(AppState::InGame)),)
            .register_ldtk_entity::<StairsBundle>("Stairs")
            .register_ldtk_entity::<StairsBundle>("Enemy")
            .insert_resource(LevelSelection::index(4))
            .register_ldtk_int_cell::<WallBundle>(1)
            .init_resource::<LevelWalls>();
    }
}

#[derive(Default, Component)]
pub struct Player;
  
#[derive(Default, Component)]
pub struct Stair;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Component)]
pub struct Enemy;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,    
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
struct StairsBundle {
    stair: Stair,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
struct EnemyBundle {
    enemy: Enemy,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Default, Resource)]
pub struct LevelWalls {
    pub wall_locations: HashSet<GridCoords>,
    pub level_width: i32,
    pub level_height: i32,
}

impl LevelWalls {
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}

#[derive(AssetCollection, Resource)]
struct Playeranimation{
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 24, rows = 8, padding_x = 16., padding_y = 8., offset_x = 8., offset_y = 8.))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "puny_characters/human_worker_red.png")]
    sprite: Handle<Image>,
}

fn patch_player(
    mut commands: Commands,
    asset: Res<Playeranimation>,
    player_query: Query<Entity, With<Player>>,
){      
    if let Ok(entity) = player_query.get_single() {
        /*commands.entity(entity).insert((
            SpriteSheetBundle{
                atlas: asset.layout.clone().into(),
                texture: asset.sprite.clone(),
                ..default()
            },
            AnimationTimer {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                frame_count: 2,
            },
        ));*/
    }      
   
}

fn update_player_animation(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>
){
    for (mut sprite, mut animation) in &mut sprites{
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            sprite.index += 1;
            if sprite.index >= animation.frame_count {
                sprite.index = 0;
            }
        }
    }
}

