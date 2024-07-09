use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use camera::MainCamera;
pub mod camera;

pub mod combat;
pub mod enemy;
pub mod events;
pub mod grid;
pub mod input;
pub mod ldtk;
pub mod player;
pub mod ui;

pub const GRID_SIZE: i32 = 16;
pub const ACTION_DELAY: f32 = 0.2;

pub const CURSOR_Z_INDEX: f32 = 100.0;
pub const ABILITY_Z_INDEX: f32 = 11.0;

#[derive(Component, Reflect)]
pub struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct IdleAnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ActionTimer(Timer);

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
            self.nth = 1;
            Some(self.indecies[0])
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
    Menu,
}

#[derive(Component)]
pub struct GameCursor;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.2;
    camera.projection.viewport_origin = Vec2::ZERO;
    commands.spawn((camera, MainCamera));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("game.ldtk"),
        ..Default::default()
    });

    let cursor_scale = 0.1;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Cursors_v2/Light/Arrows/Arrow1.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, CURSOR_Z_INDEX),
                scale: Vec3::splat(cursor_scale),
                ..default()
            },
            ..default()
        },
        GameCursor,
        Name::new("Cursor"),
    ));
}
