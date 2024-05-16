use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{player::Player, AppState};
#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (plater_enemy_range_detection).run_if(in_state(AppState::InGame)),
        )
        .add_event::<CombatEvent>();
    }
}

#[derive(Event)]
pub struct CombatEvent;

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

    for enemy_pos in enemies_pos.iter() {
        let x_diff = (player.x - enemy_pos.x).abs();
        let y_diff = (player.y - enemy_pos.y).abs();

        if x_diff <= 5 && y_diff <= 5 {
            combat_event.send(CombatEvent);
            info!("Player is in range of enemy");
        }
    }
}
