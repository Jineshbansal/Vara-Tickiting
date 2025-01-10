use super::{common::Storage, funds::FundService};
use funcs::{cancel_and_refund, purchase_ticket, transfer};
use sails_rs::{gstd::msg, prelude::*};
pub mod funcs;

#[derive(Clone)]
pub struct AudienceService {
    pub funds: FundService,
}

#[sails_rs::service(extends = FundService)]
impl AudienceService {
    pub fn new() -> Self {
        Self {
            funds: FundService::new(),
        }
    }

    pub fn purchase_ticket(&mut self, ticket_count: u8, event_id: u32) -> bool {
        let audience = Storage::get_audience();
        purchase_ticket((ticket_count, event_id, msg::source()), audience);
        self.funds.purchase_ticket(event_id, ticket_count)
    }

    pub fn cancel_and_refund(&mut self, ticket_count: u8, event_id: u32) -> bool {
        let audience = Storage::get_audience();
        cancel_and_refund((ticket_count, event_id, msg::source()), audience)

        // TODO! refund process unimplemented. has to execute the transfer function from
        // this contarct only using exec::something() ???
        // using msg::send()
    }

    pub fn transfer_ticket(&self, ticket_count: u8, event_id: u32, transfer_id: ActorId) -> bool {
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
