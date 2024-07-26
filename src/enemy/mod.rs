use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use health_bar::HealthBarMaterial;
use rand::Rng;

pub mod health_bar;
pub mod slime;

use crate::{
    events::CombatEvent, get_single, grid::GridPosition, ldtk::LevelWalls, player::Player,
    AppState, Health,
};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_enemy_range_detection,
                show_healthbar,
                handle_attacking_mark,
                update_health_bar,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .register_type::<EnemyBehaviorState>()
        .register_type::<Health>()
        .register_type::<Enemy>();
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component, Reflect)]
pub struct AttackRange(i32);

impl Default for AttackRange {
    fn default() -> Self {
        AttackRange(1)
    }
}

#[derive(Component, Default, Reflect)]
pub struct EnemyAttacking;

#[derive(Default, Reflect, PartialEq)]
pub enum EnemyBehaviorState {
    #[default]
    Idle,
    Fleeing,
    Pursuing,
    Patrolling,
}

#[derive(Component, Default, Reflect)]
pub struct Enemy {
    pub behavior_state: EnemyBehaviorState,
}

impl Enemy {
    pub fn move_towards_player(
        &self,
        player_pos: &GridCoords,
        enemy_pos: &GridCoords,
        level_walls: &LevelWalls,
        occupied_coords: &[GridCoords],
    ) -> GridCoords {
        match self.behavior_state {
            EnemyBehaviorState::Idle => {
                let next_pos;
                loop {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(-1..=1);
                    let y = rng.gen_range(-1..=1);
                    let new_coords = GridCoords::new(enemy_pos.x + x, enemy_pos.y + y);
                    if !level_walls.wall_locations.contains(&new_coords)
                        && !occupied_coords.contains(&new_coords)
                        && new_coords != *enemy_pos
                    {
                        next_pos = GridCoords::new(x, y);
                        break;
                    }
                }
                next_pos
            }
            EnemyBehaviorState::Fleeing => todo!(),
            EnemyBehaviorState::Pursuing => {
                let start_pos = GridPosition::new(enemy_pos.to_owned());
                let path =
                    start_pos.pathfind(player_pos.to_owned(), level_walls, Some(occupied_coords));
                let next_pos = if let Some(path) = path {
                    // if there is only the starting position and the destination left in the path, return 0,0 to stop
                    // in front of the player
                    if path.len() >= 3 {
                        *path.get(1).unwrap() - *enemy_pos
                    } else {
                        GridCoords::new(0, 0)
                    }
                } else {
                    GridCoords::new(0, 0)
                };
                next_pos
            }
            EnemyBehaviorState::Patrolling => todo!(),
        }
    }
}

fn player_enemy_range_detection(
    player_pos: Query<&GridCoords, With<Player>>,
    mut enemies: Query<(&GridCoords, &mut Enemy)>,
    mut combat_event: EventWriter<CombatEvent>,
) {
    let player = if let Ok(player_pos) = player_pos.get_single() {
        player_pos
    } else {
        return;
    };

    let mut is_in_combat = false;

    for (enemy_pos, mut enemy) in enemies.iter_mut() {
        let x_diff = (player.x - enemy_pos.x).abs();
        let y_diff = (player.y - enemy_pos.y).abs();

        if x_diff < 5 && y_diff < 5 {
            enemy.behavior_state = EnemyBehaviorState::Pursuing;
            is_in_combat = true;
        } else {
            enemy.behavior_state = EnemyBehaviorState::Idle;
        }
    }

    combat_event.send(CombatEvent(is_in_combat));
}

fn handle_attacking_mark(
    mut commands: Commands,
    player_pos: Query<&GridCoords, With<Player>>,
    enemies: Query<(Entity, &GridCoords, &AttackRange), With<Enemy>>,
) {
    let player_pos = get_single!(player_pos);
    for (entity, enemy_pos, attack_range) in enemies.iter() {
        let x_diff = (player_pos.x - enemy_pos.x).abs();
        let y_diff = (player_pos.y - enemy_pos.y).abs();

        if x_diff <= attack_range.0 && y_diff <= attack_range.0 {
            commands.entity(entity).insert(EnemyAttacking);
        } else {
            commands.entity(entity).remove::<EnemyAttacking>();
        }
    }
}

fn show_healthbar(
    mut combat_et_reader: EventReader<CombatEvent>,
    mut health_bars: Query<&mut Visibility, With<HealthBar>>,
) {
    if let Some(event) = combat_et_reader.read().next() {
        for mut health_bar in health_bars.iter_mut() {
            if event.0 {
                *health_bar = Visibility::Visible;
            }
            if !event.0 {
                *health_bar = Visibility::Hidden;
            }
        }
    }
}

fn update_health_bar(
    mut health_bar_materials: ResMut<Assets<HealthBarMaterial>>,
    query: Query<(&Handle<HealthBarMaterial>, &Health)>,
) {
    for (handle, health) in query.iter() {
        let per = health.current_health as f32 / health.max_health as f32;
        let material = health_bar_materials.get_mut(handle).unwrap();
        material.percent = per;
    }
}
