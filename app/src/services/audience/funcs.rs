use sails_rs::collections::HashMap;
use sails_rs::prelude::*;

use crate::services::funds::FundStorage;

pub fn purchase_ticket(
    (tickets, event_id, addr): (u8, u32, ActorId),
    audience: &mut HashMap<u32, Vec<(ActorId, U256)>>,
) -> bool {
    let ticket_price = FundStorage::get_prices()
        .get(&event_id)
        .expect("Event does not exist");

    for _ in 0..tickets {
        audience
            .entry(event_id)
            .and_modify(|x| x.push((addr, *ticket_price)))
            .or_insert(vec![(addr, *ticket_price)]);
    }
    true
}

pub fn cancel_and_refund(
    (tickets, event_id, addr): (u8, u32, ActorId),
    audience: &mut HashMap<u32, Vec<(ActorId, U256)>>,
) -> bool {
    if let Some(audience_list) = audience.get_mut(&event_id) {
        let mut removed = 0;

        audience_list.retain(|aud| {
            if removed < tickets && aud.0 == addr {
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
    audience: &mut HashMap<u32, Vec<(ActorId, U256)>>,
) -> bool {
    if let Some(audience_list) = audience.get_mut(&event_id) {
        let mut removed = 0;

        audience_list.iter_mut().for_each(|aud| {
            if removed < tickets && aud.0 == owner_addr {
                removed += 1;
                aud.0 = transfer_addr;
            }
        });

        return true;
    } else {
        return false;
    }
}
