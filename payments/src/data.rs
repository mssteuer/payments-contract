use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, runtime::get_call_stack, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::account::AccountHash;
use casper_types::{system::CallStackElement, ContractPackageHash, URef, U512};
use contract_utils::{get_key, set_key};

use crate::constants::{
    CONTRACT_NAME, CONTRACT_OWNER, CONTRACT_PURSE, CONTRACT_PURSE_CSPR_BALANCE,
};

pub fn get_contract_purse() -> Option<URef> {
    runtime::get_key(CONTRACT_PURSE)
        .unwrap_or_revert()
        .into_uref()
}

pub fn new_contract_purse() {
    let purse_uref_add: URef = system::create_purse();
    runtime::put_key(CONTRACT_PURSE, purse_uref_add.into())
}

pub fn set_contract_cspr_balance(balance: U512) {
    set_key(CONTRACT_PURSE_CSPR_BALANCE, balance)
}

pub fn get_contract_cspr_balance() -> U512 {
    get_key(CONTRACT_PURSE_CSPR_BALANCE).unwrap_or_revert()
}

pub fn set_contract_owner(owner: AccountHash) {
    set_key(CONTRACT_OWNER, owner)
}

pub fn set_contract_name(contract_name: String) {
    set_key(CONTRACT_NAME, contract_name)
}

pub fn get_contract_owner() -> AccountHash {
    get_key(CONTRACT_OWNER).unwrap_or_revert()
}

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}
