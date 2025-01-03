#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
# Load environment variables from the .env file
export $(grep -v '^#' $SCRIPT_DIR/../.env | xargs)

# Check if the required variables are set
if [ -z "$CHAIN_ID" ] || [ -z "$WALLET_NAME" ] || [ -z "$CONTRACT_ADDRESS" ]; then
    echo "Error: One or more required environment variables are missing."
    exit 1
fi

# Query the contract state
RESP=$(wasmd query wasm contract-state all "$CONTRACT_ADDRESS" -o json | jq -r '.models[0].value' | base64 -d)
echo "Contract state: $RESP"

#####################################
#####################################
# Get an item from the vending machine (e.g., chocolate)
echo "**********Getting chocolate item from the vending machine...**********"
RESP=$(wasmd tx wasm execute $CONTRACT_ADDRESS \
    '{"get_item": {"item_type": "chocolate"}}' \
    --from $WALLET_NAME \
    --chain-id $CHAIN_ID \
    -o json \
    --gas $GAS \
    --gas-adjustment $GAS_ADJUSTMENT \
    --keyring-backend=test \
    -y)

# Check for errors in the response
if [[ $(echo "$RESP" | jq -r '.code') != "0" ]]; then
    echo "Error executing transaction: $(echo "$RESP" | jq -r '.log')"
    exit 1
fi

sleep 6

# Fetch the transaction details
RESP=$(wasmd q tx $(echo "$RESP"| jq -r '.txhash') -o json)
echo "Item retrieval successful!"

# Query the items count from the vending machine
echo "Querying items count from the vending machine..."
RESP=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
    '{"items_count": {}}' \
    -o json)

# Display the items count
ITEMS_COUNT=$(echo "$RESP" | jq -r '.data')
echo "Items count: $ITEMS_COUNT"


#####################################
#####################################
# Get an item from the vending machine (e.g., water)
echo "**********Getting water item from the vending machine...**********"
RESP=$(wasmd tx wasm execute $CONTRACT_ADDRESS \
    '{"get_item": {"item_type": "water"}}' \
    --from $WALLET_NAME \
    --chain-id $CHAIN_ID \
    -o json \
    --gas $GAS \
    --gas-adjustment $GAS_ADJUSTMENT \
    --keyring-backend=test \
    -y)

# Check for errors in the response
if [[ $(echo "$RESP" | jq -r '.code') != "0" ]]; then
    echo "Error executing transaction: $(echo "$RESP" | jq -r '.log')"
    exit 1
fi

sleep 6

# Fetch the transaction details
RESP=$(wasmd q tx $(echo "$RESP"| jq -r '.txhash') -o json)
echo "Item retrieval successful!"

# Query the items count from the vending machine
echo "Querying items count from the vending machine..."
RESP=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
    '{"items_count": {}}' \
    -o json)

# Display the items count
ITEMS_COUNT=$(echo "$RESP" | jq -r '.data')
echo "Items count: $ITEMS_COUNT"

#####################################
#####################################
# Get an item from the vending machine (e.g., chips)
echo "**********Getting chips item from the vending machine...**********"
RESP=$(wasmd tx wasm execute $CONTRACT_ADDRESS \
    '{"get_item": {"item_type": "chips"}}' \
    --from $WALLET_NAME \
    --chain-id $CHAIN_ID \
    -o json \
    --gas $GAS \
    --gas-adjustment $GAS_ADJUSTMENT \
    --keyring-backend=test \
    -y)

# Check for errors in the response
if [[ $(echo "$RESP" | jq -r '.code') != "0" ]]; then
    echo "Error executing transaction: $(echo "$RESP" | jq -r '.log')"
    exit 1
fi

sleep 6

# Fetch the transaction details
RESP=$(wasmd q tx $(echo "$RESP"| jq -r '.txhash') -o json)
echo "Item retrieval successful!"

# Query the items count from the vending machine
echo "Querying items count from the vending machine..."
RESP=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
    '{"items_count": {}}' \
    -o json)

# Display the items count
ITEMS_COUNT=$(echo "$RESP" | jq -r '.data')
echo "Items count: $ITEMS_COUNT"

echo "All items retrieved successfully!"

#####################################
#####################################
# Refill the vending machine
echo "**********Refilling the vending machine with ( chocolate: 10, water: 10, chips: 10)...**********"
RESP=$(wasmd tx wasm execute $CONTRACT_ADDRESS \
    '{"refill": {"chocolate": 10, "water": 10, "chips": 10}}' \
    --from $WALLET_NAME \
    --chain-id $CHAIN_ID \
    -o json \
    --gas $GAS \
    --gas-adjustment $GAS_ADJUSTMENT \
    --keyring-backend=test \
    -y)

# Check for errors in the response
if [[ $(echo "$RESP" | jq -r '.code') != "0" ]]; then
    echo "Error executing transaction: $(echo "$RESP" | jq -r '.log')"
    exit 1
fi

sleep 6

# Fetch the transaction details
RESP=$(wasmd q tx $(echo "$RESP"| jq -r '.txhash') -o json)
echo "Item refilled successful!"

# Query the items count from the vending machine
echo "Querying items count from the vending machine..."
RESP=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
    '{"items_count": {}}' \
    -o json)

# Display the items count
ITEMS_COUNT=$(echo "$RESP" | jq -r '.data')
echo "Items count: $ITEMS_COUNT"
