## GENERATE BINARY FILE
```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6

```
--------------------------------------------------------------------------------
## UPLOAD BINARY FILE TO BLOCKCHAIN

```sh
['DEPLOY MULTI TRANSACTION']

wasmd tx wasm store ./artifacts/smart_contract_pro.wasm --chain-id wasmd --gas 7000000 --from alice

```

## CONTRACT_ID=OUTPUT_CONTRACT_ID


## **initialization**
```sh
['initialization']

wasmd tx wasm instantiate $CONTRACT_ID '{"fee":"10", "from_bank_addr":"wasm1me8cu9jh0kkwtjvp0tuuuxdarf5pxtffwpvtwg","from_bank_fee":"40","to_bank_addr":"wasm1exr27xs5tl3t7hfj6kdt5hq3fxnd4ccqu2c4e0","to_bank_fee":"40","service_addr":"wasm14qaff2nct5zlj07c7m0fqnsqclsdan4pugdg9j","service_fee":"20"}' --label "transactionContract" --amount 10stake --no-admin --chain-id wasmd --from alice

```

## CONTRACT_ADDRESS = "wasm1fventeva948ue0fzhp6xselr522rnqwger9wg7r0g9f4jemsqh6sd6k3tx"

## MULTI TRANSACTION 

```sh
['TRANSACTION CREATE REQUEST']

wasmd tx wasm execute $CONTRACT_ADDRESS '{"transfer":{"to":"wasm1vwh7k0l4gggnsl0we0tzaq9lfhpsqm456xafpu"}}' --amount 200stake --chain-id wasmd --from alice

```

 ## GET CURRENT TRANSACTION FEE PERCENTAGE 

```sh
['GET_CURRENT_FEE_STATE']

wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"get_current_fee_state":{}}' --chain-id wasmd

```

## UPDATE CONFIG STATE

```sh
['UPDATE_CONFIG']

wasmd tx wasm execute $CONTRACT_ADDRESS '{"update_fee":{"fee":"20", "from_bank_addr":"wasm1me8cu9jh0kkwtjvp0tuuuxdarf5pxtffwpvtwg","sender_bank_fee":"45","receiver_bank_addr":"wasm1exr27xs5tl3t7hfj6kdt5hq3fxnd4ccqu2c4e0","receiver_bank_fee":"45","service_addr":"wasm14qaff2nct5zlj07c7m0fqnsqclsdan4pugdg9j","service_fee":"10"}}' --chain-id wasmd --from alice

```
## CHECK UPDATED CONFIG STATE

```sh
['GET_CONFIG']

wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"get_config_state":{}}' --chain-id wasmd -o json | jq

```
