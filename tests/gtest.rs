use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use vara_ticket_client::traits::*;

const ACTOR_ID: u64 = 42;
const USER_ID: [u64; 2] = [12, 13];

#[tokio::test]
async fn do_something() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);
    system.mint_to(USER_ID[0], 100_000_000_000_000);
    system.mint_to(USER_ID[1], 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(vara_ticket::WASM_BINARY);

    let program_factory = vara_ticket_client::VaraTicketFactory::new(remoting.clone());

    let program_id = program_factory
        .new("EZZ".to_string(), "EZZ".to_string(), 18)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = vara_ticket_client::VaraTicket::new(remoting.clone());

    let result = service_client
        .name("EZZ".to_string(), "EZZ".to_string(), 18) // Call service's method (see app/src/lib.rs:14)
        .send_recv(program_id)
        .await
        .unwrap();

    // assert_eq!(result, "Hello from VaraTicket!".to_string())
    println!("{result}");
}
