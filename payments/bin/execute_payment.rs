#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{runtime_args, ContractHash, RuntimeArgs, URef, U512, PublicKey};
use payments::constants::{ARG_AMOUNT, ARG_PAYMENT_PROCESSOR_CONTRACT_HASH, ARG_RECIPIENT, GET_CONTRACT_CSPR_DEPOSIT_UREF_ENTRY_POINT_NAME, PROCESS_PAYMENT_ENTRY_POINT_NAME};

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let recipient: PublicKey = runtime::get_named_arg(ARG_RECIPIENT);
    let payment_processor_contract_hash: ContractHash =
        runtime::get_named_arg(ARG_PAYMENT_PROCESSOR_CONTRACT_HASH);

    let contract_cspr_deposit_purse: URef = runtime::call_contract(
        payment_processor_contract_hash,
        GET_CONTRACT_CSPR_DEPOSIT_UREF_ENTRY_POINT_NAME,
        runtime_args! {},
    );

    system::transfer_from_purse_to_purse(
        account::get_main_purse(),
        contract_cspr_deposit_purse,
        amount,
        None,
    )
    .unwrap_or_revert();

    runtime::call_contract(
        payment_processor_contract_hash,
        PROCESS_PAYMENT_ENTRY_POINT_NAME,
        runtime_args! {
            ARG_AMOUNT => amount,
            ARG_RECIPIENT => recipient,
        },
    )
}
