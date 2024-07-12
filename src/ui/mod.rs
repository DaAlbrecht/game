use self::{action_bar::ActionBarPlugin, player_widget::PlayerWidgetPlugin};
use bevy::prelude::*;
use game_cursor::GameCursorPlugin;

mod action_bar;
pub mod game_cursor;
mod player_widget;

pub struct UiPlugin;

#[derive(Component)]
struct PlayerHud;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionBarPlugin)
            .add_plugins(PlayerWidgetPlugin)
            .add_plugins(GameCursorPlugin)
            .add_systems(Update, toggle_ui);
    }
}

fn toggle_ui(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Visibility, With<PlayerHud>>) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    for mut ui_visibility in query.iter_mut() {
        if *ui_visibility == Visibility::Visible {
            *ui_visibility = Visibility::Hidden;
        } else {
            *ui_visibility = Visibility::Visible;
        }
    }
}
