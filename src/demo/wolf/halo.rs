use bevy::asset::Asset;
use bevy::ecs::prelude::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HaloMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Default for HaloMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::new(1., 0., 0., 1.),
            color_texture: None,
        }
    }
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for HaloMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/halo_material.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode2d {
    //     AlphaMode2d::Mask(0.5)
    // }
}
