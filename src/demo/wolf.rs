use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    camera::{GAME_HEIGHT, GAME_WIDTH},
    demo::level::Level,
};

pub fn plugin(app: &mut App) {
    app.load_resource::<WolfAssets>();
    app.add_systems(
        Update,
        spawn.in_set(AppSystems::Update).in_set(PausableSystems),
    );
}

impl FromWorld for WolfAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            wolf: assets.load_with_settings(
                "images/wolf.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WolfAssets {
    #[dependency]
    pub wolf: Handle<Image>,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wolf;

pub struct WolfSpawnStatus(Timer);

#[cfg(feature = "dev")]
const SECONDS_TO_SPAWN: f32 = 1.;
#[cfg(not(feature = "dev"))]
const SECONDS_TO_SPAWN: f32 = 5.;
const MAX_NUMBER: usize = 1;

impl Default for WolfSpawnStatus {
    fn default() -> Self {
        Self(Timer::from_seconds(SECONDS_TO_SPAWN, TimerMode::Repeating))
    }
}

fn spawn(
    mut commands: Commands,
    mut timer: Local<WolfSpawnStatus>,
    time: Res<Time>,
    level: Query<Entity, With<Level>>,
    wolves: Query<(), With<Wolf>>,
    assets: If<Res<WolfAssets>>,
) {
    let Some(level) = level.iter().next() else {
        return;
    };

    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let count_wolves = wolves.count();

    if count_wolves >= MAX_NUMBER {
        return;
    }

    let mut rng = rng();

    let position_x = rng.random_range((-GAME_WIDTH / 2. + 16.)..(GAME_WIDTH / 2. - 16.));
    let position_y = rng.random_range((-GAME_HEIGHT / 2. + 16.)..(GAME_HEIGHT / 2. - 16.));
    let transform = Transform {
        translation: Vec3::new(position_x, position_y, 0.),
        scale: Vec2::splat(1.).extend(0.),
        ..Default::default()
    };

    commands.spawn((
        Name::new("Wolf"),
        transform,
        Wolf,
        Sprite::from_image(assets.wolf.clone()),
        ChildOf(level),
    ));
}
