use bevy::prelude::{info, Camera2dBundle, Commands};

use naia_bevy_client::{transport::webrtc, Client};

pub fn init(mut commands: Commands, mut client: Client) {
    info!("Naia Bevy Client Demo started");
    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());
    client.connect(socket);

    // Setup Camera
    commands.spawn(Camera2dBundle::default());
}
