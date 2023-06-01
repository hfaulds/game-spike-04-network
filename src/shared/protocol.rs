use crate::{channels::ChannelsPlugin, components::ComponentsPlugin, messages::MessagesPlugin};
use naia_bevy_shared::{LinkConditionerConfig, Protocol};
use std::time::Duration;

// Protocol Build
pub fn protocol() -> Protocol {
    Protocol::builder()
        // Config
        .tick_interval(Duration::from_millis(40))
        .link_condition(LinkConditionerConfig::good_condition())
        .enable_client_authoritative_entities()
        .add_plugin(ChannelsPlugin)
        .add_plugin(MessagesPlugin)
        .add_plugin(ComponentsPlugin)
        .build()
}
