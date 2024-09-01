import { ConnectButton } from "@rainbow-me/rainbowkit";
import { useConfiguration } from "../hooks/useConfiguration";
import { CreateRecurringTransaction } from "./CreateRecurringTransaction";
import { Info } from "./Info";
import ShowRecurringTransactions from "./ShowRecurringTransactions";
import { StopRecurringTransaction } from "./StopRecurringTransaction";
import { Welcome } from "./Welcome";

const App = () => {
  const {
    address,
    isConnected,
    handleEnsChange,
    handleRpcChange,
    handleUrlChange,
    createConfigAndProfile,
    profileAndKeysCreated,
    storeEnv,
    writeContractIsPending,
    publishProfile,
    ensResolverFound,
    hash,
    writeContractError,
    recipientError,
    rpcError,
    urlError,
    ensInput,
    rpc,
    url,
    userProfile,
    userProfileError,
    ensOwnershipError,
    userEns,
    userEnsError,
  } = useConfiguration();

  return (
    <div className="ds-container">
      <div className="main-container">
        <h1 className="ds-title">ReTransICP</h1>

        {/* Rainbowkit connect button */}
        <div className="connect-btn">
          <ConnectButton />
        </div>
      </div>

      <div className="steps-container">
        <Welcome address={address} />

        <CreateRecurringTransaction
          handleRecipientChange={handleEnsChange}
          handleAmountChange={handleUrlChange}
          handlePeriodChange={handleRpcChange}
          recipient={ensInput}
          url={url}
          rpc={rpc}
          recipientError={recipientError}
          amountError={rpcError}
          urlError={urlError}
          createRecurringTransaction={createConfigAndProfile}
          isConnected={isConnected}
        />

        <ShowRecurringTransactions
          ensResolverFound={ensResolverFound}
          hash={hash}
          userProfile={userProfile}
          profileAndKeysCreated={profileAndKeysCreated}
          publishProfile={publishProfile}
          writeContractError={writeContractError}
          writeContractIsPending={writeContractIsPending}
          userProfileError={userProfileError}
          ensOwnershipError={ensOwnershipError}
          userEns={userEns}
          userEnsError={userEnsError}
        />

        <StopRecurringTransaction
          profileAndKeysCreated={profileAndKeysCreated}
          storeEnv={storeEnv}
        />
      </div>

      {/* <Docker /> */}

      <Info />
    </div>
  );
};

export default App;
