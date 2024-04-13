use assets::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use camera::MainCamera;
pub mod assets;
pub mod camera;
pub mod movement;

pub const GRID_SIZE: i32 = 16;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameplaySet {
    InputSet,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.3;
    camera.projection.viewport_origin = Vec2::ZERO;
    commands.spawn((camera, MainCamera));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("game.ldtk"),
        ..Default::default()
    });
}

pub fn patch_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainCamera>>,
    player_query: Query<&GridCoords, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(player_grid_coords) = player_query.get_single() {
        if let Ok(camera_entity) = camera_query.get_single() {
            commands.entity(camera_entity).insert(*player_grid_coords);
            next_state.set(AppState::InGame);
        }
    }
}
