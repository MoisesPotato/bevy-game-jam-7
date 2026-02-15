use std::time::Duration;

use bevy::prelude::*;

use crate::{
    PausableSystems, intro::message::MESSAGES, screens::Screen, theme::palette::RESURRECT_PALETTE,
};

use message::Message;

mod message;

pub fn plugin(app: &mut App) {
    app.init_resource::<PlayedIntro>();
    app.init_resource::<IntroPause>();
    app.add_message::<Resume>();

    app.add_systems(OnEnter(Screen::Intro), spawn_intro);
    app.add_systems(
        Update,
        (advance_intro, resume)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Intro)),
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct PlayedIntro(pub bool);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Intro {
    paused: bool,
    time_to_next_message: Timer,
    displayed_messages: usize,
    next_pause: IntroPause,
}

#[derive(Resource, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Resource)]
pub enum IntroPause {
    WaitBleat,
    WaitEat,
    #[default]
    None,
}
impl IntroPause {
    fn cycle(&mut self) {
        match self {
            Self::WaitBleat => *self = Self::WaitEat,
            Self::WaitEat => *self = Self::None,
            Self::None => {
                error!("How did we get here?");
                *self = Self::None;
            }
        }
    }
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
            paused: false,
            time_to_next_message: Timer::new(Duration::ZERO, TimerMode::Once),
            displayed_messages: 0,
            next_pause: IntroPause::WaitBleat,
        },
    ));
}

fn advance_intro(
    mut commands: Commands,
    time: Res<Time>,
    mut intro: Single<(Entity, &mut Intro)>,
    old_messages: Query<Entity, With<IntroText>>,
    mut pause: ResMut<IntroPause>,
) {
    if intro.1.paused {
        return;
    }

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

    match message {
        Message::Text {
            text,
            clears_screen,
            delay,
        } => {
            intro
                .1
                .time_to_next_message
                .set_duration(Duration::from_secs_f32(*delay));
            intro.1.time_to_next_message.reset();
            intro.1.displayed_messages += 1;

            commands.spawn((
                Name::new("Intro Text"),
                Text((*text).into()),
                TextFont::from_font_size(48.0),
                TextLayout::new(Justify::Left, LineBreak::WordBoundary),
                TextColor(RESURRECT_PALETTE[9]),
                IntroText,
                ChildOf(intro.0),
            ));

            if *clears_screen {
                for id in old_messages {
                    commands.entity(id).despawn();
                }
            }
        }
        Message::Pause => {
            info!(
                msg = intro.1.displayed_messages,
                next_pause = ?intro.1.next_pause,
                "Hit a pause"
            );
            *pause = intro.1.next_pause;
            intro.1.next_pause.cycle();
            intro.1.paused = true;
            info!(?pause);
            match *pause {
                IntroPause::WaitBleat => {
                    commands.insert_resource(BleatEnabled);
                }
                IntroPause::WaitEat => {
                    commands.insert_resource(CabbageEnabled);
                }
                IntroPause::None => {}
            }
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IntroText;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct BleatEnabled;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct CabbageEnabled;

#[derive(Message)]
pub struct Resume(pub IntroPause);

fn resume(
    mut intro: Single<&mut Intro>,
    mut recv: MessageReader<Resume>,
    mut pause: ResMut<IntroPause>,
) {
    for m in recv.read() {
        if m.0 != *pause {
            continue;
        }
        info!(pause = ?*pause, "Resuming");
        *pause = IntroPause::None;
        intro
            .time_to_next_message
            .set_duration(Duration::from_secs_f32(0.2));
        intro.time_to_next_message.reset();
        intro.displayed_messages += 1;
        intro.paused = false;
    }
}
