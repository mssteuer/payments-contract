use alloc::string::String;

use casper_event_standard::{emit, Event, Schemas};
use casper_types::{Key, U512};

pub enum Event {
    Payment(Payment),
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Payment {
    pub token: String,
    pub amount: U512,
    pub checkout_id: u64,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct TransferFundsTo {
    pub target: Key,
}

pub fn record_event(event: Event) {
    match event {
        Event::Payment(ev) => emit(ev),
    }
}

pub fn init_events() {
    let schemas = Schemas::new().with::<Payment>().with::<TransferFundsTo>();
    casper_event_standard::init(schemas);
}
