

deploy-token:
	soroban contract deploy \
		--wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
        --source SCBDHL6YTFK4FUQIWRXPM2HZ6KAA7YECCOK6Y7RTYLTWRNJ2XDHHBH5R \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022'

deploy-excellar:
	soroban contract deploy \
		--wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
		--source SAVQKTSXS3T2VNXQRESDPWEAYT5HCSA6GRXPCGUF6HZDM2EOLGYDHFY6 \
		--rpc-url https://rpc-futurenet.stellar.org:443 \
		--network-passphrase 'Test SDF Future Network ; October 2022'
