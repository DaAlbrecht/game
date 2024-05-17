use bevy::prelude::*;

use crate::enemy::CombatEvent;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, in_combat)
            .add_event::<FreeWalkEvents>();
    }
}

#[derive(Event)]
pub struct FreeWalkEvents {
    pub walking_state: WalkingState,
}
pub enum WalkingState {
    Idle,
    Walking,
}

fn in_combat(mut combat_reader: EventReader<CombatEvent>) {
    if combat_reader.read().next().is_some() {}
}
