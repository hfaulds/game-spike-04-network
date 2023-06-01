use bevy::{prelude::*, DefaultPlugins};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};
use shared::protocol::protocol;

mod systems;
use systems::events;
use systems::init;
use systems::input;

mod resources;

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
        .configure_set(Tick.after(ReceiveEvents))
        .add_system(events::tick_events.in_set(Tick))
        .configure_set(MainLoop.after(Tick))
        .add_system(input::key_input.in_set(MainLoop))
        // Run App
        .run();
}
