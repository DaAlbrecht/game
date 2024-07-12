use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use rand::Rng;
pub mod slime;

use crate::{events::CombatEvent, grid::GridPosition, ldtk::LevelWalls, player::Player, AppState};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_enemy_range_detection, show_healthbar).run_if(in_state(AppState::InGame)),
        )
        .register_type::<EnemyBehaviorState>()
        .register_type::<Enemy>();
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component, Default, Reflect)]
pub struct Enemy {
    pub behavior_state: EnemyBehaviorState,
}

#[derive(Default, Reflect, PartialEq)]
pub enum EnemyBehaviorState {
    #[default]
    Idle,
    Fleeing,
    Pursuing,
    Patrolling,
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
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(-1..=1);
                let y = rng.gen_range(-1..=1);
                GridCoords::new(x, y)
            }
            EnemyBehaviorState::Fleeing => todo!(),
            EnemyBehaviorState::Pursuing => {
                let start_pos = GridPosition::new(enemy_pos.to_owned());
                let path = start_pos.pathfind(player_pos.to_owned(), level_walls, occupied_coords);
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
