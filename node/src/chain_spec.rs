use sp_core::{Pair, Public, sr25519};
use canvas_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature,
	ContractsConfig,
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
	vec![
		(
			hex!("74608217b1709e1d3a4fe65b132db5c3f321e625026080833189661aa5e20712").unchecked_into(),
			hex!("a5abc21ac95ae63dd6e61e5bec263ab46d1efe16d3dcc085d0de297318cb662d").unchecked_into(),
		),
		(
			hex!("44f3876fe4f653533c65e79461a476b8d6a107fb71b6ec0f3485bb53b4e7b842").unchecked_into(),
			hex!("281be34a71b661b257153e1145522fd0820cfff6a3601b40e7f85d3bc155240d").unchecked_into(),
		),
	]
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
			true,
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

	Ok(ChainSpec::from_genesis(
		"Canvas Testnet 1",
		"canvas_testnet1",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			testnet_authorities(),
			testnet_root(),
			vec![testnet_root()],
			true,
		),
		vec![
			"/ip4/35.233.19.96/tcp/30333/p2p/QmNvYhAZSBtahCqCXznYiq8e24Yes1GraPFYCc3DyA5f3z".parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/35.205.110.21/tcp/30333/p2p/QmPKFc9B2oeQFc5oxbNsRENwSYibzzafKmcHs9wBZCJH4U".parse()
				.expect("MultiaddrWithPeerId"),
		],
		None,
		Some("prc"),
		None,
		None
	))
}

fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool
) -> GenesisConfig {

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
					enable_println,
					..Default::default()
			},
		}),
	}
}
