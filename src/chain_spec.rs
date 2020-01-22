use sp_core::{Pair, Public, sr25519};
use paracon_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY, Signature,
	ContractsConfig, MILLICENTS,
};
use sp_consensus_aura::sr25519::{AuthorityId as AuraId};
use grandpa_primitives::{AuthorityId as GrandpaId};
use sc_service;
use sp_runtime::traits::{Verify, IdentifyAccount};
use hex_literal::hex;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Paracon Test net 1
	TestNet1,
}

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
pub fn get_authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn testnet_authorities() -> Vec<(AuraId, GrandpaId)> {
	use sp_core::crypto::UncheckedInto;
	vec![
		/// This is subkey inspect ($root//1//aura, $root//1//gran)
		(
			// 5DAvQj61HsAJ5mTm6SGKt64dG3vboeBPGZxiYwueuR4uVgFR
			hex!("30f9f64c0406e80be9ee39c5de3e8620f986b5ecfc0e2c19d065011fff768c0f").unchecked_into(),
			// 5Gxhe16P2X7ckpxYRkgFFBTq8S5YMD3CjXzqGhvL5oV3LkjH
			hex!("d898f5ae2cc29b9570560fa2c1e4001f01aad39a6e7225625dff94d68f83d30e").unchecked_into(),
		),
		/// This is subkey inspect ($root//2//aura, $root//2//gran)
		(
			// 5DAvQj61HsAJ5mTm6SGKt64dG3vboeBPGZxiYwueuR4uVgFR
			hex!("caa112263ca51f6f8bf4134e44bf9558dbd29a54f7c804773f87ec9a9a814b64").unchecked_into(),
			// 5Gxhe16P2X7ckpxYRkgFFBTq8S5YMD3CjXzqGhvL5oV3LkjH
			hex!("97ee8ceef59b4b96885a892a6af398afae163ecfea3ff93ac70ef6178a0a5eed").unchecked_into(),
		),
	]
}

pub fn testnet_root() -> AccountId {
	hex!("baa78c7154c7f82d6d377177e20bcab65d327eca0086513f9964f5a0f6bdad56").into()
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(vec![
					get_authority_keys_from_seed("Alice"),
				],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::TestNet1 => ChainSpec::from_genesis(
				"Paracon Testnet 1",
				"paracon_testnet1",
				|| testnet_genesis(
					testnet_authorities(),
					testnet_root(),
					vec![testnet_root()],
					true,
				),
				vec![],
				None,
				Some("prc"),
				None,
				None
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"" | "local" => Some(Alternative::TestNet1),
			_ => None,
		}
	}
}

fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool
) -> GenesisConfig {
    let mut contracts_config = ContractsConfig {
        current_schedule: Default::default(),
        gas_price: 1 * MILLICENTS,
    };
    contracts_config.current_schedule.enable_println = enable_println;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		contracts: Some(contracts_config),
	}
}
