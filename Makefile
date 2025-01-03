# Define the chain ID and node name as variables
CHAIN_ID := docs-chain-1
MONIKER := demo
KEYRING_BACKEND := test
ACCOUNT_ALICE := alice
ACCOUNT_BOB := bob
STAKING_AMOUNT := 250000000stake
GENESIS_AMOUNT := 1000000000000stake

# Initialize the node with a moniker and specific chain ID
init:
	@echo "Initializing the node with chain ID $(CHAIN_ID)..."
	wasmd init $(MONIKER) --chain-id=$(CHAIN_ID)

# Add key pairs for alice and bob accounts
keys:
	@echo "Adding key pairs for $(ACCOUNT_ALICE) and $(ACCOUNT_BOB)..."
	wasmd keys add $(ACCOUNT_ALICE) --keyring-backend=$(KEYRING_BACKEND)
	wasmd keys add $(ACCOUNT_BOB) --keyring-backend=$(KEYRING_BACKEND)

# Add genesis accounts with initial balances
genesis-accounts:
	@echo "Adding genesis accounts with initial balances for $(ACCOUNT_ALICE) and $(ACCOUNT_BOB)..."
	wasmd genesis add-genesis-account $(ACCOUNT_ALICE) "$(GENESIS_AMOUNT)" --keyring-backend=$(KEYRING_BACKEND)
	wasmd genesis add-genesis-account $(ACCOUNT_BOB) "$(GENESIS_AMOUNT)" --keyring-backend=$(KEYRING_BACKEND)

# Create a genesis transaction for alice, making her a validator
gentx:
	@echo "Creating genesis transaction for $(ACCOUNT_ALICE) to become a validator..."
	wasmd genesis gentx $(ACCOUNT_ALICE) "$(STAKING_AMOUNT)" --chain-id=$(CHAIN_ID) --amount="$(STAKING_AMOUNT)" --keyring-backend=$(KEYRING_BACKEND)

# Collect genesis transactions to finalize the genesis file
collect-gentxs:
	@echo "Collecting genesis transactions..."
	wasmd genesis collect-gentxs

# Combine all setup steps in a single command
setup: init keys genesis-accounts gentx collect-gentxs
	@echo "Chain setup complete! You can now start your node."

# Start the node
start:
	@echo "Starting the node..."
	make setup
	wasmd start

# Clean up any generated files (optional)
clean:
	@echo "Cleaning up generated files..."
	rm -rf ~/.wasmd
