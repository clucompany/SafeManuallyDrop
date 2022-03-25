
use SafeManuallyDrop::PanicManuallyDrop as ManuallyDrop;
use std::ops::Deref;

fn main() {
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
}
