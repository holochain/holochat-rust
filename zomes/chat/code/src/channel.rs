use hdk::{
    self,
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::cas::content::Address,
    holochain_core_types::dna::entry_types::Sharing,
    holochain_core_types::entry::Entry,
    holochain_core_types::error::HolochainError,
    holochain_core_types::json::JsonString,
    holochain_core_types::hash::HashString,
    AGENT_ADDRESS,
};

use super::message;
use crate::utils::{GetLinksLoadResult,get_links_and_load_type};

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
        true => Entry::App("public_channel".into(), channel.into()),
        false => Entry::App("direct_channel".into(), channel.into()),
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

pub fn handle_get_my_channel(channel_address : HashString) -> JsonString
{
    match hdk::get_entry(channel_address)
    {
        Ok(Some(entry)) => entry.into(),
        Ok(None) =>{}.into(),
        Err(err) =>err.into()
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
                hdk::commit_entry(&Entry::App("message".into(), message.into()))
                    .and_then(|message_addr| hdk::link_entries(&addr, &message_addr, "message_in"))
                    .map(|_| JsonString::from("success"))
            })
        })
        .map(|s| s.map_err(|err| JsonString::from(err.to_string())))
        .unwrap_or_else(|| Ok(JsonString::from("err")))
        .unwrap_or_else(|_|JsonString::from("err"))
        
}

fn from_channel(channel_name: String) -> Option<Entry> {
    get_my_channels().ok().and_then(|channels| {
        channels
            .iter()
            .filter(|f| f.name == channel_name)
            .map(|channel| match channel.public {
                true => Entry::App("public_channel".into(), channel.into()),
                false => Entry::App("direct_channel".into(), channel.into()),
            })
            .next()
    })
}

// end public zome functions

fn get_my_channels() -> ZomeApiResult<Vec<Channel>> {
    let room_links : GetLinksLoadResult<Channel> = get_links_and_load_type(&AGENT_ADDRESS, "rooms")?;
    let rooms = room_links.iter().map(|link|{
        link.entry.clone()
    }).collect::<Vec<_>>();
    Ok(rooms)
}


fn get_messages(channel_name: String) -> ZomeApiResult<Vec<message::Message>> {
    
    let channel = from_channel(channel_name.clone()).ok_or(ZomeApiError::from(format!(
            "Channel {} doesn't exist",
            channel_name
        )))?;
    let address = hdk::entry_address(&channel)?;
    let get_links_result  : GetLinksLoadResult<message::Message> = get_links_and_load_type(&address, "message_in")?;
    let messages = get_links_result.iter().map(|links|{
        links.entry.clone()
    }).collect::<Vec<_>>();
    Ok(messages)
    
}
