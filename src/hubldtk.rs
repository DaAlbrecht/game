use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;

use crate::{enemy::slime::Slime, ldtk::PlayerBundle, player::Player};

pub struct LtdkHubPlugin;

impl Plugin for LtdkHubPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<ObiliskBundle>("Obilisk")
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<FenceBundle>(1)
            .init_resource::<LevelFence>();
    }
}

#[derive(Default, Component)]
pub struct Fence;

#[derive(Default, Bundle, LdtkIntCell)]
struct FenceBundle {
    fence: Fence,
}

#[derive(Default, Resource)]
pub struct LevelFence {
    pub fence_locations: HashSet<GridCoords>,
    pub level_width: i32,
    pub level_height: i32,
}

impl LevelFence {
    pub fn in_fence(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.fence_locations.contains(grid_coords)
    }
}

#[derive(Default, Component)]
pub struct Obilisk;

#[derive(Default, Bundle, LdtkEntity)]
struct ObiliskBundle {
    stair: Obilisk,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
