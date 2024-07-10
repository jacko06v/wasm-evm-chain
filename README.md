
<div align="center">




</div>

rimosse tutte le parachain, rimasta solo la chain dev => solo-chain

run with: 
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



