//! Player-specific behavior.

use std::f32::consts::PI;

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

pub fn sheep(player_assets: &PlayerAssets) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let player_animation = PlayerAnimation::new();

    let mut rng = rng();
    let angle = 2. * PI * rng.random::<f32>();
    let distance = 200. * (1. - rng.random::<f32>().powi(2));

    (
        Name::new("Sheep"),
        Sheep,
        // Player,
        Sprite::from_image(player_assets.sheep.clone()),
        Transform {
            translation: (distance * Vec2::from_angle(angle)).extend(0.),
            rotation: Quat::IDENTITY,
            scale: Vec2::splat(2.).extend(1.),
        },
        ScreenWrap,
        player_animation,
    )
}

pub fn choose(mut commands: Commands, sheep: Query<Entity, With<Sheep>>) {
    let mut count = 0;
    for id in sheep {
        commands.entity(id).remove::<(Player, MovementController)>();
        count += 1;
    }

    let new_player = rng().random_range(0..count);

    let Some(id) = sheep.iter().nth(new_player) else {
        error!("Sheep somehow disappeared?");
        return;
    };

    commands
        .entity(id)
        .insert((Player, MovementController::default()));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Sheep;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    sheep: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            sheep: assets.load_with_settings(
                "images/sheep.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
