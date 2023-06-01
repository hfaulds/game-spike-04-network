use bevy::prelude::*;
use naia_bevy_server::{RoomKey, UserKey};
use std::collections::HashMap;

#[derive(Resource)]
pub struct Global {
    pub main_room_key: RoomKey,
    pub user_to_ship: HashMap<UserKey, Entity>,
}

impl Global {
    pub fn new(main_room_key: RoomKey) -> Self {
        Self {
            main_room_key,
            user_to_ship: HashMap::new(),
        }
    }
}
