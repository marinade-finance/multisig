#!/bin/bash
#
#  Use this script to prepare a MNDE transfer from GR1LBT4cU89cJWE74CP6BsJTf2kriQ9TX59tbDsfxgSi to [destination-account]
#  the trasnfer will be uploaded to be approved by Multisig#4
#
#  [destination-account] can be a native account (the MNDE associated token account, ATA, will be selected and created if needed)
#  
#  Example: To prepare a transfer of 1000 MNDE to ORCA-mSOL pool incentives (DJLZ8xV7ZzEKFXtuA7TDUBGnzikHBXB9y9EpDyddcQdK), the command is: 
#
#  > bash scripts/prepare-mnde-transfer 1000 DJLZ8xV7ZzEKFXtuA7TDUBGnzikHBXB9y9EpDyddcQdK orca
#
MULTISIG_4_TREASURY=9aN4drMhmd8AX3eRdYvH1gbZiPmwgGJfjvneCECF97HD
set -ex
./target/debug/multisig --cluster http://marinade.rpcpool.com propose-spl-token-transfer \
    --amount $1 \
    --from GR1LBT4cU89cJWE74CP6BsJTf2kriQ9TX59tbDsfxgSi \
    --to $2 \
    --multisig-address $MULTISIG_4_TREASURY \
    --auth 9cBg3Ankxf4ijde8tjRKAGix5EGRbASnvigUA3JW8WSu \
    ${@: 3}


