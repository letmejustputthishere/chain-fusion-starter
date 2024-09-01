interface WelcomeProps {
  address: `0x${string}` | undefined;
}

export function Welcome(props: WelcomeProps) {
  return (
    <>
      <div className="description-text">
        You can use this web app to create, display or delete recurring ERC20
        token transfers from your address to another address. <br />
      </div>
    </>
  );
}
