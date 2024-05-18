use bevy::prelude::*;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatEvent>()
            .add_event::<PlayerTurnOver>()
            .add_event::<FreeWalkEvents>();
    }
}

#[derive(Event)]
pub struct FreeWalkEvents {
    pub walking_state: WalkingState,
}

#[derive(Event)]
pub struct PlayerTurnOver;

pub enum WalkingState {
    Idle,
    Walking,
}

#[derive(Event)]
pub struct CombatEvent(pub bool);
