import { useEffect, useState } from "react";
import { normalize } from "viem/ens";
import recurringTransactionsSmartContract from "../contracts/RecurringTransactions.json";
import {
  useAccount,
  useChainId,
  useEnsResolver,
  useSignMessage,
  useWriteContract,
} from "wagmi";
import { configureEnv } from "../utils/configureEnv";
import {
  areAllPropertiesValid,
  isAccountOwnerOfEnsName,
  validateEns,
} from "../utils/ensUtils";
import {
  ZERO_ADDRESS,
  EURE_SMART_CONTRACT_ADDRESS,
  RECURRING_TRANSACTIONS_SMART_CONTRACT_ADDRESS,
} from "../utils/constants";
import { ethers } from "ethers";

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
  const chainId = useChainId();

  // connected account
  const { isConnected, address, connector } = useAccount();

  const {
    data: signMessageData,
    error,
    signMessage,
    variables,
  } = useSignMessage();

  const {
    data: ensResolver,
    isError,
    isLoading: ensResolverIsLoading,
  } = useEnsResolver({ name: normalize(ensDomain) });

  const {
    data: hash,
    writeContract,
    isPending: writeContractIsPending,
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

  // const handleUserProfileChange = (
  //   event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  // ) => {
  //   setUserProfile(event.target.value);
  //   setUserProfileError(null);
  //   reset();
  // };

  // const handleUserEnsChange = (
  //   event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  // ) => {
  //   const ensValue = event.target.value;
  //   setUserEns(ensValue);
  //   setUserEnsError(null);
  //   setEnsOwnershipError(null);
  //   reset();
  //   // sets ENS domain to get the resolver if ens name is valid
  //   if (validateEns(ensValue)) {
  //     setEnsDomain(ensValue);
  //   }
  // };

  const writeRecurringTransaction = async () => {
    // const isValid = await areAllPropertiesValid(
    //   ensInput,
    //   setEnsError,
    //   rpc,
    //   setRpcError,
    //   url,
    //   setUrlError
    // );
    // if (!isValid) {
    //   return;
    // }

    // setEnsDomain(recipient);
    // const dsEnsAndUrl = JSON.stringify({
    //   ens: ensDomain,
    //   url: amount,
    // });

    //reset();

    const periodBigInt = BigInt(period);
    const amountBigInt = BigInt(amount);

    const recipientAddress = recipient;

    // check if recipient is a valid address
    if (!ethers.isAddress(recipient)) {
      setRecipientError("Invalid recipient address");
      console.log("Invalid recipient address");
      return;
    }

    console.log("0");
    console.log(periodBigInt);
    console.log(amountBigInt);
    console.log(recipientAddress);

    console.log("1");
    console.log(writeContractIsPending);
    console.log(writeContractError);

    console.log("1.1");

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
      //value: BigInt(0),
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

  const validateProfile = (): boolean => {
    try {
      if (!userProfile.length) {
        setUserProfileError("Invalid profile data");
        return false;
      }

      const jsonProfile = JSON.parse(userProfile);

      if (
        !jsonProfile.publicEncryptionKey ||
        !jsonProfile.publicSigningKey ||
        !jsonProfile.url
      ) {
        setUserProfileError("Invalid profile data");
        return false;
      }

      return true;
    } catch (error) {
      console.log("Invalid profile data : ", error);
      setUserProfileError("Invalid profile data");
      return false;
    }
  };

  const publishProfile = async () => {
    // validate ens name
    const isEnsValid = validateEns(userEns, setUserEnsError);

    if (!isEnsValid) {
      return;
    }

    // validate user profile data
    const isProfileValid = validateProfile();

    if (!isProfileValid) {
      return;
    }

    // validate the ens name access to add text records
    const isEnsNameOwner = await isAccountOwnerOfEnsName(
      userEns,
      address as string,
      setEnsOwnershipError,
      chainId
    );

    if (!isEnsNameOwner) {
      return;
    }

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

  useEffect(() => {
    if (isError && !ensResolverIsLoading) {
      console.log("error: ", error);
      setEnsResolverFound(false);
    }
    if (!isError && !ensResolverIsLoading && ensResolver !== ZERO_ADDRESS) {
      console.log("ens resolver found: ", ensResolver);
      setEnsResolverFound(true);
    }
  }, [ensResolver, isError, ensResolverIsLoading, error]);

  useEffect(() => {
    (async () => {
      if (variables?.message && signMessageData) {
        console.log("4");
        setProfileAndKeysCreated(true);
        setUserEns(recipient);
      }
    })();
  }, [signMessageData, variables?.message, amount]);

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
  };
};
