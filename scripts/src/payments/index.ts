import {
    CLPublicKey,
    RuntimeArgs,
    CasperClient,
    Contracts,
    Keys,
    CLValueBuilder,
    CLByteArray,
} from "casper-js-sdk";

import {
    ExecutePaymentArgs,
    PaymentContractInstallArgs, TransferToArgs,
} from "./types";

import {
    getBinary
} from "../common";

const {Contract} = Contracts;

export * from "./types";

export class CsprClickPaymentsClient {
    private casperClient: CasperClient;

    public contractClient: Contracts.Contract;

    constructor(public nodeAddress: string, public networkName: string) {
        this.casperClient = new CasperClient(nodeAddress);
        this.contractClient = new Contract(this.casperClient);
    }

    public install(
        args: PaymentContractInstallArgs,
        paymentAmount: string,
        deploySender: CLPublicKey,
        keys?: Keys.AsymmetricKey[],
        wasm?: Uint8Array
    ) {

        const wasmToInstall =
            wasm || getBinary(`./payment_processor_contract.wasm`);

        const runtimeArgs = RuntimeArgs.fromMap({
            contract_name: CLValueBuilder.string(args.contractName),
            cep18_contract_hash: CLValueBuilder.byteArray(Uint8Array.from(Buffer.from(args.cep18ContractHash, "hex"))),
            cep18_symbol: CLValueBuilder.string(args.cep18Symbol),
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

    public execute_payment(
        contract_hash: string,
        args: ExecutePaymentArgs,
        paymentAmount: string,
        deploySender: CLPublicKey,
        keys?: Keys.AsymmetricKey[],
        wasm?: Uint8Array

    ) {

        this.contractClient.setContractHash(
            `hash-${contract_hash}`,
        );

        const runtimeArgs = RuntimeArgs.fromMap({
            payment_processor_contract_hash: CLValueBuilder.byteArray(Uint8Array.from(Buffer.from(args.paymentsContractHash, "hex"))),
            token: CLValueBuilder.string(args.token),
            amount: CLValueBuilder.u512(args.amount),
            checkout_id: CLValueBuilder.u64(args.checkoutId),
        });

        return this.contractClient.install(
            wasm,
            runtimeArgs,
            paymentAmount,
            deploySender,
            this.networkName,
            keys || []
        );
    }

    public call_transfer_to(
        contract_hash: string,
        args: TransferToArgs,
        paymentAmount: string,
        deploySender: CLPublicKey,
        keys?: Keys.AsymmetricKey[]
    ) {

        this.contractClient.setContractHash(
            `hash-${contract_hash}`,
        );

        const runtimeArgs = RuntimeArgs.fromMap({
            target: new CLByteArray(Uint8Array.from(Buffer.from(args.target, "hex"))),
        });

        return this.contractClient.callEntrypoint("transfer_funds_to", runtimeArgs, deploySender, this.networkName, paymentAmount, keys || []);
    }
}
