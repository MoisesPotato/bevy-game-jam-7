use std::{f32::consts::PI, time::Duration};

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

#[derive(Reflect, Debug)]
pub enum State {
    Moving {
        neighbor_count: u8,
        goal: Vec2,
        speed: f32,
    },
    Idle,
}

const MIND_CAPACITY: u8 = 100;

impl State {
    fn add_goal(&mut self, vec: Vec2) {
        let Self::Moving {
            neighbor_count,
            goal,
            ..
        } = self
        else {
            error!("Add a goal to idle sheep");
            return;
        };
        if *neighbor_count == 0 {
            *goal = vec;
            *neighbor_count = 1;
        } else if *neighbor_count < MIND_CAPACITY {
            let n = f32::from(*neighbor_count);
            *goal = (*goal * n + vec) / (n + 1.);
            *neighbor_count += 1;
        }
    }

    const fn new_moving() -> Self {
        Self::Moving {
            neighbor_count: 0,
            goal: Vec2::ZERO,
            speed: 100.,
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
            State::Idle => mind.state = State::new_moving(),
        }
    }

    let mut combinations = sheep.iter_combinations_mut::<2>();
    while let Some([(trans1, mut mind1), (trans2, mut mind2)]) = combinations.fetch_next() {
        let vec = (trans1.translation - trans2.translation).xy();
        let dist = vec.length();
        if dist > RANGE {
            continue;
        }

        let avoid_factor = if dist < AVOID_RANGE {
            -AVOID_RANGE / dist
        } else {
            1.
        };

        if mind1.time_left.just_finished() && matches!(mind1.state, State::Moving { .. }) {
            mind1.state.add_goal(-avoid_factor * vec);
        }

        if mind2.time_left.just_finished() && matches!(mind2.state, State::Moving { .. }) {
            mind2.state.add_goal(avoid_factor * vec);
        }
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
