#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
# Load environment variables from the .env file
export $(grep -v '^#' $SCRIPT_DIR/../.env | xargs)

# Check if the required variables are set
if [ -z "$CHAIN_ID" ] || [ -z "$WALLET_NAME" ] || [ -z "$CONTRACT_PATH" ]; then
    echo "Error: One or more required environment variables are missing."
    exit 1
fi

# Upload the contract to the blockchain (store the contract)
echo "Uploading contract..."
RESP=$(wasmd tx wasm store $SCRIPT_DIR/../$CONTRACT_PATH \
    --from $WALLET_NAME \
    --chain-id $CHAIN_ID \
    --gas $GAS \
    --gas-adjustment $GAS_ADJUSTMENT \
    -o json \
    --keyring-backend=test \
    -y)


# Wait for the transaction to be processed
sleep 6

# Fetch the transaction details
RESP=$(wasmd q tx $(echo "$RESP"| jq -r '.txhash') -o json)
# Extract the code ID
CODE_ID=$(echo "$RESP" | jq -r '.events[]| select(.type=="store_code").attributes[]| select(.key=="code_id").value')
 
# Print code id
echo "Contract uploaded successfully with code_id: $CODE_ID"

# Set up addresses
ALICE_ADDR=$(wasmd keys show alice -a --keyring-backend=test)

# Instantiate the contract (pass the initial values for chocolate, water, and chips)
echo "Instantiating contract with (chocolate: 10, water: 20, chips: 30)..."
RESP=$(wasmd tx wasm instantiate "$CODE_ID" \
    '{"chocolate": 10, "water": 20, "chips": 30}' \
    --admin="$ALICE_ADDR" \
    --from $WALLET_NAME \
    --chain-id $CHAIN_ID \
    -o json \
    --gas $GAS \
    --gas-adjustment $GAS_ADJUSTMENT \
    --label "local0.1.0" \
    --keyring-backend=test \
    -y)

echo "Contract instantiated successfully!"

# Wait for the transaction to be processed
sleep 6

# Query the contract address
CONTRACT_ADDRESS=$(wasmd query wasm list-contract-by-code "$CODE_ID" -o json | jq -r '.contracts[-1]')

echo "*****Contract address*****: $CONTRACT_ADDRESS"
echo "*****Contract Details*****: "
wasmd q wasm contract $CONTRACT_ADDRESS -o json
