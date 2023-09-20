

deploy-token:
	soroban contract deploy \
		--wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
        --source SCVGDYKZQFNDLJ2DI4HS7JH3Q65T2SCAR7TBZOKZXRHUEYRM5VW4GXWA \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022'

deploy-excellar:
	soroban contract deploy \
		--wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
		--source SABYE6EH6LCPAIWKK2QUVC2XLH6THRPOQGUA64AESNO6JNNDUB2ZNAU4 \
		--rpc-url https://rpc-futurenet.stellar.org:443 \
		--network-passphrase 'Test SDF Future Network ; October 2022'


check-contract:
	soroban contract invoke \
		--id 6b2f06f572ec04e3641d3ff3d5ebf8cbbbfc6f00cfc8d413c8e2a2c2e8e1da04 \
		--rpc-url https://rpc-futurenet.stellar.org:443 \
		--network-passphrase 'Test SDF Future Network ; October 2022' \
		 -- --help