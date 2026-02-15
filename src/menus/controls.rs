//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{
    input::{
        ButtonState,
        common_conditions::input_just_pressed,
        keyboard::{self, KeyboardInput},
    },
    prelude::*,
};

use crate::{
    menus::{
        Menu,
        settings::{go_back, go_back_on_click},
    },
    theme::prelude::*,
};

use PlayerAction::{Bleat, Down, Left, Right, Up};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ControlScheme>();
    app.add_systems(OnEnter(Menu::Controls), (spawn_menu, update_labels).chain());
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Controls).and(input_just_pressed(KeyCode::Escape))),
    );
    app.add_systems(Update, execute_change_key.run_if(in_state(Menu::Controls)));

    app.add_systems(
        Update,
        update_labels
            .run_if(in_state(Menu::Controls))
            .run_if(resource_changed::<ControlScheme>),
    );
    // app.add_systems(
    //     Update,
    //     update_global_volume_label.run_if(in_state(Menu::Settings)),
    // );
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Controls"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Controls),
        children![
            widget::header("Customize controls"),
            grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn grid() -> impl Bundle {
    (
        Name::new("Controls Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            Up.label(),
            Up.key_change(),
            Down.label(),
            Down.key_change(),
            Left.label(),
            Left.key_change(),
            Right.label(),
            Right.key_change(),
            Bleat.label(),
            Bleat.key_change(),
        ],
    )
}

#[derive(Clone, Copy, Reflect, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Bleat,
}

impl PlayerAction {
    const fn str(self) -> &'static str {
        match self {
            Up => "Up",
            Down => "Down",
            Left => "Left",
            Right => "Right",
            Bleat => "Bleat",
        }
    }

    fn label(self) -> impl Bundle {
        (
            widget::label(self.str()),
            Node {
                justify_self: JustifySelf::End,
                ..default()
            },
        )
    }

    fn key_change(self) -> impl Bundle {
        (
            Name::new(format!("{} button", self.str())),
            Node {
                justify_self: JustifySelf::Start,
                ..default()
            },
            children![
                widget::button_with_bundle(
                    "",
                    start_change_key,
                    KeyChange {
                        which: self,
                        changing: false
                    },
                    KeyLabel { which: self }
                ),
                // widget::button_small("-", lower_global_volume),
                // (
                //     Name::new("Current Volume"),
                //     Node {
                //         padding: UiRect::horizontal(px(10)),
                //         justify_content: JustifyContent::Center,
                //         ..default()
                //     },
                //     children![(widget::label(""), GlobalVolumeLabel)],
                // ),
                // widget::button_small("+", raise_global_volume),
            ],
        )
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct KeyLabel {
    which: PlayerAction,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct KeyChange {
    which: PlayerAction,
    changing: bool,
}

fn start_change_key(
    event: On<Pointer<Click>>,
    buttons: Query<(Entity, &mut KeyChange)>,
    labels: Query<(&mut Text, &KeyLabel), Without<KeyChange>>,
) {
    let this_id = event.original_event_target();

    let Ok((_, button)) = buttons.get(this_id) else {
        error!(%this_id, "Who triggered this?");
        return;
    };

    let mut code = None;
    if button.changing {
        return;
    }
    for (button_id, mut state) in buttons {
        state.changing = button_id == this_id;
        if button_id == this_id {
            code = Some(state.which);
        }
    }

    if let Some(code) = code {
        for (mut text, label) in labels {
            if label.which == code {
                text.0 = "Press a key...".into();
            }
        }
    }
}

fn execute_change_key(
    mut key: MessageReader<KeyboardInput>,
    mut buttons: Query<&mut KeyChange>,
    mut control_scheme: ResMut<ControlScheme>,
) {
    for key in key.read() {
        if matches!(key.state, ButtonState::Released) {
            continue;
        }
        for mut button in &mut buttons {
            if !button.changing {
                continue;
            }
            button.changing = false;
            let code = control_scheme.get_mut(button.which);

            code.0 = key.key_code;
            code.1 = match &key.logical_key {
                bevy::input::keyboard::Key::Character(str) => str.to_uppercase(),
                keyboard::Key::ArrowDown => "Down".into(),
                keyboard::Key::ArrowUp => "Up".into(),
                keyboard::Key::ArrowLeft => "Left".into(),
                keyboard::Key::ArrowRight => "Right".into(),
                _ => format!("{:?}", key.logical_key),
            }
        }
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct ControlScheme {
    up: (KeyCode, String),
    down: (KeyCode, String),
    left: (KeyCode, String),
    right: (KeyCode, String),
    bleat: (KeyCode, String),
}

impl Default for ControlScheme {
    fn default() -> Self {
        Self {
            up: (KeyCode::KeyW, "W".into()),
            down: (KeyCode::KeyS, "S".into()),
            left: (KeyCode::KeyA, "A".into()),
            right: (KeyCode::KeyD, "D".into()),
            bleat: (KeyCode::Space, "Space".into()),
        }
    }
}

fn update_labels(scheme: Res<ControlScheme>, label: Query<(&mut Text, &KeyLabel)>) {
    for (mut text, label) in label {
        text.0.clone_from(&scheme.get(label.which).1);
    }
}

impl ControlScheme {
    const fn get(&self, key: PlayerAction) -> &(KeyCode, String) {
        match key {
            Up => &self.up,
            Down => &self.down,
            Left => &self.left,
            Right => &self.right,
            Bleat => &self.bleat,
        }
    }

    const fn get_mut(&mut self, key: PlayerAction) -> &mut (KeyCode, String) {
        match key {
            Up => &mut self.up,
            Down => &mut self.down,
            Left => &mut self.left,
            Right => &mut self.right,
            Bleat => &mut self.bleat,
        }
    }

    pub fn by_keycode(&self, code: KeyCode) -> impl Iterator<Item = PlayerAction> {
        [Up, Down, Left, Right, Bleat]
            .into_iter()
            .filter(move |k| self.get(*k).0 == code)
    }
}
