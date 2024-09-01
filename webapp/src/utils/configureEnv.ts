export const configureEnv = (url: string, address: string) => {
  return (
    "# URL=" +
    url +
    "\n" +
    "# ACCOUNT_USED_FOR_KEY_CREATION=" +
    address +
    "\n" +
    "# MESSAGE_USED_FOR_KEY_CREATION="
  );
};
