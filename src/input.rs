use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::GridCoords;
use leafwing_input_manager::prelude::*;

use crate::{
    player::{Player, PlayerMove},
    AppState, GameCursor,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerInputAction>::default())
            .add_plugins(InputManagerPlugin::<MenuAction>::default())
            .init_resource::<ActionState<MenuAction>>()
            .insert_resource(MenuAction::default_input_map())
            .add_event::<PlayerMove>()
            .add_systems(OnEnter(AppState::InGame), add_player_input_manager)
            .add_systems(Update, toggle_menu)
            .add_systems(Update, (move_player).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerInputAction {
    Up,
    Down,
    Left,
    Right,
    Ability1,
    Ability2,
}

impl PlayerInputAction {
    const DIRECTIONS: [Self; 4] = [
        PlayerInputAction::Up,
        PlayerInputAction::Down,
        PlayerInputAction::Left,
        PlayerInputAction::Right,
    ];

    fn direction(self) -> Option<GridCoords> {
        match self {
            PlayerInputAction::Up => Some(GridCoords::new(0, 1)),
            PlayerInputAction::Down => Some(GridCoords::new(0, -1)),
            PlayerInputAction::Left => Some(GridCoords::new(-1, 0)),
            PlayerInputAction::Right => Some(GridCoords::new(1, 0)),
            _ => None,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    Pause,
}

impl MenuAction {
    fn default_input_map() -> InputMap<Self> {
        use MenuAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(Pause, KeyCode::Escape);

        input_map
    }
}

#[derive(Bundle)]
struct PlayerInputBundle {
    input_manager: InputManagerBundle<PlayerInputAction>,
}

impl PlayerInputBundle {
    fn default_input_map() -> InputMap<PlayerInputAction> {
        use PlayerInputAction::*;
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

        input_map.insert(Ability2, KeyCode::KeyE);
        input_map.insert(Ability2, GamepadButtonType::North);

        input_map
    }
}

fn add_player_input_manager(mut commands: Commands, player: Query<Entity, With<Player>>) {
    let player = if let Ok(player) = player.get_single() {
        player
    } else {
        panic!("No player entity found");
    };

    commands.entity(player).insert(PlayerInputBundle {
        input_manager: InputManagerBundle::with_map(PlayerInputBundle::default_input_map()),
    });
}

pub fn move_player(
    query: Query<&ActionState<PlayerInputAction>, With<Player>>,
    mut move_direction: EventWriter<PlayerMove>,
) {
    let action_state = query.single();
    for input_direction in PlayerInputAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                move_direction.send(PlayerMove(direction));
                return;
            }
        }
    }
    move_direction.send(PlayerMove(GridCoords::new(0, 0)));
}

fn toggle_menu(
    action_state: Res<ActionState<MenuAction>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut game_cursor: Query<&mut Visibility, With<GameCursor>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match state.get() {
        AppState::Loading => (),
        AppState::InGame => {
            if action_state.just_pressed(&MenuAction::Pause) {
                let mut primary_window = windows.single_mut();
                primary_window.cursor.visible = !primary_window.cursor.visible;

                let mut game_cursor = game_cursor.single_mut();
                *game_cursor = Visibility::Hidden;
                next_state.set(AppState::Menu);
            }
        }
        AppState::Menu => {
            if action_state.just_pressed(&MenuAction::Pause) {
                let mut primary_window = windows.single_mut();
                primary_window.cursor.visible = !primary_window.cursor.visible;

                let mut game_cursor = game_cursor.single_mut();
                *game_cursor = Visibility::Visible;
                next_state.set(AppState::InGame);
            }
        }
    }
}
