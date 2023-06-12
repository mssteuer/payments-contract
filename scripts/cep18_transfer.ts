import {getDeploy, parseAccountKeys} from "./src/common";
import {CEP18Client} from "./src/cep18";

const {program} = require('commander');

program
    .option('--sender_keys_path [value]', 'path to sender keys')
    .option('--keys_algo [value]', 'Crypto algo ed25519 | secp256K1', 'ed25519')
    .option('--node_url [value]', 'node URL in format {http://localhost:11101/rpc}')
    .option('--network_name [value]', 'network_name')
    .option('--cep18_contract_hash [value]', 'hash of payments contract')
    .option('--recipient [value]', 'recipient`s public key')
    .option('--amount [value]', 'transfer amount');

program.parse();
const options = program.opts();

const cep18_transfer = async () => {
    let SENDER = parseAccountKeys(options.sender_keys_path, options.keys_algo);

    const client = new CEP18Client(options.node_url, options.network_name);
    console.log(`Executing transfer...`);

    const paymentAmount = 1500000000;

    const installDeployHash = await client.call_transfer(
        options.cep18_contract_hash,
        {
            recipient: options.recipient,
            amount: options.amount,
        },
        paymentAmount.toString(),
        SENDER.publicKey,
        [SENDER],
    );

    const paymentDeployHash = await installDeployHash.send(options.node_url);

    console.log(`... Transfer deployHash: ${paymentDeployHash}`);

    const deploy = await getDeploy(options.node_url!, paymentDeployHash);

    console.log(`... Transfer successful`);
};

cep18_transfer();
