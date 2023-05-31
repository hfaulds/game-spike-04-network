use bevy::{
    prelude::{App, ClearColor, Color, SystemSet},
    DefaultPlugins,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};
use shared::protocol::protocol;

mod systems;
use systems::init;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MainLoop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct Tick;

pub fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(init)
        /*.add_systems(
            (
                events::connect_events,
                events::disconnect_events,
                events::reject_events,
                events::spawn_entity_events,
                events::despawn_entity_events,
                events::insert_component_events,
                events::update_component_events,
                events::remove_component_events,
                events::message_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )*/
        // Tick Event
        /*.configure_set(Tick.after(ReceiveEvents))
        .add_system(events::tick_events.in_set(Tick))*/
        // Realtime Gameplay Loop
        /*
        .configure_set(MainLoop.after(Tick))
        .add_systems(
            (
                input::key_input,
                input::cursor_input,
                sync::sync_clientside_sprites,
                sync::sync_serverside_sprites,
                sync::sync_cursor_sprite,
            )
                .chain()
                .in_set(MainLoop),
        )
        */
        // Run App
        .run();
}
