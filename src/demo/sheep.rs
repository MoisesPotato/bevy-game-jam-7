use std::{
    cmp::{Ordering, min},
    collections::{HashMap, hash_map::Entry},
    f32::consts::PI,
    time::Duration,
};

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use rand::{Rng, rng, seq::IndexedRandom};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    audio::sound_effect,
    demo::{
        animation::SheepAnimation,
        movement::ScreenWrap,
        player::{Player, PlayerAssets},
    },
};

pub fn plugin(app: &mut App) {
    app.load_resource::<SheepAssets>();

    app.add_systems(
        Update,
        (collision, think, walk)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        bleat
            .run_if(input_just_pressed(KeyCode::KeyB))
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

pub fn sheep(
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = SheepAnimation::new();

    let mut rng = rng();
    let angle = 2. * PI * rng.random::<f32>();
    let distance = 250. * (1. - rng.random::<f32>().powi(2));
    let pos = distance * Vec2::from_angle(angle);

    (
        Name::new("Sheep"),
        Sheep,
        SheepMind::new_idle(),
        Sprite::from_atlas_image(
            player_assets.sheep.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        Transform {
            translation: pos.extend(0.),
            rotation: Quat::IDENTITY,
            scale: Vec2::splat(2.).extend(1.),
        },
        ScreenWrap,
        player_animation,
    )
}

const RANGE: f32 = 300.;
const AVOID_RANGE: f32 = 100.;
const COLLISION_DISTANCE: f32 = 50.;

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

fn walk(sheep: Query<(&mut Transform, &SheepMind), Without<Player>>, time: Res<Time>) {
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
        }
    }
}

/// B for bleat
fn bleat(mut commands: Commands, assets: If<Res<SheepAssets>>) {
    let rng = &mut rand::rng();
    let random_bleat = assets.bleats.choose(rng).unwrap().clone();
    commands.spawn(sound_effect(random_bleat));
}
