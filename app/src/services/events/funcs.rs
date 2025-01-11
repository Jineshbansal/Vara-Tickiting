use sails_rs::gstd::{exec, msg};
use sails_rs::collections::HashMap;
use sails_rs::prelude::*;

use crate::services::common::Event;
use crate::services::funds::FundStorage;
use super::VaraEventError;


pub fn create_event(
    host_id: &ActorId,
    event: Event,
    events: &mut HashMap<ActorId, Vec<Event>>,
) -> bool {
    if let Some(list) = events.get(host_id) {
        for list_event in list {
            if list_event.event_id == event.event_id {
                return false;
            }
        }
    }

    let ticket_prices = FundStorage::get_prices();
    ticket_prices.insert(event.event_id.clone(), event.initial_price);
    events.insert(*host_id, vec![event.clone()]);
    let gas_limit:u64 = 100_000_000_000;
    match send_delayed_notification( gas_limit, 10, event.event_id) {
        Ok(_) => (),
        Err(_) => return false,
    };
    

    true
}

pub fn update_event(
    host_id: &ActorId,
    new_event: Event,
    events: &mut HashMap<ActorId, Vec<Event>>,
) -> bool {
    if let Some(list) = events.get_mut(host_id) {
        for list_event in list.iter_mut() {
            if list_event.event_id == new_event.event_id {
                *list_event = new_event;

                return true;
            }
        }
        return false;
    }
    false
}

pub fn cancel_event(
    host_id: &ActorId,
    event_id: u32,
    events: &mut HashMap<ActorId, Vec<Event>>,
) -> bool {
    if let Some(list) = events.get_mut(host_id) {
        for (index, list_event) in list.iter().enumerate() {
            if list_event.event_id == event_id {
                list.remove(index);
                let ticket_prices = FundStorage::get_prices();
                ticket_prices.remove(&event_id);

                return true;
            }
        }
        return false;
    }
    false
}


fn send_delayed_notification(
    gas_limit: u64,
    delay: u32,
    event_id:u32,
) -> Result<(), VaraEventError> {

    let request = [
        "Events".encode(),
        "SendNotification".to_string().encode(),
        (event_id).encode(),
    ]
    .concat();
    msg::send_bytes_with_gas_delayed(exec::program_id(), request, gas_limit, 0,1).expect("Failed to send delayed message");

    Ok(())

}
