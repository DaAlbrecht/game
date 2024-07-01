use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{input::PlayerAction, player::Player, AppState};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (test).run_if(in_state(AppState::InGame)));
    }
}

fn test(query: Query<&ActionState<PlayerAction>, With<Player>>) {
    let action_state = query.single();

    if action_state.just_pressed(&PlayerAction::Ability1) {
        println!("Ability 1 just pressed");
    }
}
