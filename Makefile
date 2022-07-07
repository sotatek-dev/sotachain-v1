local1:
	./target/debug/node-template purge-chain --base-path /tmp/node01 --chain specs/localnetRaw.json

	./target/debug/node-template key insert --base-path /tmp/node01 \
		--chain specs/localnetRaw.json \
		--scheme Sr25519 \
		--suri 0x2a509394e1f7e17b3725410ab5dd7024dc87ddecfad619e83f02653d6862ccc7 \
		--key-type babe

	./target/debug/node-template key insert --base-path /tmp/node01 \
		--chain specs/localnetRaw.json \
		--scheme Ed25519 \
		--suri 0x2a509394e1f7e17b3725410ab5dd7024dc87ddecfad619e83f02653d6862ccc7 \
		--key-type gran

	./target/debug/node-template \
		--base-path /tmp/node01 \
		--chain ./specs/localnetRaw.json \
		--port 30333 \
		--ws-port 9945 \
		--rpc-port 9933 \
		# --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
		--validator \
		--rpc-methods Unsafe \
		--name MyNode01 \
		--node-key=60031555070980f6da48042ef9435bc5c57ed041d6aad96ceaacb2a7cd50263f

local2:
	./target/debug/node-template purge-chain --base-path /tmp/node02 --chain specs/localnetRaw.json

	./target/debug/node-template key insert --base-path /tmp/node02 \
		--chain specs/localnetRaw.json \
		--scheme Sr25519 \
		--suri 0x6ec39e03b6a4811f5a5913ab5219ffa27703f2d20f943faf1eaf7d6b0a287e02 \
		--key-type babe

	./target/debug/node-template key insert --base-path /tmp/node02 \
		--chain specs/localnetRaw.json \
		--scheme Ed25519 \
		--suri 0x6ec39e03b6a4811f5a5913ab5219ffa27703f2d20f943faf1eaf7d6b0a287e02 \
		--key-type gran

	./target/debug/node-template \
		--base-path /tmp/node02 \
		--chain ./specs/localnetRaw.json \
		--port 30334 \
		--ws-port 9946 \
		--rpc-port 9934 \
		# --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
		--validator \
		--rpc-methods Unsafe \
		--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMM15SSRWdL6WJJH5qP85VUPC2XJNMuS2AvdiETS57SZS \
		--name MyNode02 \
		--node-key=15e445e25b8e50fd5babe8c46e01fbb28c92efef35c4e3bebee98e0d95c1e015

local3:
	./target/debug/node-template purge-chain --base-path /tmp/node03 --chain specs/localnetRaw.json

	./target/debug/node-template key insert --base-path /tmp/node03 \
		--chain specs/localnetRaw.json \
		--scheme Sr25519 \
		--suri 0x7d417cd229b05376df2603b9e3bec1ae83328e1b3d2fa05555f3d4f0e152c7f9 \
		--key-type babe

	./target/debug/node-template key insert --base-path /tmp/node03 \
		--chain specs/localnetRaw.json \
		--scheme Ed25519 \
		--suri 0x7d417cd229b05376df2603b9e3bec1ae83328e1b3d2fa05555f3d4f0e152c7f9 \
		--key-type gran

	./target/debug/node-template \
		--base-path /tmp/node03 \
		--chain ./specs/localnetRaw.json \
		--port 30335 \
		--ws-port 9947 \
		--rpc-port 9935 \
		# --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
		--validator \
		--rpc-methods Unsafe \
		--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMM15SSRWdL6WJJH5qP85VUPC2XJNMuS2AvdiETS57SZS \
		--name MyNode03 \
		--node-key=e6f104e170b2347056e49de99d63ac37c3c5f6ab7bbdd0bf49b93b041d0388b9