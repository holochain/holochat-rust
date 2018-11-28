use hdk::{
    self, 
    holochain_core_types::hash::HashString,
    holochain_core_types::entry::Entry,
    error::ZomeApiResult,
};

// #[derive(Serialize, Deserialize, Debug)]
pub struct GetLinksLoadElement {
	pub address: HashString,
	pub entry: Entry
}

pub type GetLinksLoadResult = Vec<GetLinksLoadElement>;



pub fn get_links_and_load<S: Into<String>>(
    base: &HashString, 
    tag: S
) -> ZomeApiResult<GetLinksLoadResult>  {
	hdk::get_links(base, tag)
		.map(|result| {
			result.addresses().iter()
				.map(|address| {
					hdk::get_entry(address.to_owned())
						.map(|entry: Option<Entry>| {
							GetLinksLoadElement{
								address: address.to_owned(),
								entry: entry.unwrap()
							}
						})
						.ok()
				})
				.filter_map(|elem| elem)
				.collect()
		})

}

