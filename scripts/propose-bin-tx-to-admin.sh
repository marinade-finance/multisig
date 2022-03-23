#!/bin/bash
#
#  Use this script to propose a bin tx created with mardmin. E.g.: change cap, delta stake window, liq-pool params, etc.
#  tx will be sent to be approved by Multisig#3 Admin
#
#  > bash scripts/prepare-mnde-transfer 1000 DJLZ8xV7ZzEKFXtuA7TDUBGnzikHBXB9y9EpDyddcQdK orca
#
MULTISIG_3_ADMIN=7mSA2bgzmUCi4wh16NQEfT76XMqJULni6sheZRCjcyx7
set -ex
./target/debug/multisig --cluster http://marinade.rpcpool.com propose-binary-transaction \
    --multisig-address $MULTISIG_3_ADMIN \
    --data $1 \
    ${@: 3}
