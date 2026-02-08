use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    demo::{animation::PlayerAnimation, movement::ScreenWrap, player::PlayerAssets},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (think, walk)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Sheep;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SheepMind {
    neighbor_count: u8,
    goal: Vec2,
    speed: f32,
}

impl Default for SheepMind {
    fn default() -> Self {
        Self {
            neighbor_count: 0,
            goal: Vec2::ZERO,
            speed: 100.,
        }
    }
}

impl SheepMind {
    fn add_goal(&mut self, vec: Vec2) {
        if self.neighbor_count == 0 {
            self.goal = vec;
            self.neighbor_count = 1;
        } else {
            let n = f32::from(self.neighbor_count);
            self.goal = (self.goal * n + vec) / (n + 1.);
            self.neighbor_count += 1;
        }
    }
}

pub fn sheep(player_assets: &PlayerAssets) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let player_animation = PlayerAnimation::new();

    let mut rng = rng();
    let angle = 2. * PI * rng.random::<f32>();
    let distance = 200. * (1. - rng.random::<f32>().powi(2));
    let pos = distance * Vec2::from_angle(angle);

    (
        Name::new("Sheep"),
        Sheep,
        SheepMind::default(),
        Sprite::from_image(player_assets.sheep.clone()),
        Transform {
            translation: pos.extend(0.),
            rotation: Quat::IDENTITY,
            scale: Vec2::splat(2.).extend(1.),
        },
        ScreenWrap,
        player_animation,
    )
}

const RANGE: f32 = 40.;

fn think(mut sheep: Query<(&Transform, &mut SheepMind)>) {
    for (_, mut mind) in &mut sheep {
        *mind = SheepMind::default();
    }
    let mut combo = sheep.iter_combinations_mut::<2>();
    while let Some([(trans1, mut mind1), (trans2, mut mind2)]) = combo.fetch_next() {
        let vec = (trans1.translation - trans2.translation).xy();
        if vec.length() > RANGE {
            continue;
        }

        mind1.add_goal(-vec);
        mind2.add_goal(vec);
    }
}

fn walk(sheep: Query<(&mut Transform, &SheepMind)>, time: Res<Time>) {
    for (mut transf, mind) in sheep {
        let goal = mind.speed * mind.goal.normalize_or_zero().extend(0.);
        transf.translation += time.delta_secs() * goal;
    }
}
