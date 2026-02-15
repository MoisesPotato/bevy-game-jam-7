use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};

use crate::{
    AppSystems, PausableSystems,
    menus::{
        ControlScheme,
        PlayerAction::{self, Bleat, Down, Left, Right, Up},
    },
};

pub fn plugin(app: &mut App) {
    app.init_resource::<PlayerInput>();
    app.add_systems(
        Update,
        record_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
#[allow(clippy::struct_excessive_bools)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub bleat: bool,
}

impl PlayerInput {
    const fn get(&self, key: PlayerAction) -> bool {
        match key {
            Up => self.up,
            Down => self.down,
            Left => self.left,
            Right => self.right,
            Bleat => self.bleat,
        }
    }

    const fn get_mut(&mut self, key: PlayerAction) -> &mut bool {
        match key {
            Up => &mut self.up,
            Down => &mut self.down,
            Left => &mut self.left,
            Right => &mut self.right,
            Bleat => &mut self.bleat,
        }
    }
}

fn record_input(
    mut input: MessageReader<KeyboardInput>,
    scheme: Res<ControlScheme>,
    mut output: ResMut<PlayerInput>,
) {
    for message in input.read() {
        for key in scheme.by_keycode(message.key_code) {
            match message.state {
                ButtonState::Pressed => *output.get_mut(key) = true,
                ButtonState::Released => *output.get_mut(key) = false,
            }
        }
    }
}

pub fn just_pressed(key: PlayerAction) -> impl SystemCondition<()> {
    IntoSystem::into_system(move |input: Res<PlayerInput>, mut pressed: Local<bool>| {
        let new = input.get(key);
        let output = new && !*pressed;
        *pressed = new;
        output
    })
}
