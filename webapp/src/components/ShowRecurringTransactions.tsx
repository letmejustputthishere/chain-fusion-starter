interface PublishProfileProps {
  profileAndKeysCreated: boolean;
  userProfile: string;
  writeContractIsPending: boolean;
  ensResolverFound: boolean;
  publishProfile: () => void;
  hash: `0x${string}` | undefined;
  writeContractError: any;
  userProfileError: string | null;
  ensOwnershipError: string | null;
  userEns: string;
  userEnsError: string | null;
}

export default function ShowRecurringTransactions(props: PublishProfileProps) {
  return (
    <div className="steps-container description-text step">
      <h2 className="heading-text">Show Recurring Transactions</h2>
      Todo: show list of recurring transactions for this address
      {/* <div className="base-input-container">
        <div className="input-description profile-msg-container">
          <span className="input-heading-hidden">ENS:</span>

          {props.userEnsError && (
            <span className="error">{props.userEnsError}</span>
          )}

          {props.ensOwnershipError && (
            <span className="error">{props.ensOwnershipError}</span>
          )}
        </div>

        <div className="input-container">
          <span className="input-heading">ENS:</span>
          <input
            className="input-field"
            value={props.userEns}
            onChange={(event) => props.handleUserEnsChange(event)}
          />
        </div>

        <div className="input-description profile-msg-container">
          <span className="input-heading-hidden">Profile:</span>

          {props.userProfileError && (
            <span className="error">{props.userProfileError}</span>
          )}

          {props.writeContractError && (
            <span className="error">
              {props.writeContractError.details
                ? props.writeContractError.details
                : JSON.stringify(props.writeContractError)}
            </span>
          )}
        </div>

        <div className="input-container">
          <span className="input-heading">Profile:</span>
          <input
            className="input-field"
            value={props.userProfile}
            onChange={(event) => props.handleUserProfileChange(event)}
          />
        </div>
      </div>
      <div className="input-description">
        <span className="input-heading-hidden">Profile:</span>

        <span>
          Write contract state:{" "}
          {props.writeContractIsPending ? "pending" : "done"}
        </span>
      </div>
      <div className="input-description">
        <span className="input-heading-hidden">Profile:</span>

        {props.hash && (
          <span className="success">Profile published successfully!</span>
        )}
      </div>
      <button
        className={"btn env-btn active-btn ".concat(
          !props.userProfile || !props.userEns ? "disabled-btn" : ""
        )}
        disabled={!props.userProfile || !props.userEns}
        onClick={props.publishProfile}
      >
        Publish profile
      </button> */}
    </div>
  );
}
