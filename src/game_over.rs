use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles, demo::cabbage::Score, intro::PlayedIntro,
    menus::start_already, screens::Screen, theme::widget,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), spawn);
}

fn spawn(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        widget::ui_root("Game Over"),
        GlobalZIndex(2),
        DespawnOnExit(Screen::GameOver),
        children![
            widget::header("Game Over"),
            widget::label(format!("We ate {} cabbage", score.0)),
            widget::button("Restart", restart),
            widget::button("Main Menu", to_menu),
        ],
    ));
}

fn to_menu(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn restart(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    next_screen: ResMut<NextState<Screen>>,
    played_intro: Res<PlayedIntro>,
) {
    start_already(resource_handles, next_screen, played_intro);
}
