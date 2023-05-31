use bevy::prelude::{
    info, shape, Assets, Camera2dBundle, Color, ColorMaterial, Commands, Mesh, ResMut,
};

use naia_bevy_client::{transport::webrtc, Client};

pub fn init(
    mut commands: Commands,
    mut client: Client,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Naia Bevy Client Demo started");
    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());
    client.connect(socket);

    // Setup Camera
    commands.spawn(Camera2dBundle::default());
}
