use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::{utils::translation_to_grid_coords, GridCoords};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{
    camera::MainCamera, enemy::Enemy, input::move_player, AppState, CURSOR_Z_INDEX, GRID_SIZE,
};

pub struct GameCursorPlugin;

impl Plugin for GameCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_systems(Startup, hide_grab)
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
            .add_plugins(ResourceInspectorPlugin::<CursorPos>::default());
    }
}

#[derive(Component)]
pub struct GameCursor;

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
        //*cursor_visivility = Visibility::Hidden;
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
