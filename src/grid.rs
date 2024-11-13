use bevy::ecs::entity;
use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::*;
use bevy::render::view::visibility;
use bevy::utils::info;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::egui::epaint::text::cursor;
use leafwing_input_manager::prelude::ActionState;
use pathfinding::prelude::astar;

use crate::camera::MainCamera;

use crate::input::PlayerInputAction;
use crate::ldtk::{Floor, Grid, LevelWalls, Los_Grid, Stair, Wall};
use crate::ui::game_cursor::{CursorDirection, CursorPos};
use crate::{get_single, player};
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
                toggel_grid,
                display_los_grid,
            )
                .in_set(GameplaySet::InputSet),
        )
        .register_type::<Collider>()
        .add_systems(OnExit(AppState::Loading), (spawn_grid, spawn_los_grid))
        .insert_resource(GridToggled(false));
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

#[derive(Resource)]
pub struct GridToggled(pub bool);

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

fn spawn_grid(
    floor: Query<Entity, With<Floor>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for entity in floor.iter() {
        let grid_id = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("GridYellow.png"),
                    transform: Transform::from_xyz(-8.0, -8.0, 5.0),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Grid, //Component to identify the yellow grid
            ))
            .id();

        commands.entity(entity).add_child(grid_id);
    }
}

fn toggel_grid(
    mut grid: Query<&mut Visibility, With<Grid>>,
    query: Query<&ActionState<PlayerInputAction>, With<Player>>,
    mut grid_toggled: ResMut<GridToggled>,
) {
    if let Ok(action_state) = query.get_single() {
        if action_state.just_pressed(&PlayerInputAction::Tab) {
            let mut toggled_to_visible = false;
            for mut visibility in grid.iter_mut() {
                *visibility = if *visibility == Visibility::Hidden {
                    toggled_to_visible = true;
                    Visibility::Visible
                } else {
                    toggled_to_visible = false;
                    Visibility::Hidden
                };
            }

            if toggled_to_visible == true {
                grid_toggled.0 = true
            } else {
                grid_toggled.0 = false
            }

            info!("Toggled grid visibility");
        }
    } else {
        warn!("No Player entity found in toggel_grid system");
    }
}

fn spawn_los_grid(
    floor: Query<Entity, With<Floor>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for entity in floor.iter() {
        let los_grid_id = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("GridRed.png"),
                    transform: Transform::from_xyz(-8.0, -8.0, 6.0),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Los_Grid,
            ))
            .id();

        commands.entity(entity).add_child(los_grid_id);
    }
}

fn display_los_grid(
    floor: Query<(Entity, &GridCoords, Option<&Children>), With<Floor>>,
    player_grid: Query<&GridCoords, With<Player>>,
    cursor_direction: Res<CursorDirection>,
    mut visibility_param_set: ParamSet<(
        Query<&mut Visibility, With<Los_Grid>>, // ParamSet 0: Query for red grid
        Query<&mut Visibility, With<Grid>>,     // ParamSet 1: Query for yellow grid
    )>,
    grid_toggled: Res<GridToggled>,
) {
    // If the grid is not toggled, hide all los grids
    if !grid_toggled.0 {
        for mut visibility in visibility_param_set.p0().iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for mut visibility in visibility_param_set.p1().iter_mut() {
            *visibility = Visibility::Hidden;
        }
        return;
    }
    let player_pos = get_single!(player_grid);
    // iterate over each floor tile
    for (entity, coords, children) in floor.iter() {
        // Determine the direction and the range of the grid cells to be made visible
        let should_toggle = match *cursor_direction {
            CursorDirection::Up => coords.x == player_pos.x && coords.y >= player_pos.y,
            CursorDirection::Down => coords.x == player_pos.x && coords.y <= player_pos.y,
            CursorDirection::Left => coords.y == player_pos.y && coords.x <= player_pos.x,
            CursorDirection::Right => coords.y == player_pos.y && coords.x >= player_pos.x,
            CursorDirection::UpRight => {
                (coords.x >= player_pos.x)
                    && (coords.y >= player_pos.y)
                    && (coords.y - player_pos.y == coords.x - player_pos.x)
            }
            CursorDirection::UpLeft => {
                (coords.x <= player_pos.x)
                    && (coords.y >= player_pos.y)
                    && (coords.y - player_pos.y == player_pos.x - coords.x)
            }
            CursorDirection::DownRight => {
                (coords.x >= player_pos.x)
                    && (coords.y <= player_pos.y)
                    && (player_pos.y - coords.y == coords.x - player_pos.x)
            }
            CursorDirection::DownLeft => {
                (coords.x <= player_pos.x)
                    && (coords.y <= player_pos.y)
                    && (player_pos.y - coords.y == player_pos.x - coords.x)
            }
            _ => false,
        };
        if let Some(children) = children {
            for &child in children.iter() {
                // Check if the child is a red grid (Los_Grid)
                if let Ok(mut los_visibility) = visibility_param_set.p0().get_mut(child) {
                    if should_toggle {
                        *los_visibility = Visibility::Visible;

                        // Now, find the corresponding yellow grid among the children
                        for &sibling in children.iter() {
                            if let Ok(mut yellow_visibility) =
                                visibility_param_set.p1().get_mut(sibling)
                            {
                                *yellow_visibility = Visibility::Hidden;
                            }
                        }
                    } else {
                        *los_visibility = Visibility::Hidden;

                        // Restore the visibility of the yellow grid
                        for &sibling in children.iter() {
                            if let Ok(mut yellow_visibility) =
                                visibility_param_set.p1().get_mut(sibling)
                            {
                                *yellow_visibility = Visibility::Visible;
                            }
                        }
                    }
                }
            }
        }
    }
}
/*
if should_toggle {
    if let Some(children) = children {
        for &child in children.iter() {
            if let Ok(mut visibility) = visibility_query.get_mut(child) {
                // Toggle visibility only if it is currently hidden
                if *visibility == Visibility::Hidden {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
} else {
    if let Some(children) = children {
        for &child in children.iter() {
            if let Ok(mut visibility) = visibility_query.get_mut(child) {
                // Hide the grid cells that are outside the desired range
                if *visibility == Visibility::Visible {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
 */
