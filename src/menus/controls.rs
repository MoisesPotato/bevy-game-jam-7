//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{
    input::{ButtonState, common_conditions::input_just_pressed, keyboard::KeyboardInput},
    prelude::*,
};

use crate::{
    menus::{
        Menu,
        settings::{go_back, go_back_on_click},
    },
    theme::prelude::*,
};

use KeyAction::{Bleat, Down, Left, Right, Up};

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

#[derive(Clone, Copy, Reflect, Debug)]
enum KeyAction {
    Up,
    Down,
    Left,
    Right,
    Bleat,
}

impl KeyAction {
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
    which: KeyAction,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct KeyChange {
    which: KeyAction,
    changing: bool,
}

fn start_change_key(event: On<Pointer<Click>>, buttons: Query<(Entity, &mut KeyChange)>) {
    let this_id = event.original_event_target();

    let Ok((_, button)) = buttons.get(this_id) else {
        error!(%this_id, "Who triggered this?");
        return;
    };

    if !button.changing {
        for (button_id, mut state) in buttons {
            state.changing = button_id == this_id;
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
            let code = match button.which {
                Up => &mut control_scheme.up,
                Down => &mut control_scheme.down,
                Left => &mut control_scheme.left,
                Right => &mut control_scheme.right,
                Bleat => &mut control_scheme.bleat,
            };

            code.0 = key.key_code;
            code.1 = match &key.logical_key {
                bevy::input::keyboard::Key::Character(str) => str.to_string(),
                _ => format!("{:?}", key.logical_key),
            }
        }
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct ControlScheme {
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
        match label.which {
            Up => {
                text.0.clone_from(&scheme.up.1);
            }
            Down => {
                text.0.clone_from(&scheme.down.1);
            }
            Left => {
                text.0.clone_from(&scheme.left.1);
            }
            Right => {
                text.0.clone_from(&scheme.right.1);
            }
            Bleat => {
                text.0.clone_from(&scheme.bleat.1);
            }
        }
    }
}
