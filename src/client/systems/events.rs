use std::default::Default;

use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
};

use naia_bevy_client::events::InsertComponentEvents;

use shared::components::Ship;

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
