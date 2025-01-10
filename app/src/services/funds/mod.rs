#![allow(static_mut_refs)]

use func::{burn, mint};
use sails_rs::{
    collections::{HashMap, HashSet},
    gstd::{exec, msg},
    prelude::*,
};
use vft_service::{Service as VftService, Storage};

mod func;

#[derive(Default)]
pub struct FundStorage {
    pub minters: HashSet<ActorId>,
    pub burners: HashSet<ActorId>,
    pub admins: HashSet<ActorId>,
    pub ticket_prices: HashMap<u32, U256>,
    pub onboard_price: U256,
    pub cancel_fine: U256,
}

static mut FUND_STORAGE: Option<FundStorage> = None;

#[derive(Clone)]
pub struct FundService {
    pub vft: VftService,
}

impl FundStorage {
    pub fn get_prices() -> &'static mut HashMap<u32, U256> {
        unsafe {
            &mut FUND_STORAGE
                .as_mut()
                .expect("Not initialised")
                .ticket_prices
        }
    }
}

impl FundService {
    pub fn seed(name: String, symbol: String, decimals: u8) -> Self {
        let admin = msg::source();
        unsafe {
            FUND_STORAGE = Some(FundStorage {
                admins: [admin].into(),
                minters: [admin].into(),
                burners: [admin].into(),
                ticket_prices: HashMap::new(),
                onboard_price: U256::from(500),
                cancel_fine: U256::from(250),
            });
        };
        FundService {
            vft: <VftService>::seed(name, symbol, decimals),
        }
    }

    pub fn get_mut(&mut self) -> &'static mut FundStorage {
        unsafe { FUND_STORAGE.as_mut().expect("Not initialised") }
    }

    pub fn get(&self) -> &'static FundStorage {
        unsafe { FUND_STORAGE.as_ref().expect("Not initliased") }
    }
}

impl AsRef<VftService> for FundService {
    fn as_ref(&self) -> &VftService {
        &self.vft
    }
}

#[sails_rs::service(extends = VftService)]
impl FundService {
    pub fn new() -> Self {
        Self {
            vft: VftService::new(),
        }
    }

    pub fn get_ticket_prices(&self) -> Vec<(u32, U256)> {
        let prices = self.get().ticket_prices.clone();
        prices.into_iter().collect()
    }

    pub fn mint(&mut self, to: ActorId, value: U256) -> bool {
        if !self.get().minters.contains(&msg::source()) {
            panic!("Not allowed to mint")
        };

        mint(Storage::balances(), Storage::total_supply(), to, value)
    }

    pub fn burn(&mut self, from: ActorId, value: U256) -> bool {
        if !self.get().burners.contains(&msg::source()) {
            panic!("Not allowed to burn")
        };

        burn(Storage::balances(), Storage::total_supply(), from, value)
    }

    pub fn purchase_ticket(&mut self, event_id: u32, ticket_count: u8) -> bool {
        let ticket_price = self
            .get()
            .ticket_prices
            .get(&event_id)
            .expect("Event does not exist");

        self.vft
            .transfer(exec::program_id(), *ticket_price * ticket_count)
    }

    pub fn create_event(&mut self) -> bool {
        self.vft
            .transfer(exec::program_id(), self.get().onboard_price)
    }

    pub fn cancel_event(&mut self) -> bool {
        self.vft
            .transfer(exec::program_id(), self.get().cancel_fine)
    }
}
