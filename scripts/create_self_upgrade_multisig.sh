#!/bin/bash
set -e
multisig --cluster d create-multisig --multisig-account ../keys/self_upgrade_multisig.json --output-multisig-pda ../target/self_upgrade_multisig_pda.pubkey  --threshold $1 --owner ${@:2}