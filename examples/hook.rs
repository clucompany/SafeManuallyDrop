
use std::ops::Deref;

// For better performance, we recommend using AutoSafeHookManuallyDrop instead 
// of AlwaysSafeHookManuallyDrop. The AutoSafeHookManuallyDrop type depends on 
// the type of build, debug or release will be with the safe or insecure version
// of ManuallyDrop.
use SafeManuallyDrop::AlwaysSafeHookManuallyDrop as ManuallyDrop;

fn main() {
	unsafe {
		ManuallyDrop::set_hook(|args| {
			println!("!!!{:?}", args);
			
			for _ in 0..3 {
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}
			
			println!("exit");
			std::process::exit(0x0100);
		});
	}
	
	let mut data = ManuallyDrop::new(vec![1, 2, 3, 4]);
	println!("data: {:?}", data.deref());
	
	#[allow(unused_unsafe)] // to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
	unsafe {
		assert_eq!(data.is_next_trig(), false); // VALID
		ManuallyDrop::drop(&mut data); // VALID
		assert_eq!(data.is_next_trig(), true); // VALID
		
		// <<-- HOOK
		ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
	}
}
