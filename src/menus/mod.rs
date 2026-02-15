//! The game's menus and transitions between them.

mod controls;
mod credits;
mod main;
mod pause;
mod settings;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.add_plugins((
        credits::plugin,
        main::plugin,
        settings::plugin,
        pause::plugin,
        controls::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Controls,
    Settings,
    Pause,
}
