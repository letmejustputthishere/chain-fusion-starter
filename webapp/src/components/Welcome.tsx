interface WelcomeProps {
  address: `0x${string}` | undefined;
  balance: any;
  balanceIsLoading: boolean;
  balanceIsError: boolean;
}

export function Welcome(props: WelcomeProps) {
  return (
    <>
      <div className="description-text">
        You can use this web app to create, display or delete recurring ERC20
        token transfers from your address to another address. <br />
      </div>
      <div className="description-text">
        Your address is: <b>{props.address}</b>
        <br />
        Your balance is: <b>{props.balance ? props.balance : "unknown"}</b>
        {props.balanceIsLoading && <p>Loading balance...</p>}
        {props.balanceIsError && <p>Error loading balance</p>}
      </div>
    </>
  );
}
