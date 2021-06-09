use sp_core::{Pair, Public, sr25519};
use canvas_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use hex_literal::hex;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn testnet_authorities() -> Vec<(AuraId, GrandpaId)> {
	use sp_core::crypto::UncheckedInto;

	// ./scripts/prepare-test-net.sh 2
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		AuraId,
		GrandpaId,
	)> = vec![
		(
			//5Dkp2xwztRtRkXdmLDJUm8wvdHWSApgYgJY3rB5brKZkWJPM
			hex!["4ad2d854d1bc5bbfdf93b689cb1aa3986684126c2b16d86924019de6798b0971"].into(),
			//5DvnJs3jM7sJkgfpVDxwu9GNmpFyzJSn2jG9w1hxBBWwqi92
			hex!["526d7d3d5357d20e4cd75d1a452bd5b4903caf160f55d6bd19807efbdf165319"].into(),
			//5GKFbTTgrVS4Vz1UWWHPqMZQNFWZtqo7H2KpCDyYhEL3aS26
			hex!["bc09354c12c054c8f6b3da208485eacec4ac648bad348895273b37bab5a0937c"].unchecked_into(),
			//5ErzrnqhfgXJeuFsRvwNsrbqBAbiePoqeeDZZYzaTDxwtANY
			hex!["7bc6fd5dc6e832b294bbf2ae21df67f990a526793a9ded12a5e54e40a5a94d1d"].unchecked_into(),
		),
		(
			//5Gnadix7NJH2K7Akr5hVKyXqtYqTsQJbXnKFFt274MsG8n7h
			hex!["d0e0ece66fb861b82383e85326a2e179316021105492820ca544ea8743620b59"].into(),
			//5FADgcPNMtcLtrmd16rZk6HJfDWqtEDT4N3jMSqtFo8tJhRr
			hex!["88e95527362f479ebf30502db2f7d88329e034f5d77aed585042c548fa93ae01"].into(),
			//5EPRJHm2GpABVWcwnAujcrhnrjFZyDGd5TwKFzkBoGgdRyv2
			hex!["66be63b7bcbfb91040e5248e2d1ceb822cf219c57848c5924ffa3a1f8e67ba72"].unchecked_into(),
			//5HNx7nq3brkA3St9CAU1KijnscvAsa5ornNuk1H1iSuXy1nd
			hex!["eb17972691ec3a7d09a316baddc8838362ade2c12a21a506d697903e16577bfd"].unchecked_into(),
		),
    ];
    initial_authorities
        .into_iter()
        .map(|(_, _, aura_id, grandpa_id)| (aura_id, grandpa_id))
        .collect()
}

pub fn testnet_root() -> AccountId {
	hex!("baa78c7154c7f82d6d377177e20bcab65d327eca0086513f9964f5a0f6bdad56").into()
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
		),
		vec![],
		None,
		None,
		None,
		None,
	))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;
	let mut properties = sc_service::Properties::new();
	properties.insert("tokenSymbol".to_string(), "CAN".into());
	properties.insert("tokenDecimals".to_string(), 12.into());
	properties.insert("ss58Format".to_string(), 42.into());

	Ok(ChainSpec::from_genesis(
		"Canvas Testnet",
		"canvas_testnet2",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			testnet_authorities(),
			testnet_root(),
			vec![
				testnet_root(),

				// AccountId of the Canvas Testnet faucet
				hex!("18c64aa111a8a0e6e4eed41d6d906c7614d745e48be3cfc13b6128e1d51f4405").into(),

				// AccountId of an account which `ink-waterfall` uses for automated testing
				hex!("0e47e2344d523c3cc5c34394b0d58b9a4200e813a038e6c5a6163cc07d70b069").into(),
			],
		),
		vec![
			"/ip4/34.90.191.237/tcp/30333/p2p/12D3KooWKg3Rpxcr9oJ8n6khoxpGKWztCZydtUZk2cojHqnfLrpj".parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/35.204.68.28/tcp/30333/p2p/12D3KooWPEXYrz8tHU3nDtPoPw4V7ou5dzMEWSTuUj7vaWiYVAVh".parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/34.90.139.15/tcp/30333/p2p/12D3KooWEVU8AFNary4nP4qEnEcwJaRuy59Wefekzdu9pKbnVEhk".parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/35.204.99.97/tcp/30333/p2p/12D3KooWP6pV3ZmcXzGDjv8ZMgA6nZxfAKDxSz4VNiLx6vVCQgJX".parse()
				.expect("MultiaddrWithPeerId"),
		],
		None,
		Some("prc"),
		Some(properties),
		None
	))
}

fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
	}
}
