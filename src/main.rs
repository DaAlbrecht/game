use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::{
    assets::{AssetPlugin, Player},
    camera::CameraPlugin,
    movement::MovementPlugin,
    patch_camera, setup, AppState,
};
use iyes_perf_ui::{diagnostics::PerfUiEntryFPS, PerfUiPlugin, PerfUiRoot};

fn main() {
    let mut app = App::new();
    app.insert_state(AppState::Loading)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(AssetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin {
            state: AppState::InGame,
        })
        .add_systems(Startup, setup)
        .add_systems(
            PostUpdate,
            patch_camera.run_if(any_with_component::<Player>.and_then(run_once())),
        );

    if cfg!(debug_assertions) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_plugins(WorldInspectorPlugin::default())
            .add_systems(Startup, debug_plugins);
    }
    app.run();
}

fn debug_plugins(mut commands: Commands) {
    commands.spawn((PerfUiRoot::default(), PerfUiEntryFPS::default()));
}
