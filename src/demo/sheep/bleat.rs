use std::time::Duration;

use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::{
    audio::sound_effect,
    demo::{movement::MovementController, sheep::SheepAssets},
};
pub fn tick(time: Res<Time>, sheep: Query<&mut RecentBleat>) {
    for mut recent in sheep {
        recent.timer.tick(time.delta());
    }
}

const PLAYER_BLEAT_DELAY_SECS: f32 = 1.;

/// B for bleat
pub fn with_b(
    mut commands: Commands,
    player_sheep: Query<(Entity, &mut RecentBleat), With<MovementController>>,
    assets: If<Res<SheepAssets>>,
) {
    let rng = &mut rand::rng();
    let random_bleat = assets.bleats.choose(rng).unwrap().clone();
    let sound_id = commands
        .spawn((sound_effect(random_bleat), BleatSound {}))
        .id();

    for (id, mut recent) in player_sheep {
        if !recent.timer.is_finished() {
            continue;
        }

        recent
            .timer
            .set_duration(Duration::from_secs_f32(PLAYER_BLEAT_DELAY_SECS));
        recent.timer.reset();

        let child_id = commands
            .spawn((
                Name::new("Bleat image"),
                BleatImage { sound_id },
                Transform::from_translation(Vec3::new(SOUND_DIST, 0., 0.)),
                Sprite::from_image(assets.sound.clone()),
            ))
            .id();
        commands.entity(id).add_children(&[child_id]);
    }
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

pub const SOUND_DIST: f32 = 16.;

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
    pub timer: Timer,
}
