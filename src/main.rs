use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use game::{
    camera::CameraPlugin,
    combat::CombatPlugin,
    enemy::{health_bar::HealthBarPlugin, slime::SlimePlugin, EnemyPlugin},
    events::EventsPlugin,
    grid::GridPlugin,
    input::InputPlugin,
    ldtk::LdtkAssetPlugin,
    player::PlayerPlugin,
    setup,
    ui::UiPlugin,
    AppState,
};
use iyes_perf_ui::{prelude::PerfUiEntryFPS, ui::root::PerfUiRoot, PerfUiPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<AppState>()
        .add_plugins(LdtkPlugin)
        .add_plugins(LdtkAssetPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(CameraPlugin {
            state: AppState::InGame,
        })
        .add_plugins(InputPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SlimePlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(HealthBarPlugin)
        .add_systems(Startup, setup);

    if cfg!(debug_assertions) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_plugins(WorldInspectorPlugin::default())
            //.add_systems(Last, print_resources)
            .add_systems(Startup, debug_plugins);
    }
    app.run();
}

fn debug_plugins(mut commands: Commands) {
    commands.spawn((
        Name::new("perfui"),
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
    ));
}

#[allow(dead_code)]
fn print_resources(world: &World) {
    let components = world.components();

    let mut r: Vec<_> = world
        .storages()
        .resources
        .iter()
        .map(|(id, _)| components.get_info(id).unwrap())
        .map(|info| info.name())
        .collect();

    // sort list alphabetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}
