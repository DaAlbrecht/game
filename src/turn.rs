use bevy::prelude::*;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FreeWalkEvents>();
    }
}

#[derive(Default, PartialEq, Reflect)]
pub enum TurnMode {
    #[default]
    FreeWalk,
    Combat,
}

#[derive(Event)]
pub struct FreeWalkEvents {
    pub walking_state: WalkingState,
}
pub enum WalkingState {
    Idle,
    Walking,
}
