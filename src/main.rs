use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::{
    assets::{AssetPlugin, Player},
    camera::CameraPlugin,
    movement::MovementPlugin,
    patch_camera, setup, AppState,
};

fn main() {
    App::new()
        .insert_state(AppState::Loading)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(AssetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin {
            state: AppState::InGame,
        })
        .add_systems(Startup, setup)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(
            PostUpdate,
            patch_camera.run_if(any_with_component::<Player>.and_then(run_once())),
        )
        .run();
}
