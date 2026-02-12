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

    timer
        .0
        .set_duration(Duration::from_millis(rng().random_range(1000..3000)));
    timer.0.reset();

    player::choose(commands, sheep);
}
