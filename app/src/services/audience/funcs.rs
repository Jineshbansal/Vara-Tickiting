use sails_rs::collections::HashMap;
use sails_rs::prelude::*;

pub fn purchase_ticket(
    (tickets, event_id, addr): (u8, u32, ActorId),
    audience: &mut HashMap<u32, Vec<ActorId>>,
) -> bool {
    for _ in 0..tickets {
        audience
            .entry(event_id)
            .and_modify(|x| x.push(addr))
            .or_insert(vec![addr]);
    }
    true
}

pub fn cancel_refund(
    (tickets, event_id, addr): (u8, u32, ActorId),
    audience: &mut HashMap<u32, Vec<ActorId>>,
) -> bool {
    if let Some(audience_list) = audience.get_mut(&event_id) {
        let mut removed = 0;

        audience_list.retain(|aud| {
            if removed < tickets && *aud == addr {
                removed += 1;
                false // Remove this entry
            } else {
                true // Keep this entry
            }
        });

        true
    } else {
        false
    }
    // TODO! Refund 80% of the ticket price
}

pub fn transfer(
    (tickets, event_id, owner_addr, transfer_addr): (u8, u32, ActorId, ActorId),
    audience: &mut HashMap<u32, Vec<ActorId>>,
) -> bool {
    if let Some(audience_list) = audience.get_mut(&event_id) {
        let mut removed = 0;

        audience_list.iter_mut().for_each(|aud| {
            if removed < tickets && *aud == owner_addr {
                removed += 1;
                *aud = transfer_addr;
            }
        });

        return true;
    } else {
        return false;
    }
}
