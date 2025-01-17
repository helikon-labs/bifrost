// This file is part of Bifrost.

// Copyright (C) Liebi Technologies PTE. LTD.
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
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bifrost_vtoken_voting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bifrost-jenkins`, CPU: `Intel(R) Xeon(R) CPU E5-26xx v4`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_vtoken_voting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/vtoken-voting/src/weights.rs
// --template=./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_vtoken_voting.
pub trait WeightInfo {
	fn vote_new() -> Weight;
	fn vote_existing() -> Weight;
	fn unlock() -> Weight;
	fn remove_delegator_vote() -> Weight;
	fn kill_referendum() -> Weight;
	fn add_delegator() -> Weight;
	fn set_referendum_status() -> Weight;
	fn set_undeciding_timeout() -> Weight;
	fn set_vote_locking_period() -> Weight;
	fn notify_vote() -> Weight;
	fn notify_remove_delegator_vote() -> Weight;
	fn set_vote_cap_ratio() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: VtokenVoting UndecidingTimeout (r:1 w:0)
	/// Proof: VtokenVoting UndecidingTimeout (max_values: None, max_size: Some(26), added: 2501, mode: MaxEncodedLen)
	/// Storage: VtokenVoting DelegatorVote (r:2 w:1)
	/// Proof: VtokenVoting DelegatorVote (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Slp DelegatorsIndex2Multilocation (r:1 w:0)
	/// Proof Skipped: Slp DelegatorsIndex2Multilocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: Slp DelegatorLedgers (r:1 w:0)
	/// Proof Skipped: Slp DelegatorLedgers (max_values: None, max_size: None, mode: Measured)
	/// Storage: VtokenVoting PendingVotingInfo (r:1 w:1)
	/// Proof: VtokenVoting PendingVotingInfo (max_values: None, max_size: Some(117), added: 2592, mode: MaxEncodedLen)
	/// Storage: VtokenVoting ReferendumInfoFor (r:1 w:1)
	/// Proof: VtokenVoting ReferendumInfoFor (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: VtokenVoting VotingFor (r:1 w:1)
	/// Proof: VtokenVoting VotingFor (max_values: None, max_size: Some(13663), added: 16138, mode: MaxEncodedLen)
	/// Storage: VtokenVoting ClassLocksFor (r:1 w:1)
	/// Proof: VtokenVoting ClassLocksFor (max_values: None, max_size: Some(5162), added: 7637, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1271), added: 3746, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmInterface XcmWeightAndFee (r:1 w:0)
	/// Proof Skipped: XcmInterface XcmWeightAndFee (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	/// Proof Skipped: PolkadotXcm QueryCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VtokenVoting PendingReferendumInfo (r:0 w:1)
	/// Proof: VtokenVoting PendingReferendumInfo (max_values: None, max_size: Some(34), added: 2509, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm Queries (r:0 w:1)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `14356`
		//  Estimated: `17821`
		// Minimum execution time: 306_277_000 picoseconds.
		Weight::from_parts(319_464_000, 17821)
			.saturating_add(RocksDbWeight::get().reads(15_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: VtokenVoting UndecidingTimeout (r:1 w:0)
	/// Proof: VtokenVoting UndecidingTimeout (max_values: None, max_size: Some(26), added: 2501, mode: MaxEncodedLen)
	/// Storage: VtokenVoting DelegatorVote (r:2 w:1)
	/// Proof: VtokenVoting DelegatorVote (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Slp DelegatorsIndex2Multilocation (r:1 w:0)
	/// Proof Skipped: Slp DelegatorsIndex2Multilocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: Slp DelegatorLedgers (r:1 w:0)
	/// Proof Skipped: Slp DelegatorLedgers (max_values: None, max_size: None, mode: Measured)
	/// Storage: VtokenVoting PendingVotingInfo (r:1 w:1)
	/// Proof: VtokenVoting PendingVotingInfo (max_values: None, max_size: Some(117), added: 2592, mode: MaxEncodedLen)
	/// Storage: VtokenVoting ReferendumInfoFor (r:1 w:1)
	/// Proof: VtokenVoting ReferendumInfoFor (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: VtokenVoting VotingFor (r:1 w:1)
	/// Proof: VtokenVoting VotingFor (max_values: None, max_size: Some(13663), added: 16138, mode: MaxEncodedLen)
	/// Storage: VtokenVoting ClassLocksFor (r:1 w:1)
	/// Proof: VtokenVoting ClassLocksFor (max_values: None, max_size: Some(5162), added: 7637, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1271), added: 3746, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmInterface XcmWeightAndFee (r:1 w:0)
	/// Proof Skipped: XcmInterface XcmWeightAndFee (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	/// Proof Skipped: PolkadotXcm QueryCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm Queries (r:0 w:1)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5531`
		//  Estimated: `17128`
		// Minimum execution time: 274_492_000 picoseconds.
		Weight::from_parts(283_068_000, 17128)
			.saturating_add(RocksDbWeight::get().reads(15_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: VtokenVoting ReferendumInfoFor (r:1 w:0)
	/// Proof: VtokenVoting ReferendumInfoFor (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: VtokenVoting VoteLockingPeriod (r:1 w:0)
	/// Proof: VtokenVoting VoteLockingPeriod (max_values: None, max_size: Some(26), added: 2501, mode: MaxEncodedLen)
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: VtokenVoting VotingFor (r:1 w:1)
	/// Proof: VtokenVoting VotingFor (max_values: None, max_size: Some(13663), added: 16138, mode: MaxEncodedLen)
	/// Storage: VtokenVoting ClassLocksFor (r:1 w:1)
	/// Proof: VtokenVoting ClassLocksFor (max_values: None, max_size: Some(5162), added: 7637, mode: MaxEncodedLen)
	/// Storage: Tokens Locks (r:1 w:1)
	/// Proof: Tokens Locks (max_values: None, max_size: Some(1271), added: 3746, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2103`
		//  Estimated: `17128`
		// Minimum execution time: 165_014_000 picoseconds.
		Weight::from_parts(167_638_000, 17128)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `VtokenVoting::DelegatorVotes` (r:1 w:0)
	/// Proof: `VtokenVoting::DelegatorVotes` (`max_values`: None, `max_size`: Some(5136), added: 7611, mode: `MaxEncodedLen`)
	/// Storage: `VtokenVoting::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `VtokenVoting::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmInterface::XcmWeightAndFee` (r:1 w:0)
	/// Proof: `XcmInterface::XcmWeightAndFee` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	/// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `VtokenVoting::PendingRemoveDelegatorVote` (r:0 w:1)
	/// Proof: `VtokenVoting::PendingRemoveDelegatorVote` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	/// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_delegator_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1048`
		//  Estimated: `8601`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 8601)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: VtokenVoting ReferendumInfoFor (r:1 w:1)
	/// Proof: VtokenVoting ReferendumInfoFor (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	fn kill_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `370`
		//  Estimated: `3553`
		// Minimum execution time: 41_865_000 picoseconds.
		Weight::from_parts(46_801_000, 3553)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Slp DelegatorsIndex2Multilocation (r:1 w:0)
	/// Proof Skipped: Slp DelegatorsIndex2Multilocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: VtokenVoting DelegatorVote (r:1 w:1)
	/// Proof: VtokenVoting DelegatorVote (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	fn add_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `372`
		//  Estimated: `3837`
		// Minimum execution time: 51_783_000 picoseconds.
		Weight::from_parts(52_655_000, 3837)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: VtokenVoting ReferendumInfoFor (r:1 w:1)
	/// Proof: VtokenVoting ReferendumInfoFor (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn set_referendum_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `321`
		//  Estimated: `3553`
		// Minimum execution time: 41_679_000 picoseconds.
		Weight::from_parts(42_389_000, 3553)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: VtokenVoting UndecidingTimeout (r:0 w:1)
	/// Proof: VtokenVoting UndecidingTimeout (max_values: None, max_size: Some(26), added: 2501, mode: MaxEncodedLen)
	fn set_undeciding_timeout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 24_444_000 picoseconds.
		Weight::from_parts(25_058_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: VtokenVoting VoteLockingPeriod (r:0 w:1)
	/// Proof: VtokenVoting VoteLockingPeriod (max_values: None, max_size: Some(26), added: 2501, mode: MaxEncodedLen)
	fn set_vote_locking_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_145_000 picoseconds.
		Weight::from_parts(24_174_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: VtokenVoting PendingVotingInfo (r:1 w:0)
	/// Proof: VtokenVoting PendingVotingInfo (max_values: None, max_size: Some(117), added: 2592, mode: MaxEncodedLen)
	/// Storage: VtokenVoting PendingReferendumInfo (r:1 w:0)
	/// Proof: VtokenVoting PendingReferendumInfo (max_values: None, max_size: Some(34), added: 2509, mode: MaxEncodedLen)
	fn notify_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `361`
		//  Estimated: `3582`
		// Minimum execution time: 44_382_000 picoseconds.
		Weight::from_parts(44_901_000, 3582)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: VtokenVoting PendingRemoveDelegatorVote (r:1 w:0)
	/// Proof: VtokenVoting PendingRemoveDelegatorVote (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	fn notify_remove_delegator_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `3501`
		// Minimum execution time: 38_747_000 picoseconds.
		Weight::from_parts(39_364_000, 3501)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}

	fn set_vote_cap_ratio() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `3501`
		// Minimum execution time: 38_747_000 picoseconds.
		Weight::from_parts(39_364_000, 3501)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}
