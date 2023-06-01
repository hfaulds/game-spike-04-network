use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
};
use naia_bevy_client::events::InsertComponentEvents;
use naia_bevy_client::{events::MessageEvents, Client};
use shared::{
    channels::SyncShipPosition as SyncShipPositionChannel, components::Ship,
    messages::SyncShipPosition,
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
