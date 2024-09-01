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
    handleUserProfileChange,
    ensOwnershipError,
    userEns,
    userEnsError,
    handleUserEnsChange,
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
          handleEnsChange={handleEnsChange}
          handleUrlChange={handleUrlChange}
          handleRpcChange={handleRpcChange}
          ens={ensInput}
          url={url}
          rpc={rpc}
          recipientError={recipientError}
          rpcError={rpcError}
          urlError={urlError}
          createConfigAndProfile={createConfigAndProfile}
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
          handleUserProfileChange={handleUserProfileChange}
          userProfileError={userProfileError}
          ensOwnershipError={ensOwnershipError}
          userEns={userEns}
          userEnsError={userEnsError}
          handleUserEnsChange={handleUserEnsChange}
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
