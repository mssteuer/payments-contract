export type PaymentContractInstallArgs = {
  contractName: string;
};

export type ExecutePaymentArgs = {
  paymentsContractHash: string;
  amount: string;
  recipient: string;
};

export type TransferToArgs = {
  target: string;
};