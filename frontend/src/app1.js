// import { ProgramMetadata } from '@gear-js/api';
// import { useReadFullState } from '@gear-js/react-hooks';
import { GearApi } from '@gear-js/api';
import { Sails } from 'sails-js';
import { SailsIdlParser } from 'sails-js-parser';
import { Keyring } from '@polkadot/api';
import { Program } from './lib1.js';
import { readFileSync } from 'fs';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { waitReady } from '@polkadot/wasm-crypto'; 
await waitReady();

// Set the account using the KeyringPair instance

const parser = await SailsIdlParser.new();
const sails = new Sails(parser);

const idl = `
type Event = struct {
  event_id: u32,
  venue: str,
  time: str,
  description: str,
  };
  
  constructor {
    New : ();
    };
    
    service Common {
      AddAdmin : (addr: actor_id) -> bool;
      query DisplayEvents : () -> map (actor_id, vec Event);
      query GetAdmins : () -> vec actor_id;
      };
      
      service Events {
        CancelEvent : (event_id: u32) -> bool;
        CreateEvent : (event_details: struct { u32, str, str, str }) -> bool;
        UpdateEvent : (event_details: struct { u32, str, str, str }) -> bool;
        
        events {
          Created: struct { event_id: u32, venue: str, time: str, description: str };
          Updated: struct { event_id: u32, venue: str, time: str, description: str };
          Cancelled: struct { event_id: u32 };
          }
          };
          
          `
          
          async function State() {
            console.log("hello");
            try{
              sails.parseIdl(idl);
              const gearApi = await GearApi.create({
                providerAddress: 'wss://testnet.vara.network',
              });
              const provider = new WsProvider('wss://testnet.vara.network'); // Replace with your node URL
              const api = await ApiPromise.create({ provider });
              const program = new Program(gearApi);
              const code = readFileSync('../../target/wasm32-unknown-unknown/wasm32-unknown-unknown/release/vara_ticket.opt.wasm');
              const keyring = new Keyring({ type: 'sr25519' });
              const pair = keyring.addFromUri('loud region column elbow torch grace iron safe mirror wedding angle panel'); // Replace with your account URI

              const ctorBuilder = await program.newCtorFromCode(code, null, null).withAccount(pair).calculateGas();
              const { blockHash, msgId, txHash } = await ctorBuilder.signAndSend();
              console.log(
                `\nProgram deployed. \n\tprogram id ${program.programId}, \n\tblock hash: ${blockHash}, \n\ttx hash: ${txHash}, \n\tinit message id: ${msgId}`,
              );
              // Get the balance of the pair (account)
              const { data: balance } = await api.query.system.account(pair.address);
            
              console.log(`Free balance: ${balance.free.toHuman()}`);
              console.log(`Reserved balance: ${balance.reserved.toHuman()}`);
            
            console.log("transaction:","end");

  }catch(e){
    console.log("error:",e);
  }
}

// listenToEvents().catch((error) => {
//   console.error('An error occurred:', error);
// });

State();
