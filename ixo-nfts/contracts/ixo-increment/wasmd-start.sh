#!/bin/bash

rm -rf $HOME/.wasmd

wasmd init node0 --chain-id=test-1

wasmd keys add validator --keyring-backend=test

wasmd genesis add-genesis-account $(wasmd keys show -a validator  --keyring-backend=test) 100000000stake

wasmd genesis gentx validator 10000000stake --keyring-backend=test --chain-id=test-1
wasmd genesis collect-gentxs 

wasmd start