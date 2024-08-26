interface EnvProps {
  profileAndKeysCreated: boolean;
  storeEnv: () => void;
}

export function StopRecurringTransaction(props: EnvProps) {
  return (
    <div className="step description-text">
      <h2 className="heading-text">Stop a recurring transaction</h2>
      <p>todo: load list of transactions and allow user to stop them</p>
      {/* <button
        className={"btn env-btn active-btn ".concat(
          !props.profileAndKeysCreated ? "disabled-btn" : ""
        )}
        disabled={!props.profileAndKeysCreated}
        onClick={props.storeEnv}
      >
        Store .env
      </button>{" "} */}
    </div>
  );
}
