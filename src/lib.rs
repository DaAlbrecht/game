use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use camera::MainCamera;
pub mod assets;
pub mod camera;
pub mod enemy;
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
    camera.projection.scale = 0.2;
    camera.projection.viewport_origin = Vec2::ZERO;
    commands.spawn((camera, MainCamera));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("game.ldtk"),
        ..Default::default()
    });
}
