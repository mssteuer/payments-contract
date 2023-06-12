#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, ContractHash, Key, U256};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Parameter, RuntimeArgs, URef, U512,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use payments::PaymentProcessor;

use payments::constants::{
    ARG_AMOUNT, ARG_CEP18_CONTRACT_HASH, ARG_CHECKOUT_ID, ARG_CONTRACT_NAME, ARG_TARGET, ARG_TOKEN,
    CEP18_TOKEN, CSPR_TOKEN, GET_CONTRACT_CSPR_DEPOSIT_UREF_ENTRY_POINT_NAME,
    GET_UPDATED_CEP18_DEPOSIT_DATA_ENTRY_POINT_NAME, PROCESS_PAYMENT_ENTRY_POINT_NAME,
    TRANSFER_FUNDS_TO_ENTRY_POINT_NAME,
};

#[derive(Default)]
struct PaymentProcessorContract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for PaymentProcessorContract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl PaymentProcessor<OnChainContractStorage> for PaymentProcessorContract {}

impl PaymentProcessorContract {
    fn constructor(&mut self, contract_name: String, cep18_contract_hash: ContractHash) {
        PaymentProcessor::init(self, contract_name, cep18_contract_hash);
    }
}

#[no_mangle]
fn constructor() {
    let contract_name: String = runtime::get_named_arg(ARG_CONTRACT_NAME);
    let cep18_contract_hash: ContractHash = runtime::get_named_arg(ARG_CEP18_CONTRACT_HASH);
    PaymentProcessorContract::default().constructor(contract_name, cep18_contract_hash);
}

#[no_mangle]
fn process_payment() {
    let token: String = runtime::get_named_arg(ARG_TOKEN);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let checkout_id: u64 = runtime::get_named_arg(ARG_CHECKOUT_ID);

    if CSPR_TOKEN.eq(&token) {
        PaymentProcessorContract::default()
            .process_cspr_payment(amount, checkout_id)
            .unwrap_or_revert();
    } else if CEP18_TOKEN.eq(&token) {
        PaymentProcessorContract::default()
            .process_cep18_payment(U256::from(amount.as_u128()), checkout_id)
            .unwrap_or_revert();
    }
}

#[no_mangle]
fn get_contract_cspr_deposit_uref() {
    let ret = PaymentProcessorContract::default().contract_cspr_deposit_uref();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_updated_cep18_deposit_data() {
    PaymentProcessorContract::default().update_contract_cep18_balance();

    let cep18_contract_address = PaymentProcessorContract::default().cep18_contract_address();
    let cep18_deposit_address = PaymentProcessorContract::default().contract_cep18_deposit_address();
    runtime::ret(
        CLValue::from_t((cep18_contract_address, Key::from(cep18_deposit_address)))
            .unwrap_or_revert(),
    );
}

#[no_mangle]
fn transfer_funds_to() {
    PaymentProcessorContract::assert_caller_is_owner();
    let target = runtime::get_named_arg::<AccountHash>(ARG_TARGET);
    PaymentProcessorContract::default().transfer_funds_to(target)
}

#[no_mangle]
fn call() {
    // Read arguments for the constructor call.
    let contract_name: String = runtime::get_named_arg(ARG_CONTRACT_NAME);
    let cep18_contract_hash: ContractHash = runtime::get_named_arg(ARG_CEP18_CONTRACT_HASH);

    // Prepare constructor args
    let constructor_args = runtime_args! {
        ARG_CONTRACT_NAME => contract_name.clone(),
        ARG_CEP18_CONTRACT_HASH => cep18_contract_hash,
    };

    let contract_package_hash_name: String = format!("{}_contract_package_hash", contract_name);
    let contract_access_uref: String = format!("{}_access_uref", contract_name);

    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        None,
        Some(contract_package_hash_name.clone()),
        Some(contract_access_uref),
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(&contract_package_hash_name)
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        PROCESS_PAYMENT_ENTRY_POINT_NAME,
        vec![
            Parameter::new(ARG_TOKEN, String::cl_type()),
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
            Parameter::new(ARG_CHECKOUT_ID, Vec::<u64>::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        GET_CONTRACT_CSPR_DEPOSIT_UREF_ENTRY_POINT_NAME,
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        GET_UPDATED_CEP18_DEPOSIT_DATA_ENTRY_POINT_NAME,
        vec![],
        CLType::Tuple2([Box::new(CLType::ByteArray(32)), Box::new(CLType::Key)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        TRANSFER_FUNDS_TO_ENTRY_POINT_NAME,
        vec![Parameter::new(ARG_TARGET, AccountHash::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
