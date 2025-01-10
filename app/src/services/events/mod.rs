use super::common::{Event, Storage};
use funcs::{cancel_event, create_event, update_event};
use sails_rs::{gstd::msg, prelude::*};
mod funcs;

// Host can create event, update event and cancel event
// TODO! If we implement ERC20 minting, can also add a withdraw funds functionality for the host
#[derive(Encode, TypeInfo)]
enum  EventMessage{
    Created { event_id: u32, venue: String, time: String, description: String },
    Updated { event_id: u32, venue: String, time: String, description: String },
    Cancelled { event_id: u32 },    
}

pub struct EventService(());

#[sails_rs::service(events=EventMessage)]
// TODO! Events implementation
impl EventService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn create_event(&mut self, event_details: (u32, String, String, String)) -> bool {
        let events = Storage::get_events();
        let event = Event {
            event_id: event_details.0.clone(),
            venue: event_details.1.clone(),
            time: event_details.2.clone(),
            description: event_details.3.clone(),
        };
        let _ = self.notify_on(EventMessage::Created { 
            event_id:event_details.0,
            venue: event_details.1,
            time: event_details.2,
            description: event_details.3,});
        create_event(&msg::source(), event, events)
    }

    pub fn update_event(&mut self, event_details: (u32, String, String, String)) -> bool {
        let events = Storage::get_events();
        let new_event = Event {
            event_id: event_details.0.clone(),
            venue: event_details.1.clone(),
            time: event_details.2.clone(),
            description: event_details.3.clone(),
        };
        let _ = self.notify_on(EventMessage::Updated { 
            event_id:event_details.0,
            venue: event_details.1,
            time: event_details.2,
            description: event_details.3,});
        update_event(&msg::source(), new_event, events)
    }

    pub fn cancel_event(&mut self, event_id: u32) -> bool {
        let events = Storage::get_events();
        let _ = self.notify_on(EventMessage::Cancelled { event_id });
        cancel_event(&msg::source(), event_id, events)
    }
}
