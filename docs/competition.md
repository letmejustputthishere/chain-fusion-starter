# Unique selling points

Everyone know at least one service that "already does that". Here is a list of services that offer recurring transactions and how they compare to this project.

As a reminder, here are our goals mixed into a user story:

1. Alice does not need to create an account anywhere
2. She uses a simple web frontend to create the recurring transaction. Parameters: Token, amount, recipient, frequency. She sets up: EUR, 10, Bob, weekly
3. The creation of the transaction involves 2 transactions: Token Approval and creating the transaction. With the second transaction, Alice sends a payment for the service directly (e.g. 0.1 xdai for 10 executions or similar)
4. The tokens remain on Alice's account until the respective transaction is executed
5. Then they are on Bob's account
6. Bob does not need to create an account anywhere
7. Bob does not need to pick up his money anywhere
8. Security is provided by the verified smart contract

## B2C Services

These services are targeted towards consumers. They can be used without developing additional software.

- [Superfluid](https://app.superfluid.finance/)

  - money streaming service
  - tokens need to be wrapped into [supertokens](https://docs.superfluid.finance/docs/protocol/super-tokens/overview) -> not great
  - **puts the burden of claiming the payment on the receiver**

- [ReCur](https://github.com/BitDiem/recur)

  - smart contracts still available
  - service [bitDiem](https://x.com/bitdiem) disappeared

- [Aion](https://www.aion.ethpantheon.com/index.html#howitworks)
  - [predecessor   EthTempus](https://github.com/jfdelgad/ETH-Tempus)
  - seems abandoned
- [Sablier](https://sablier.com/)
  - **creates streams the recipient needs to claim**

# B2B Services

For smart contract automation (could be used as execution infrastructure in ReTrans). Not suitable for end-users like Alice and Bob.

- [Gelato](https://app.gelato.network/functions/create)
  - can call smart contract function
  - must be paid
    - through subscription
    - from 1Balance
    - can pay for itself if smart contract implements [special interface](https://docs.gelato.network/web3-services/web3-functions/subscription-and-payments#transaction-pays-for-itself)
  - possible solution: deploy smart contract with payment parameters, grant allowance, have gelato execute the function
- Chainlink Automation
  - Chainlink function needs a smart contract and a subscription. The contracts are called [Upkeeps](https://docs.chain.link/quickstarts/time-based-upkeep)
  - seems not to support [gnosis](https://docs.chain.link/chainlink-automation/overview/supported-networks)
- Internet Computer
  - can trigger anything using rpc
  - needs smart contracts and extensive setup

For employers that need to pay employees or similar

- [OnChainPay](https://onchainpay.io/features.html)
  - targeted towards businesses that charge for subscriptions, so the recipient needs to set everything up
  - no support for gnosis chain
  - 0.7% fees charged from the recipient
- Bitwage -> adresses employers and employees, who both need to sign up
- [Chronologic](https://blog.chronologic.network/)
  - used to be called EthereumAlarmClock, which points to this project now
  - seems abandoned, I can not find the dapp
  - does not mention recurring transactions
- [Hedgey finance](https://hedgey.finance/) -> permissioned, needs account and large setup

Others

- https://card.builtby.mom/ -> only for gnosis card safe. Unsure what they offer.
