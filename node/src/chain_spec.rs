// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use canvas_runtime::{AccountId, AuraId, Signature};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<canvas_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CAN".into());
	properties.insert("tokenDecimals".into(), 12.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				id,
			)
		},
		vec![],
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CAN".into());
	properties.insert("tokenDecimals".into(), 12.into());

	ChainSpec::from_genesis(
		// Name
		"Canvas",
		// ID
		"canvas-rococo",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("canvas-local"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn rococo_testnet_root() -> AccountId {
	hex!("baa78c7154c7f82d6d377177e20bcab65d327eca0086513f9964f5a0f6bdad56").into()
}

pub fn rococo_testnet_authorities() -> Vec<AuraId> {
	use sp_core::crypto::UncheckedInto;

	// ./scripts/prepare-test-net.sh 2
	let initial_authorities: Vec<(AccountId, AccountId, AuraId)> = vec![
		(
			// 5Dkp2xwztRtRkXdmLDJUm8wvdHWSApgYgJY3rB5brKZkWJPM
			hex!["4ad2d854d1bc5bbfdf93b689cb1aa3986684126c2b16d86924019de6798b0971"].into(),
			// 5DvnJs3jM7sJkgfpVDxwu9GNmpFyzJSn2jG9w1hxBBWwqi92
			hex!["526d7d3d5357d20e4cd75d1a452bd5b4903caf160f55d6bd19807efbdf165319"].into(),
			// 5GKFbTTgrVS4Vz1UWWHPqMZQNFWZtqo7H2KpCDyYhEL3aS26
			hex!["bc09354c12c054c8f6b3da208485eacec4ac648bad348895273b37bab5a0937c"]
				.unchecked_into(),
		),
		(
			// 5Gnadix7NJH2K7Akr5hVKyXqtYqTsQJbXnKFFt274MsG8n7h
			hex!["d0e0ece66fb861b82383e85326a2e179316021105492820ca544ea8743620b59"].into(),
			// 5FADgcPNMtcLtrmd16rZk6HJfDWqtEDT4N3jMSqtFo8tJhRr
			hex!["88e95527362f479ebf30502db2f7d88329e034f5d77aed585042c548fa93ae01"].into(),
			// 5EPRJHm2GpABVWcwnAujcrhnrjFZyDGd5TwKFzkBoGgdRyv2
			hex!["66be63b7bcbfb91040e5248e2d1ceb822cf219c57848c5924ffa3a1f8e67ba72"]
				.unchecked_into(),
		),
	];
	initial_authorities.into_iter().map(|(_, _, aura_id)| aura_id).collect()
}

pub fn rococo_testnet_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CAN".into());
	properties.insert("tokenDecimals".into(), 12.into());

	ChainSpec::from_genesis(
		// Name
		"Canvas",
		// ID
		"canvas-rococo",
		ChainType::Live,
		move || {
			testnet_genesis(
				rococo_testnet_root(),
				rococo_testnet_authorities(),
				// Warning: The configuration for a production chain should not contain
				// any endowed accounts here, otherwise it'll be minting extra native tokens
				// from the relay chain on the parachain.
				vec![
					rococo_testnet_root(),
					// AccountId of an account which `ink-waterfall` uses for automated testing
					hex!("0e47e2344d523c3cc5c34394b0d58b9a4200e813a038e6c5a6163cc07d70b069").into(),
				],
				id,
			)
		},
		// Bootnodes
		vec![
			"/ip4/34.90.191.237/tcp/30333/p2p/12D3KooWKg3Rpxcr9oJ8n6khoxpGKWztCZydtUZk2cojHqnfLrpj"
				.parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/35.204.68.28/tcp/30333/p2p/12D3KooWPEXYrz8tHU3nDtPoPw4V7ou5dzMEWSTuUj7vaWiYVAVh"
				.parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/34.90.139.15/tcp/30333/p2p/12D3KooWEVU8AFNary4nP4qEnEcwJaRuy59Wefekzdu9pKbnVEhk"
				.parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/35.204.99.97/tcp/30333/p2p/12D3KooWP6pV3ZmcXzGDjv8ZMgA6nZxfAKDxSz4VNiLx6vVCQgJX"
				.parse()
				.expect("MultiaddrWithPeerId"),
		],
		// Telemetry
		None,
		// Protocol ID
		Some("canvas-rococo"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> canvas_runtime::GenesisConfig {
	canvas_runtime::GenesisConfig {
		system: canvas_runtime::SystemConfig {
			code: canvas_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: canvas_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		sudo: canvas_runtime::SudoConfig { key: root_key },
		parachain_info: canvas_runtime::ParachainInfoConfig { parachain_id: id },
		aura: canvas_runtime::AuraConfig { authorities: initial_authorities },
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
