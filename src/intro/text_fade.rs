use std::ops::Range;

use bevy::{color::ColorRange, prelude::*};

use crate::{demo::level::BG_COLOR, theme::palette::WHITE};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct FadeIn {
    timer: Timer,
}

const DURATION: f32 = 0.5;

impl FadeIn {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(DURATION, TimerMode::Once),
        }
    }
}

const RANGE: Range<Color> = BG_COLOR..WHITE;

pub fn apply(time: Res<Time>, text: Query<(&mut TextColor, &mut FadeIn)>) {
    for (mut text, mut effect) in text {
        effect.timer.tick(time.delta());
        if effect.timer.is_finished() {
            continue;
        }
        let percent = effect.timer.elapsed_secs() / DURATION;

        let color = RANGE.at(percent);

        text.0 = color;
    }
}
