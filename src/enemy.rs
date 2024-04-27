use bevy::prelude::*;

use crate::{player::Player, AppState};
#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (plater_enemy_range_detection).run_if(in_state(AppState::InGame)),
        );
    }
}

fn plater_enemy_range_detection(
    player_transform: Query<&Transform, With<Player>>,
    enemies_transform: Query<&Transform, With<Enemy>>,
) {
    let player = if let Ok(player_transform) = player_transform.get_single() {
        player_transform
    } else {
        return;
    };

    for enemy_transform in enemies_transform.iter() {
        let distance = player.translation.distance(enemy_transform.translation);
        if distance < 40. {
            info!("Player is in range of enemy");
        }
    }
}
