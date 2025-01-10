#![allow(static_mut_refs)]

use func::{burn, mint};
use sails_rs::{
    collections::{HashMap, HashSet},
    gstd::msg,
    prelude::*,
};
use vft_service::{Service as VftService, Storage};

mod func;

#[derive(Default)]
pub struct FundStorage {
    minters: HashSet<ActorId>,
    burners: HashSet<ActorId>,
    admins: HashSet<ActorId>,
    audience: HashMap<ActorId, u32>,
}

static mut FUND_STORAGE: Option<FundStorage> = None;

#[derive(Clone)]
pub struct FundService {
    vft: VftService,
}

impl FundService {
    pub fn seed(name: String, symbol: String, decimals: u8) -> Self {
        let admin = msg::source();
        unsafe {
            FUND_STORAGE = Some(FundStorage {
                admins: [admin].into(),
                minters: [admin].into(),
                burners: [admin].into(),
                audience: HashMap::new(),
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
}
