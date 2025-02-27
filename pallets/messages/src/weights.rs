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

//! Autogenerated weights for pallet_messages
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_messages
// --extrinsic
// *
// --chain=frequency-bench
// --execution
// wasm
// --heap-pages=4096
// --wasm-execution
// compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/messages/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
	rustdoc::all,
	missing_docs,
	unused_parens,
	unused_imports
)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_messages.
pub trait WeightInfo {
	fn add_onchain_message(n: u32, ) -> Weight;
	fn add_ipfs_message() -> Weight;
}

/// Weights for pallet_messages using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: Messages Messages (r:1 w:1)
	fn add_onchain_message(n: u32, ) -> Weight {
		Weight::from_ref_time(139_240_171 as u64)
			// Standard Error: 41
			.saturating_add(Weight::from_ref_time(1_530 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Messages Messages (r:1 w:1)
	fn add_ipfs_message() -> Weight {
		Weight::from_ref_time(131_967_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: Messages Messages (r:1 w:1)
	fn add_onchain_message(n: u32, ) -> Weight {
		Weight::from_ref_time(139_240_171 as u64)
			// Standard Error: 41
			.saturating_add(Weight::from_ref_time(1_530 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Messages Messages (r:1 w:1)
	fn add_ipfs_message() -> Weight {
		Weight::from_ref_time(131_967_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
