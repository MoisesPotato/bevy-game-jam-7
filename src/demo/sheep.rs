use std::{
    cmp::{Ordering, min},
    f32::consts::PI,
    time::Duration,
};

use bevy::prelude::*;
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    demo::{
        animation::PlayerAnimation,
        movement::ScreenWrap,
        player::{Player, PlayerAssets},
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        think
            // .run_if(on_timer(Duration::from_millis(500)))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    app.add_systems(
        Update,
        walk.in_set(AppSystems::Update).in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Sheep;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SheepMind {
    state: State,
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
        const COLLISION_DISTANCE: f32 = 20.;

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
                speed: 200.,
            };
        } else {
            let goal = neighbors
                .iter()
                .map(|v| {
                    if v.length() <= AVOID_RANGE {
                        -v * 2.
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
        SheepMind::new_idle(),
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

const RANGE: f32 = 100.;
const AVOID_RANGE: f32 = 40.;

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
            let goal = *speed * goal.normalize_or_zero().extend(0.);
            transf.translation += time.delta_secs() * goal;
        }
    }
}
