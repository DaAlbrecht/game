use bevy::prelude::*;
use sickle_ui::{
    prelude::*,
    ui_builder::{UiBuilder, UiBuilderExt, UiRoot},
    ui_commands::SetTextExt,
};

use super::PlayerHud;

pub struct ActionBarPlugin;
impl Plugin for ActionBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_action_bar_widget);
    }
}

#[derive(Component)]
struct AbilitySlot;

#[derive(Component)]
struct ActionBarWidget;

pub trait ActionBarWidgetExt {
    fn action_bar_widget(&mut self) -> UiBuilder<Entity>;
}

impl ActionBarWidgetExt for UiBuilder<'_, UiRoot> {
    fn action_bar_widget(&mut self) -> UiBuilder<Entity> {
        self.container(
            (ImageBundle::default(), (ActionBarWidget, PlayerHud)),
            |action_bar| {
                let entity = action_bar.id();
                action_bar
                    .commands()
                    .entity(entity)
                    .insert(Name::new("ActionBarWidget"));
                action_bar
                    .style()
                    .position_type(PositionType::Absolute)
                    .right(Val::Percent(50.0))
                    .top(Val::Percent(80.0))
                    .background_color(Color::NONE)
                    .width(Val::Auto);

                let margin = UiRect::all(Val::Px(5.0));

                let mut row = action_bar.row(|_| {});

                row.container(ImageBundle::default(), |column| {
                    let entity = column.id();
                    column
                        .commands()
                        .entity(entity)
                        .insert((Name::new("Q_Ability_slot"), AbilitySlot));
                    column
                        .style()
                        .width(Val::Px(50.0))
                        .height(Val::Px(50.0))
                        .margin(margin)
                        .image(ImageSource::Path("fireball_on_cd.png".to_string()));

                    let mut label = column.label(LabelConfig::default());

                    label
                        .style()
                        .position_type(PositionType::Absolute)
                        .bottom(Val::Px(0.0))
                        .right(Val::Px(0.0));

                    label.entity_commands().set_text("q", None);
                });

                row.container(ImageBundle::default(), |column| {
                    let entity = column.id();
                    column
                        .commands()
                        .entity(entity)
                        .insert((Name::new("E_Ability_slot"), AbilitySlot));
                    column
                        .style()
                        .width(Val::Px(50.0))
                        .height(Val::Px(50.0))
                        .margin(margin)
                        .image(ImageSource::Path("earth.png".to_string()));

                    let mut label = column.label(LabelConfig::default());

                    label
                        .style()
                        .position_type(PositionType::Absolute)
                        .bottom(Val::Px(0.0))
                        .right(Val::Px(0.0));

                    label.entity_commands().set_text("e", None);
                });
            },
        )
    }
}

pub struct ActionBarWidgetConfig {}

impl ActionBarWidgetConfig {
    pub fn from() -> Self {
        Self {}
    }
}

fn spawn_action_bar_widget(mut commands: Commands) {
    commands.ui_builder(UiRoot).action_bar_widget();
}
