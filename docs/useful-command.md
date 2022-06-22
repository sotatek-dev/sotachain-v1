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

# Get stash account
subkey inspect "wild zone lunar kiss nominee drama puppy voyage stock proof hundred cabbage//stash"

# Generate genesis
./target/debug/node-template build-spec --disable-default-bootnode --chain local > customSpec.json
./target/debug/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```

## Tools

- subkey: https://docs.substrate.io/v3/tools/subkey/

## Chain parameters

```
# genesis file
specs/localnet.json

# source code

runtime/src/constants.rs
runtime/src/lib.rs

```

## Node information

### Node 1

```
# peerId
12D3KooWMM15SSRWdL6WJJH5qP85VUPC2XJNMuS2AvdiETS57SZS

# nodeKey
60031555070980f6da48042ef9435bc5c57ed041d6aad96ceaacb2a7cd50263f

# secret sr25519
Secret phrase:     wild zone lunar kiss nominee drama puppy voyage stock proof hundred cabbage
Network ID:        substrate
Secret seed:       0x2a509394e1f7e17b3725410ab5dd7024dc87ddecfad619e83f02653d6862ccc7
Public key (hex):  0x8eef6d878a81b2bb29b911d892a73ab92a8e8920c63144771ccf0d6bbb7afc5b
Account ID:        0x8eef6d878a81b2bb29b911d892a73ab92a8e8920c63144771ccf0d6bbb7afc5b
Public key (SS58): 5FJ7nHwrXQpVgoNyvtRWLeNHLbnJtEwxM24Zypsvj663Dgta
SS58 Address:      5FJ7nHwrXQpVgoNyvtRWLeNHLbnJtEwxM24Zypsvj663Dgta

# secret ed25519
Secret phrase:     wild zone lunar kiss nominee drama puppy voyage stock proof hundred cabbage
Network ID:        substrate
Secret seed:       0x2a509394e1f7e17b3725410ab5dd7024dc87ddecfad619e83f02653d6862ccc7
Public key (hex):  0xeb06fe636fdade9dc7e79c74babc04738d6605c77c82e60de0061c80da78223f
Account ID:        0xeb06fe636fdade9dc7e79c74babc04738d6605c77c82e60de0061c80da78223f
Public key (SS58): 5HNsBqSZwzgk53dXNa9G7AfQHqyxGwQMLK1Vv6YzZvUiScfm
SS58 Address:      5HNsBqSZwzgk53dXNa9G7AfQHqyxGwQMLK1Vv6YzZvUiScfm

ETH Address: 0x91Ad9A8497257113D429781341D88007cE27eADC
```

### Node 2

```
# peerId
12D3KooWPA12MkYUZBfrm6i6Jq9duJMBfkgCdnxQmFiSZeG3iEVu

# nodeKey
15e445e25b8e50fd5babe8c46e01fbb28c92efef35c4e3bebee98e0d95c1e015

# secret sr25519
Secret phrase:     near scene hold west carpet steak omit boring tone prepare execute appear
Network ID:        substrate
Secret seed:       0x6ec39e03b6a4811f5a5913ab5219ffa27703f2d20f943faf1eaf7d6b0a287e02
Public key (hex):  0xb2cf5efb0cdfeaf8d6f126b2c35eb96a6af690b6ce71865c7f74f0de54c6e20e
Account ID:        0xb2cf5efb0cdfeaf8d6f126b2c35eb96a6af690b6ce71865c7f74f0de54c6e20e
Public key (SS58): 5G79yx3jVZnWSkEcxz3VAyD389xS2tHXRSwgQzGhkartEstd
SS58 Address:      5G79yx3jVZnWSkEcxz3VAyD389xS2tHXRSwgQzGhkartEstd

# secret ed25519
Secret phrase:     near scene hold west carpet steak omit boring tone prepare execute appear
Network ID:        substrate
Secret seed:       0x6ec39e03b6a4811f5a5913ab5219ffa27703f2d20f943faf1eaf7d6b0a287e02
Public key (hex):  0xcb18c17b7bdca4a994fdab480be5412d852f5bf9dd0b7ec161ca2faa4e268d06
Account ID:        0xcb18c17b7bdca4a994fdab480be5412d852f5bf9dd0b7ec161ca2faa4e268d06
Public key (SS58): 5GezvzhTmpBQiBe8Ryi9g4Wrafh8vaSbDj3sCyVWGG4G3Lei
SS58 Address:      5GezvzhTmpBQiBe8Ryi9g4Wrafh8vaSbDj3sCyVWGG4G3Lei

ETH Address: 0x65630e6c62563d712A05F7BBABE967713063F828
```

### Node 3

```
# peerId
12D3KooWNzqYQjqW4V7wtBEDq2JFcAt7EmiuoZWC95DNNHLFZmE9

# nodeKey
e6f104e170b2347056e49de99d63ac37c3c5f6ab7bbdd0bf49b93b041d0388b9

# secret sr25519
Secret phrase:     sea barely tiny photo version chief climb music ozone liar stairs repeat
Network ID:        substrate
Secret seed:       0x7d417cd229b05376df2603b9e3bec1ae83328e1b3d2fa05555f3d4f0e152c7f9
Public key (hex):  0xaa1fa7001ab474f6e0f655e3e8bb2a212dd8a882c1e488aa522e13380f725a78
Account ID:        0xaa1fa7001ab474f6e0f655e3e8bb2a212dd8a882c1e488aa522e13380f725a78
Public key (SS58): 5FumQAmHPJjwVL9dwQaUAi8Pai8nm9AFr6bkiakc6Wwz2Rg1
SS58 Address:      5FumQAmHPJjwVL9dwQaUAi8Pai8nm9AFr6bkiakc6Wwz2Rg1

# secret ed25519
Secret phrase:     sea barely tiny photo version chief climb music ozone liar stairs repeat
Network ID:        substrate
Secret seed:       0x7d417cd229b05376df2603b9e3bec1ae83328e1b3d2fa05555f3d4f0e152c7f9
Public key (hex):  0x04d1917b2a73815cbc8a57360a18c4ec97404e98c17ee3121f179150672905bb
Account ID:        0x04d1917b2a73815cbc8a57360a18c4ec97404e98c17ee3121f179150672905bb
Public key (SS58): 5CB2JZAxk4QQDmcs1w4wiECLPq1Nn7CHXfB3reutDzQr5Jog
SS58 Address:      5CB2JZAxk4QQDmcs1w4wiECLPq1Nn7CHXfB3reutDzQr5Jog

ETH Address: 0xa35b079F06181DF2E7B315592481Ee8F2E21F154
```