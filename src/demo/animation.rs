//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    demo::{
        movement::MovementController,
        player::PlayerAssets,
        sheep::{self, SheepMind},
    },
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (
                update_animation_movement::<MovementController>,
                update_animation_movement::<SheepMind>,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut SheepAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

trait Movement {
    fn dx(&self) -> f32;
    fn moving(&self) -> bool;
}

impl Movement for MovementController {
    fn dx(&self) -> f32 {
        self.intent.x
    }

    fn moving(&self) -> bool {
        self.intent == Vec2::ZERO
    }
}

impl Movement for SheepMind {
    fn dx(&self) -> f32 {
        match self.state {
            sheep::State::Moving { goal, .. } => goal.x,
            sheep::State::Obseerving { .. } | sheep::State::Idle => 0.,
        }
    }

    fn moving(&self) -> bool {
        match self.state {
            sheep::State::Moving { .. } => true,
            sheep::State::Obseerving { .. } | sheep::State::Idle => false,
        }
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement<T: Movement + Component>(
    mut player_query: Query<(&T, &mut Sprite, &mut SheepAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.dx();
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.moving() {
            PlayerAnimationState::Walking
        } else {
            PlayerAnimationState::Idling
        };
        animation.update_state(animation_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&SheepAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the
/// animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: If<Res<PlayerAssets>>,
    mut step_query: Query<&SheepAnimation, With<MovementController>>,
) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::rng();
            let random_step = player_assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SheepAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq, Eq)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
}

impl SheepAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 2;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 5;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    fn _walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::_walking(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    /// Return sprite index in the atlas.
    pub const fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => self.frame,
            PlayerAnimationState::Walking => Self::IDLE_FRAMES + self.frame,
        }
    }
}
