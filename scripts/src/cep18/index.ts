import {
    CLPublicKey,
    RuntimeArgs,
    CasperClient,
    Contracts,
    Keys,
    CLValueBuilder,
} from "casper-js-sdk";

import {
    CEP18InstallArgs,
    CEP18TransferArgs,
} from "./types";

import {
    getBinary
} from "../common";

const {Contract} = Contracts;

export * from "./types";

export class CEP18Client {
    private casperClient: CasperClient;

    public contractClient: Contracts.Contract;

    constructor(public nodeAddress: string, public networkName: string) {
        this.casperClient = new CasperClient(nodeAddress);
        this.contractClient = new Contract(this.casperClient);
    }

    public install(
        args: CEP18InstallArgs,
        paymentAmount: string,
        deploySender: CLPublicKey,
        keys?: Keys.AsymmetricKey[],
        wasm?: Uint8Array
    ) {

        const wasmToInstall =
            wasm || getBinary(`./cep18.wasm`);

        const runtimeArgs = RuntimeArgs.fromMap({
            name: CLValueBuilder.string(args.name),
            symbol: CLValueBuilder.string(args.symbol),
            decimals: CLValueBuilder.u8(args.decimals),
            total_supply: CLValueBuilder.u256(args.totalSupply),
            events_mode: CLValueBuilder.u8(args.eventsMode),
            enable_mint_burn: CLValueBuilder.u8(args.mintBurn),
        });

        return this.contractClient.install(
            wasmToInstall,
            runtimeArgs,
            paymentAmount,
            deploySender,
            this.networkName,
            keys || []
        );
    }

    public call_transfer(
        contract_hash: string,
        args: CEP18TransferArgs,
        paymentAmount: string,
        deploySender: CLPublicKey,
        keys?: Keys.AsymmetricKey[]
    ) {

        this.contractClient.setContractHash(
            `hash-${contract_hash}`,
        );

        const runtimeArgs = RuntimeArgs.fromMap({
            recipient: CLValueBuilder.key(CLPublicKey.fromHex(args.recipient)),
            amount: CLValueBuilder.u256(args.amount),
        });

        return this.contractClient.callEntrypoint("transfer", runtimeArgs, deploySender, this.networkName, paymentAmount, keys || []);
    }
}
