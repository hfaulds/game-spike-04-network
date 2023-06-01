use bevy::{prelude::*, DefaultPlugins};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};
use shared::protocol::protocol;

mod systems;
use systems::events;
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
        .add_systems(
            (events::insert_component_events, events::sync_ship_positions)
                .chain()
                .in_set(ReceiveEvents),
        )
        // Tick Event
        //.configure_set(Tick.after(ReceiveEvents))
        //.add_system(events::tick_events.in_set(Tick))
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
