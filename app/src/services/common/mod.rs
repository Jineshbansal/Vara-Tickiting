#![allow(static_mut_refs)]

use sails_rs::{collections::BTreeMap, collections::HashMap, gstd::msg, prelude::*};

use super::audience;

pub static mut STORAGE: Option<Storage> = None;
#[derive(Default, Debug, Clone)]
pub struct Storage {
    pub events: HashMap<ActorId, Vec<Event>>,
    pub audience: HashMap<u32, Vec<(ActorId, U256)>>,
    pub admin: Vec<ActorId>,
}

#[derive(Default, Debug, Clone, TypeInfo, Encode, Decode)]
pub struct Event {
    pub event_id: u32,
    pub venue: String,
    pub time: String,
    pub description: String,
    pub initial_price: U256,
}

impl Storage {
    pub fn get_audience() -> &'static mut HashMap<u32, Vec<(ActorId, U256)>> {
        unsafe { &mut STORAGE.as_mut().expect("Not yet initialised").audience }
    }

    pub fn get_events() -> &'static mut HashMap<ActorId, Vec<Event>> {
        unsafe { &mut STORAGE.as_mut().expect("Not yet initialised").events }
    }

    pub fn get_admin() -> &'static mut Vec<ActorId> {
        unsafe { &mut STORAGE.as_mut().expect("Not yet initialised").admin }
    }
}

#[derive(Clone)]
pub struct CommonService(());

impl CommonService {
    pub fn init() -> Self {
        let admin = msg::source();
        unsafe {
            STORAGE = Some(Storage {
                events: HashMap::new(),
                audience: HashMap::new(),
                admin: vec![admin],
            })
        }
        Self(())
    }

    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Not yet initilised") }
    }

    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Not yet initialised") }
    }
}

#[sails_rs::service]
impl CommonService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn add_admin(&mut self, addr: ActorId) -> bool {
        let admins = &mut self.get_mut().admin;
        if admins.contains(&msg::source()) && !admins.contains(&addr) {
            admins.push(addr);
            return true;
        }
        false
    }

    // TODO! did not returned all the events
    pub fn display_events(&self) -> Vec<(ActorId, Vec<Event>)> {
        let events = self.get().events.clone();
        events.into_iter().collect()
    }

    pub fn get_admins(&self) -> Vec<ActorId> {
        let admins = self.get().admin.clone();
        admins
    }

    pub fn get_events_name(&self) -> Vec<(ActorId, Vec<Event>)> {
        let events: HashMap<ActorId, Vec<Event>> = self.get().events.clone();
        events.into_iter().collect()
    }

    pub fn get_audience(&self) -> Vec<(u32, Vec<(ActorId, U256)>)> {
        let audience = self.get().audience.clone();
        audience.into_iter().collect()
    }
}

fn convert_to_btree<K, V>(hash_map: HashMap<K, V>) -> BTreeMap<K, V>
where
    K: Ord,
{
    BTreeMap::from_iter(hash_map.into_iter())
}
