use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AbilityKeyEvent>()
            .add_systems(Update, (test).run_if(on_event::<AbilityKeyEvent>()));
    }
}

#[derive(Event)]
pub enum AbilityKeyEvent {
    Q,
    E,
}

fn test(mut ability_er: EventReader<AbilityKeyEvent>) {
    for event in ability_er.read() {
        match event {
            AbilityKeyEvent::Q => println!("Q"),
            AbilityKeyEvent::E => println!("E"),
        }
    }
}
