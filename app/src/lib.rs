#![no_std]
mod services;

use sails_rs::prelude::*;
use services::{common::CommonService, events::EventService};

struct VaraTicketService(());

#[sails_rs::service]
impl VaraTicketService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        "Hello from VaraTicket!".to_string()
    }

    // Service's query
    pub fn get_something(&self) -> String {
        "Hello from VaraTicket!".to_string()
    }
}

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
}
