use bevy::{
    app::{App, ScheduleRunnerSettings},
    log::LogPlugin,
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};
use shared::protocol::protocol;

mod resources;
mod systems;

use systems::events;
use systems::init;

fn main() {
    App::default()
        .add_plugins(MinimalPlugins)
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(3)))
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::new(
            ServerConfig {
                require_auth: false,
                ..Default::default()
            },
            protocol(),
        ))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_startup_system(init)
        .add_systems(
            (
                events::connect_events,
                events::tick_events,
                events::sync_ship_transforms,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        .run();
}
