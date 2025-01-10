#![no_std]
mod services;

use sails_rs::prelude::*;
use services::{audience::AudienceService, common::CommonService, events::EventService};

pub struct VaraTicketProgram(());

#[sails_rs::program]
impl VaraTicketProgram {
    // Program's constructor
    pub fn new() -> Self {
        CommonService::init();
        Self(())
    }

    pub fn common(&self) -> CommonService {
        CommonService::new()
    }

    pub fn events(&self) -> EventService {
        EventService::new()
    }

    pub fn audience(&self) -> AudienceService {
        AudienceService::new()
    }
}
