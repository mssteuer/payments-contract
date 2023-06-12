# CSPR.click Payments Contract

[cspr-click-payments-contract](https://github.com/make-software/) represents a special contract used to execute payments for purchasing digital assets on CSPR.click web wallet.

The contract is able to handle payments in CSPR token as well as with a [CEP18](https://github.com/casper-ecosystem/cep18)-based token.

[cspr-click-payments-contract](https://github.com/make-software/) consists of two smart contracts:
 - [payment_processor_contract](./payments/bin/payment_processor_contract.rs). Provides the following entry points:
   - *get_contract_cspr_deposit_uref* - Returns the ADD access rights purse Uref for payment.
   - *get_updated_cep18_deposit_data* - Returns the CEP18 contract hash and the target account for payment.
   - *process_payment* - Verifies that the payment has been made. In such case emits the `Payment` event.
   - *transfer_funds_to* - Transfers the specified amount of `CSPR` from the contract purse to a recipient account. Emits the `TransferFundsTo` event.

- [execute_payment](./payments/bin/execute_payment.rs) a wasm contract that transfers the specified amount and type of tokens from the caller to a recipient. The recipient differs depending on the type of token:

  * For `CSPR` tokens, the recipient is a purse owned by the contract. The contract owner can later or transfer the funds to another account using the entry point `trasnfer_funds_to`.
  * For `CEP-18` tokens, the recipient is the contract owner account. The contract owner can later on transfer these tokens to another account using standard `CEP18` methods.


### Installation

First, install the scripts dependencies:

```shell
cd ./scripts/ && npm install
```

To deploy the *CEP18* contract refer to the official instructions [here](https://github.com/casper-ecosystem/cep18/). 
Or use the installation script running the command below adjusting the parameter values to your needs:

```shell
npm run scripts:cep18_install -- --node_url http://localhost:11101/rpc \
--network_name casper-net-1 \
--wasm cep18.wasm \
--name CLICK.test \
--symbol CLICKT \
--decimals 3 \
--total_supply 1000000000000 \
--owner_keys_path {some/path-to/owner/keys}
```

To deploy the *payment processor contract*, run the install script as follows:

```shell
npm run scripts:install_payments -- --node_url http://localhost:11101/rpc \
--network_name casper-net-1 \
--contract_name csprclick-digital-payments \
--cep18_contract_hash  010203..0f00 
--cep18_symbol CLICKT \
--owner_keys_path {some/path-to/owner/keys}
```

### Usage 

After contracts installation, you can perform an *execute payment* operation with CSPR:

```shell
npm run scripts:pay -- --node_url http://localhost:11101/rpc \
--network_name casper-net-1 \
--payments_contract_hash 0a0b0c..0900 \
--token CSPR \
--amount 70000000000 \
--checkout_id 123456 \
--buyer_keys_path {some/path-to/owner/keys}
```
Or with the CEP18 token:

```shell
npm run scripts:pay -- --node_url http://localhost:11101/rpc \
--network_name casper-net-1 \
--payments_contract_hash 0a0b0c..0900  \
--token CLICKT \
--amount 5000 \
--checkout_id 123456 \
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
