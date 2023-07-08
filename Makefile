

deploy-token:
	soroban contract deploy \
		--wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
        --source SCBDHL6YTFK4FUQIWRXPM2HZ6KAA7YECCOK6Y7RTYLTWRNJ2XDHHBH5R \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022'

deploy-excellar:
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/excellar.wasm \
		--source SASA7CRB4F6HZEMMJVIHA3PTSCIFSZ7YIU7VHFJX3YSJTELSID46Y3UG \
		--rpc-url https://rpc-futurenet.stellar.org:443 \
		--network-passphrase 'Test SDF Future Network ; October 2022'