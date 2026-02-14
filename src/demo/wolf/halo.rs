use bevy::asset::Asset;
use bevy::ecs::prelude::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

use crate::theme::palette::RESURRECT_PALETTE;

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HaloMaterial {
    #[uniform(0)]
    pub background_color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}

impl HaloMaterial {
    pub fn new(image: Handle<Image>) -> Self {
        Self {
            background_color: RESURRECT_PALETTE[35].into(),
            color_texture: Some(image),
        }
    }
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for HaloMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/halo_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}
