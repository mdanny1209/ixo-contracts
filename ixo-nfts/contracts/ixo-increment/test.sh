DEPLOYER=$(wasmd keys show -a validator --keyring-backend=test)
wasmd tx wasm store artifacts/ixo_increment.wasm --from=validator --keyring-backend=test --chain-id=test-1 --gas=auto --gas-adjustment=1.3 -y
wasmd tx wasm instantiate 1 '{}' --from=validator --label "Increment" --chain-id=test-1 --gas=auto --gas-adjustment=1.3 -b=sync --keyring-backend=test --admin="$DEPLOYER" -y
CONTRACT=wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d

wasmd query wasm contract-state smart $CONTRACT '{"get_count":{}}'
wasmd tx wasm execute $CONTRACT '{"increment":{}}' --from=validator --gas=auto --gas-adjustment=1.3 --chain-id=test-1 -y --keyring-backend=test 
