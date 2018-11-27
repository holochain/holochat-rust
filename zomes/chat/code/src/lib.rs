#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate machine_ip;


use hdk::{
    holochain_core_types::hash::HashString,
    holochain_core_types::entry::{Entry,entry_type::EntryType},
    holochain_core_types::dna::zome::entry_types::Sharing
};


mod message;
mod channel;
mod member;
mod utils;

use crate::member::{
    StoreProfile
};

define_zome! {

	entries: [
		message::message_definition(),
    	channel::public_channel_definition(),
    	channel::direct_channel_definition(),
		member::member_id_definition(),
        member::profile_definition()
	]

    genesis: || {
        {
            let member_entry = Entry::new(EntryType::App("member".into()), member::Member{id: machine_ip::get().unwrap().to_string(), profile:None});
            hdk::commit_entry(&member_entry).map_err(|_| "member not committed").unwrap();
            Ok(())
        }
    }

	functions: {
		main (Public) {
			create_channel: {
				inputs: |name: String, description: String, initial_members: Vec<member::Member>, public: bool|,
				outputs: |result: JsonString|,
				handler: channel::handle_create_channel
			}
			get_my_channels: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: channel::handle_get_my_channels
			}
            post_message: {
				inputs: |channel_name: String, message: message::Message|,
				outputs: |result: JsonString|,
				handler: channel::handle_post_message
			}
			get_messages: {
				inputs: |channel_address: String, min_count: u32|,
				outputs: |result: JsonString|,
				handler: channel::handle_get_messages
			}
			get_profile: {
				inputs: |member_id: member::Member|,
				outputs: |result: JsonString|,
				handler: member::handle_get_profile
			}
		}
	}
 }
