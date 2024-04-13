use bevy::app::FixedMain;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::assets::{LevelWalls, Player, Stair, Wall};
use crate::camera::MainCamera;
use crate::{GameplaySet, GRID_SIZE};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                translate_grid_coords_entities,
                cache_wall_locations,
                check_stairs,
            )
                .in_set(GameplaySet::InputSet),
        )
        .add_systems(
            FixedMain,
            (move_player_from_input).in_set(GameplaySet::InputSet),
        );
    }
}

fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let movement_direction = if input.pressed(KeyCode::KeyW) {
        GridCoords::new(0, 1)
    } else if input.pressed(KeyCode::KeyA) {
        GridCoords::new(-1, 0)
    } else if input.pressed(KeyCode::KeyS) {
        GridCoords::new(0, -1)
    } else if input.pressed(KeyCode::KeyD) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<
        (&mut Transform, &GridCoords),
        (Changed<GridCoords>, Without<MainCamera>),
    >,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
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
    }
}
