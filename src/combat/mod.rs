use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use leafwing_input_manager::action_state::ActionState;

use crate::{
    get_single_mut,
    input::PlayerInputAction,
    player::{Player, PlayerAction},
    AppState, ABILITY_Z_INDEX,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (fireball, move_fireball).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
struct FireBall {
    pub speed: f32,
}

fn fireball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&ActionState<PlayerInputAction>, With<Player>>,
    player_pos: Query<&Transform, With<Player>>,
    mut player_action: Query<&mut PlayerAction, With<Player>>,
) {
    let action_state = query.single();
    let mut player_action = get_single_mut!(player_action);

    if action_state.just_pressed(&PlayerInputAction::Ability1)
        && *player_action != PlayerAction::Combat
    {
        *player_action = PlayerAction::Combat;
        let mut fireball_transform = *player_pos.single();
        fireball_transform.translation.z = ABILITY_Z_INDEX;
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(2.0))),
                material: materials.add(Color::RED),
                transform: fireball_transform,
                ..default()
            },
            Name::new("Fireball"),
            FireBall { speed: 1.0 },
        ));
    }
}

fn move_fireball(mut fire_ball: Query<(&mut Transform, &FireBall), With<FireBall>>) {
    for (mut transform, fire_ball) in fire_ball.iter_mut() {
        transform.translation.x += fire_ball.speed;
    }
}
