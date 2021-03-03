// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-03-03, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/staking/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_staking.
pub trait WeightInfo {
	fn bond() -> Weight;
	fn bond_extra() -> Weight;
	fn unbond() -> Weight;
	fn withdraw_unbonded_update(s: u32, ) -> Weight;
	fn withdraw_unbonded_kill(s: u32, ) -> Weight;
	fn validate() -> Weight;
	fn kick(k: u32, ) -> Weight;
	fn nominate(n: u32, ) -> Weight;
	fn chill() -> Weight;
	fn set_payee() -> Weight;
	fn set_controller() -> Weight;
	fn set_validator_count() -> Weight;
	fn force_no_eras() -> Weight;
	fn force_new_era() -> Weight;
	fn force_new_era_always() -> Weight;
	fn set_invulnerables(v: u32, ) -> Weight;
	fn force_unstake(s: u32, ) -> Weight;
	fn cancel_deferred_slash(s: u32, ) -> Weight;
	fn payout_stakers_dead_controller(n: u32, ) -> Weight;
	fn payout_stakers_alive_staked(n: u32, ) -> Weight;
	fn rebond(l: u32, ) -> Weight;
	fn set_history_depth(e: u32, ) -> Weight;
	fn reap_stash(s: u32, ) -> Weight;
	fn new_era(v: u32, n: u32, ) -> Weight;
	fn submit_solution_better(v: u32, n: u32, a: u32, w: u32, ) -> Weight;
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight;
	fn get_npos_targets(v: u32, ) -> Weight;
}

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn bond() -> Weight {
		(82_888_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn bond_extra() -> Weight {
		(66_676_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn unbond() -> Weight {
		(60_847_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(61_940_000 as Weight)
			// Standard Error: 0
			.saturating_add((73_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		(98_008_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_916_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn validate() -> Weight {
		(21_018_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn kick(k: u32, ) -> Weight {
		(28_311_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((19_701_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	fn nominate(n: u32, ) -> Weight {
		(34_340_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((5_879_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn chill() -> Weight {
		(20_287_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn set_payee() -> Weight {
		(14_044_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_controller() -> Weight {
		(30_776_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_validator_count() -> Weight {
		(2_390_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_no_eras() -> Weight {
		(2_682_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_new_era() -> Weight {
		(2_662_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_new_era_always() -> Weight {
		(2_667_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_invulnerables(v: u32, ) -> Weight {
		(2_820_000 as Weight)
			// Standard Error: 0
			.saturating_add((35_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_unstake(s: u32, ) -> Weight {
		(66_791_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_911_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		(5_937_192_000 as Weight)
			// Standard Error: 391_000
			.saturating_add((34_816_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		(142_187_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((54_299_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		(161_199_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((69_141_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	fn rebond(l: u32, ) -> Weight {
		(42_500_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((83_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_history_depth(e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 69_000
			.saturating_add((34_842_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	fn reap_stash(s: u32, ) -> Weight {
		(69_772_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_913_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn new_era(v: u32, n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 883_000
			.saturating_add((603_316_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 44_000
			.saturating_add((85_313_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(13 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn submit_solution_better(v: u32, n: u32, a: u32, w: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 64_000
			.saturating_add((1_505_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 25_000
			.saturating_add((682_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 64_000
			.saturating_add((78_804_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 134_000
			.saturating_add((8_484_000 as Weight).saturating_mul(w as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(w as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 113_000
			.saturating_add((29_967_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 113_000
			.saturating_add((69_931_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_540_000
			.saturating_add((26_303_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
	}
	fn get_npos_targets(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 33_000
			.saturating_add((11_344_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn bond() -> Weight {
		(82_888_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn bond_extra() -> Weight {
		(66_676_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn unbond() -> Weight {
		(60_847_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(61_940_000 as Weight)
			// Standard Error: 0
			.saturating_add((73_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		(98_008_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_916_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn validate() -> Weight {
		(21_018_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn kick(k: u32, ) -> Weight {
		(28_311_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((19_701_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	fn nominate(n: u32, ) -> Weight {
		(34_340_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((5_879_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn chill() -> Weight {
		(20_287_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn set_payee() -> Weight {
		(14_044_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_controller() -> Weight {
		(30_776_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_validator_count() -> Weight {
		(2_390_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_no_eras() -> Weight {
		(2_682_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_new_era() -> Weight {
		(2_662_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_new_era_always() -> Weight {
		(2_667_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_invulnerables(v: u32, ) -> Weight {
		(2_820_000 as Weight)
			// Standard Error: 0
			.saturating_add((35_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_unstake(s: u32, ) -> Weight {
		(66_791_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_911_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		(5_937_192_000 as Weight)
			// Standard Error: 391_000
			.saturating_add((34_816_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		(142_187_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((54_299_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		(161_199_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((69_141_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	fn rebond(l: u32, ) -> Weight {
		(42_500_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((83_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_history_depth(e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 69_000
			.saturating_add((34_842_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	fn reap_stash(s: u32, ) -> Weight {
		(69_772_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_913_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn new_era(v: u32, n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 883_000
			.saturating_add((603_316_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 44_000
			.saturating_add((85_313_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn submit_solution_better(v: u32, n: u32, a: u32, w: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 64_000
			.saturating_add((1_505_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 25_000
			.saturating_add((682_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 64_000
			.saturating_add((78_804_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 134_000
			.saturating_add((8_484_000 as Weight).saturating_mul(w as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((4 as Weight).saturating_mul(a as Weight)))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(w as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 113_000
			.saturating_add((29_967_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 113_000
			.saturating_add((69_931_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 1_540_000
			.saturating_add((26_303_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
	}
	fn get_npos_targets(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 33_000
			.saturating_add((11_344_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
}
