use std::time::Duration;

use bevy::prelude::*;

use crate::{
    PausableSystems, intro::message::MESSAGES, screens::Screen, theme::palette::RESURRECT_PALETTE,
};

mod message;

pub fn plugin(app: &mut App) {
    app.init_resource::<PlayedIntro>();

    app.add_systems(OnEnter(Screen::Intro), spawn_intro);
    app.add_systems(Update, advance_intro.in_set(PausableSystems));
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct PlayedIntro(pub bool);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Intro {
    time_to_next_message: Timer,
    displayed_messages: usize,
}

fn spawn_intro(
    mut commands: Commands,
    mut played: ResMut<PlayedIntro>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if played.0 {
        next_state.set(Screen::Gameplay);
        return;
    }
    played.0 = true;
    commands.spawn((
        Name::new("Tutorial text"),
        Node {
            position_type: PositionType::Absolute,
            width: percent(50),
            left: px(10),
            height: percent(100),
            top: px(10),
            display: Display::Flex,
            row_gap: px(10),
            column_gap: px(30),
            flex_direction: FlexDirection::Column,
            padding: px(0).into(),
            ..default()
        },
        DespawnOnExit(Screen::Intro),
        Intro {
            time_to_next_message: Timer::new(Duration::ZERO, TimerMode::Once),
            displayed_messages: 0,
        },
    ));
}

fn advance_intro(
    mut commands: Commands,
    time: Res<Time>,
    mut intro: Single<(Entity, &mut Intro)>,
    old_messages: Query<Entity, With<IntroText>>,
) {
    intro.1.time_to_next_message.tick(time.delta());

    if !intro.1.time_to_next_message.just_finished() {
        return;
    }

    intro
        .1
        .time_to_next_message
        .set_duration(Duration::from_secs_f32(1.));
    intro.1.time_to_next_message.reset();

    let Some(message) = MESSAGES.get(intro.1.displayed_messages) else {
        error!("TODO Done!!");
        return;
    };

    intro
        .1
        .time_to_next_message
        .set_duration(Duration::from_secs_f32(message.delay));
    intro.1.time_to_next_message.reset();
    intro.1.displayed_messages += 1;

    commands.spawn((
        Name::new("Intro Text"),
        Text((message.text).into()),
        TextFont::from_font_size(48.0),
        TextLayout::new(Justify::Left, LineBreak::WordBoundary),
        TextColor(RESURRECT_PALETTE[9]),
        IntroText,
        ChildOf(intro.0),
    ));

    if message.clears_screen {
        for id in old_messages {
            commands.entity(id).despawn();
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IntroText;
