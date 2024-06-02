# What IS this?

This is a simple solution to a simple problem.
Smart contracts on Etherem Virtual Machines (EVMs) can not do anything without being triggered by an external entity. This leads to absurd situations, like Alice having to **manually** transfer ERC20 tokens to BOB each week. Like a peasant. This is not the future we were promised. This is not the future we want. This is not the future we deserve. This is not the future we will accept. This is not the future we will build. This is not the future we will live in. This is not the future we will die in. This is not the future we will be buried in. This is not the future we will be remembered in. This is not the future we will be forgotten ...

Anyway. We can use the Internet Computer to trigger an EVM smart contract to do something. And, even better, the Internet computer can listen to events the smart contract emits. This pretty much removes the necessity for a separate user interface, because now the smart contract can do order a wake-up call for tomorrow morning 10am. Actually 9am because it's a smart contract and it's always on time.

Check it out. It's pretty cool.

# How to use

## Prerequisites

Install dfx, rust, and caddy for local development. Installation involves the old `curl | sh` **ANTI-PATTERN**. Sorry about that, I did not create this, and at 2am in the morning I can't change it. But I totally understand if you don't want to run it. I didn't want to either. NOBODY WANTS TO RUN `curl | sh`, and for good reason.
Even worse, caddy wants to run as root. **TO INSTALL A \*\*\*\*\*\* ROOT CA CERTIFICATE**. No words here.

After that, try to calm down.

Then run `./deploy.sh` to get the party going. Read the script to learn more, it's pretty nice and well documented (credits go to Moritz Fuller).

## Developing

From there, adapting the code can be done within a reasonable amount of time. The rust code is in `src/chain_fusion_backend` and the solidity code is in `src/foundry/`. The frontend is in `www/`. Just kidding, there is no frontend. But you can add one. It's a free world.

# Credits

This work is heavily inspired by (as in most of it copied from) [Chain Fusion](https://internetcomputer.org/chainfusion), particularly [this repo](https://github.com/letmejustputthishere/chain-fusion-starter/assets/32162112/e787cf9c-0bfc-4ce3-8211-8df61cf06a0b).

So, thanks to Moritz Fuller for the preliminary work.

Also, thanks to Dominik Wörner for lots of support during the Eth Prague hackathon, where this groundbreaking work was done.

[maryjanyes](https://github.com/maryjanyes) did most of the rust coding.
[malteish](https://github.com/malteish) did most of the architecture, infrastructure, documentation and solidity stuff.
