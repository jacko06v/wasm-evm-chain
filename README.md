
<div align="center">




</div>

rimosse tutte le parachain, rimasta solo la chain dev => solo-chain

## server 
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



