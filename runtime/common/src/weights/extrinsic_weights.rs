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

pub mod constants {
	use frame_support::{
		parameter_types,
		weights::{constants::WEIGHT_REF_TIME_PER_NANOS, Weight},
	};

	parameter_types! {
		/// Executing a NO-OP `System::remarks` Extrinsic.
		pub const ExtrinsicBaseWeight: Weight = Weight::from_ref_time(WEIGHT_REF_TIME_PER_NANOS).saturating_mul(125_000);
	}

	#[cfg(test)]
	mod test_weights {
		use frame_support::weights::{
			constants::{
				ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_MICROS, WEIGHT_REF_TIME_PER_MILLIS,
			},
			Weight,
		};

		/// Checks that the weight exists and is sane.
		// NOTE: If this test fails but you are sure that the generated values are fine,
		// you can delete it.
		#[test]
		fn sane() {
			let w = ExtrinsicBaseWeight::get();

			// At least 10 µs.
			assert!(
				w.ref_time() >=
					Weight::from_ref_time(WEIGHT_REF_TIME_PER_MICROS)
						.saturating_mul(10)
						.ref_time(),
				"Weight should be at least 10 µs."
			);
			// At most 1 ms.
			assert!(
				w.ref_time() <= Weight::from_ref_time(WEIGHT_REF_TIME_PER_MILLIS).ref_time(),
				"Weight should be at most 1 ms."
			);
		}
	}
}
