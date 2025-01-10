use super::{
    audience::AudienceService,
    common::{Event, Storage},
};
use funcs::{cancel_event, create_event, update_event};
use sails_rs::{gstd::msg, prelude::*};
pub mod funcs;

// Host can create event, update event and cancel event
// TODO! If we implement ERC20 minting, can also add a withdraw funds functionality for the host

pub struct EventService {
    audience: AudienceService,
}

#[sails_rs::service(extends = AudienceService)]
// TODO! Events implementation
impl EventService {
    pub fn new() -> Self {
        Self {
            audience: AudienceService::new(),
        }
    }

    pub fn create_event(&mut self, event_details: (u32, String, String, String, U256)) -> bool {
        let events = Storage::get_events();
        let event = Event {
            event_id: event_details.0,
            venue: event_details.1,
            time: event_details.2,
            description: event_details.3,
            initial_price: event_details.4,
        };
        create_event(&msg::source(), event, events);

        self.audience.funds.create_event()
    }

    pub fn update_event(&self, event_details: (u32, String, String, String, U256)) -> bool {
        let events = Storage::get_events();
        let new_event = Event {
            event_id: event_details.0,
            venue: event_details.1,
            time: event_details.2,
            description: event_details.3,
            initial_price: event_details.4,
        };

        update_event(&msg::source(), new_event, events)
    }

    pub fn cancel_event(&mut self, event_id: u32) -> bool {
        let events = Storage::get_events();

        cancel_event(&msg::source(), event_id, events)
    }
}

impl AsRef<AudienceService> for EventService {
    fn as_ref(&self) -> &AudienceService {
        &self.audience
    }
}
