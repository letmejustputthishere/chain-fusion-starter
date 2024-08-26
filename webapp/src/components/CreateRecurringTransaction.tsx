import { useEffect, useState } from "react";

interface ConfigureProfileProps {
  ens: string;
  rpc: string;
  url: string;
  handleEnsChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  handleUrlChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  handleRpcChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  createConfigAndProfile: () => void;
  ensError: string | null;
  rpcError: string | null;
  urlError: string | null;
  isConnected: boolean;
}

export function CreateRecurringTransaction(props: ConfigureProfileProps) {
  const [showSuccessMsg, setShowSuccessMsg] = useState<boolean>(false);

  const isDisabled =
    !props.isConnected || !props.ens.length || !props.url.length ? true : false;

  // show success message of profile creation for 3 seconds & then clears the msg from UI
  //   useEffect(() => {
  //     if (props.profile) {
  //       setShowSuccessMsg(true);
  //       setTimeout(() => {
  //         setShowSuccessMsg(false);
  //       }, 3000);
  //     }
  //   }, [props.profile]);

  return (
    <div className="description-text step">
      <h2 className="heading-text">Create a recurring transaction</h2>
      To create the profile and config file, please connect the account the
      delivery service will use. Also, we need this information:
      {/* <div className="base-input-container">
        <div className="input-description">
          <span className="input-heading-hidden">ENS:</span>
          {props.ensError && <span className="error">{props.ensError}</span>}
        </div>
        <div className="input-container">
          <span className="input-heading">ENS:</span>
          <input
            className="input-field"
            value={props.ens}
            onChange={(event) => props.handleEnsChange(event)}
          />
        </div>
        <div className="input-description">
          <span className="input-heading-hidden">ENS:</span>
          The ens domain your delivery service will use, e.g.
          myPersonalDeliveryService.eth
        </div>
      </div>
      <div className="base-input-container">
        <div className="input-description">
          <span className="input-heading-hidden">URL:</span>
          {props.urlError && <span className="error">{props.urlError}</span>}
        </div>
        <div className="input-container">
          <span className="input-heading">URL:</span>
          <input
            className="input-field"
            value={props.url}
            onChange={(event) => props.handleUrlChange(event)}
          />
        </div>
        <div className="input-description">
          <span className="input-heading-hidden">URL:</span>
          The url your delivery service will use, e.g.
          https://my-personal-delivery-service.com
        </div>
      </div>
      <div className="base-input-container">
        <div className="input-description">
          <span className="input-heading-hidden">RPC:</span>
          {props.rpcError && <span className="error">{props.rpcError}</span>}
        </div>
        <div className="input-container">
          <span className="input-heading">RPC:</span>
          <input
            className="input-field"
            value={props.rpc}
            onChange={(event) => props.handleRpcChange(event)}
          />
        </div>
      </div>
      <div className="input-description">
        <span className="input-heading-hidden">RPC:</span>

        {showSuccessMsg && (
          <span className="success">Profile created successfully!</span>
        )}
      </div>
      <div>
        <button
          className={"btn env-btn active-btn ".concat(
            isDisabled ? "disabled-btn" : ""
          )}
          disabled={isDisabled}
          onClick={() => props.createConfigAndProfile()}
        >
          Create profile and .env
        </button>
      </div> */}
    </div>
  );
}
