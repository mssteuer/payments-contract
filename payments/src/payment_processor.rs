use alloc::string::String;
use core::ops::Add;

use casper_contract::contract_api::runtime::get_caller;
use casper_contract::contract_api::{runtime, system, system::get_purse_balance};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::account::AccountHash;
use casper_types::runtime_args;
use casper_types::RuntimeArgs;
use casper_types::{ContractHash, Key, URef, U256, U512};

use contract_utils::{ContractContext, ContractStorage};

use crate::constants::{ARG_ADDRESS, BALANCE_OF_ENTRY_POINT_NAME, CSPR_TOKEN};
use crate::data;
use crate::data::{get_cep18_contract_hash, get_contract_owner, set_contract_cep18_balance};
use crate::errors::Error;
use crate::events::{init_events, Payment};
use casper_event_standard::emit;

pub trait PaymentProcessor<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        contract_name: String,
        cep18_contract_hash: ContractHash,
        cep18_symbol: String,
    ) {
        data::set_cep18_contract_hash(cep18_contract_hash);
        data::set_cep18_symbol(cep18_symbol);
        data::set_contract_cep18_balance(U256::zero());
        data::new_contract_purse();
        data::set_contract_cspr_balance(U512::zero());
        data::set_contract_name(contract_name);
        data::set_contract_owner(get_caller());

        init_events();
    }

    fn process_cspr_payment(&mut self, amount: U512, checkout_id: u64) -> Result<u64, Error> {
        let deposit_purse_balance =
            get_purse_balance(self.contract_cspr_deposit_uref()).unwrap_or_revert();
        let deposit_balance: U512 = deposit_purse_balance as U512;
        let contract_balance = data::get_contract_cspr_balance();

        if deposit_balance != contract_balance.add(amount) {
            runtime::revert(Error::BalanceInsufficient);
        }

        emit(Payment {
            token: String::from(CSPR_TOKEN),
            amount,
            checkout_id,
        });
        Ok(checkout_id)
    }

    fn process_cep18_payment(
        &mut self,
        token: String,
        amount: U256,
        checkout_id: u64,
    ) -> Result<u64, Error> {
        let cep18_symbol = data::get_cep18_symbol();
        if !cep18_symbol.eq(&token) {
            runtime::revert(Error::WrongToken);
        }

        let balance: U256 = runtime::call_contract(
            get_cep18_contract_hash(),
            BALANCE_OF_ENTRY_POINT_NAME,
            runtime_args! {
                ARG_ADDRESS => Key::from(get_contract_owner()),
            },
        );
        let contract_balance = data::get_contract_cep18_balance();

        if balance != contract_balance.add(amount) {
            runtime::revert(Error::BalanceInsufficient);
        }

        emit(Payment {
             token,
             amount: U512::from(amount.as_u128()),
             checkout_id,
        });
        Ok(checkout_id)
    }

    fn transfer_funds_to(&self, target: AccountHash) {
        let contract_cspr_deposit = self.contract_cspr_deposit_uref();
        let deposit_purse_balance =
            get_purse_balance(self.contract_cspr_deposit_uref()).unwrap_or_revert();

        system::transfer_from_purse_to_account(
            contract_cspr_deposit.into_read_write(),
            target,
            deposit_purse_balance,
            None,
        )
        .unwrap_or_revert();

        data::set_contract_cspr_balance(U512::zero());
    }

    fn update_contract_cspr_deposit_balance(&self) {
        let deposit_purse_balance =
            get_purse_balance(self.contract_cspr_deposit_uref()).unwrap_or_revert();
        data::set_contract_cspr_balance(deposit_purse_balance);
    }

    fn contract_cspr_deposit_uref(&self) -> URef {
        let contract_purse = data::get_contract_purse().unwrap_or_revert();
        contract_purse.into_add()
    }

    fn contract_cep18_deposit_address(&self) -> AccountHash {
        get_contract_owner()
    }

    fn cep18_contract_address(&self) -> ContractHash {
        get_cep18_contract_hash()
    }

    fn update_contract_cep18_balance(&self, token: String) {
        let cep18_symbol = data::get_cep18_symbol();
        if !cep18_symbol.eq(&token) {
            runtime::revert(Error::WrongToken);
        }

        let balance: U256 = runtime::call_contract(
            get_cep18_contract_hash(),
            BALANCE_OF_ENTRY_POINT_NAME,
            runtime_args! {
                ARG_ADDRESS => Key::from(get_contract_owner()),
            },
        );

        set_contract_cep18_balance(balance);
    }

    fn assert_caller_is_owner() {
        if get_contract_owner() != get_caller() {
            runtime::revert(Error::PermissionDenied);
        }
    }
}
