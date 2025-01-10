// import { ProgramMetadata } from '@gear-js/api';
// import { useReadFullState } from '@gear-js/react-hooks';
import { GearApi } from '@gear-js/api';
import { Sails } from 'sails-js';
import { SailsIdlParser } from 'sails-js-parser';
import { Keyring } from '@polkadot/api';
import { Program } from './lib.js';
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
            const programId = '0x...';
            const metadataHex = '0x...';
            const payload = null;
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
            
            //   sails.setApi(gearApi);
              
            //   // console.log(sails.ctors);
            //   // console.log(sails.ctors.ConstructorName);
            //   // console.log(sails.services);
            //   sails.setProgramId('0x71912cb483e3f7d15a0da254b4b06554f180cebf551d43831e8f0d8d2dae79bb');
            //   const alice = 'kGkLEU3e3XXkJp2WK4eNpVmSab5xUNL9QtmLPh8QfCL2EgotW';
            //   // const transaction = await sails.services.Common.queries.GetAdmins(alice,null,null);
            //   const eventDetails = [123, "Venue Name", "2025-01-10", "Event Description"];

            //   const transaction=  sails.services.Events.functions.CreateEvent(eventDetails);
            //   transaction.withAccount(pair);
            //   // Calculate gas limit with default options
            //   await transaction.calculateGas();

            //   // Calculate gas limit allowing other panics and increasing gas limit by 10%
            //   await transaction.calculateGas(true, 10);
            //   // Set the gas limit manually
            //   transaction.withGas(100_000_000_000n);
            //   const voucherId = '0x24fa2e4990aae8205e3893301d3dc8df32dce5e7b940871e328b6ca106bbc772'; // Replace with actual voucher ID

            //   const fee = await transaction.transactionFee();
            //   console.log('Transaction fee:', fee.toString());
            //   const { msgId, blockHash, txHash, response, isFinalized } = await transaction.signAndSend();
              
            //   console.log('Message id:', msgId);
            //   console.log('Transaction hash:', txHash);
            //   console.log('Block hash:', blockHash);
            //   console.log('Is finalized:', await isFinalized);
              
            //   const result = await response();
            //   console.log(result);
            //   sails.services.Events.events.Created.subscribe((data) => {
            //     console.log("data:",data);
    // });
    console.log("transaction:","end");

  }catch(e){
    console.log("error:",e);
  }
}

// listenToEvents().catch((error) => {
//   console.error('An error occurred:', error);
// });

State();

