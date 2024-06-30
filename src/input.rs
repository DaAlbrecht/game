use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{combat::AbilityKeyEvent, player::MoveDirection, AppState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveDirection>().add_systems(
            Update,
            (move_player, use_ability).run_if(in_state(AppState::InGame)),
        );
    }
}

fn move_player(
    key_input: Res<ButtonInput<KeyCode>>,
    mut move_direction: EventWriter<MoveDirection>,
) {
    let movement_direction = if key_input.pressed(KeyCode::KeyW) {
        Some(GridCoords::new(0, 1))
    } else if key_input.pressed(KeyCode::KeyA) {
        Some(GridCoords::new(-1, 0))
    } else if key_input.pressed(KeyCode::KeyS) {
        Some(GridCoords::new(0, -1))
    } else if key_input.pressed(KeyCode::KeyD) {
        Some(GridCoords::new(1, 0))
    } else {
        None
    };

    match movement_direction {
        Some(direction) => move_direction.send(MoveDirection(direction)),
        None => move_direction.send(MoveDirection(GridCoords::new(0, 0))),
    };
}

fn use_ability(key_input: Res<ButtonInput<KeyCode>>, mut ability_ew: EventWriter<AbilityKeyEvent>) {
    if key_input.just_pressed(KeyCode::KeyQ) {
        ability_ew.send(AbilityKeyEvent::Q);
    } else if key_input.just_pressed(KeyCode::KeyE) {
        ability_ew.send(AbilityKeyEvent::E);
    }
}
