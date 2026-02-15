use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use rand::{Rng, rng};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    camera::{GAME_HEIGHT, GAME_WIDTH},
    demo::{level::Level, movement::HumanMind, sheep::Sheep},
    theme::palette::RESURRECT_PALETTE,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CabbageAssets>();
    app.add_systems(
        Update,
        (spawn, eat)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    app.add_systems(
        Update,
        update_score
            .run_if(resource_changed::<Score>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.insert_resource(Score(0));
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Score(pub u64);

impl FromWorld for CabbageAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            cabbage: assets.load_with_settings(
                "images/cabbage.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CabbageAssets {
    #[dependency]
    pub cabbage: Handle<Image>,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Cabbage;

pub struct CabbageTimer(Timer);

const SECONDS_TO_CABBAGE: f32 = 1.;
const SPAWN_CHANCE: f32 = 0.8;
const MAX_NUMBER: usize = 8;

impl Default for CabbageTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            SECONDS_TO_CABBAGE,
            TimerMode::Repeating,
        ))
    }
}

fn spawn(
    mut commands: Commands,
    mut timer: Local<CabbageTimer>,
    time: Res<Time>,
    level: Query<Entity, With<Level>>,
    cabbages: Query<(), With<Cabbage>>,
    assets: If<Res<CabbageAssets>>,
) {
    let Some(level) = level.iter().next() else {
        return;
    };

    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let count_cabbages = cabbages.count();

    if count_cabbages >= MAX_NUMBER {
        return;
    }

    let mut rng = rng();

    if rng.random::<f32>() > SPAWN_CHANCE {
        return;
    }

    let position_x = rng.random_range((-GAME_WIDTH / 2. + 16.)..(GAME_WIDTH / 2. - 16.));
    let position_y = rng.random_range((-GAME_HEIGHT / 2. + 16.)..(GAME_HEIGHT / 2. - 16.));
    let transform = Transform {
        translation: Vec3::new(position_x, position_y, 0.),
        scale: Vec2::splat(1.).extend(0.),
        ..Default::default()
    };

    commands.spawn((
        Name::new("Cabbage"),
        transform,
        Cabbage,
        Sprite::from_image(assets.cabbage.clone()),
        ChildOf(level),
    ));
}

/// This is taxicab distance
const EAT_BOX: f32 = 16.;

fn eat(
    mut commands: Commands,
    cabbages: Query<(Entity, &Transform), With<Cabbage>>,
    sheep: Query<(&Transform, Option<&HumanMind>), With<Sheep>>,
    mut score: ResMut<Score>,
) {
    for (id, transform) in cabbages {
        let position = transform.translation;

        for (transform, mind) in sheep {
            let sheep_pos = transform.translation;

            let dist = (position.x - sheep_pos.x).abs() + (position.y - sheep_pos.y).abs();
            if dist > EAT_BOX {
                continue;
            }

            if mind.is_some() {
                score.0 += 1;
            }

            commands.entity(id).despawn();
            break;
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ScoreUI;

pub fn spawn_score(commands: &mut Commands, level: Entity) {
    commands
        .spawn((
            Transform::default(),
            Name::new("Score UI"),
            Text::new("Score: "),
            TextFont::from_font_size(24.),
            Node {
                position_type: PositionType::Absolute,
                top: px(12),
                right: px(12),
                ..Default::default()
            },
            TextColor(RESURRECT_PALETTE[9]),
            ChildOf(level),
        ))
        .with_child((TextSpan::new("0"), ScoreUI));
}

fn update_score(score: Res<Score>, ui: Query<&mut TextSpan, With<ScoreUI>>) {
    for mut text in ui {
        *text = TextSpan::new((score.0).to_string());
    }
}
