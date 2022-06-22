# Useful commands

## Commands

```sh

# Generate nodekey
subkey generate-node-key

# Generate account
subkey generate

# Get ed25519 key of an account
subkey inspect "wild zone lunar kiss nominee drama puppy voyage stock proof hundred cabbage" --scheme ed25519

# Inspect default dev account
subkey inspect "//Alice"

# Generate genesis
./target/debug/node-template build-spec --disable-default-bootnode --chain local > customSpec.json
./target/debug/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```

## Tools

- subkey: https://docs.substrate.io/v3/tools/subkey/