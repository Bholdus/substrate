// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_ranked_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /Users/gav/Core/substrate/target/release/substrate
// benchmark
// pallet
// --pallet
// pallet-ranked-collective
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --output=../../../frame/ranked-collective/src/weights.rs
// --template=../../../.maintain/frame-weight-template.hbs
// --header=../../../HEADER-APACHE2
// --record-proof

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ranked_collective.
pub trait WeightInfo {
	fn add_member() -> Weight;
	fn remove_member(r: u32, ) -> Weight;
	fn promote_member(r: u32, ) -> Weight;
	fn demote_member(r: u32, ) -> Weight;
	fn vote() -> Weight;
	fn cleanup_poll(n: u32, ) -> Weight;
}

/// Weights for pallet_ranked_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	fn add_member() -> Weight {
		(11_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	fn remove_member(r: u32, ) -> Weight {
		(16_855_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((8_107_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	fn promote_member(r: u32, ) -> Weight {
		(11_936_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((9_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	fn demote_member(r: u32, ) -> Weight {
		(17_582_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((142_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedCollective Members (r:1 w:0)
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:1)
	// Storage: RankedCollective Voting (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote() -> Weight {
		(22_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:0)
	// Storage: RankedCollective Voting (r:0 w:1)
	fn cleanup_poll(n: u32, ) -> Weight {
		(6_188_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((867_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	fn add_member() -> Weight {
		(11_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	fn remove_member(r: u32, ) -> Weight {
		(16_855_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((8_107_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	fn promote_member(r: u32, ) -> Weight {
		(11_936_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((9_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	fn demote_member(r: u32, ) -> Weight {
		(17_582_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((142_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedCollective Members (r:1 w:0)
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:1)
	// Storage: RankedCollective Voting (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote() -> Weight {
		(22_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:0)
	// Storage: RankedCollective Voting (r:0 w:1)
	fn cleanup_poll(n: u32, ) -> Weight {
		(6_188_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((867_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
}