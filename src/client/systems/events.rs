use super::super::resources::Global;
use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
};
use naia_bevy_client::{
    events::{ClientTickEvent, InsertComponentEvents, MessageEvents},
    Client,
};
use shared::{
    channels::{PlayerCommandChannel, SyncShipPositionChannel},
    components::Ship,
    messages::{MovementCommand, SyncShipPosition},
};
use std::default::Default;

const SQUARE_SIZE: f32 = 32.0;

pub fn insert_component_events(
    mut commands: Commands,
    mut event_reader: EventReader<InsertComponentEvents>,
) {
    for events in event_reader.iter() {
        for entity in events.read::<Ship>() {
            commands.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        }
    }
}

pub fn sync_ship_positions(
    client: Client,
    mut event_reader: EventReader<MessageEvents>,
    mut positions: Query<&mut Transform, With<Ship>>,
) {
    for events in event_reader.iter() {
        for message in events.read::<SyncShipPositionChannel, SyncShipPosition>() {
            let entity = message.entity.get(&client).unwrap();
            let mut transform = positions.get_mut(entity).unwrap();
            transform.translation.x = message.x;
            transform.translation.y = message.y;
        }
    }
}

pub fn tick_events(
    mut client: Client,
    mut tick_reader: EventReader<ClientTickEvent>,
    mut global: ResMut<Global>,
) {
    let Some(command) = global.queued_command.take() else {
        return;
    };

    for ClientTickEvent(client_tick) in tick_reader.iter() {
        client.send_tick_buffer_message::<PlayerCommandChannel, MovementCommand>(
            client_tick,
            &command,
        );
    }
}
