pub mod action_bar;
use bevy::prelude::*;

use self::action_bar::ActionBarPlugin;

pub struct UiPlugin;

#[derive(Component)]
struct MainUi;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionBarPlugin)
            .add_systems(Update, toggle_ui);
    }
}

fn toggle_ui(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Visibility, With<MainUi>>) {
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
