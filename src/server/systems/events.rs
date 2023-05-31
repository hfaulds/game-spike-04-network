use super::super::resources::Global;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use naia_bevy_server::{events::ConnectEvent, Server};

pub fn connect_events(
    global: Res<Global>,
    mut server: Server,
    mut event_reader: EventReader<ConnectEvent>,
) {
    for ConnectEvent(user_key) in event_reader.iter() {
        let address = server
            .user_mut(user_key)
            .enter_room(&global.main_room_key)
            .address();

        info!("Naia Server connected to: {}", address);
    }
}
