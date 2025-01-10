 use sails_rs::collections::HashMap;
use sails_rs::prelude::*;

use crate::services::common::Event;

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

    events.insert(*host_id, vec![event]);
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
                return true;
            }
        }
        return false;
    }
    false
}
