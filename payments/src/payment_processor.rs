use alloc::string::String;
use core::ops::Add;

use casper_contract::contract_api::runtime::get_caller;
use casper_contract::contract_api::{runtime, system, system::get_purse_balance};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::account::AccountHash;
use casper_types::{Key, URef, U512, PublicKey};

use contract_utils::{ContractContext, ContractStorage};

use crate::data;
use crate::data::{get_contract_owner};
use crate::errors::Error;
use crate::events::{init_events, Payment};
use casper_event_standard::emit;

pub trait PaymentProcessor<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        contract_name: String,
    ) {
        data::new_contract_purse();
        data::set_contract_cspr_balance(U512::zero());
        data::set_contract_name(contract_name);
        data::set_contract_owner(get_caller());

        init_events();
    }

    fn process_cspr_payment(&mut self, amount: U512, recipient: PublicKey) -> Result<u64, Error> {
        let deposit_purse_balance =
            get_purse_balance(self.contract_cspr_deposit_uref()).unwrap_or_revert();
        let deposit_balance: U512 = deposit_purse_balance as U512;
        let contract_balance = data::get_contract_cspr_balance();

        if deposit_balance != contract_balance.add(amount) {
            runtime::revert(Error::BalanceInsufficient);
        }

        emit(Payment {
            account: Key::from(get_caller()),
            amount,
            recipient
        });

        Ok(1)
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

    fn assert_caller_is_owner() {
        if get_contract_owner() != get_caller() {
            runtime::revert(Error::PermissionDenied);
        }
    }
}
