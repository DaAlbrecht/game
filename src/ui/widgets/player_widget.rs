use bevy::{color::palettes::css, prelude::*};
use sickle_ui::{
    prelude::*,
    ui_builder::{UiBuilder, UiBuilderExt, UiRoot},
};

use crate::ui::PlayerHud;

pub(crate) struct PlayerWidgetPlugin;

impl Plugin for PlayerWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_widget);
    }
}

#[derive(Component)]
struct PlayerWidget;

pub trait PlayerWidgetExt {
    fn player_widget(&mut self) -> UiBuilder<Entity>;
}

impl PlayerWidgetExt for UiBuilder<'_, UiRoot> {
    fn player_widget(&mut self) -> UiBuilder<Entity> {
        self.container(
            (ImageBundle::default(), (PlayerWidget, PlayerHud)),
            |player_widget| {
                let entity = player_widget.id();
                player_widget
                    .commands()
                    .entity(entity)
                    .insert(Name::new("PlayerWidget"));

                player_widget
                    .style()
                    .position_type(PositionType::Absolute)
                    .left(Val::Percent(10.0))
                    .top(Val::Percent(70.0))
                    .background_color(css::GREEN.into())
                    .width(Val::Percent(20.0))
                    .height(Val::Percent(20.0));
            },
        )
    }
}

fn spawn_player_widget(mut commands: Commands) {
    commands.ui_builder(UiRoot).player_widget();
}
