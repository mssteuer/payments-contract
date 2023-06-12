import {
    CsprClickPaymentsClient,
} from "./src/payments";

import {getAccountInfo, getAccountNamedKeyValue, getBinary, getDeploy, parseAccountKeys} from "./src/common";
import {CEP18Client} from "./src/cep18";

const {program} = require('commander');

program
    .option('--wasm [value]', 'path to CEP18 contract wasm file')
    .option('--owner_keys_path [value]', 'path to contract owners keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}')
    .option('--network_name [value]', 'network_name')
    .option('--name [value]', 'CEP18 contract name')
    .option('--symbol [value]', 'CEP18 token symbol')
    .option('--decimals [value]', 'CEP18 contract decimals number')
    .option('--total_supply [value]', 'CEP18 contract total supply')
    .option('--events_mode [value]', 'CEP18 contract events mode (0=NoEvents, 1=CES)', '1')
    .option('--mint_burn [value]', 'CEP18 contract events mode (0=Disabled, 1=Enabled', '0')

program.parse();

const options = program.opts();

const cep18_install = async () => {
    const CONTRACT_OWNER = parseAccountKeys(options.owner_keys_path, options.keys_algo);

    const client = new CEP18Client(options.node_url, options.network_name);

    console.log(`Installing CEP18 contract...`);

    const paymentsContractWasmBytes = getBinary(options.wasm);

    const installDeploy = await client.install(
        {
            name: options.name,
            symbol: options.symbol,
            decimals: options.decimals,
            totalSupply: options.total_supply,
            eventsMode: options.events_mode,
            mintBurn: options.mint_burn,
        },
        "150000000000",
        CONTRACT_OWNER.publicKey,
        [CONTRACT_OWNER],
        paymentsContractWasmBytes
    );

    const installDeployHash = await installDeploy.send(options.node_url);

    console.log(`... CEP18 contract installation deployHash: ${installDeployHash}`);

    await getDeploy(options.node_url, installDeployHash);

    const accountInfos = await getAccountInfo(
        options.node_url,
        CONTRACT_OWNER.publicKey
    );

    console.log(`... Account Info: `);
    console.log(JSON.stringify(accountInfos, null, 2));

    const paymentsContractHash = await getAccountNamedKeyValue(
        accountInfos, `cep18_contract_hash_${options.name}`
    );

    const paymentsContractPackageHash = await getAccountNamedKeyValue(
        accountInfos, `cep18_contract_package_${options.name}`
    );

    console.log(`... CEP18 contract installed successfully.`);
    console.log(`... Contract hash:         ${paymentsContractHash}`);
    console.log(`... Contract package hash: ${paymentsContractPackageHash}`);
    console.log(`----------------------------------------------------------------------------------------------------`);
};

cep18_install();
