use bevy::prelude::*;
use sickle_ui::{
    ui_builder::{UiBuilder, UiBuilderExt, UiRoot},
    ui_style::{
        SetBackgroundColorExt, SetNodeHeightExt, SetNodePositionTypeExt, SetNodeRightExt,
        SetNodeTopExt, SetNodeWidthExt,
    },
    widgets::container::UiContainerExt,
};

use super::PlayerHud;

pub(crate) struct PlayerWidgetPlugin;
impl Plugin for PlayerWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_widget);
    }
}

#[derive(Component)]
struct PlayerWidget;

pub trait PlayerWidgetExt<'w, 's> {
    fn player_widget<'a>(&'a mut self) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> PlayerWidgetExt<'w, 's> for UiBuilder<'w, 's, '_, UiRoot> {
    fn player_widget<'a>(&'a mut self) -> UiBuilder<'w, 's, 'a, Entity> {
        self.container(
            (ImageBundle::default(), (PlayerWidget, PlayerHud)),
            |player_widget| {
                player_widget
                    .style()
                    .position_type(PositionType::Absolute)
                    .right(Val::Percent(20.0))
                    .top(Val::Percent(80.0))
                    .background_color(Color::GREEN)
                    .width(Val::Px(50.0))
                    .height(Val::Px(50.0));
                //.width(Val::Auto);
            },
        )
    }
}

fn spawn_player_widget(mut commands: Commands) {
    commands.ui_builder(UiRoot).player_widget();
}
