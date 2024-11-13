use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::{utils::translation_to_grid_coords, GridCoords};

use crate::{
    camera::MainCamera, enemy::Enemy, get_single, input::move_player,
    player::Player, AppState, CURSOR_Z_INDEX, GRID_SIZE,
};

pub struct GameCursorPlugin;

impl Plugin for GameCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_systems(Startup, hide_grab)
            .insert_resource(CursorDirection::default())
            .add_systems(
                FixedUpdate,
                (update_cursor_pos, update_game_cursor).after(move_player),
            )
            .add_systems(
                FixedUpdate,
                (show_cursor, cursor_mode)
                    .after(update_game_cursor)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, cursor_direction)
            .register_type::<GameCursor>();
    }
}

#[derive(Component, Reflect)]
pub struct GameCursor {
    pub active_time: Timer,
}

#[derive(Resource, Reflect, Default)]
pub struct CursorPos {
    pub world_coords: Vec3,
    pub screen_coords: Vec3,
    pub ui_coords: Vec3,
}

impl CursorPos {
    pub fn world_position(&self) -> GridCoords {
        translation_to_grid_coords(
            self.world_coords.truncate(),
            IVec2::new(GRID_SIZE, GRID_SIZE),
        )
    }
}

#[derive(Component, Default, PartialEq, Debug, Reflect, Resource)]
pub enum CursorDirection {
    #[default]
    Undefined,
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
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
            *cursor = CursorPos {
                world_coords: cursor_pos_in_world_pilot_mode(
                    &windows,
                    cursor_moved.position,
                    cam_t,
                    cam,
                ),
                ui_coords: cursor_pos_in_ui(&windows, cursor_moved.position, cam),
                screen_coords: cursor_moved.position.extend(0.),
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
    let ndc_to_world = cam_t.compute_matrix() * cam.clip_from_view().inverse();
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
    let ndc_to_world = cam_t.compute_matrix() * cam.clip_from_view().inverse();
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
    let ndc_to_world = t.compute_matrix() * cam.clip_from_view().inverse();
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
    mut game_cursor: Query<&mut GameCursor>,
    timer: Res<Time>,
) {
    let mut cursor_visivility = if let Ok(cursor_visivility) = cursor_visivility.get_single_mut() {
        cursor_visivility
    } else {
        return;
    };

    let mut game_cursor = if let Ok(game_cursor) = game_cursor.get_single_mut() {
        game_cursor
    } else {
        return;
    };

    if cursor_moved_er.read().next().is_some() {
        game_cursor.active_time.reset();
    } else {
        game_cursor.active_time.tick(timer.delta());
    }

    if game_cursor.active_time.finished() {
        *cursor_visivility = Visibility::Hidden;
    } else {
        *cursor_visivility = Visibility::Visible;
    }
}

fn cursor_mode(
    cursor_pos: Res<CursorPos>,
    mut cursor_image: Query<&mut Handle<Image>, With<GameCursor>>,
    enemies_pos: Query<&GridCoords, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    let cursor_pos = cursor_pos.world_position();
    let mut cursor_image = if let Ok(cursor_image) = cursor_image.get_single_mut() {
        cursor_image
    } else {
        error!("No image handle found for game cursor, expected to always find one");
        return;
    };

    for enemy_pos in enemies_pos.iter() {
        if cursor_pos == *enemy_pos {
            *cursor_image = asset_server.load("Cursors_v2/Light/Arrows/Arrow4.png");
            return;
        }
    }
    *cursor_image = asset_server.load("Cursors_v2/Light/Arrows/Arrow1.png");
}

fn cursor_direction(
    cursor_pos: Res<CursorPos>,
    player_grid: Query<&GridCoords, With<Player>>,
    mut cursor_direction: ResMut<CursorDirection>,
) {
    let player_pos = get_single!(player_grid);
    let cursor_pos = cursor_pos.world_position();

    let vec_diff = Vec2::new(
        cursor_pos.x as f32 - player_pos.x as f32,
        cursor_pos.y as f32 - player_pos.y as f32,
    );

    *cursor_direction = match vec_diff {
        Vec2 { x, y } if x >= y * 2.0 && x >= y * -2.0 => CursorDirection::Right,
        Vec2 { x, y } if x < y * 2.0 && x > y * 0.5 => CursorDirection::UpRight,
        Vec2 { x, y } if x <= y * 0.5 && x >= y * -0.5 => CursorDirection::Up,
        Vec2 { x, y } if x < y * -0.5 && x > y * -2.0 => CursorDirection::UpLeft,
        Vec2 { x, y } if x <= y * -2.0 && x <= y * 2.0 => CursorDirection::Left,
        Vec2 { x, y } if x > y * 2.0 && x < y * 0.5 => CursorDirection::DownLeft,
        Vec2 { x, y } if x >= y * 0.5 && x <= y * -0.5 => CursorDirection::Down,
        Vec2 { x, y } if x > y * -0.5 && x < y * -2.0 => CursorDirection::DownRight,
        _ => return, //no change
    };
}
