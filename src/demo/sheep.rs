use std::{
    cmp::{Ordering, min},
    collections::{HashMap, hash_map::Entry},
    f32::consts::PI,
    time::Duration,
};

use bevy::{
    input::common_conditions::input_just_pressed, prelude::*, time::common_conditions::on_timer,
};
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    camera::{GAME_HEIGHT, GAME_WIDTH},
    demo::{
        animation::SheepAnimation,
        level::N_SHEEP,
        movement::{HumanMind, ScreenWrap},
        player::PlayerAssets,
    },
    screens::Screen,
};

pub mod bleat;
pub mod ego;

pub fn plugin(app: &mut App) {
    app.load_resource::<SheepAssets>();

    app.add_systems(
        Update,
        (collision, think, walk)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        Update,
        bleat::with_b
            .run_if(input_just_pressed(KeyCode::KeyB))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        Update,
        (bleat::tick, bleat::spread, bleat::random, ego::jump)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(Update, bleat::despawn_image.in_set(AppSystems::Update));
    app.add_systems(
        Update,
        respawn_dead
            .run_if(on_timer(Duration::from_secs_f32(
                // Hopefully it doesn't align with other lol
                1.62,
            )))
            .in_set(AppSystems::Update),
    );

    app.add_plugins(ego::plugin);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Sheep;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SheepMind {
    pub state: State,
    time_left: Timer,
}

const SHEEP_AWARENESS: usize = 4;

#[derive(Reflect, Debug)]
pub enum State {
    Moving { goal: Vec2, speed: f32 },
    Obseerving { neighbors: Vec<Vec2> },
    Idle,
}

impl State {
    const fn new_thinking() -> Self {
        Self::Obseerving { neighbors: vec![] }
    }

    fn target_if_thinking(&mut self, goal: Vec2) {
        let Self::Obseerving { neighbors } = self else {
            return;
        };

        neighbors.push(goal);
    }

    fn conclude_from_observation(&mut self) {
        let Self::Obseerving { neighbors } = self else {
            return;
        };

        let count = min(neighbors.len(), SHEEP_AWARENESS);

        if count == 0 {
            *self = Self::Moving {
                goal: Vec2::ZERO,
                speed: 0.,
            };
            return;
        }

        neighbors.sort_unstable_by(|a, b| {
            a.length()
                .partial_cmp(&b.length())
                .unwrap_or(Ordering::Equal)
        });

        if neighbors.len() > 1 {
            assert!(neighbors[0].length() <= neighbors[1].length());
        }

        if neighbors[0].length() < COLLISION_DISTANCE {
            *self = Self::Moving {
                goal: -neighbors[0],
                speed: 150.,
            };
        } else {
            let goal = neighbors
                .iter()
                .map(|v| {
                    if v.length() <= AVOID_RANGE {
                        -v * 0.5
                    } else {
                        *v
                    }
                })
                .take(count)
                .sum::<Vec2>()
                / (count as f32);
            *self = Self::Moving { goal, speed: 100. }
        }
    }
}

impl SheepMind {
    fn new_idle() -> Self {
        Self {
            time_left: Timer::new(
                Duration::from_millis(rng().random_range(400..600)),
                TimerMode::Repeating,
            ),
            state: State::Idle,
        }
    }
}

/// No transform, no screenwrap
pub fn new_sheep(
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let mut rng = rng();
    let angle = 2. * PI * rng.random::<f32>();
    let distance = GAME_HEIGHT / 4. * (1. - rng.random::<f32>().powi(2));
    let pos = distance * Vec2::from_angle(angle);

    (
        sheep_base(player_assets, texture_atlas_layouts),
        SheepMind::new_idle(),
        Transform {
            translation: pos.extend(0.),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(1.),
        },
        ScreenWrap,
    )
}

const RANGE: f32 = 150.;
const AVOID_RANGE: f32 = 50.;
const COLLISION_DISTANCE: f32 = 25.;

fn collision(
    mut sheep: Query<(Entity, &mut Transform), With<Sheep>>,
    mut distances: Local<HashMap<Entity, Vec2>>,
) {
    distances.clear();

    let mut combinations = sheep.iter_combinations_mut::<2>();
    while let Some([(id1, trans1), (id2, trans2)]) = combinations.fetch_next() {
        let vec = (trans1.translation - trans2.translation).xy();
        let dist = vec.length();

        if dist >= COLLISION_DISTANCE {
            continue;
        }

        for (id, vec) in [(id1, vec), (id2, -vec)] {
            match distances.entry(id) {
                Entry::Occupied(mut entry) => {
                    if dist < entry.get().length() {
                        entry.insert(vec);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec);
                }
            }
        }
    }

    for (id, mut transf) in sheep {
        let Some(vec) = distances.get(&id) else {
            continue;
        };
        let need_dist = COLLISION_DISTANCE - vec.length();
        transf.translation += (need_dist * vec.normalize_or_zero()).extend(0.);
    }
}

fn think(mut sheep: Query<(&Transform, &mut SheepMind)>, time: Res<Time>) {
    for (_, mut mind) in &mut sheep {
        mind.time_left.tick(time.delta());
        if !mind.time_left.just_finished() {
            continue;
        }
        match &mut mind.state {
            State::Moving { .. } => mind.state = State::Idle,
            State::Idle => mind.state = State::new_thinking(),
            State::Obseerving { .. } => {
                error!("Sheep should be done thinking");
                mind.state = State::Idle;
            }
        }
    }

    let mut combinations = sheep.iter_combinations_mut::<2>();
    while let Some([(trans1, mut mind1), (trans2, mut mind2)]) = combinations.fetch_next() {
        let vec = (trans1.translation - trans2.translation).xy();
        let dist = vec.length();
        if dist > RANGE {
            continue;
        }

        mind1.state.target_if_thinking(-vec);
        mind2.state.target_if_thinking(vec);
    }

    for (_, mut mind) in &mut sheep {
        mind.state.conclude_from_observation();
    }
}

fn walk(sheep: Query<(&mut Transform, &SheepMind), Without<HumanMind>>, time: Res<Time>) {
    for (mut transf, mind) in sheep {
        if let State::Moving { goal, speed, .. } = &mind.state {
            let speed = *speed
                * speed_from_time(
                    mind.time_left.elapsed().as_secs_f32()
                        / mind.time_left.duration().as_secs_f32(),
                );
            let goal = speed * goal.normalize_or_zero().extend(0.);
            transf.translation += time.delta_secs() * goal;
        }
    }
}

fn speed_from_time(time_fraction: f32) -> f32 {
    4. * time_fraction * (1. - time_fraction)
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct SheepAssets {
    #[dependency]
    pub bleats: Vec<Handle<AudioSource>>,
    #[dependency]
    pub sound: Handle<Image>,
}

impl FromWorld for SheepAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bleats: vec![
                assets.load("audio/sound_effects/bleat1.ogg"),
                assets.load("audio/sound_effects/bleat2.ogg"),
                assets.load("audio/sound_effects/bleat3.ogg"),
                assets.load("audio/sound_effects/bleat4.ogg"),
                assets.load("audio/sound_effects/bleat5.ogg"),
                assets.load("audio/sound_effects/bleat6.ogg"),
                assets.load("audio/sound_effects/bleat7.ogg"),
                assets.load("audio/sound_effects/bleat8.ogg"),
                assets.load("audio/sound_effects/bleat9.ogg"),
                assets.load("audio/sound_effects/bleat10.ogg"),
                assets.load("audio/sound_effects/bleat11.ogg"),
                assets.load("audio/sound_effects/bleat12.ogg"),
            ],
            sound: assets.load("images/sound.png"),
        }
    }
}

fn respawn_dead(
    mut commands: Commands,
    query: Query<(), With<Sheep>>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let count = query.count();

    if count >= N_SHEEP {
        return;
    }

    commands.spawn(sheep_at_edge(&player_assets, &mut texture_atlas_layouts));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct SheepAtEdge;

const DIST_FROM_EDGE: f32 = 20.;

fn sheep_at_edge(
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let total_edge_len: f32 = 2. * (GAME_WIDTH + GAME_HEIGHT);
    let spawn_point: f32 = total_edge_len * rng().random::<f32>();

    let (pos, speed) = if spawn_point < GAME_WIDTH {
        // Top
        (
            Vec2::new(spawn_point, GAME_HEIGHT / 2. + DIST_FROM_EDGE),
            Vec2::new(0., -1.),
        )
    } else if spawn_point < GAME_WIDTH + GAME_HEIGHT {
        // Right
        (
            Vec2::new(GAME_WIDTH / 2. + DIST_FROM_EDGE, spawn_point - GAME_WIDTH),
            Vec2::new(-1., 0.),
        )
    } else if spawn_point < 2. * GAME_WIDTH + GAME_HEIGHT {
        // Bottom
        (
            Vec2::new(
                spawn_point - GAME_HEIGHT - GAME_WIDTH,
                -GAME_HEIGHT / 2. - DIST_FROM_EDGE,
            ),
            Vec2::new(0., 1.),
        )
    } else {
        // Left
        (
            Vec2::new(
                -GAME_WIDTH / 2. - DIST_FROM_EDGE,
                spawn_point - 2. * GAME_WIDTH - GAME_HEIGHT,
            ),
            Vec2::new(1., 0.),
        )
    };

    (
        Transform {
            translation: pos.extend(0.),
            ..Default::default()
        },
        SheepAtEdge,
        sheep_base(player_assets, texture_atlas_layouts),
        // Remove this
        ScreenWrap,
    )
}

/// No transform, no mind, no screenwrap
fn sheep_base(
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = SheepAnimation::new();

    (
        Name::new("Sheep"),
        Sheep,
        bleat::RecentBleat {
            time_to_bleat: Timer::from_seconds(0., TimerMode::Once),
            time_to_spread: {
                let mut timer = Timer::from_seconds(bleat::TIME_TO_SPREAD_SECS, TimerMode::Once);
                timer.finish();
                timer.tick(Duration::new(1, 0));
                timer
            },
        },
        Sprite::from_atlas_image(
            player_assets.sheep.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        player_animation,
    )
}
