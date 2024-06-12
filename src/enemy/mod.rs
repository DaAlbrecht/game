use bevy::prelude::*;
use bevy_ecs_ldtk::{utils::grid_coords_to_translation, GridCoords};
pub mod slime;

use crate::{events::CombatEvent, player::Player, AppState, GRID_SIZE};

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
    pub fn move_towards_player(&self, player_pos: &GridCoords, enemy_pos: &GridCoords) -> Vec2 {
        let grid_size = IVec2::splat(GRID_SIZE);

        if (player_pos.x - enemy_pos.x).abs() < 2 && (player_pos.y - enemy_pos.y).abs() < 2 {
            return Vec2::ZERO;
        }

        let player_transform = grid_coords_to_translation(*player_pos, grid_size);
        let enemy_transfrom = grid_coords_to_translation(*enemy_pos, grid_size);

        (player_transform - enemy_transfrom).normalize().round()
    }
}
