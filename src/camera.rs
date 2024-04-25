use bevy::{math::f32, prelude::*, window::PrimaryWindow};

use crate::{player::Player, smooth_damp, AppState};

pub struct CameraPlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for CameraPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), patch_camera)
            .add_systems(Update, update_camera.run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn patch_camera(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        debug!("Camera2d not found");
        return;
    };

    let Ok(player) = player.get_single() else {
        debug!("Player not found");
        return;
    };

    let Ok(window) = window_query.get_single() else {
        debug!("Window not found");
        return;
    };
    let player_position = player.translation.truncate();

    let width_offset = window.width() / 6.0;
    let height_offset = window.height() / 6.0;

    let direction = Vec3::new(
        player_position.x - width_offset,
        player_position.y - height_offset,
        camera.translation.z,
    );
    camera.translation = direction;
    next_state.set(AppState::InGame);
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        debug!("Camera2d not found");
        return;
    };

    let Ok(player) = player.get_single() else {
        debug!("Player not found");
        return;
    };

    let Ok(window) = window_query.get_single() else {
        debug!("Window not found");
        return;
    };

    let player_position = player.translation.truncate();

    let width_offset = window.width() / 6.0;
    let height_offset = window.height() / 6.0;

    let direction = Vec3::new(
        player_position.x - width_offset,
        player_position.y - height_offset,
        camera.translation.z,
    );

    camera.translation = smooth_damp(
        camera.translation,
        direction,
        0.07,
        f32::INFINITY,
        time.delta_seconds(),
    );
}
