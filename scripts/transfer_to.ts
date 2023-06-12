import {
    CsprClickPaymentsClient,
} from "./src/payments";

import {
    getDeploy,
    parseAccountKeys
} from "./src/common";

const {program} = require('commander');

program
    .option('--owner_keys_path [value]', 'path to owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}')
    .option('--network_name [value]', 'network_name')
    .option('--payments_contract_hash [value]', 'hash of payments contract')
    .option('--target [value]', 'account_hash: who wants to receive from contract');

program.parse();
const options = program.opts();

const transfer_to = async () => {
    const CONTRACT_OWNER = parseAccountKeys(options.owner_keys_path, options.keys_algo);;

    const client = new CsprClickPaymentsClient(options.node_url, options.network_name);
    console.log(`Running transfer_funds_to endpoint...`, options);

    const transferToDeploy = await client.call_transfer_to(
        options.payments_contract_hash,
        {
            target: options.target
        },
        "4000000000",
        CONTRACT_OWNER.publicKey,
        [CONTRACT_OWNER],
    );

    const transferToDeployHash = await transferToDeploy.send(options.node_url);

    console.log(`... transfer_funds_to endpoint installation deployHash: ${transferToDeployHash}`);

    const deploy = await getDeploy(options.node_url!, transferToDeployHash);

    console.log(`... transfer_funds_to successful`);
};

transfer_to();
