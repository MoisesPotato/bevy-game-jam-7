//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    controls::PlayerInput,
    demo::{
        movement::HumanMind,
        sheep::{Sheep, ego::ParticleSpawner},
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

pub fn choose(
    mut commands: Commands,
    sheep: Query<(Entity, &Transform, Option<&HumanMind>), With<Sheep>>,
) {
    let mut count = 0;
    for (id, pos, human) in sheep {
        commands.entity(id).remove::<HumanMind>();
        if human.is_some() {
            commands.spawn((
                Transform::from_translation(pos.translation),
                ParticleSpawner::default(),
            ));
        }
        count += 1;
    }

    if count == 0 {
        error!("No sheep");
        return;
    }

    let new_player = rng().random_range(0..count);

    let Some((id, _, _)) = sheep.iter().nth(new_player) else {
        error!("Sheep somehow disappeared?");
        return;
    };

    commands.entity(id).insert(HumanMind::default());
}

fn record_player_directional_input(
    input: Res<PlayerInput>,
    mut controller_query: Query<&mut HumanMind>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.up {
        intent.y += 1.0;
    }
    if input.down {
        intent.y -= 1.0;
    }
    if input.left {
        intent.x -= 1.0;
    }
    if input.right {
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
    pub sheep: Handle<Image>,
    #[dependency]
    pub layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 7, 1, None, None);
        Self {
            sheep: assets.load_with_settings(
                "images/sheep.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            layout: assets.add(layout),
        }
    }
}
