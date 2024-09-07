import { useEffect, useState } from "react";
import { normalize } from "viem/ens";
import recurringTransactionsSmartContract from "../contracts/RecurringTransactions.json";
import {
  useAccount,
  useChainId,
  useEnsResolver,
  useSignMessage,
  useBalance,
  useWriteContract,
} from "wagmi";
import { configureEnv } from "../utils/configureEnv";
import { areAllPropertiesValid, validateEns } from "../utils/ensUtils";
import {
  ZERO_ADDRESS,
  EURE_SMART_CONTRACT_ADDRESS,
  RECURRING_TRANSACTIONS_SMART_CONTRACT_ADDRESS,
} from "../utils/constants";
import { ethers } from "ethers";
import { write } from "fs";
import eureTokenABI from "../contracts/EurE_v1.2.2.json";

export const useConfiguration = () => {
  // ens domain for profile
  const [ensDomain, setEnsDomain] = useState<string>("");

  // create profile
  const [recipient, setRecipient] = useState<string>("");
  const [amount, setAmount] = useState<string>("");
  const [period, setPeriod] = useState<string>("");

  // publish profile
  const [userProfile, setUserProfile] = useState<string>("");
  const [userEns, setUserEns] = useState<string>("");

  const [ensResolverFound, setEnsResolverFound] = useState<boolean>(false);
  const [keyCreationMessage, setKeyCreationMessage] = useState<string>("");
  const [profileAndKeysCreated, setProfileAndKeysCreated] =
    useState<boolean>(false);

  // errors
  const [recipientError, setRecipientError] = useState<string | null>(null);
  const [amountError, setAmountError] = useState<string | null>(null);
  const [periodError, setPeriodError] = useState<string | null>(null);
  const [ensOwnershipError, setEnsOwnershipError] = useState<string | null>(
    null
  );
  const [userProfileError, setUserProfileError] = useState<string | null>(null);
  const [userEnsError, setUserEnsError] = useState<string | null>(null);

  // connected chain
  // const chainId = useChainId();

  // connected account
  const { isConnected, address, connector } = useAccount();

  const {
    data: balanceData,
    isLoading: balanceIsLoading,
    isError: balanceIsError,
  } = useBalance({
    address,
  });

  const {
    data: hash,
    writeContract,
    isPending: writeContractIsPending,
    isError: writeContractIsError,
    error: writeContractError,
    reset, // resets the state of write contract hook
  } = useWriteContract();

  const handleRecipientChange = (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => {
    setRecipient(event.target.value);
    setRecipientError(null);
  };

  const handleValueChange = (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => {
    setAmount(event.target.value);
    setAmountError(null);
  };

  const handlePeriodChange = (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => {
    setPeriod(event.target.value);
    setPeriodError(null);
  };

  const writeRecurringTransaction = async () => {
    const numberOfExecutions = 10; // todo: get from frontend
    const periodBigInt = BigInt(period);
    const amountBigInt = BigInt(amount);

    const recipientAddress = recipient;

    // check if recipient is a valid address
    if (!ethers.isAddress(recipient)) {
      setRecipientError("Invalid recipient address");
      console.log("Invalid recipient address");
      return;
    }

    // Increase approval
    const totalAmount = amountBigInt * BigInt(numberOfExecutions);
    try {
      writeContract({
        address: EURE_SMART_CONTRACT_ADDRESS,
        abi: eureTokenABI,
        functionName: "approve",
        args: [RECURRING_TRANSACTIONS_SMART_CONTRACT_ADDRESS, totalAmount],
      });
    } catch (error) {
      console.error("Error increasing approval:", error);
      // Handle the error appropriately
      return;
    }

    console.log("0");
    console.log(periodBigInt);
    console.log(amountBigInt);
    console.log(recipientAddress);

    writeContract({
      address: RECURRING_TRANSACTIONS_SMART_CONTRACT_ADDRESS,
      abi: recurringTransactionsSmartContract.abi,
      functionName: "createJob",
      args: [
        periodBigInt,
        amountBigInt,
        recipientAddress,
        EURE_SMART_CONTRACT_ADDRESS,
      ],
      value: BigInt(1e16),
    });

    console.log("2");
    console.log(writeContractIsPending);
    console.log(writeContractError);
  };

  const storeEnv = () => {
    const env = configureEnv(amount, address as string);
    const blob = new Blob([env], { type: "text/plain" });
    const buttonUrl = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = buttonUrl;
    link.click();
  };

  const publishProfile = async () => {
    // validate ens name
    // const isEnsValid = validateEns(userEns, setUserEnsError);

    // if (!isEnsValid) {
    //   return;
    // }

    // // validate user profile data
    // const isProfileValid = validateProfile();

    // if (!isProfileValid) {
    //   return;
    // }

    // // validate the ens name access to add text records
    // const isEnsNameOwner = await isAccountOwnerOfEnsName(
    //   userEns,
    //   address as string,
    //   setEnsOwnershipError,
    //   chainId
    // );

    // if (!isEnsNameOwner) {
    //   return;
    // }

    console.log("publishing profile has been deleted");
  };

  // clears all input field & error on change of account
  useEffect(() => {
    console.log("Account changed : ", address);
    setRecipient("");
    setPeriod("");
    setAmount("");
    setUserEns("");
    setUserProfile("");
    setRecipientError(null);
    setPeriodError(null);
    setAmountError(null);
    setUserEnsError(null);
    setUserProfileError(null);
    setEnsOwnershipError(null);

    reset();
  }, [address]);

  // useEffect(() => {
  //   if (isError && !ensResolverIsLoading) {
  //     console.log("error: ", error);
  //     setEnsResolverFound(false);
  //   }
  //   if (!isError && !ensResolverIsLoading && ensResolver !== ZERO_ADDRESS) {
  //     console.log("ens resolver found: ", ensResolver);
  //     setEnsResolverFound(true);
  //   }
  // }, [ensResolver, isError, ensResolverIsLoading, error]);

  // useEffect(() => {
  //   (async () => {
  //     if (variables?.message && signMessageData) {
  //       console.log("4");
  //       setProfileAndKeysCreated(true);
  //       setUserEns(recipient);
  //     }
  //   })();
  // }, [signMessageData, variables?.message, amount]);

  // useEffect(() => {
  //   (async () => {
  //     if (address) {

  return {
    address,
    handleEnsChange: handleRecipientChange,
    handleUrlChange: handleValueChange,
    handleRpcChange: handlePeriodChange,
    isConnected,
    createConfigAndProfile: writeRecurringTransaction,
    profileAndKeysCreated,
    storeEnv,
    writeContractIsPending,
    ensResolverFound,
    publishProfile,
    hash,
    writeContractIsError,
    writeContractError,
    recipientError,
    urlError: amountError,
    rpcError: periodError,
    connector,
    ensInput: recipient,
    url: amount,
    rpc: period,
    ensOwnershipError,
    setEnsOwnershipError,
    userProfile,
    userProfileError,
    userEns,
    userEnsError,
    balanceData,
    balanceIsLoading,
    balanceIsError,
  };
};
