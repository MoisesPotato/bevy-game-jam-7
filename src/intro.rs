use std::time::Duration;

use bevy::prelude::*;

use crate::{screens::Screen, theme::palette::RESURRECT_PALETTE};

pub fn plugin(app: &mut App) {
    app.init_state::<InIntro>();
    app.init_resource::<PlayedIntro>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_intro);
    app.add_systems(Update, advance_intro);
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct InIntro(bool);

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct PlayedIntro(bool);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Intro {
    time_to_next_message: Timer,
    displayed_messages: usize,
}

fn spawn_intro(
    mut commands: Commands,
    mut played: ResMut<PlayedIntro>,
    mut next_state: ResMut<NextState<InIntro>>,
) {
    if played.0 {
        return;
    }
    played.0 = true;
    next_state.set(InIntro(true));
    commands.spawn((
        Name::new("Tutorial text"),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            left: px(30),
            height: percent(100),
            display: Display::Flex,
            row_gap: px(10),
            column_gap: px(30),
            flex_direction: FlexDirection::Column,
            top: px(10),
            ..default()
        },
        DespawnOnExit(InIntro(true)),
        Intro {
            time_to_next_message: Timer::new(Duration::ZERO, TimerMode::Once),
            displayed_messages: 0,
        },
    ));
}

const MESSAGES: &[&str] = &["Test test", "Second message", "Third Message"];

fn advance_intro(mut commands: Commands, time: Res<Time>, mut intro: Single<(Entity, &mut Intro)>) {
    intro.1.time_to_next_message.tick(time.delta());

    if !intro.1.time_to_next_message.just_finished() {
        return;
    }

    intro
        .1
        .time_to_next_message
        .set_duration(Duration::from_secs_f32(1.));
    intro.1.time_to_next_message.reset();

    let Some(text) = MESSAGES.get(intro.1.displayed_messages) else {
        error!("TODO Done!!");
        return;
    };
    intro.1.displayed_messages += 1;

    commands.spawn((
        Name::new("Intro Text"),
        Text((*text).into()),
        TextFont::from_font_size(24.0),
        TextColor(RESURRECT_PALETTE[9]),
        ChildOf(intro.0),
    ));
}
