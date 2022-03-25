
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
		println!("ManuallyDrop has no protections by default,");
		println!("ManuallyDrop will be the same as in std.");
		println!("Or use (AlwaysSafeManuallyDrop, PanicManuallyDrop, HookManuallyDrop, CounterManuallyDrop) specific data types with specific behavior.");
	}
}
