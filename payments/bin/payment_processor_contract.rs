#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{collections::BTreeSet, format, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, PublicKey};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Parameter, RuntimeArgs, URef, U512,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use payments::{PaymentProcessor};

use payments::constants::{ARG_AMOUNT, ARG_CONTRACT_NAME, ARG_RECIPIENT, ARG_TARGET, GET_CONTRACT_CSPR_DEPOSIT_UREF_ENTRY_POINT_NAME, PROCESS_PAYMENT_ENTRY_POINT_NAME, TRANSFER_FUNDS_TO_ENTRY_POINT_NAME};

#[derive(Default)]
struct PaymentProcessorContract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for PaymentProcessorContract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl PaymentProcessor<OnChainContractStorage> for PaymentProcessorContract {}

impl PaymentProcessorContract {
    fn constructor(
        &mut self,
        contract_name: String,
    ) {
        PaymentProcessor::init(self, contract_name);
    }
}

#[no_mangle]
fn constructor() {
    let contract_name: String = runtime::get_named_arg(ARG_CONTRACT_NAME);
    PaymentProcessorContract::default().constructor(
        contract_name,
    );
}

#[no_mangle]
fn process_payment() {
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let recipient: PublicKey = runtime::get_named_arg(ARG_RECIPIENT);

    PaymentProcessorContract::default()
        .process_cspr_payment(amount, recipient)
        .unwrap_or_revert();

}

#[no_mangle]
fn get_contract_cspr_deposit_uref() {
    PaymentProcessorContract::default().update_contract_cspr_deposit_balance();
    let ret = PaymentProcessorContract::default().contract_cspr_deposit_uref();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
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

    // Prepare constructor args
    let constructor_args = runtime_args! {
        ARG_CONTRACT_NAME => contract_name.clone(),
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
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
            Parameter::new(ARG_RECIPIENT, String::cl_type())
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
        TRANSFER_FUNDS_TO_ENTRY_POINT_NAME,
        vec![Parameter::new(ARG_TARGET, AccountHash::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
