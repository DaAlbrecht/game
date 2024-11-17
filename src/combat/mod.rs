use bevy::{
    color::palettes::css,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_ecs_ldtk::GridCoords;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    enemy::Enemy,
    get_single, get_single_mut,
    input::PlayerInputAction,
    player::{Player, PlayerAction},
    ui::game_cursor::{AttackCursor, CursorPos},
    AppState, Health, ABILITY_Z_INDEX,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (fireball, move_fireball).run_if(in_state(AppState::InGame)),
        )
        .observe(on_target_hit);
    }
}

#[derive(Component)]
struct Ability {
    target: Entity,
    origin: Entity,
}

#[derive(Component)]
struct Fireball {
    speed: f32,
    damage: i32,
}

#[derive(Event)]
struct HitEvent {
    target: Entity,
    origin: Entity,
    ability: Entity,
    damage: i32,
}

const FIREBALL_DAMAGE: i32 = 50;

//TODO: isntead of checking for enemies, we rather should use a 'target' component.
fn fireball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_action: Query<&mut PlayerAction, With<Player>>,
    input_q: Query<&ActionState<PlayerInputAction>, With<Player>>,
    player_q: Query<(Entity, &Transform), With<Player>>,
    enemies_q: Query<(Entity, &GridCoords), With<Enemy>>,
    attack_cursor_q: Query<&AttackCursor>,
    cursor_pos: Res<CursorPos>,
) {
    // only allow to cast fireball when the player has a target
    // TODO: account for the range of spells
    if attack_cursor_q.get_single().is_err() {
        return;
    }

    let action_state = input_q.single();
    let mut player_action = get_single_mut!(player_action);

    let enemy_entity = enemies_q.iter().find_map(|(entity, coords)| {
        if coords == &cursor_pos.world_position() {
            Some(entity)
        } else {
            None
        }
    });

    let enemy_entity = match enemy_entity {
        Some(entity) => entity,
        None => return,
    };

    if action_state.just_pressed(&PlayerInputAction::Ability1)
        && *player_action != PlayerAction::Combat
    {
        *player_action = PlayerAction::Combat;
        let (player_entity, fireball_transform) = get_single!(player_q);
        let mut fireball_transform = *fireball_transform;
        fireball_transform.translation.z = ABILITY_Z_INDEX;
        let fireball_entity = commands
            .spawn((
                Ability {
                    origin: player_entity,
                    target: enemy_entity,
                },
                Fireball {
                    speed: 1.0,
                    damage: FIREBALL_DAMAGE,
                },
                Name::new("Fireball"),
            ))
            .id();

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
    mut fireball_q: Query<(Entity, &mut Transform, &Fireball, &Ability), With<Ability>>,
    enemies_q: Query<&Transform, (With<Enemy>, Without<Ability>)>,
    mut commands: Commands,
    mut player_action: Query<&mut PlayerAction, With<Player>>,
) {
    let mut player_action = get_single_mut!(player_action);
    for (ability_entity, mut transform, fire_ball, ability) in fireball_q.iter_mut() {
        let target = enemies_q.get(ability.target).unwrap();

        let direction = target.translation - transform.translation;
        let distance = direction.length();
        let direction = direction.normalize();
        let movement = direction * fire_ball.speed;

        if distance < 1.0 {
            commands.trigger(HitEvent {
                target: ability.target,
                origin: ability.origin,
                ability: ability_entity,
                damage: fire_ball.damage,
            });
            *player_action = PlayerAction::Idle;
        } else {
            transform.translation += movement;
        }
    }
}

fn on_target_hit(
    trigger: Trigger<HitEvent>,
    mut health_q: Query<&mut Health>,
    mut commands: Commands,
) {
    let hit_event = trigger.event();
    commands.entity(hit_event.ability).despawn();
    if let Ok(mut target_health) = health_q.get_mut(hit_event.target) {
        target_health.current_health -= hit_event.damage;
    }
}
