export const resolverAbi = [
  {
    name: "setText",
    type: "function",
    stateMutability: "nonpayable",
    inputs: [
      { internalType: "bytes32", name: "node", type: "bytes32" },
      { internalType: "string", name: "key", type: "string" },
      {
        internalType: "string",
        name: "value",
        type: "string",
      },
    ],
    outputs: [],
  },
] as const;
