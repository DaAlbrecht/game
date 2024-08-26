use bevy::prelude::*;

use crate::player::PlayerAction;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatEvent>()
            .add_event::<TurnOver>()
            .add_event::<GridToggledEvent>();
    }
}

#[derive(Event)]
pub struct TurnOver(pub PlayerAction);

#[derive(Event)]
pub struct CombatEvent(pub bool);

#[derive(Event)]
pub struct GridToggledEvent(pub bool);
