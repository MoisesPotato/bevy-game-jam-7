//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles, intro::PlayedIntro, screens::Screen, theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        enter_gameplay_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        DespawnOnExit(Screen::Loading),
        children![widget::label("Loading...")],
    ));
}

fn enter_gameplay_screen(
    mut next_screen: ResMut<NextState<Screen>>,
    played_intro: Res<PlayedIntro>,
) {
    if played_intro.0 {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Intro);
    }
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
