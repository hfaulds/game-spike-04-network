use naia_bevy_shared::{EntityProperty, Message};
use naia_bevy_shared::{Protocol, ProtocolPlugin};

#[derive(Message)]
pub struct SyncShipPosition {
    pub entity: EntityProperty,
    pub x: f32,
    pub y: f32,
}

impl SyncShipPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            entity: EntityProperty::new(),
        }
    }
}

// Plugin
pub struct MessagesPlugin;

impl ProtocolPlugin for MessagesPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol.add_message::<SyncShipPosition>();
    }
}
