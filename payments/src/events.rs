use casper_event_standard::{emit, Event, Schemas};
use casper_types::{Key, PublicKey, U512};

pub enum Event {
    Payment(Payment),
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Payment {
    pub account: Key,
    pub amount: U512,
    pub recipient: PublicKey,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct TransferFundsTo {
    pub target: Key,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct TransferTokenTo {
    pub recipient: Key,
}

pub fn record_event(event: Event) {
    match event {
        Event::Payment(ev) => emit(ev),
    }
}

pub fn init_events() {
    let schemas = Schemas::new().with::<Payment>().with::<TransferFundsTo>().with::<TransferTokenTo>();
    casper_event_standard::init(schemas);
}
