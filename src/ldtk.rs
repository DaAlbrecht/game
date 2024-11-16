use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;

use crate::{enemy::slime::Slime, player::Player};

pub struct LdtkAssetPlugin;

impl Plugin for LdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<StairsBundle>("Stairs")
            .register_ldtk_entity::<SlimeBundle>("Slime")
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<WallBundle>(1)
            .init_resource::<LevelWalls>()
            .register_ldtk_int_cell::<FloorBundle>(2)
            .init_resource::<LevelFloor>();
    }
}

#[derive(Default, Component)]
pub struct Stair;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Component)]
pub struct Floor;

#[derive(Default, Component)]
pub struct Grid;

#[derive(Default, Component)]
pub struct LosGrid;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
struct StairsBundle {
    stair: Stair,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
struct SlimeBundle {
    slime: Slime,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Default, Bundle, LdtkIntCell)]
struct FloorBundle {
    floor: Floor,
}

#[derive(Default, Resource)]
pub struct LevelWalls {
    pub wall_locations: HashSet<GridCoords>,
    pub level_width: i32,
    pub level_height: i32,
}

#[derive(Default, Resource)]
pub struct LevelFloor {
    pub floor_locations: HashSet<GridCoords>,
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

impl LevelFloor {
    pub fn in_floor(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.floor_locations.contains(grid_coords)
    }
}
