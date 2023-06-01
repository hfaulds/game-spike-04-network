use super::super::resources::Global;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use naia_bevy_server::{
    events::{ConnectEvent, TickEvent},
    CommandsExt, Server,
};
use shared::{
    channels::SyncShipPosition as SyncShipPositionChannel, components::Ship,
    messages::SyncShipPosition,
};

pub fn connect_events(
    mut commands: Commands,
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

        let entity = commands
            .spawn_empty()
            .enable_replication(&mut server)
            .insert(Ship::new_complete(0))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(50.0))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
            .id();

        server.room_mut(&global.main_room_key).add_entity(&entity);
    }
}

pub fn sync_ship_transforms(
    mut server: Server,
    positions: Query<(Entity, &Transform), With<Ship>>,
) {
    for (entity, transform) in positions.iter() {
        let mut position_message =
            SyncShipPosition::new(transform.translation.x, transform.translation.y);
        position_message.entity.set(&server, &entity);

        server.broadcast_message::<SyncShipPositionChannel, SyncShipPosition>(&position_message);
    }
}

pub fn tick_events(mut server: Server, mut _tick_reader: EventReader<TickEvent>) {
    for (_, user_key, entity) in server.scope_checks() {
        // You'd normally do whatever checks you need to in here..
        // to determine whether each Entity should be in scope or not.

        // This indicates the Entity should be in this scope.
        server.user_scope(&user_key).include(&entity);

        // And call this if Entity should NOT be in this scope.
        // server.user_scope(..).exclude(..);
    }
}
