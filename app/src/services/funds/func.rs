use sails_rs::prelude::*;
use vft_service::{funcs, utils::BalancesMap};

pub fn mint(balances: &mut BalancesMap, total_supply: &mut U256, to: ActorId, value: U256) -> bool {
    if value.is_zero() {
        return false;
    }

    let new_total_supply = total_supply.checked_add(value).unwrap();
    let new_to = funcs::balance_of(balances, to).checked_add(value).unwrap();

    balances.insert(to, new_to);
    *total_supply = new_total_supply;

    true
}

pub fn burn(
    balances: &mut BalancesMap,
    total_supply: &mut U256,
    from: ActorId,
    value: U256,
) -> bool {
    if value.is_zero() {
        return false;
    }

    let new_total_supply = total_supply.checked_sub(value).unwrap();
    let new_from = funcs::balance_of(balances, from)
        .checked_sub(value)
        .unwrap();

    balances.insert(from, new_from);
    *total_supply = new_total_supply;
    true
}
