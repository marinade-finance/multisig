#!/bin/bash
set -e
echo "deploy testnet AFFECTING testnet USERS?"
read -p "Press any key to continue..."
cargo build 
anchor build
cp target/idl/multisig.json res/
solana program deploy -v -u testnet target/deploy/multisig.so 
#  --upgrade-authority marinade_finance-keypair.json
