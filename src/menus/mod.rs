//! The game's menus and transitions between them.

mod controls;
mod credits;
mod main;
mod pause;

pub use controls::{ControlScheme, PlayerAction};
pub use main::start_already;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.add_plugins((
        credits::plugin,
        main::plugin,
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
    Pause,
}
