export type PaymentContractInstallArgs = {
  contractName: string;
  cep18ContractHash: string;
  cep18Symbol: string;
};

export type ExecutePaymentArgs = {
  paymentsContractHash: string;
  token: string;
  amount: string;
  checkoutId: string;
};

export type TransferToArgs = {
  target: string;
};