use bevy::prelude::*;

use crate::resources::Global;
use shared::messages::MovementCommand;

pub fn key_input(mut global: ResMut<Global>, keyboard_input: Res<Input<KeyCode>>) {
    let up = keyboard_input.pressed(KeyCode::W);
    let down = keyboard_input.pressed(KeyCode::S);
    let left = keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::D);
    global.queued_command = Some(MovementCommand {
        up,
        down,
        left,
        right,
    });
}
