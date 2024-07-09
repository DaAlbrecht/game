use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::GridCoords;
use leafwing_input_manager::prelude::*;

use crate::{
    camera::MainCamera,
    player::{Player, PlayerMove},
    AppState, GameCursor, CURSOR_Z_INDEX,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_plugins(InputManagerPlugin::<MenuAction>::default())
            .init_resource::<CursorPos>()
            .init_resource::<ActionState<MenuAction>>()
            .insert_resource(MenuAction::default_input_map())
            .add_event::<PlayerMove>()
            .add_systems(Startup, hide_grab)
            .add_systems(OnEnter(AppState::InGame), add_player_input_manager)
            .add_systems(Update, toggle_menu)
            .add_systems(Update, (move_player).run_if(in_state(AppState::InGame)))
            .add_systems(
                FixedUpdate,
                (update_cursor_pos, update_game_cursor).after(move_player),
            )
            .add_systems(
                FixedUpdate,
                (show_cursor)
                    .after(update_game_cursor)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource, Reflect, Default)]
pub struct CursorPos {
    pub world_coords: Vec3,
    pub screen_coords: Vec3,
    pub ui_coords: Vec3,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
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

fn move_player(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut move_direction: EventWriter<PlayerMove>,
) {
    let action_state = query.single();
    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                move_direction.send(PlayerMove(direction));
                return;
            }
        }
    }
    move_direction.send(PlayerMove(GridCoords::new(0, 0)));
}

//Special thanks to RaminKav from: https://github.com/RaminKav/BevySurvivalGame/tree/master
pub fn update_cursor_pos(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Transform, &Camera), With<MainCamera>>,
    mut cursor_moved_er: EventReader<CursorMoved>,
    mut cursor: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_er.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera.iter() {
            if cfg!(not(target_os = "macos")) {
                *cursor = CursorPos {
                    world_coords: cursor_pos_in_world(&windows, cursor_moved.position, cam_t, cam),
                    ui_coords: cursor_pos_in_ui(&windows, cursor_moved.position, cam),
                    screen_coords: cursor_moved.position.extend(0.),
                };
            } else {
                *cursor = CursorPos {
                    world_coords: cursor_pos_in_world_pilot_mode(
                        &windows,
                        cursor_moved.position,
                        cam_t,
                        cam,
                    ),
                    ui_coords: cursor_pos_in_ui(&windows, cursor_moved.position, cam),
                    screen_coords: cursor_moved.position.extend(0.),
                };
            }
        }
    }
}

pub fn cursor_pos_in_world_pilot_mode(
    windows: &Query<&Window, With<PrimaryWindow>>,
    cursor_pos: Vec2,
    cam_t: &Transform,
    cam: &Camera,
) -> Vec3 {
    let window = windows.single();
    let cursor_pos = Vec2::new(cursor_pos.x, window.height() - cursor_pos.y);

    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen position [0..resolution] to ndc [-1..1]
    // (ndc = normalized device coordinates)
    let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    ndc_to_world.project_point3(ndc.extend(0.0))
}

pub fn cursor_pos_in_world(
    windows: &Query<&Window, With<PrimaryWindow>>,
    cursor_pos: Vec2,
    cam_t: &Transform,
    cam: &Camera,
) -> Vec3 {
    let window = windows.single();

    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen position [0..resolution] to ndc [-1..1]
    // (ndc = normalized device coordinates)
    let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    ndc_to_world.project_point3(ndc.extend(0.0))
}

pub fn cursor_pos_in_ui(
    windows: &Query<&Window, With<PrimaryWindow>>,
    cursor_pos: Vec2,
    cam: &Camera,
) -> Vec3 {
    let window = windows.single();

    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen position [0..resolution] to ndc [-1..1]
    // (ndc = normalized device coordinates)
    let t = Transform::from_translation(Vec3::new(0., 0., 0.));
    let ndc_to_world = t.compute_matrix() * cam.projection_matrix().inverse();
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    ndc_to_world.project_point3(ndc.extend(0.0))
}

fn update_game_cursor(
    mut cursor_transform: Query<&mut Transform, With<GameCursor>>,
    cursor_pos: Res<CursorPos>,
) {
    let mut cursor_transform = if let Ok(cursor_transform) = cursor_transform.get_single_mut() {
        cursor_transform
    } else {
        return;
    };

    let cursor_pos = cursor_pos.world_coords;
    let cursor_pos = Vec3::new(cursor_pos.x, cursor_pos.y, CURSOR_Z_INDEX);
    cursor_transform.translation = cursor_pos;
}

fn hide_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.visible = false;
}

fn show_cursor(
    mut cursor_moved_er: EventReader<CursorMoved>,
    mut cursor_visivility: Query<&mut Visibility, With<GameCursor>>,
) {
    let mut cursor_visivility = if let Ok(cursor_visivility) = cursor_visivility.get_single_mut() {
        cursor_visivility
    } else {
        return;
    };

    if cursor_moved_er.read().next().is_some() {
        *cursor_visivility = Visibility::Visible;
    } else {
        *cursor_visivility = Visibility::Hidden;
    }
}

fn toggle_menu(
    action_state: Res<ActionState<MenuAction>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut game_cursor: Query<&mut Visibility, With<GameCursor>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match state.get() {
        AppState::Loading => return,
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
