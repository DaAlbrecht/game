use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<HealthBarMaterial>::default());
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct HealthBarMaterial {
    #[uniform(0)]
    pub foreground_color: LinearRgba,
    #[uniform(0)]
    pub background_color: LinearRgba,
    #[uniform(0)]
    pub percent: f32,
}

impl Material2d for HealthBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/health_bar.wgsl".into()
    }
}
