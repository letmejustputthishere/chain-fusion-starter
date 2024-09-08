import { useState } from "react";

interface ConfigureProfileProps {
  recipient: string;
  period: string;
  amount: string;
  executions: string;
  handleRecipientChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  handleAmountChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  handlePeriodChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  handleExecutionsChange: (
    event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>
  ) => void;
  createRecurringTransaction: () => void;
  recipientError: string | null;
  amountError: string | null;
  urlError: string | null;
  isConnected: boolean;
  writeContractIsError: boolean;
  writeContractIsPending: boolean;
  writeContractError: any;
}

export function CreateRecurringTransaction(props: ConfigureProfileProps) {
  const [showSuccessMsg, setShowSuccessMsg] = useState<boolean>(false);

  const isDisabled =
    !props.isConnected || !props.recipient.length || !props.amount.length
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
      of tokens to the recipient address again. This will continue until the
      total number of executions is reached. <br />
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
            value={props.amount}
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
          <span className="input-heading-hidden">Period:</span>
          {props.amountError && (
            <span className="error">{props.amountError}</span>
          )}
        </div>
        <div className="input-container">
          <span className="input-heading">Period:</span>
          <input
            className="input-field"
            value={props.period}
            onChange={(event) => props.handlePeriodChange(event)}
          />
        </div>
        <div className="input-description">
          <span className="input-heading-hidden">Period:</span>
          After which duration the tokens should be sent again (and again and
          again). In seconds.
        </div>
      </div>
      <div className="base-input-container">
        <div className="input-description">
          <span className="input-heading-hidden">Number of executions:</span>
          {props.amountError && (
            <span className="error">{props.amountError}</span>
          )}
        </div>
        <div className="input-container">
          <span className="input-heading">Number of executions:</span>
          <input
            className="input-field"
            value={props.executions}
            onChange={(event) => props.handleExecutionsChange(event)}
          />
        </div>
        <div className="input-description">
          <span className="input-heading-hidden">Number of executions:</span>
          Total number of times the transaction should be executed.
        </div>
      </div>
      <div className="base-input-container">
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
        <div className="input-description">
          <span className="input-heading-hidden">Number of executions:</span>A
          service fee of 0.01 xDai is charged for each execution. This payment
          will be part of the transactions you sign when you push the button.
        </div>
        <div>
          {props.writeContractIsError &&
            "Error when writing contract: " + props.writeContractError}
        </div>
      </div>
    </div>
  );
}
