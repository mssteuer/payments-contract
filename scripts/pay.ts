import {
    CsprClickPaymentsClient,
} from "./src/payments";

import {getBinary, getDeploy, parseAccountKeys} from "./src/common";

const {program} = require('commander');

program
    .option('--buyer_keys_path [value]', 'path to buyer keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}')
    .option('--network_name [value]', 'network_name')
    .option('--payments_contract_hash [value]', 'hash of payments contract')
    .option('--token [value]', 'token symbol')
    .option('--amount [value]', 'buy payment amount')
    .option('--recipient [value]', 'where to deliver the purchase to');

program.parse();
const options = program.opts();

const pay = async () => {
    let BUYER = parseAccountKeys(options.buyer_keys_path, options.keys_algo);

    const client = new CsprClickPaymentsClient(options.node_url, options.network_name);
    console.log(`Running payment contract...`);

    const paymentContractWasmBytes = getBinary("../target/wasm32-unknown-unknown/release/execute_payment.wasm");

    const executionPaymentAmount = 2000000000;

    const installDeployHash = client.execute_payment(
        options.payments_contract_hash,
        {
            paymentsContractHash: options.payments_contract_hash,
            amount: options.amount,
            recipient: options.recipient,
        },
        executionPaymentAmount.toString(),
        BUYER.publicKey,
        [BUYER],
        paymentContractWasmBytes
    );

    const paymentDeployHash = await installDeployHash.send(options.node_url);

    console.log(`... Payment contract installation deployHash: ${paymentDeployHash}`);

    await getDeploy(options.node_url!, paymentDeployHash);

    console.log(`... Payment successful`);
};

pay();