use bevy::prelude::*;
use shared::messages::MovementCommand;

#[derive(Resource, Default)]
pub struct Global {
    pub queued_command: Option<MovementCommand>,
}
