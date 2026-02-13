use bevy::prelude::*;
use bevy_modern_pixel_camera::{
    plugin::PixelCameraPlugin,
    zoom::{PixelViewport, PixelZoom},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(PixelCameraPlugin)
        .add_systems(Startup, spawn);
}

pub const GAME_WIDTH: f32 = 640.;
pub const GAME_HEIGHT: f32 = 320.;

fn spawn(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Msaa::Off,
        PixelZoom::FitSize {
            width: GAME_WIDTH as i32,
            height: GAME_HEIGHT as i32,
        },
        PixelViewport,
    ));
}
