// This file is part of Bifrost.

// Copyright (C) 2019-2021 Liebi Technologies (UK) Ltd.
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

pub use codec::Encode;
use cumulus_test_relay_sproof_builder::RelayStateSproofBuilder;
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::app_crypto::sp_core::keccak_256,
	traits::{
		schedule::DispatchTime, Currency, GenesisBuild, OnFinalize, OnInitialize, OriginTrait,
		ValidatorSet,
	},
	weights::constants::*,
};
use frame_system::RawOrigin;
pub use node_primitives::*;
pub use orml_traits::{Change, GetByKey, MultiCurrency};
pub use sp_runtime::{
	traits::{AccountIdConversion, BadOrigin, Convert, Zero},
	DispatchError, DispatchResult, FixedPointNumber, MultiAddress,
};
use xcm::{
	opaque::v0::prelude::{BuyExecution, DepositAsset},
	v0::{
		ExecuteXcm,
		Junction::{self, *},
		MultiAsset,
		MultiLocation::*,
		NetworkId, Outcome, Xcm,
	},
};
pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];

#[cfg(feature = "with-asgard-runtime")]
pub use asgard_imports::*;
use xcm::v0::MultiLocation;

#[cfg(feature = "with-asgard-runtime")]
mod asgard_imports {
	pub use asgard_runtime::{
		create_x2_parachain_multilocation, AccountId, Balance, Balances, BifrostCrowdloanId,
		BlockNumber, Call, Currencies, CurrencyId, Event, ExistentialDeposit, ExistentialDeposits,
		NativeCurrencyId, Origin, OriginCaller, ParachainInfo, ParachainSystem, Perbill, Proxy,
		RelayCurrencyId, RelaychainSovereignSubAccount, Runtime, Salp, Scheduler, Session,
		SlotLength, System, Tokens, TreasuryPalletId, Utility, Vesting, XTokens, XcmConfig,
	};
	pub use bifrost_runtime_common::constants::{currency::*, time::*};
	pub use frame_support::parameter_types;
	pub use sp_runtime::traits::AccountIdConversion;
}

fn run_to_block(n: u32) {
	while System::block_number() < n {
		Scheduler::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		Scheduler::on_initialize(System::block_number());
		Scheduler::on_initialize(System::block_number());
		Session::on_initialize(System::block_number());
	}
}

fn set_relaychain_block_number(number: BlockNumber) {
	ParachainSystem::on_initialize(number);

	let (relay_storage_root, proof) =
		RelayStateSproofBuilder::default().into_state_root_and_proof();

	assert_ok!(ParachainSystem::set_validation_data(
		Origin::none(),
		cumulus_primitives_parachain_inherent::ParachainInherentData {
			validation_data: cumulus_primitives_core::PersistedValidationData {
				parent_head: Default::default(),
				relay_parent_number: number,
				relay_parent_storage_root: relay_storage_root,
				max_pov_size: Default::default(),
			},
			relay_chain_state: proof,
			downward_messages: Default::default(),
			horizontal_messages: Default::default(),
		}
	));
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![BifrostCrowdloanId::get().into_account()]
}

pub struct ExtBuilder {
	balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: vec![] }
	}
}

impl ExtBuilder {
	pub fn balances(mut self, balances: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		let native_currency_id = NativeCurrencyId::get();
		let existential_deposit = ExistentialDeposit::get();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: self
				.balances
				.clone()
				.into_iter()
				.filter(|(_, currency_id, _)| *currency_id == native_currency_id)
				.map(|(account_id, _, initial_balance)| (account_id, initial_balance))
				.chain(get_all_module_accounts().iter().map(|x| (x.clone(), existential_deposit)))
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			balances: self
				.balances
				.into_iter()
				.filter(|(_, currency_id, _)| *currency_id != native_currency_id)
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_membership::GenesisConfig::<Runtime, pallet_membership::Instance1> {
			members: vec![],
			phantom: Default::default(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		<parachain_info::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
			&parachain_info::GenesisConfig { parachain_id: 2001.into() },
			&mut t,
		)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn sanity_check_weight_per_time_constants_are_as_expected() {
	assert_eq!(WEIGHT_PER_SECOND, 1_000_000_000_000);
	assert_eq!(WEIGHT_PER_MILLIS, WEIGHT_PER_SECOND / 1000);
	assert_eq!(WEIGHT_PER_MICROS, WEIGHT_PER_MILLIS / 1000);
	assert_eq!(WEIGHT_PER_NANOS, WEIGHT_PER_MICROS / 1000);
}

#[test]
fn parachain_subaccounts_are_unique() {
	ExtBuilder::default().build().execute_with(|| {
		let parachain: AccountId = ParachainInfo::parachain_id().into_account();
		assert_eq!(
			parachain,
			hex_literal::hex!["70617261d1070000000000000000000000000000000000000000000000000000"]
				.into()
		);

		assert_eq!(RelaychainSovereignSubAccount::get(), create_x2_parachain_multilocation(0));

		assert_eq!(
			create_x2_parachain_multilocation(0),
			MultiLocation::X2(
				Junction::Parent,
				Junction::AccountId32 {
					network: NetworkId::Any,
					id: [
						90, 83, 115, 109, 142, 150, 241, 192, 7, 207, 13, 99, 10, 207, 82, 9, 178,
						6, 17, 97, 122, 242, 60, 233, 36, 200, 226, 83, 40, 235, 93, 40
					]
				}
			)
		);
	});
}

#[test]
fn salp() {
	ExtBuilder::default()
		.balances(vec![
			(AccountId::from(ALICE), RelayCurrencyId::get(), 100 * DOLLARS),
			(AccountId::from(BOB), RelayCurrencyId::get(), 100 * DOLLARS),
		])
		.build()
		.execute_with(|| {
			assert_ok!(Salp::create(RawOrigin::Root.into(), 3_000, 1_000, 1, SlotLength::get()));
			assert_ok!(Salp::funds(3_000).ok_or(()));
			assert_eq!(Salp::current_trie_index(), 1);
		});
}

#[test]
fn bancor() {}