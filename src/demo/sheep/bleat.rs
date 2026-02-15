use std::time::Duration;

use bevy::prelude::*;
use rand::{Rng, rng, seq::IndexedRandom};

use crate::{
    audio::sound_effect,
    demo::{movement::HumanMind, sheep::SheepAssets},
    intro::{IntroPause, Resume},
};

/// Tick bleat timers
pub fn tick(time: Res<Time>, sheep: Query<&mut RecentBleat>) {
    for mut recent in sheep {
        recent.time_to_bleat.tick(time.delta());
        recent.time_to_spread.tick(time.delta());
    }
}

const PLAYER_BLEAT_DELAY_SECS: f32 = 1.;
const SHEEP_BLEAT_DELAY_SECS: f32 = 3.;

/// B for bleat
pub fn with_b(
    mut commands: Commands,
    player_sheep: Query<(Entity, &mut RecentBleat), With<HumanMind>>,
    assets: Res<SheepAssets>,
    mut writer: MessageWriter<Resume>,
    pause: Res<IntroPause>,
) {
    for (id, mut recent) in player_sheep {
        if !recent.time_to_bleat.is_finished() {
            continue;
        }
        bleat(&mut commands, &assets, id, &mut recent, true);
        if matches!(*pause, IntroPause::WaitBleat) {
            info!("Managed to bleat");
            writer.write(Resume(IntroPause::WaitBleat));
        }
    }
}

pub const TIME_TO_SPREAD_SECS: f32 = 0.5;

fn bleat(
    commands: &mut Commands,
    assets: &Res<SheepAssets>,
    id: Entity,
    recent: &mut RecentBleat,
    human_triggered: bool,
) {
    let rng = &mut rand::rng();
    let random_bleat = assets.bleats.choose(rng).unwrap().clone();
    let sound_id = commands
        .spawn((sound_effect(random_bleat, 0.3), BleatSound {}))
        .id();

    recent
        .time_to_bleat
        .set_duration(Duration::from_secs_f32(if human_triggered {
            PLAYER_BLEAT_DELAY_SECS
        } else {
            SHEEP_BLEAT_DELAY_SECS
        }));
    recent.time_to_bleat.reset();
    recent.time_to_spread.reset();

    let child_id = commands
        .spawn((
            Name::new("Bleat image"),
            BleatImage { sound_id },
            Transform::from_translation(Vec3::new(SOUND_VISUAL_DIST, 0., 0.)),
            Sprite::from_image(assets.sound.clone()),
        ))
        .id();
    commands.entity(id).add_children(&[child_id]);
}

pub fn despawn_image(
    mut commands: Commands,
    mut removed: RemovedComponents<BleatSound>,
    bleats: Query<(Entity, &BleatImage)>,
) {
    for sound in removed.read() {
        for (id, img) in bleats {
            if img.sound_id == sound {
                commands.entity(id).despawn();
            }
        }
    }
}

pub const SOUND_VISUAL_DIST: f32 = 16.;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct BleatImage {
    sound_id: Entity,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct BleatSound;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RecentBleat {
    pub time_to_bleat: Timer,
    pub time_to_spread: Timer,
}

const RANGE: f32 = 100.;
const BLEAT_SPREAD_CHANCE: f32 = 0.05;

pub fn spread(
    mut commands: Commands,
    assets: If<Res<SheepAssets>>,
    mut sheep: Query<(Entity, &Transform, &mut RecentBleat, Option<&HumanMind>)>,
) {
    let mut rng = rng();

    let mut combinations = sheep.iter_combinations_mut::<2>();
    while let Some(
        [
            (id1, trans1, mut bleat1, player1),
            (id2, trans2, mut bleat2, player2),
        ],
    ) = combinations.fetch_next()
    {
        let vec = (trans1.translation - trans2.translation).xy();
        let dist = vec.length();
        if dist > RANGE {
            continue;
        }

        if bleat1.time_to_spread.just_finished()
            && bleat2.time_to_bleat.is_finished()
            && rng.random::<f32>() < BLEAT_SPREAD_CHANCE
        {
            bleat(&mut commands, &assets, id2, &mut bleat2, player2.is_some());
        } else if bleat2.time_to_spread.just_finished()
            && bleat1.time_to_bleat.is_finished()
            && rng.random::<f32>() < BLEAT_SPREAD_CHANCE
        {
            bleat(&mut commands, &assets, id1, &mut bleat1, player1.is_some());
        }
    }
}

/// This runs per sheep, per tenth of a second
const SPONTANEOUS_CHANCE: f32 = 0.0005;

pub struct SheepTimer(Timer);

impl Default for SheepTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

pub fn random(
    mut commands: Commands,
    mut timer: Local<SheepTimer>,
    time: Res<Time>,
    assets: If<Res<SheepAssets>>,
    sheep: Query<(Entity, &mut RecentBleat, Option<&HumanMind>)>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rng();
    for (id, mut recent, player) in sheep {
        if recent.time_to_bleat.is_finished() && rng.random::<f32>() < SPONTANEOUS_CHANCE {
            bleat(&mut commands, &assets, id, &mut recent, player.is_some());
        }
    }
}
