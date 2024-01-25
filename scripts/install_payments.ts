import {
    CsprClickPaymentsClient,
} from "./src/payments";

import {getAccountInfo, getAccountNamedKeyValue, getBinary, getDeploy, parseAccountKeys} from "./src/common";

const {program} = require('commander');

program
    .option('--owner_keys_path [value]', 'path to contract owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}')
    .option('--network_name [value]', 'network_name')
    .option('--contract_name [value]', 'New contract name', "CSPR.click Payments");

program.parse();

const options = program.opts();

const install_payments = async () => {
    const CONTRACT_OWNER = parseAccountKeys(options.owner_keys_path, options.keys_algo);

    const client = new CsprClickPaymentsClient(options.node_url, options.network_name);

    console.log(`Installing CSPR.click Payments contract...`);

    const paymentsContractWasmBytes = getBinary("../target/wasm32-unknown-unknown/release/payment_processor_contract.wasm");

    const installDeploy = client.install(
        {
            contractName: options.contract_name,
        },
        "110000000000",
        CONTRACT_OWNER.publicKey,
        [CONTRACT_OWNER],
        paymentsContractWasmBytes
    );

    const installDeployHash = await installDeploy.send(options.node_url);

    console.log(`... Payments contract installation deployHash: ${installDeployHash}`);

    await getDeploy(options.node_url, installDeployHash);

    const accountInfos = await getAccountInfo(
        options.node_url,
        CONTRACT_OWNER.publicKey
    );

    console.log(`... Account Info: `);
    console.log(JSON.stringify(accountInfos, null, 2));

    const paymentsContractHash = await getAccountNamedKeyValue(
        accountInfos,
        `${options.contract_name}_contract_hash`
    );

    const paymentsContractPackageHash = await getAccountNamedKeyValue(
        accountInfos,
        `${options.contract_name}_contract_package_hash`
    );

    console.log(`... Payments contract installed successfully. ${paymentsContractHash}`);
    console.log(`... Contract hash:         ${paymentsContractHash}`);
    console.log(`... Contract package hash: ${paymentsContractPackageHash}`);
    console.log(`----------------------------------------------------------------------------------------------------`);
};

install_payments();
