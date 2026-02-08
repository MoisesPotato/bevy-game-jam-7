use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{Rng, rng};

use crate::demo::{animation::PlayerAnimation, movement::ScreenWrap, player::PlayerAssets};

pub fn plugin(app: &mut App) {}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Sheep;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SheepMind {}

pub fn sheep(player_assets: &PlayerAssets) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let player_animation = PlayerAnimation::new();

    let mut rng = rng();
    let angle = 2. * PI * rng.random::<f32>();
    let distance = 200. * (1. - rng.random::<f32>().powi(2));

    (
        Name::new("Sheep"),
        Sheep,
        // Player,
        Sprite::from_image(player_assets.sheep.clone()),
        Transform {
            translation: (distance * Vec2::from_angle(angle)).extend(0.),
            rotation: Quat::IDENTITY,
            scale: Vec2::splat(2.).extend(1.),
        },
        ScreenWrap,
        player_animation,
    )
}
