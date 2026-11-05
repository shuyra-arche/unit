pub struct Identity {
    pub aliases: alloc::vec::Vec<&'static str>,
    pub grants: alloc::vec::Vec<system_api::identity::Grant>,
}
