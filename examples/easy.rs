
use SafeManuallyDrop::ManuallyDrop;
use std::ops::Deref;

fn main() {
	/*
		ManuallyDrop - Depending on the build flag, a protected version of ManuallyDrop 
		or an unprotected version of ManuallyDrop with a default trigger. 
	*/
	if ManuallyDrop::is_safe_mode() {
		// ManuallyDrop is protected, let's do the standard behavior ManuallyDrop
		// but at the end we'll make the undefined behavior ManuallyDrop.
		
		// 
		let mut data = ManuallyDrop::new(vec![1, 2, 3, 4]);
		println!("data: {:?}", data.deref());
		
		#[allow(unused_unsafe)] // to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
		unsafe {
			assert_eq!(data.is_next_trig(), false); // VALID
			ManuallyDrop::drop(&mut data); // VALID
			assert_eq!(data.is_next_trig(), true); // VALID
			
			// <<-- PANIC
			/*
				thread 'main' panicked at 'Undefined behavior when using 
				ManuallyDrop(combo_replace_manudropstate), instead of the expected default 
				state, the current state: DropModeTrig.', src/core/trig/hook.rs:14:5
			*/
			ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
		}
	}else {
		println!("#[0] ManuallyDrop is an alias for AutoSafeManuallyDrop, ");
		println!("#[1] ManuallyDrop in the release build has no protection by default,");
		println!("#[2] if ManuallyDrop is not protected it will be the same as in std.");
		println!("#[3] To run the protected version, use `cargo run --example easy` or ");
		println!("`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=\"true\" cargo run --example easy --release`");
		println!();
		println!("Or use concrete types instead of auto (AutoSafeManuallyDrop, AutoSafePanicManuallyDrop, AutoSafeHookManuallyDrop, AutoSafeCounterManuallyDrop, AlwaysSafeManuallyDrop, AlwaysSafePanicManuallyDrop, AlwaysSafeHookManuallyDrop, AlwaysSafeCounterManuallyDrop) specific data types with specific behavior.");
	}
}
