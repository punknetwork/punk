// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use sc_chain_spec::ChainSpecExtension;
use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use serde::{Serialize, Deserialize};
use node_runtime::{
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig, CouncilConfig,
	DemocracyConfig,GrandpaConfig, ImOnlineConfig, SessionConfig, SessionKeys, StakerStatus,
	StakingConfig, ElectionsConfig, IndicesConfig, SocietyConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, wasm_binary_unwrap,
};
use node_runtime::Block;
use node_runtime::constants::currency::*;
use sc_service::ChainType;
use hex_literal::hex;
use sc_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use sp_consensus_babe::{AuthorityId as BabeId};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};

pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<
	GenesisConfig,
	Extensions,
>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		 // 5CdGS3T9AYdA6FXZSW4ByPujTX3pzV769NgejPTY2BRW9S8T
		 hex!["18d582f6a45f8a80314a455a94dabff25e5690e337c402846bb8b0673012d873"].into(),
		 // 5G4iYCPqdcib1TpvozbJ8vN656W8jTqUs6U8qG6uQrm792DJ
		 hex!["b0f33a605f1f4b232e7a40bb605c626864921d5a1b589479f0e81135d99917db"].into(),
		 // 5EGQvzp9DnVzLejZGrgaJhXQDN6sXiQTJ6vPYxawoXr1ZfTX
		 hex!["61666f973951c79e16f460bcb6e1e3d101cc89092b71fd3113b10742e7010f60"].unchecked_into(),
		 // 5H11ZoyFXYdC99Z7ChLYWveoXJYkTqnQL1eyYWRTTJVJjBXp
		 hex!["da5bccf4c4caf2e6b73069d1164b8e56810f2751cf36fbe6adb27ba2a2290568"].unchecked_into(),
		 // 5EGQvzp9DnVzLejZGrgaJhXQDN6sXiQTJ6vPYxawoXr1ZfTX
		 hex!["61666f973951c79e16f460bcb6e1e3d101cc89092b71fd3113b10742e7010f60"].unchecked_into(),
		 // 5H11ZoyFXYdC99Z7ChLYWveoXJYkTqnQL1eyYWRTTJVJjBXp
		 hex!["da5bccf4c4caf2e6b73069d1164b8e56810f2751cf36fbe6adb27ba2a2290568"].unchecked_into(),
	 ),(
		 // 5Co5kaRnLrhiiUmW7AxRKuEpQCLgMkCUzosAEuNWi43JfPU5
		 hex!["205203146b2f389bb5115464aa900d863c12cef21e87f118ea1d278a48d64e6a"].into(),
		 // 5FpQnTTqRrgUfRK2hpzoD2T9qa84z44KzT3Um9V2tewnM9W3
		 hex!["a60a06cf95be4a46df291f2e62c101cf4f42e38b550480aeeca65e5885f84154"].into(),
		 // 5CPH7b8mNWcw3TTgq5zshbtaEUAgRxDVEtrGdL6GJsqoSUGB
		 hex!["0e2a5f04c1cfeb3b4f32bf9d42d79750df55b011d1c6da3fc8bd173fe7aa028d"].unchecked_into(),
		 // 5F6Z6HDPBXXymYUmgL7gnap2WDrSsB4iPztHHnHRVJUVjnn3
		 hex!["861daf3a155596ba14f26c307b6c30de262cf705d2d4197598450c00d6ce4b37"].unchecked_into(),
		 // 5CPH7b8mNWcw3TTgq5zshbtaEUAgRxDVEtrGdL6GJsqoSUGB
		 hex!["0e2a5f04c1cfeb3b4f32bf9d42d79750df55b011d1c6da3fc8bd173fe7aa028d"].unchecked_into(),
		 // 5F6Z6HDPBXXymYUmgL7gnap2WDrSsB4iPztHHnHRVJUVjnn3
		 hex!["861daf3a155596ba14f26c307b6c30de262cf705d2d4197598450c00d6ce4b37"].unchecked_into(),
	 ),(
		 // 5EcVaP9u7vTdpYkyeTNv9FpdRyp6WdxPhVkZMGJrsAoRoJrw
		 hex!["70b6faa087700c169b626d376373fdf6d0390e7035e9a14d3d01cd75f1cc8b5b"].into(),
		 // 5FWmgGfdsgewRG4Z6CfM7Zprwwzzc72u4X9q6PewUdhQ5qHF
		 hex!["9895f14d54582f5ef442803153d677d13ae5e94ff01ac5994003d27d95e0b356"].into(),
		 // 5CGwLFrQqaATppAADLVwvTptvL21HbqHcKnqvMCxijvtn8PQ
		 hex!["09544ed6677503fda933af86886772f527fbb303090645c3c5130874e6111eda"].unchecked_into(),
		 // 5FP8m4g4YpFa7NWqoXAKDkxxF9J1NixFohcxdEadDastq4KF
		 hex!["92c2f32963c50d4b9e4f30cb127ee84325816e37d218dce7c961a01896e6c066"].unchecked_into(),
		 // 5CGwLFrQqaATppAADLVwvTptvL21HbqHcKnqvMCxijvtn8PQ
		 hex!["09544ed6677503fda933af86886772f527fbb303090645c3c5130874e6111eda"].unchecked_into(),
		 // 5FP8m4g4YpFa7NWqoXAKDkxxF9J1NixFohcxdEadDastq4KF
		 hex!["92c2f32963c50d4b9e4f30cb127ee84325816e37d218dce7c961a01896e6c066"].unchecked_into(),
	 )];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5CdGS3T9AYdA6FXZSW4ByPujTX3pzV769NgejPTY2BRW9S8T
		"5ec0e36b5b1e5c16ebf3c61cc655ed9da2e43304f92f52b236707ea15b6fc93c"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		root_key,
		Some(endowed_accounts),
		false,
	)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Rock N\'Roll",
		"Rock N\'Roll",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
			.expect("Staging telemetry url is valid; qed")),
		Some("TPunk"),
		Some(json!({"tokenDecimals": 18, "tokenSymbol": "TPunk", "ss58Format": 101}).as_object()
			.expect("network properties generation can not fail; qed")
			.to_owned()),
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	enable_println: bool,
) -> GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
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
		]
	});
	initial_authorities.iter().for_each(|x|
		if !endowed_accounts.contains(&x.0) {
			endowed_accounts.push(x.0.clone())
		}
	);

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;

	GenesisConfig {
		frame_system: SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|x| (x, ENDOWMENT))
				.collect()
		},
		pallet_indices: IndicesConfig {
			indices: vec![],
		},
		pallet_session: SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		pallet_staking: StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		},
		pallet_democracy: DemocracyConfig::default(),
		pallet_elections_phragmen: ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		},
		pallet_collective_Instance1: CouncilConfig::default(),
		pallet_collective_Instance2: TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		},
		pallet_contracts: ContractsConfig {
			// println should only be enabled on development chains
			current_schedule: pallet_contracts::Schedule::default()
				.enable_println(enable_println),
		},
		pallet_sudo: SudoConfig {
			key: root_key,
		},
		pallet_babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(node_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		pallet_im_online: ImOnlineConfig {
			keys: vec![],
		},
		pallet_authority_discovery: AuthorityDiscoveryConfig {
			keys: vec![],
		},
		pallet_grandpa: GrandpaConfig {
			authorities: vec![],
		},
		pallet_membership_Instance1: Default::default(),
		pallet_treasury: Default::default(),
		pallet_society: SocietyConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			pot: 0,
			max_members: 999,
		},
		pallet_vesting: Default::default(),
		pallet_gilt: Default::default(),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, new_light_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
			false,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sc_service_test::connectivity(
			integration_test_config_with_two_authorities(),
			|config| {
				let NewFullBase { task_manager, client, network, transaction_pool, .. }
					= new_full_base(config,|_, _| ())?;
				Ok(sc_service_test::TestNetComponents::new(task_manager, client, network, transaction_pool))
			},
			|config| {
				let (keep_alive, _, client, network, transaction_pool) = new_light_base(config)?;
				Ok(sc_service_test::TestNetComponents::new(keep_alive, client, network, transaction_pool))
			}
		);
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		staging_testnet_config().build_storage().unwrap();
	}
}
