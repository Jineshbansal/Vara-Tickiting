#![no_std]
mod services;

use sails_rs::prelude::*;
use services::{common::CommonService, events::EventService, funds::FundService};

pub struct VaraTicketProgram(());

#[sails_rs::program]
impl VaraTicketProgram {
    // Program's constructor
    pub fn new() -> Self {
        CommonService::init();
        FundService::seed("EZZ".to_string(), "EZZ".to_string(), 18);
        Self(())
    }

    pub fn common(&self) -> CommonService {
        CommonService::new()
    }

    pub fn events(&self) -> EventService {
        EventService::new()
    }
}
