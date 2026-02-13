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

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CabbageAssets>();
    app.add_systems(
        Update,
        spawn.in_set(AppSystems::Update).in_set(PausableSystems),
    );
}

impl FromWorld for CabbageAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            cabbage: assets.load_with_settings(
                "images/cabbage.png",
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
pub struct CabbageAssets {
    #[dependency]
    pub cabbage: Handle<Image>,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Cabbage;

pub struct CabbageTimer(Timer);

const SECONDS_TO_CABBAGE: f32 = 3.;
const SPAWN_CHANCE: f32 = 0.8;
const MAX_NUMBER: usize = 3;

impl Default for CabbageTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            SECONDS_TO_CABBAGE,
            TimerMode::Repeating,
        ))
    }
}

pub fn spawn(
    mut commands: Commands,
    mut timer: Local<CabbageTimer>,
    time: Res<Time>,
    level: Query<Entity, With<Level>>,
    cabbages: Query<(), With<Cabbage>>,
    assets: If<Res<CabbageAssets>>,
) {
    let Some(level) = level.iter().next() else {
        return;
    };

    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let count_cabbages = cabbages.count();

    if count_cabbages >= MAX_NUMBER {
        return;
    }

    let mut rng = rng();

    if rng.random::<f32>() > SPAWN_CHANCE {
        return;
    }

    let position_x = rng.random_range((-GAME_WIDTH / 2. + 16.)..(GAME_WIDTH / 2. - 16.));
    let position_y = rng.random_range((-GAME_HEIGHT / 2. + 16.)..(GAME_HEIGHT / 2. - 16.));
    let transform = Transform {
        translation: Vec3::new(position_x, position_y, 0.),
        scale: Vec2::splat(1.).extend(0.),
        ..Default::default()
    };

    commands.spawn((
        Name::new("Cabbage"),
        transform,
        Cabbage,
        Sprite::from_image(assets.cabbage.clone()),
        ChildOf(level),
    ));
}
