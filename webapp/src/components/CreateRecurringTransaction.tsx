import { useEffect, useState } from "react";

interface ConfigureProfileProps {
  recipient: string;
  rpc: string;
  url: string;
  handleRecipientChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  handleAmountChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  handlePeriodChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  createRecurringTransaction: () => void;
  recipientError: string | null;
  amountError: string | null;
  urlError: string | null;
  isConnected: boolean;
}

export function CreateRecurringTransaction(props: ConfigureProfileProps) {
  const [showSuccessMsg, setShowSuccessMsg] = useState<boolean>(false);

  const isDisabled =
    !props.isConnected || !props.recipient.length || !props.url.length
      ? true
      : false;

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
      To create a recurring token transfer, please provide the details below.
      You will also have to grant an allowance to the smart contract so it can
      access your tokens. <br />
      What will happen: The smart contract will send the specified amount of
      tokens to the recipient address immediately. <br />
      After the specified period, the smart contract will send the same amount
      of tokens to the recipient address again. This will continue until you
      stop the recurring transaction, or the allowance is exhausted. <br />
      <div className="base-input-container">
        <div className="input-description">
          <span className="input-heading-hidden">Recipient</span>
          {props.recipientError && (
            <span className="error">{props.recipientError}</span>
          )}
        </div>
        <div className="input-container">
          <span className="input-heading">Recipient address:</span>
          <input
            className="input-field"
            value={props.recipient}
            onChange={(event) => props.handleRecipientChange(event)}
          />
        </div>
        <div className="input-description">
          <span className="input-heading-hidden">Recipient address::</span>
          The address that will receive the tokens you send
        </div>
      </div>
      <div className="base-input-container">
        <div className="input-description">
          <span className="input-heading-hidden">Amount:</span>
          {props.urlError && <span className="error">{props.urlError}</span>}
        </div>
        <div className="input-container">
          <span className="input-heading">Amount:</span>
          <input
            className="input-field"
            value={props.url}
            onChange={(event) => props.handleAmountChange(event)}
          />
        </div>
        <div className="input-description">
          <span className="input-heading-hidden">Amount:</span>
          How many tokens should be sent, in token bits.
        </div>
      </div>
      <div className="base-input-container">
        <div className="input-description">
          <span className="input-heading-hidden">Amount:</span>
          {props.amountError && (
            <span className="error">{props.amountError}</span>
          )}
        </div>
        <div className="input-container">
          <span className="input-heading">Period:</span>
          <input
            className="input-field"
            value={props.rpc}
            onChange={(event) => props.handlePeriodChange(event)}
          />
        </div>
        <div className="input-description">
          <span className="input-heading-hidden">Period:</span>
          After which duration the tokens should be sent again (and again and
          again). In seconds.
        </div>
      </div>
      <div className="input-description">
        <span className="input-heading-hidden">Status:</span>

        {showSuccessMsg && (
          <span className="success">
            Recurring transaction created successfully!
          </span>
        )}
      </div>
      <div>
        <button
          className={"btn env-btn active-btn ".concat(
            isDisabled ? "disabled-btn" : ""
          )}
          disabled={isDisabled}
          onClick={() => props.createRecurringTransaction()}
        >
          Create recurring transaction
        </button>
      </div>
    </div>
  );
}
