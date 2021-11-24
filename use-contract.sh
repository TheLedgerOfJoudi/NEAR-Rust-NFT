

near call $CONTRACT set_owner '{"token_id": "firstNFT", "account_id" : "hdsaleh.testnet"}' --accountId $CONTRACT
near view $CONTRACT get_owner '{"token_id": "firstNFT"}' --accountId $CONTRACT



#near call dev-1637769975999-21918764047948 set_owner '{"token_id": "firstNFT", "account_id" : "hdsaleh.testnet"}' --accountId dev-1637769975999-21918764047948
#near view dev-1637769975999-21918764047948 get_owner '{"token_id": "firstNFT"}' --accountId dev-1637769975999-21918764047948