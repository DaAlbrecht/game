use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Turn {
            timer: Timer::new(Duration::from_secs_f64(0.1), TimerMode::Repeating),
            ..Default::default()
        })
        .add_plugins(ResourceInspectorPlugin::<Turn>::default())
        .add_systems(Update, turn_manager)
        .add_event::<TurnEvent>()
        .register_type::<TurnMode>();
    }
}

#[derive(Resource, Default, PartialEq, Reflect)]
pub struct Turn {
    pub mode: TurnMode,
    pub timer: Timer,
}

#[derive(Default, PartialEq, Reflect)]
pub enum TurnMode {
    #[default]
    Idle,
    PlayerAction,
}

#[derive(Event)]
pub struct TurnEvent(pub TurnMode);

fn turn_manager(mut ev_turn: EventReader<TurnEvent>, mut turn: ResMut<Turn>, time: Res<Time>) {
    if let Some(event) = ev_turn.read().next() {
        match event.0 {
            TurnMode::Idle => {
                turn.mode = TurnMode::Idle;
            }
            TurnMode::PlayerAction => {
                turn.mode = TurnMode::PlayerAction;
                info!("player action");
            }
        }
    }
    turn.timer.tick(time.delta());

    if turn.timer.finished() {
        turn.mode = TurnMode::Idle;
    }
}
