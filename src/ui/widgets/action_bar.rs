use bevy::{color::palettes::css, prelude::*};
use sickle_ui::{
    prelude::*,
    ui_builder::{UiBuilder, UiBuilderExt, UiRoot},
};

use crate::ui::PlayerHud;

pub struct ActionBarPlugin;
impl Plugin for ActionBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_action_bar_widget);
    }
}

#[derive(Component)]
struct ActionBarWidget;

#[derive(Component)]
pub struct AbilitySlot(u8);

pub struct ActionBarWidgetConfig {
    pub ability_count: u8,
}

impl Default for ActionBarWidgetConfig {
    fn default() -> Self {
        Self { ability_count: 4 }
    }
}

pub trait ActionBarWidgetExt {
    fn action_bar_widget(&mut self, config: ActionBarWidgetConfig) -> UiBuilder<Entity>;
}

impl ActionBarWidgetExt for UiBuilder<'_, UiRoot> {
    fn action_bar_widget(&mut self, config: ActionBarWidgetConfig) -> UiBuilder<Entity> {
        self.container(
            (NodeBundle::default(), (ActionBarWidget, PlayerHud)),
            |action_bar| {
                let entity = action_bar.id();
                action_bar
                    .commands()
                    .entity(entity)
                    .insert(Name::new("ActionBarWidget"));

                action_bar
                    .style()
                    .position_type(PositionType::Absolute)
                    .top(Val::Percent(80.0))
                    .width(Val::Percent(100.0))
                    .background_color(Color::NONE);

                let mut row = action_bar.row(|_| {});
                row.style().justify_content(JustifyContent::Center);

                for i in 0..config.ability_count {
                    row.container(NodeBundle::default(), |column| {
                        let entity = column.id();
                        column
                            .commands()
                            .entity(entity)
                            .insert((Name::new(format!("{}-AbilitySlot", i)), AbilitySlot(i)));

                        column
                            .style()
                            .width(Val::Px(50.0))
                            .height(Val::Px(50.0))
                            .background_color(css::CORAL.into())
                            .margin(UiRect::all(Val::Px(5.0)));
                    });
                }
            },
        )
    }
}

fn spawn_action_bar_widget(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .action_bar_widget(ActionBarWidgetConfig::default());
}
