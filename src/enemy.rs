use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{player::Player, slime::HealthBar, AppState};
#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((plater_enemy_range_detection, show_healthbar)).run_if(in_state(AppState::InGame)),
        )
        .add_event::<CombatEvent>();
    }
}

#[derive(Event)]
pub struct CombatEvent(bool);

fn plater_enemy_range_detection(
    player_pos: Query<&GridCoords, With<Player>>,
    enemies_pos: Query<&GridCoords, With<Enemy>>,
    mut combat_event: EventWriter<CombatEvent>,
) {
    let player = if let Ok(player_pos) = player_pos.get_single() {
        player_pos
    } else {
        return;
    };

    let mut is_in_combat = false;

    for enemy_pos in enemies_pos.iter() {
        let x_diff = (player.x - enemy_pos.x).abs();
        let y_diff = (player.y - enemy_pos.y).abs();

        if x_diff < 5 && y_diff < 5 {
            is_in_combat = true;
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
            if event.0 == true {
                *health_bar = Visibility::Visible;
            }
            if event.0 == false {
                *health_bar = Visibility::Hidden;
            }
        }
    }
}
