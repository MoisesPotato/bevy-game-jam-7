use std::{cmp::Ordering, time::Duration};

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    sprite_render::Material2dPlugin,
};
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    camera::{GAME_HEIGHT, GAME_WIDTH},
    demo::{level::Level, movement::HumanMind, sheep::Sheep, wolf::halo::HaloMaterial},
    screens::Screen,
};

mod halo;

pub fn plugin(app: &mut App) {
    app.init_asset::<HaloMaterial>();
    app.add_plugins(Material2dPlugin::<HaloMaterial>::default());
    app.load_resource::<WolfAssets>();
    app.add_systems(
        Update,
        (spawn, (think_eat, hunt).chain())
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
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
            halo_mesh: assets.add(Circle::new(1.).into()),
            halo_mat: assets.add(HaloMaterial::new(assets.load("images/cabbage.png"))),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WolfAssets {
    #[dependency]
    pub wolf: Handle<Image>,
    #[dependency]
    pub halo_mesh: Handle<Mesh>,
    #[dependency]
    pub halo_mat: Handle<HaloMaterial>,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wolf {
    prey: Option<Entity>,
    time_left: Timer,
}

impl Default for Wolf {
    fn default() -> Self {
        Self {
            prey: None,
            time_left: Timer::new(
                Duration::from_secs_f32(THINK_INTERVAL_HUNGRY),
                TimerMode::Repeating,
            ),
        }
    }
}

pub struct WolfSpawnStatus(Timer);

#[cfg(feature = "dev")]
const SECONDS_TO_SPAWN: f32 = 0.2;
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

    commands
        .spawn((
            Name::new("Wolf"),
            transform,
            Wolf::default(),
            Sprite::from_image(assets.wolf.clone()),
            ChildOf(level),
        ))
        .with_child((
            Transform {
                translation: Vec3::new(0., 0., -1.),
                scale: Vec2::splat(40.).extend(0.),
                rotation: Quat::default(),
            },
            Mesh2d(assets.halo_mesh.clone()),
            MeshMaterial2d(assets.halo_mat.clone()),
        ));
}

const THINK_INTERVAL_HUNGRY: f32 = 0.5;
const THINK_INTERVAL_FULL: f32 = 1.0;
const EAT_RANGE: f32 = 16.;

fn think_eat(
    mut commands: Commands,
    time: Res<Time>,
    wolf: Query<(&Transform, &mut Wolf)>,
    sheep: Query<(Entity, &Transform, Option<&HumanMind>), With<Sheep>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for (transf, mut wolf) in wolf {
        wolf.time_left.tick(time.delta());

        let pos = transf.translation.xy();
        let Some((id, dist, human)) = sheep
            .into_iter()
            .map(|(id, t, h)| {
                let sheep = t.translation.xy();
                let dist = (pos - sheep).length();
                (id, dist as u32, h.is_some())
            })
            .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(Ordering::Equal))
        else {
            error!("No sheep");
            return;
        };

        if (dist as f32) < EAT_RANGE {
            commands.entity(id).despawn();
            wolf.prey = None;
            wolf.time_left
                .set_duration(Duration::from_secs_f32(THINK_INTERVAL_FULL));
            wolf.time_left.reset();
            if human {
                next_screen.set(Screen::GameOver);
            }
        } else if wolf.time_left.just_finished() {
            wolf.time_left
                .set_duration(Duration::from_secs_f32(THINK_INTERVAL_HUNGRY));
            // Consider making it the sheep id
            wolf.prey = Some(id);
        }
    }
}

const SPEED: f32 = 100.;

fn hunt(
    time: Res<Time>,
    wolf: Query<(&mut Transform, &mut Wolf)>,
    sheep: Query<&Transform, Without<Wolf>>,
) {
    for (mut transform, mut think) in wolf {
        let Some(prey) = think.prey else {
            continue;
        };
        let Ok(prey) = sheep.get(prey) else {
            think.prey = None;
            continue;
        };
        let target = (prey.translation - transform.translation).normalize_or_zero();

        transform.translation += target * SPEED * time.delta_secs();
    }
}
