use super::{common::Storage, funds::FundService};
use funcs::{purchase_ticket, cancel_refund, transfer};
use sails_rs::{gstd::msg, prelude::*};
pub mod funcs;

#[derive(Clone)]
pub struct AudienceService{
    funds: FundService,
}

#[sails_rs::service(extends = FundService)]
impl AudienceService {
    pub fn new() -> Self {
        Self{
            funds: FundService::new(),
        }
    }

    pub fn purchase_ticket(&self, ticket_count: u8, event_id: u32) -> bool {
        let audience = Storage::get_audience();
        purchase_ticket((ticket_count, event_id, msg::source()), audience)
    }

    pub fn cancel_refund(&self, ticket_count: u8, event_id: u32) -> bool {
        let audience = Storage::get_audience();
        cancel_refund((ticket_count, event_id, msg::source()), audience)
    }

    pub fn transfer(&self, ticket_count: u8, event_id: u32, transfer_id: ActorId) -> bool {
        let audience = Storage::get_audience();
        transfer(
            (ticket_count, event_id, msg::source(), transfer_id),
            audience,
        )
    }
}

impl AsRef<FundService> for AudienceService {
    fn as_ref(&self) -> &FundService {
        &self.funds
    }
}