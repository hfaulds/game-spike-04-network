use super::super::resources::Global;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use naia_bevy_server::{
    events::{ConnectEvent, TickEvent},
    CommandsExt, Server,
};
use shared::{
    channels::{PlayerCommandChannel, SyncShipPositionChannel},
    components::Ship,
    messages::{MovementCommand, SyncShipPosition},
};

pub fn connect_events(
    mut commands: Commands,
    mut global: ResMut<Global>,
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
            .insert(ExternalImpulse::default())
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
            .id();

        server.room_mut(&global.main_room_key).add_entity(&entity);
        global.user_to_ship.insert(*user_key, entity);
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

pub fn tick_events(
    global: Res<Global>,
    mut server: Server,
    mut tick_reader: EventReader<TickEvent>,
    mut ship_impulses: Query<&mut ExternalImpulse, With<Ship>>,
) {
    let mut has_ticked = false;

    for TickEvent(server_tick) in tick_reader.iter() {
        has_ticked = true;

        let mut messages = server.receive_tick_buffer_messages(server_tick);
        for (user_key, command) in messages.read::<PlayerCommandChannel, MovementCommand>() {
            if command.up {
                let Some(entity) = global.user_to_ship.get(&user_key) else {
                    continue;
                };
                let Ok(mut impulse) = ship_impulses.get_mut(*entity) else {
                    continue;
                };
                impulse.impulse = Vec2::new(0.0, 5.0);
            }
        }
    }

    if has_ticked {
        for (_, user_key, entity) in server.scope_checks() {
            // You'd normally do whatever checks you need to in here..
            // to determine whether each Entity should be in scope or not.

            // This indicates the Entity should be in this scope.
            server.user_scope(&user_key).include(&entity);

            // And call this if Entity should NOT be in this scope.
            // server.user_scope(..).exclude(..);
        }
    }
}
