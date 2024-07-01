use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use leafwing_input_manager::prelude::*;

use crate::{
    combat::AbilityKeyEvent,
    player::{MoveDirection, Player},
    AppState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<MoveDirection>()
            .add_systems(OnEnter(AppState::InGame), add_input_manager)
            .add_systems(
                Update,
                (move_player, use_ability).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Ability1,
    Ability2,
}

impl PlayerAction {
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> Option<GridCoords> {
        match self {
            PlayerAction::Up => Some(GridCoords::new(0, 1)),
            PlayerAction::Down => Some(GridCoords::new(0, -1)),
            PlayerAction::Left => Some(GridCoords::new(-1, 0)),
            PlayerAction::Right => Some(GridCoords::new(1, 0)),
            _ => None,
        }
    }
}

#[derive(Bundle)]
struct PlayerInputBundle {
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerInputBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButtonType::DPadUp);

        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButtonType::DPadDown);

        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButtonType::DPadLeft);

        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButtonType::DPadRight);

        // Abilities
        input_map.insert(Ability1, KeyCode::KeyQ);
        input_map.insert(Ability1, GamepadButtonType::West);

        input_map.insert(Ability2, KeyCode::KeyW);
        input_map.insert(Ability2, GamepadButtonType::North);

        input_map
    }
}

fn add_input_manager(mut commands: Commands, player: Query<Entity, With<Player>>) {
    let player = if let Ok(player) = player.get_single() {
        player
    } else {
        panic!("No player entity found");
    };

    commands.entity(player).insert(PlayerInputBundle {
        input_manager: InputManagerBundle::with_map(PlayerInputBundle::default_input_map()),
    });
}

fn move_player(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut move_direction: EventWriter<MoveDirection>,
) {
    let action_state = query.single();
    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                move_direction.send(MoveDirection(direction));
            }
        }
    }
}

fn use_ability(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut ability_ew: EventWriter<AbilityKeyEvent>,
) {
    let action_state = query.single();
    if action_state.pressed(&PlayerAction::Ability1) {
        ability_ew.send(AbilityKeyEvent::Q);
    }
    if action_state.pressed(&PlayerAction::Ability2) {
        ability_ew.send(AbilityKeyEvent::E);
    }
}
