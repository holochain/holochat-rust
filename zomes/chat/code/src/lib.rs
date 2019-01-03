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

use hdk::holochain_core_types::{hash::HashString,json::JsonString};
mod message;
mod channel;
mod utils;


define_zome! {

	entries: [
		message::message_definition(),
    	channel::public_channel_definition(),
    	channel::direct_channel_definition()
	]

    genesis: || {
        {
			Ok(())
        }
    }

	functions: {
		main (Public) {
			create_channel: {
				inputs: |name: String, description: String,  public: bool|,
				outputs: |result: JsonString|,
				handler: channel::handle_create_channel
			}
			get_my_channels: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: channel::handle_get_my_channels
			}
			get_my_channel: {
				inputs: |channel_address:HashString|,
				outputs :|result:JsonString|,
				handler : channel::handle_get_my_channel
			}
            post_message: {
				inputs: |channel_name: String, message: message::Message|,
				outputs: |result: JsonString|,
				handler: channel::handle_post_message
			}
			get_messages: {
				inputs: |channel_name: String|,
				outputs: |result: JsonString|,
				handler: channel::handle_get_messages
			}
		
		}
	}
 }
