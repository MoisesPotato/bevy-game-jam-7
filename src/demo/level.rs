//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    demo::{player::PlayerAssets, sheep::sheep},
    screens::Screen,
    theme::palette::RESURRECT_PALETTE,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();

    app.add_systems(OnEnter(Screen::Gameplay), background);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    // #[dependency]
    // music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(_world: &mut World) -> Self {
        // let assets = world.resource::<AssetServer>();
        Self {
            // music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

const N_SHEEP: u8 = 50;

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    // level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
            // children![(
            //     Name::new("Gameplay Music"),
            //     music(level_assets.music.clone())
            // )],
        ))
        .id();

    for _ in 0..N_SHEEP {
        commands.spawn((
            sheep(&player_assets, &mut texture_atlas_layouts),
            ChildOf(level),
        ));
    }
}

fn background(mut clear_color: ResMut<ClearColor>) {
    info!("Change color");
    *clear_color = ClearColor(RESURRECT_PALETTE[35]);
}
