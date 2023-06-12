export enum EventsMode {
  NoEvents = 0,
  CES = 1,
};

export enum MintBurn {
  Disabled = 0,
  MintAndBurn = 1,
};

export type CEP18InstallArgs = {
  name: string;
  symbol: string;
  decimals: string;
  totalSupply: string;
  eventsMode: EventsMode;
  mintBurn: MintBurn;
};

export type CEP18TransferArgs = {
  recipient: string;
  amount: string;
}