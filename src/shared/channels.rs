use naia_bevy_shared::{
    Channel, ChannelDirection, ChannelMode, Protocol, ProtocolPlugin, ReliableSettings,
    TickBufferSettings,
};

#[derive(Channel)]
pub struct PlayerCommandChannel;

#[derive(Channel)]
pub struct SyncShipPositionChannel;

// Plugin
pub struct ChannelsPlugin;

impl ProtocolPlugin for ChannelsPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol
            .add_channel::<PlayerCommandChannel>(
                ChannelDirection::ClientToServer,
                ChannelMode::TickBuffered(TickBufferSettings::default()),
            )
            .add_channel::<SyncShipPositionChannel>(
                ChannelDirection::ServerToClient,
                ChannelMode::UnorderedReliable(ReliableSettings::default()),
            );
    }
}
