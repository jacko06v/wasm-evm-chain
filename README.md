
<div align="center">




</div>

removed all parachains - keeped dev chain => solochain


![Version](./version-badge.svg)

## if runned on an external server with public ip
run: 
```
./target/release/astar-collator --dev  --rpc-external
```
evm explorer: https://tryethernal.com/



## local
run: 
```
./target/release/astar-collator \
--base-path /tmp/alice \
--chain dev \
--alice \
--port 30333 \
--rpc-port 9945 \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator
```
explorer substrate: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9945


## useful pallets

- inflation - emit X tokens/year
- dynamic-evm-base-fee - calc fees for ethereum like chain
- XVM - run wasm and evm VMs
- unified-accounts - it allows to use metamask ( and ethereum compatible tools like remix/hardhat) and polkadot apps like polkadot.js app

# TODO 
- check if the solo-chain dev can be used on production
- remove every unnecessary pallet
- remove parachains trace
- *IMPORTANT* Implement purgechain subcommand


