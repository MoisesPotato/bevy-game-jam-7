//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    demo::{cabbage::spawn_score, player::PlayerAssets, sheep::new_sheep},
    screens::Screen,
    theme::palette::RESURRECT_PALETTE,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();

    app.add_systems(OnEnter(Screen::Gameplay), background);
    app.add_systems(OnEnter(Screen::Intro), background);
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

pub const N_SHEEP: usize = 50;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Level;

pub fn spawn_level(
    commands: Commands,
    // level_assets: Res<LevelAssets>,
    state: Res<State<Screen>>,
    player_assets: Res<PlayerAssets>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    level: Query<(), With<Level>>,
) {
    if level.count() > 0 {
        info!("Level is already spawned");
        return;
    }
    spawn_level_function(commands, player_assets, texture_atlas_layouts, **state);
}

/// A system that spawns the main level.
pub fn spawn_level_function(
    mut commands: Commands,
    // level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    state: Screen,
) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            Level,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                row_gap: px(20),
                ..default()
            },
            // Don't block picking events for other UI roots.
            Pickable::IGNORE,
            DespawnOnExit(Screen::Gameplay),
            // children![(
            //     Name::new("Gameplay Music"),
            //     music(level_assets.music.clone())
            // )],
        ))
        .id();

    let max_sheep = match &state {
        Screen::Intro => 10,
        Screen::Gameplay => N_SHEEP,
        _ => {
            error!("Shouldn't be spawning the level in {state:?}");
            0
        }
    };
    for _ in 0..max_sheep {
        commands.spawn((
            new_sheep(&player_assets, &mut texture_atlas_layouts, state),
            ChildOf(level),
        ));
    }

    spawn_score(&mut commands, level);
}

fn background(mut clear_color: ResMut<ClearColor>) {
    *clear_color = ClearColor(RESURRECT_PALETTE[35]);
}
