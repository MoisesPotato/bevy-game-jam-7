use std::time::Duration;

use bevy::prelude::*;
use rand::{Rng, rng};

use crate::demo::{player, sheep::Sheep};

pub struct JumpTimer(Timer);

impl Default for JumpTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0., TimerMode::Repeating))
    }
}

const JUMP_TIME_MIN: f32 = 2.;
const JUMP_TIME_MAX: f32 = 5.;

pub fn jump(
    commands: Commands,
    time: Res<Time>,
    mut timer: Local<JumpTimer>,
    sheep: Query<Entity, With<Sheep>>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    timer.0.set_duration(Duration::from_secs_f32(
        rng().random_range(JUMP_TIME_MIN..JUMP_TIME_MAX),
    ));
    timer.0.reset();

    player::choose(commands, sheep);
}
