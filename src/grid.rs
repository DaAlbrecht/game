use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use pathfinding::prelude::astar;

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
                update_colliders,
            )
                .in_set(GameplaySet::InputSet),
        )
        .register_type::<Collider>();
    }
}

#[derive(Component, Copy, Clone, Debug, Reflect)]
pub struct Collider {
    pub tile_width: i32,
    pub tile_height: i32,
    pub position: GridCoords,
}

impl Collider {
    pub fn new(tile_width: i32, tile_height: i32, position: GridCoords) -> Self {
        Self {
            tile_width,
            tile_height,
            position,
        }
    }

    pub fn get_occupied_coords(&self) -> Vec<GridCoords> {
        let mut occupied_coords = Vec::new();

        for x in 0..self.tile_width {
            for y in 0..self.tile_height {
                occupied_coords.push(GridCoords {
                    x: self.position.x + x,
                    y: self.position.y + y,
                });
            }
        }

        occupied_coords
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            tile_width: 1,
            tile_height: 1,
            position: GridCoords::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Successor {
    pub coords: GridPosition,
    pub cost: u32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct GridPosition(pub GridCoords);

impl Eq for GridPosition {}

impl GridPosition {
    pub fn new(grid_coords: GridCoords) -> Self {
        Self(grid_coords)
    }

    pub fn successors(
        &self,
        coords: &GridCoords,
        level_walls: &LevelWalls,
        occupied_coords: Option<&[GridCoords]>,
    ) -> Vec<Successor> {
        let mut successors = Vec::new();

        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                let new_coords = GridCoords {
                    x: coords.x + x,
                    y: coords.y + y,
                };

                if let Some(occupied_coords) = occupied_coords {
                    if occupied_coords.contains(&new_coords) {
                        continue;
                    }
                }

                if !level_walls.wall_locations.contains(&new_coords) {
                    successors.push(Successor {
                        coords: GridPosition(new_coords),
                        cost: 1,
                    });
                }
            }
        }

        successors
    }

    pub fn heuristic(&self, goal: &GridCoords) -> u32 {
        let dx = (self.0.x - goal.x).unsigned_abs();
        let dy = (self.0.y - goal.y).unsigned_abs();

        dx + dy
    }

    pub fn pathfind(
        &self,
        goal: GridCoords,
        level_walls: &LevelWalls,
        occupied_coords: Option<&[GridCoords]>,
    ) -> Option<Vec<GridCoords>> {
        let start = self;

        let result = astar(
            start,
            |p| {
                self.successors(&p.0, level_walls, occupied_coords)
                    .iter()
                    .map(|s| (s.coords, s.cost))
                    .collect::<Vec<_>>()
            },
            |p| p.heuristic(&goal),
            |p| p.0 == goal,
        );

        result.map(|(path, _)| path.iter().map(|p| p.0).collect())
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

pub fn update_colliders(mut query: Query<(&GridCoords, &mut Collider), With<Collider>>) {
    for (coords, mut collider) in query.iter_mut() {
        collider.position = *coords;
    }
}
