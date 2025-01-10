use super::common::Storage;
use funcs::{purchase_ticket, refund, transfer};
use sails_rs::{gstd::msg, prelude::*};
pub mod funcs;

pub struct AudienceService(());

#[sails_rs::service]
impl AudienceService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn purchase_ticket(&self, ticket_count: u8, event_id: u32) -> bool {
        let audience = Storage::get_audience();
        purchase_ticket((ticket_count, event_id, msg::source()), audience)
    }

    pub fn refund(&self, ticket_count: u8, event_id: u32) -> bool {
        let audience = Storage::get_audience();
        refund((ticket_count, event_id, msg::source()), audience)
    }

    pub fn transfer(&self, ticket_count: u8, event_id: u32, transfer_id: ActorId) -> bool {
        let audience = Storage::get_audience();
        transfer(
            (ticket_count, event_id, msg::source(), transfer_id),
            audience,
        )
    }
}
