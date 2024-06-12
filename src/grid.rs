use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::camera::MainCamera;
use crate::ldtk::{LevelWalls, Stair, Wall};
use crate::{player::Player, AppState, GameplaySet, GRID_SIZE};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                translate_grid_coords_entities,
                cache_wall_locations,
                check_stairs,
            )
                .in_set(GameplaySet::InputSet),
        );
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Without<MainCamera>>,
    time: Res<Time>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        let from = transform.translation;
        let to =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);

        let interpolation = from.lerp(to, 1.0 - f32::powf(2.0, -9.0 * time.delta_seconds()));
        if (from - to).length() < 0.30 {
            transform.translation = to;
        } else {
            transform.translation = interpolation;
        }
    }
}

fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}

fn check_stairs(
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    level_selection: ResMut<LevelSelection>,
    stair: Query<&GridCoords, With<Stair>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if players
        .iter()
        .zip(stair.iter())
        .any(|(player_grid_coords, stairs_grid_coords)| player_grid_coords == stairs_grid_coords)
    {
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        indices.level += 1;
        next_state.set(AppState::Loading);
    }
}
