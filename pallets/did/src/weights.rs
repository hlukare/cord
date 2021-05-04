// Copyright 2019-2021 Dhiway.
// This file is part of CORD Platform.

// derived from kilt project

//! Generation of weight files for benchmarking.


//! //! Autogenerated weights for pallet_did


// Executed Command:
// ./target/release/cord
// benchmark
// --chain=dev
// --execution=wasm
// --pallet=pallet_did
// --extrinsic=*
// --steps=20
// --repeat=10
// --output=./pallets/did/src/weights.rs
// --template=./.maintain/weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_did.
pub trait WeightInfo {
	fn anchor() -> Weight;
	fn remove() -> Weight;
}

/// Weights for pallet_did using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn anchor() -> Weight {
		(341_153_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove() -> Weight {
		(274_549_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn anchor() -> Weight {
		(341_153_000_u64)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove() -> Weight {
		(274_549_000_u64)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}