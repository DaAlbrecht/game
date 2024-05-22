use bevy::prelude::*;
use sickle_ui::{
    ui_builder::{UiBuilder, UiBuilderExt, UiRoot},
    ui_commands::SetTextExt,
    ui_style::{
        SetBackgroundColorExt, SetNodeAlignSelfExt, SetNodeHeightExt, SetNodeJustifyContentsExt,
        SetNodePositionTypeExt, SetNodeRightExt, SetNodeTopExt, SetNodeWidthExt,
    },
    widgets::{
        column::UiColumnExt,
        container::UiContainerExt,
        label::{LabelConfig, UiLabelExt},
    },
};

pub struct ActionBarPlugin;

impl Plugin for ActionBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_simple_widget);
    }
}

#[derive(Component)]
struct ActionBarWidget;

pub trait ActionBarWidgetExt<'w, 's> {
    fn action_bar_widget<'a>(
        &'a mut self,
        config: ActionBarWidgetConfig,
    ) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> ActionBarWidgetExt<'w, 's> for UiBuilder<'w, 's, '_, UiRoot> {
    fn action_bar_widget<'a>(
        &'a mut self,
        config: ActionBarWidgetConfig,
    ) -> UiBuilder<'w, 's, 'a, Entity> {
        self.container((ImageBundle::default(), ActionBarWidget), |banner| {
            banner
                .style()
                .position_type(PositionType::Absolute)
                // Center the children (the label) horizontally.
                .justify_content(JustifyContent::Center)
                .width(Val::Px(401.0))
                .height(Val::Px(79.0));

            // And we'll want a customizable label on the banner.
            let mut label = banner.label(LabelConfig::default());

            label
                .style()
                // Align the label relative to the top of the banner.
                .align_self(AlignSelf::Start)
                // Move us a few pixels down so we look nice relative to our font.
                .top(Val::Px(20.0));

            // We would like to set a default text style without having to pass in the AssetServer.
            label.entity_commands().set_text(config.label, None);
        })
    }
}

pub struct ActionBarWidgetConfig {
    pub label: String,
}

impl ActionBarWidgetConfig {
    pub fn from(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

fn spawn_simple_widget(mut commands: Commands) {
    commands.ui_builder(UiRoot).column(|column| {
        // We can style our widget directly in code using the style method.
        column
            .style()
            // The column will be located 100 pixels from the right and 100 pixels from the top of the screen.
            // The absolute position means we are not set relative to any parent.
            .position_type(PositionType::Absolute)
            .right(Val::Px(100.0))
            .top(Val::Px(500.))
            // We'll bound the height of our column to the total height of our contents.
            // By default, a column will be 100% of the parent's height which would be the entire length of the screen.,
            .height(Val::Auto)
            // Lets give it a visible background color.
            .background_color(Color::rgb(0.5, 0.5, 0.5));

        column
            .label(LabelConfig::default())
            .entity_commands()
            // We can use the set_text method to set the text of a label.
            .set_text("This is label 1.", None);

        column
            .label(LabelConfig::default())
            .entity_commands()
            .set_text("This is another label.", None);
    });
}
