use std::ops::Deref;
use SafeManuallyDrop::AlwaysSafePanicManuallyDrop as ManuallyDrop;

fn main() {
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
			thread 'main' panicked at 'Undefined behavior when using
			ManuallyDrop(combo_replace_manudropstate), instead of the expected default
			state, the current state: DropModeTrig.'.
		*/
		ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
	}
}
