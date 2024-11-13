use bevy::{
    color::palettes::css,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_ecs_ldtk::GridCoords;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    enemy::Enemy,
    get_single_mut,
    input::PlayerInputAction,
    player::{Player, PlayerAction},
    ui::game_cursor::{AttackCursor, CursorPos},
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
struct Fireball {
    pub speed: f32,
}

#[derive(Component)]
struct Target {
    pub origin_entity: Entity,
}

fn fireball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&ActionState<PlayerInputAction>, With<Player>>,
    player_pos: Query<&Transform, With<Player>>,
    mut player_action: Query<&mut PlayerAction, With<Player>>,
    enemies_q: Query<(Entity, &GridCoords), With<Enemy>>,
    attack_cursor_q: Query<&AttackCursor>,
    cursor_pos: Res<CursorPos>,
) {
    // only allow to cast fireball when the player has a target
    // TODO: account for the range of spells
    if attack_cursor_q.get_single().is_err() {
        return;
    }

    let action_state = query.single();
    let mut player_action = get_single_mut!(player_action);

    let enemy_entity = enemies_q.iter().find_map(|(entity, coords)| {
        if coords == &cursor_pos.world_position() {
            Some(entity)
        } else {
            None
        }
    });

    if action_state.just_pressed(&PlayerInputAction::Ability1)
        && *player_action != PlayerAction::Combat
    {
        *player_action = PlayerAction::Combat;
        let mut fireball_transform = *player_pos.single();
        fireball_transform.translation.z = ABILITY_Z_INDEX;
        let fireball_entity = commands
            .spawn((Fireball { speed: 1.0 }, Name::new("Fireball")))
            .id();

        commands.entity(enemy_entity.unwrap()).insert(Target {
            origin_entity: fireball_entity,
        });

        commands
            .entity(fireball_entity)
            .insert((MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(2.0))),
                material: materials.add(Color::from(css::RED)),
                transform: fireball_transform,
                ..default()
            },));
    }
}

fn move_fireball(
    mut fireball_q: Query<(&mut Transform, &Fireball, Entity), With<Fireball>>,
    target_q: Query<(&Transform, &Target), (With<Enemy>, Without<Fireball>)>,
    mut commands: Commands,
    mut player_action: Query<&mut PlayerAction, With<Player>>,
) {
    let mut player_action = get_single_mut!(player_action);
    for (mut transform, fire_ball, fireball_entity) in fireball_q.iter_mut() {
        for (target_transform, target) in target_q.iter() {
            if target.origin_entity == fireball_entity {
                let direction = target_transform.translation - transform.translation;
                let distance = direction.length();
                let direction = direction.normalize();
                let movement = direction * fire_ball.speed;

                if distance < 1.0 {
                    commands.entity(fireball_entity).despawn();
                    *player_action = PlayerAction::Idle;
                } else {
                    transform.translation += movement;
                }
            }
        }
    }
}
