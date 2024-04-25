use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use camera::MainCamera;
pub mod assets;
pub mod camera;
pub mod movement;
pub mod player;
pub mod slime;
pub mod turn;

pub const GRID_SIZE: i32 = 16;
pub const ACTION_DELAY: f32 = 0.2;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct IdleAnimationTimer(Timer);

pub struct IndeciesIter {
    pub indecies: Vec<usize>,
    pub nth: usize,
}

impl From<Vec<usize>> for IndeciesIter {
    fn from(indecies: Vec<usize>) -> Self {
        Self { indecies, nth: 0 }
    }
}

impl Iterator for IndeciesIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nth < self.indecies.len() {
            let index = self.indecies[self.nth];
            self.nth += 1;
            Some(index)
        } else {
            self.nth = 0;
            Some(self.indecies[self.nth])
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameplaySet {
    InputSet,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.3;
    camera.projection.viewport_origin = Vec2::ZERO;
    commands.spawn((camera, MainCamera));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("game.ldtk"),
        ..Default::default()
    });
}

//https://www.alexisbacot.com/blog/the-art-of-damping
pub fn smooth_damp(
    from: Vec3,
    to: Vec3,
    mut smooth_time: f32,
    max_speed: f32,
    delta_time: f32,
) -> Vec3 {
    smooth_time = f32::max(0.0001, smooth_time);
    let omega = 2.0 / smooth_time;

    let x = omega * delta_time;
    let exp = 1.0 / (1. + x + 0.48 * x * x + 0.235 * x * x * x);

    let mut distance_x = from.x - to.x;
    let mut distance_y = from.y - to.y;
    let max_distance = max_speed * smooth_time;

    distance_x = f32::clamp(distance_x, -max_distance, max_distance);
    distance_y = f32::clamp(distance_y, -max_distance, max_distance);

    let x = to.x + (distance_x + omega * distance_x * delta_time) * exp;
    let y = to.y + (distance_y + omega * distance_y * delta_time) * exp;
    Vec3::new(x, y, from.z)
}
