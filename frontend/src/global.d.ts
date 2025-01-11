declare global {
  export interface CommonEvent {
    event_id: number;
    venue: string;
    time: string;
    description: string;
    initial_price: number | string | bigint;
  }

};