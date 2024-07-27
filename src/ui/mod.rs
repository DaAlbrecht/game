use bevy::prelude::*;
use game_cursor::GameCursorPlugin;
use sickle_ui::SickleUiPlugin;
use widgets::{action_bar::ActionBarPlugin, player_widget::PlayerWidgetPlugin};

pub mod game_cursor;
pub mod widgets;

pub struct UiPlugin;

#[derive(Component)]
struct PlayerHud;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SickleUiPlugin)
            .add_plugins(ActionBarPlugin)
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
