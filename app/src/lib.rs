#![no_std]
mod services;

use sails_rs::prelude::*;
use services::{common::CommonService, events::EventService, funds::FundService};

pub struct VaraTicketProgram(());

#[sails_rs::program]
impl VaraTicketProgram {
    // Program's constructor
    pub fn new(name: String, symbol: String, decimals: u8) -> Self {
        CommonService::init();
        FundService::seed(name, symbol, decimals);
        Self(())
    }

    pub fn common(&self) -> CommonService {
        CommonService::new()
    }

    pub fn events(&self) -> EventService {
        EventService::new()
    }
}
