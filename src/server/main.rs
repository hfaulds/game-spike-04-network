use bevy::{
    app::{App, ScheduleRunnerSettings},
    log::LogPlugin,
    prelude::*,
};
use std::time::Duration;

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig};
use shared::protocol::protocol;

mod systems;
use systems::init;

fn main() {
    App::default()
        .add_plugins(MinimalPlugins)
        .insert_resource(
            // this is needed to avoid running the server at uncapped FPS
            ScheduleRunnerSettings::run_loop(Duration::from_millis(3)),
        )
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::new(
            ServerConfig {
                require_auth: false,
                ..Default::default()
            },
            protocol(),
        ))
        .add_startup_system(init)
        .run();
}