use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{player::Player, GRID_SIZE};

pub struct CameraPlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for CameraPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_camera, translate_grid_coords_camera).run_if(in_state(self.state.clone())),
        );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn move_camera(
    mut camera_query: Query<&mut GridCoords, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&GridCoords, With<Player>>,
) {
    let player_pos = player_query.get_single().expect("Player should exist");
    let mut camera_pos = camera_query.get_single_mut().expect("Camera should exist");
    camera_pos.x = player_pos.x - 8;
    camera_pos.y = player_pos.y - 8;
}

fn translate_grid_coords_camera(
    mut query: Query<(&mut Transform, &GridCoords), (Changed<GridCoords>, With<MainCamera>)>,
) {
    for (mut transform, grid_coords) in query.iter_mut() {
        let translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE));

        //TODO: Add clamping to match the level bounds
        transform.translation = translation.extend(transform.translation.z);
    }
}
