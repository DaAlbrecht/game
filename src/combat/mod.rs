use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use leafwing_input_manager::action_state::ActionState;

use crate::{input::PlayerAction, player::Player, AppState, ABILITY_Z_INDEX};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (test, move_fireball).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
struct FireBall {
    pub speed: f32,
}

fn test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    player_pos: Query<&Transform, With<Player>>,
) {
    let action_state = query.single();

    if action_state.just_pressed(&PlayerAction::Ability1) {
        println!("Ability 1 just pressed");

        let mut fireball_transform = *player_pos.single();
        fireball_transform.translation.z = ABILITY_Z_INDEX;
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(2.0))),
                material: materials.add(Color::RED),
                transform: fireball_transform,
                ..default()
            },
            Name::new("Ability1"),
            FireBall { speed: 1.0 },
        ));
    }
}

fn move_fireball(mut fire_ball: Query<(&mut Transform, &FireBall), With<FireBall>>) {
    for (mut transform, fire_ball) in fire_ball.iter_mut() {
        transform.translation.x += fire_ball.speed;
    }
}
