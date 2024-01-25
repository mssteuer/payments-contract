# CSPR.click Payments Contract

[payments-contract](https://github.com/mssteuer/payments-contract) represents a special contract used to execute payments for purchasing digital assets on the Casper blockchain

The contract is able to handle payments in CSPR token.

[cspr-click-payments-contract](payments-contract](https://github.com/mssteuer/payments-contract) consists of two smart contracts:
 - [payment_processor_contract](./payments/bin/payment_processor_contract.rs). Provides the following entry points:
   - *get_contract_cspr_deposit_uref* - Returns the ADD access rights purse Uref for payment.
   - *process_payment* - Verifies that the payment has been made. In such case emits the `Payment` event.
   - *transfer_funds_to* - Transfers the specified amount of `CSPR` from the contract purse to a recipient account. Emits the `TransferFundsTo` event.

- [execute_payment](./payments/bin/execute_payment.rs) a wasm contract that transfers the specified amount of CSPR tokens from the caller to a recipient. 

  * For `CSPR` tokens, the recipient is a purse owned by the contract. The contract owner can later or transfer the funds to another account using the entry point `trasnfer_funds_to`.


### Installation

First, install the scripts dependencies:

```shell
cd ./scripts/ && npm install
```

To deploy the *payment processor contract*, run the follwing casper-client command::

```shell
 casper-client put-deploy \
--node-address http://localhost:11101/rpc \
--chain-name casper-test \
--secret-key {some/path-to/owner/keys} \
--payment-amount 150000000000 \
--session-path ./target/wasm32-unknown-unknown/release/payment_processor_contract.wasm \
--session-arg "contract_name:string='name-of-your-payment-contract'"
```

### Usage 

After contracts installation, you can perform an *execute payment* operation with CSPR:

```shell
npm run scripts:pay -- --node_url http://localhost:11101/rpc \
--network_name casper-net-1 \
--payments_contract_hash 0a0b0c..0900 \
--token CSPR \
--amount 70000000000 \
--buyer_keys_path {some/path-to/owner/keys}
```

To send contract CSPR funds to another account:

```shell
npm run scripts:transfer_to -- --node_url  http://localhost:11101/rpc \
 --network_name casper-test \
 --payments_contract_hash 0a0b0c..0900 \
 --target 9b83adbf6a999ead4f3790f8380e95fd76c95cca8bf41152957b5fe78b9cf7c7 \
 --owner_keys_path {some/path-to/owner/keys}
```
