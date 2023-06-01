use bevy::prelude::*;
use naia_bevy_shared::{Property, Protocol, ProtocolPlugin, Replicate};

#[derive(Component, Replicate)]
pub struct Ship {
    id: Property<u16>,
}

// Plugin
pub struct ComponentsPlugin;

impl ProtocolPlugin for ComponentsPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol.add_component::<Ship>();
    }
}
