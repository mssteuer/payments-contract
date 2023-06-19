use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, runtime::get_call_stack, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::account::AccountHash;
use casper_types::{system::CallStackElement, ContractHash, ContractPackageHash, URef, U256, U512};
use contract_utils::{get_key, set_key};

use crate::constants::{
    CEP18_CONTRACT_HASH, CEP18_SYMBOL, CONTRACT_NAME, CONTRACT_OWNER, CONTRACT_OWNER_CEP18_BALANCE,
    CONTRACT_PURSE, CONTRACT_PURSE_CSPR_BALANCE,
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

pub fn set_cep18_contract_hash(hash: ContractHash) {
    set_key(CEP18_CONTRACT_HASH, hash)
}

pub fn get_cep18_contract_hash() -> ContractHash {
    get_key(CEP18_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_cep18_symbol(symbol: String) {
    set_key(CEP18_SYMBOL, symbol)
}

pub fn get_cep18_symbol() -> String {
    get_key(CEP18_SYMBOL).unwrap_or_revert()
}

pub fn set_contract_cspr_balance(balance: U512) {
    set_key(CONTRACT_PURSE_CSPR_BALANCE, balance)
}

pub fn get_contract_cspr_balance() -> U512 {
    get_key(CONTRACT_PURSE_CSPR_BALANCE).unwrap_or_revert()
}

pub fn set_contract_cep18_balance(balance: U256) {
    set_key(CONTRACT_OWNER_CEP18_BALANCE, balance)
}

pub fn get_contract_cep18_balance() -> U256 {
    get_key(CONTRACT_OWNER_CEP18_BALANCE).unwrap_or_revert()
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
