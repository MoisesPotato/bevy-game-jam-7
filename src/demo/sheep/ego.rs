use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    demo::{movement::HumanMind, player, sheep::Sheep},
    screens::Screen,
    theme::palette::RED,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (fire, particle)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        Update,
        (fire, particle)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Intro)),
    );
}

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
    sheep: Query<(Entity, &Transform, Option<&HumanMind>), With<Sheep>>,
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

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ParticleSpawner {
    time_left: Timer,
    charges_left: u8,
    color: Color,
    frequency_factor: f32,
    speed_factor: f32,
}

impl Default for ParticleSpawner {
    fn default() -> Self {
        Self {
            time_left: Timer::from_seconds(0., TimerMode::Once),
            charges_left: 3,
            color: RED,
            frequency_factor: 1.,
            speed_factor: 1.,
        }
    }
}

impl ParticleSpawner {
    fn n_particles() -> u8 {
        rng().random_range(2..=6)
    }

    pub fn new(color: Color, freq: f32, charges: u8, particle_speed: f32) -> Self {
        Self {
            color,
            frequency_factor: freq,
            charges_left: charges,
            speed_factor: particle_speed,
            ..Default::default()
        }
    }
}

fn fire(
    mut commands: Commands,
    time: Res<Time>,
    spawner: Query<(Entity, &Transform, &mut ParticleSpawner)>,
) {
    let mut rng = rng();
    for (id, transf, mut spawner) in spawner {
        spawner.time_left.tick(time.delta());
        if !spawner.time_left.is_finished() {
            continue;
        }

        for _ in 0..ParticleSpawner::n_particles() {
            commands.spawn((
                Transform {
                    translation: Vec3::new(transf.translation.x, transf.translation.y, -0.5),
                    rotation: Quat::from_rotation_z(rng.random_range(0. ..2. * PI)),
                    scale: Vec3::new(3., 3., 0.),
                },
                Sprite::from_color(spawner.color, Vec2::splat(1.)),
                Particle {
                    lifetime: Timer::from_seconds(
                        spawner.frequency_factor * rng.random_range(0.2..0.8),
                        TimerMode::Once,
                    ),
                    speed: spawner.speed_factor
                        * rng.random_range(20. ..50.)
                        * Vec2::from_angle(rng.random_range(0. ..2. * PI)),
                    rotation_speed: rng.random_range(-1. ..1.),
                },
            ));
        }

        spawner.charges_left -= 1;
        if spawner.charges_left == 0 {
            commands.entity(id).despawn();
            continue;
        }

        spawner
            .time_left
            .set_duration(Duration::from_secs_f32(0.05));
        spawner.time_left.reset();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Particle {
    lifetime: Timer,
    speed: Vec2,
    rotation_speed: f32,
}

fn particle(
    mut commands: Commands,
    time: Res<Time>,
    particle: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    for (id, mut t, mut particle) in particle {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.is_finished() {
            commands.entity(id).despawn();
            continue;
        }
        t.translation += particle.speed.extend(0.) * time.delta_secs();
        t.rotation *= Quat::from_rotation_z(particle.rotation_speed * time.delta_secs());
    }
}
