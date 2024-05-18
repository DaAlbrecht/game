use bevy::{math::f32, prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;

pub struct CameraPlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for CameraPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera.run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct MainCamera;

#[allow(clippy::type_complexity)]
fn update_camera(
    mut camera: Query<(&mut Transform, &OrthographicProjection), (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    level_query: Query<&LevelIid, (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let Ok((mut camera, projection)) = camera.get_single_mut() else {
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

    let Ok(level_iid) = level_query.get_single() else {
        debug!("Level not found");
        return;
    };

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("Project should be loaded if level has spawned");

    let level = ldtk_project
        .get_raw_level_by_iid(&level_iid.to_string())
        .expect("Spawned level should exist in LDtk project");

    let level_width = level.px_wid as f32;
    let level_height = level.px_hei as f32;

    let player_position = player.translation.truncate();

    let width_offset = window.width() / 2. * projection.scale;
    let height_offset = window.height() / 2. * projection.scale;

    let direction_x = f32::clamp(player_position.x, width_offset, level_width - width_offset);
    let direction_y = f32::clamp(
        player_position.y,
        height_offset,
        level_height - height_offset,
    );

    let to = Vec3::new(
        direction_x - width_offset,
        direction_y - height_offset,
        camera.translation.z,
    );

    camera.translation = to;
}
