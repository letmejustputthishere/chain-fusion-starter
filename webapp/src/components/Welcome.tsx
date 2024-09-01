interface WelcomeProps {
  address: `0x${string}` | undefined;
}

export function Welcome(props: WelcomeProps) {
  return (
    <>
      <h2 className="heading-text">Welcome {props.address}</h2>
      <div className="description-text">
        You can use this web app to set up recurring ERC20 token transfers from
        your address to another address. <br />
        MARIA MAMA PAPA <br />
        HGTRFDEK
      </div>
    </>
  );
}
