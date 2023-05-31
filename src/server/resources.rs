use bevy::prelude::*;
use naia_bevy_server::RoomKey;

#[derive(Resource)]
pub struct Global {
    pub main_room_key: RoomKey,
}
