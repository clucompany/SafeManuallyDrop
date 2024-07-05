// This example requires `support_abort_trig` support to work.

#[cfg(feature = "support_abort_trig")]
use SafeManuallyDrop::AlwaysSafeAbortManuallyDrop as ManuallyDrop;

#[cfg(not(feature = "support_abort_trig"))]
use SafeManuallyDrop::ManuallyDrop;

use std::ops::Deref;

#[allow(unreachable_code)]
fn main() {
	#[cfg(not(feature = "support_abort_trig"))]
	{
		println!("To run the example, a build with feature: support_abort_trig is required,");
		println!("exp: cargo run --example abort --all-features");
		println!("end.");

		return;
	}

	let mut data = ManuallyDrop::new(vec![1, 2, 3, 4]);
	println!("data: {:?}", data.deref());

	// to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
	#[allow(unused_unsafe)]
	unsafe {
		assert!(!data.is_next_trig()); // VALID
		ManuallyDrop::drop(&mut data); // VALID
		assert!(data.is_next_trig()); // VALID

		// <<-- PANIC
		/*
			Undefined behavior when using ManuallyDrop(combo_replace_manudropstate),
			instead of the expected default state, the current state: DropModeTrig.

			Emergency stop.
		*/
		ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
	}
}
