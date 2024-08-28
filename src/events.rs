use bevy::prelude::*;

use crate::{player::PlayerAction, ui::game_cursor::CursorDirection};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatEvent>().add_event::<TurnOver>();
    }
}

#[derive(Event)]
pub struct TurnOver(pub PlayerAction);

#[derive(Event)]
pub struct CombatEvent(pub bool);
