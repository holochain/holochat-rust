use hdk::{
    self,
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::cas::content::Address,
    holochain_core_types::dna::zome::entry_types::Sharing,
    holochain_core_types::entry::{entry_type::EntryType, Entry},
    holochain_core_types::error::HolochainError,
    holochain_core_types::json::JsonString,
    AGENT_ADDRESS,
};
use std::convert::TryFrom;

use super::message;
use super::utils;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Channel {
    pub name: String,
    pub description: String,
    pub public: bool,
}

pub fn public_channel_definition() -> ValidatingEntryType {
    entry!(
        name: "public_channel",
        description: "A channel of which anyone can become a member and post",
        sharing: Sharing::Public,
        native_type: Channel,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_channel: Channel, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            agent_channel_link(),
            channel_message_link()
        ]
    )
}

pub fn direct_channel_definition() -> ValidatingEntryType {
    entry!(
        name: "direct_channel",
        description: "A channel to which new members can only be added at creation",
        sharing: Sharing::Public,
        native_type: Channel,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_channel: Channel, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            agent_channel_link(),
            channel_message_link()
        ]
    )
}

fn agent_channel_link() -> ValidatingLinkDefinition {
    from!(
        "%agent_id",
        tag: "rooms",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn channel_message_link() -> ValidatingLinkDefinition {
    to!(
        "message",
        tag: "message_in",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

// public zome functions

pub fn handle_create_channel(
    name: String,
    description: String,
    public: bool,
) -> JsonString {
    let channel = Channel {
        name,
        description,
        public,
    };

    let entry = match public {
        true => Entry::new(EntryType::App("public_channel".into()), channel),
        false => Entry::new(EntryType::App("direct_channel".into()), channel),
    };

    match hdk::commit_entry(&entry) {
        Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "rooms") {
            Ok(_) => json!({ "address": address }).into(),
            Err(hdk_err) => hdk_err.into(),
        },
        Err(hdk_err) => hdk_err.into(),
    }
}

pub fn handle_get_my_channels() -> JsonString {
    match get_my_channels() {
        Ok(result) => result.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}




pub fn handle_get_messages(channel_name: String) -> JsonString {
    match get_messages(channel_name) {
        Ok(result) => result.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}

pub fn handle_post_message(channel_name: String, message: message::Message) -> JsonString {
    from_channel(channel_name)
        .map(|s| hdk::entry_address(&s))
        .map(|channel_addr| {
            channel_addr.and_then(|addr| {
                hdk::commit_entry(&Entry::new(EntryType::App("message".into()), message))
                    .and_then(|message_addr| hdk::link_entries(&addr, &message_addr, "message_in"))
                    .map(|_| json!({"success": true}))
            })
        })
        .map(|s| s.map_err(|err| json!({"err": err.to_string()}).into()))
        .unwrap_or_else(|| Err(json!({"success": false})))
        .into()
}

fn from_channel(channel_name: String) -> Option<Entry> {
    get_my_channels().ok().and_then(|channels| {
        channels
            .iter()
            .filter(|f| f.name == channel_name)
            .map(|channel| match channel.public {
                true => Entry::new(EntryType::App("public_channel".into()), channel),
                false => Entry::new(EntryType::App("direct_channel".into()), channel),
            })
            .next()
    })
}

// end public zome functions

fn get_my_channels() -> ZomeApiResult<Vec<Channel>> {
    utils::get_links_and_load(&AGENT_ADDRESS, "rooms").map(|results| {
        results
            .iter()
            .map(|get_links_result| {
                Channel::try_from(get_links_result.entry.value().clone()).unwrap()
            })
            .collect()
    })
}


fn get_messages(channel_name: String) -> ZomeApiResult<Vec<message::Message>> {
    match from_channel(channel_name.clone()) {
        Some(entry) => match hdk::entry_address(&entry) {
            Ok(address) => utils::get_links_and_load(&address, "message_in").map(|results| {
                results
                    .iter()
                    .map(|get_links_result| {
                        message::Message::try_from(get_links_result.entry.value().clone()).unwrap()
                    })
                    .collect()
            }),
            Err(hdk_err) => Err(hdk_err),
        },
        None => Err(ZomeApiError::from(format!(
            "Channel {} doesn't exist",
            channel_name
        ))),
    }
}
