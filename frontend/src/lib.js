import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, ActorId, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export class Program {
  constructor(api, programId) {
    this.api = api;
    this._programId = programId;

    const types = {
      Event: { event_id: 'u32', venue: 'String', time: 'String', description: 'String' },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.common = new Common(this);
    this.events = new Events(this);
  }

  get programId() {
    if (!this._programId) throw new Error(`Program ID is not set`);
    return this._programId;
  }

  newCtorFromCode(code) {
    const builder = new TransactionBuilder(
      this.api,
      this.registry,
      'upload_program',
      'New',
      'String',
      'String',
      code
    );

    this._programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId) {
    const builder = new TransactionBuilder(
      this.api,
      this.registry,
      'create_program',
      'New',
      'String',
      'String',
      codeId
    );

    this._programId = builder.programId;
    return builder;
  }
}

export class Common {
  constructor(program) {
    this._program = program;
  }

  addAdmin(addr) {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Common', 'AddAdmin', addr],
      '(String, String, [u8;32])',
      'bool',
      this._program.programId
    );
  }

  async displayEvents(originAddress, value, atBlock) {
    const payload = this._program.registry.createType('(String, String)', ['Common', 'DisplayEvents']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });

    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, BTreeMap<[u8;32], Vec<Event>>)', reply.payload);
    return result[2].toJSON();
  }

  async getAdmins(originAddress, value, atBlock) {
    const payload = this._program.registry.createType('(String, String)', ['Common', 'GetAdmins']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });

    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Vec<[u8;32]>)', reply.payload);
    return result[2].toJSON();
  }
}

export class Events {
  constructor(program) {
    this._program = program;
  }

  cancelEvent(event_id) {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Events', 'CancelEvent', event_id],
      '(String, String, u32)',
      'bool',
      this._program.programId
    );
  }

  createEvent(event_details) {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Events', 'CreateEvent', event_details],
      '(String, String, (u32, String, String, String))',
      'bool',
      this._program.programId
    );
  }

  updateEvent(event_details) {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Events', 'UpdateEvent', event_details],
      '(String, String, (u32, String, String, String))',
      'bool',
      this._program.programId
    );
  }

  subscribeToCreatedEvent(callback) {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Events' && getFnNamePrefix(payload) === 'Created') {
        callback(this._program.registry.createType('(String, String, {"event_id":"u32","venue":"String","time":"String","description":"String"})', message.payload)[2].toJSON());
      }
    });
  }

  subscribeToUpdatedEvent(callback) {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Events' && getFnNamePrefix(payload) === 'Updated') {
        callback(this._program.registry.createType('(String, String, {"event_id":"u32","venue":"String","time":"String","description":"String"})', message.payload)[2].toJSON());
      }
    });
  }

  subscribeToCancelledEvent(callback) {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Events' && getFnNamePrefix(payload) === 'Cancelled') {
        callback(this._program.registry.createType('(String, String, {"event_id":"u32"})', message.payload)[2].toJSON());
      }
    });
  }
}
