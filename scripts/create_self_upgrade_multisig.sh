#!/bin/bash
set -e
multisig --cluster $1 create-multisig --multisig-account ../keys/self_upgrade_multisig.json --output-multisig-pda ../target/self_upgrade_multisig_pda.pubkey  --threshold $2 --owner ${@:3}