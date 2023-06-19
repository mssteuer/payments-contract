#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, URef, U256, U512};
use payments::constants::{
    ARG_AMOUNT, ARG_CHECKOUT_ID, ARG_PAYMENT_PROCESSOR_CONTRACT_HASH, ARG_RECIPIENT, ARG_TOKEN,
    CSPR_TOKEN, GET_CONTRACT_CSPR_DEPOSIT_UREF_ENTRY_POINT_NAME,
    GET_UPDATED_CEP18_DEPOSIT_DATA_ENTRY_POINT_NAME, PROCESS_PAYMENT_ENTRY_POINT_NAME,
    TRANSFER_ENTRY_POINT_NAME,
};

#[no_mangle]
pub extern "C" fn call() {
    let token: String = runtime::get_named_arg(ARG_TOKEN);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let checkout_id: u64 = runtime::get_named_arg(ARG_CHECKOUT_ID);
    let payment_processor_contract_hash: ContractHash =
        runtime::get_named_arg(ARG_PAYMENT_PROCESSOR_CONTRACT_HASH);

    if CSPR_TOKEN.eq(&token) {
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
                ARG_TOKEN => token,
                ARG_AMOUNT => amount,
                ARG_CHECKOUT_ID => checkout_id,
            },
        )
    } else {
        let (cep18_contract_hash, cep18_recipient_address) =
            runtime::call_contract::<(ContractHash, Key)>(
                payment_processor_contract_hash,
                GET_UPDATED_CEP18_DEPOSIT_DATA_ENTRY_POINT_NAME,
                runtime_args! {
                    ARG_TOKEN => token.clone(),
                },
            );

        runtime::call_contract::<()>(
            cep18_contract_hash,
            TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                ARG_RECIPIENT => cep18_recipient_address,
                ARG_AMOUNT => U256::from(amount.as_u128()),
            },
        );

        runtime::call_contract(
            payment_processor_contract_hash,
            PROCESS_PAYMENT_ENTRY_POINT_NAME,
            runtime_args! {
                ARG_TOKEN => token,
                ARG_AMOUNT => amount,
                ARG_CHECKOUT_ID => checkout_id,
            },
        )
    }
}
