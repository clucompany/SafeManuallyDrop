

#[cfg(feature = "support_count_trig")]
use SafeManuallyDrop::AutoSafeCounterManuallyDrop as ManuallyDrop;

#[cfg(not(feature = "support_count_trig"))]
use SafeManuallyDrop::ManuallyDrop;

use std::ops::Deref;

#[allow(unreachable_code)]
fn main() {
	#[cfg(not(feature = "support_count_trig"))] {
		println!("To run the example, a build with feature: support_count_trig is required,");
		println!("exp: cargo run --example counter --all-features");
		println!("end.");
		
		return;
	}
	
	let mut data = ManuallyDrop::new(&[1, 2, 3, 4]);
	println!("data: {:?}", data.deref());
	
	#[allow(unused_unsafe)] // feature !always_compatible_stdapi
	unsafe {
		assert_eq!(data.is_next_trig(), false); // VALID
		// =================
		// !!! ATTENTION !!!
		// =================
		// 
		ManuallyDrop::drop(&mut data); // VALID
		assert_eq!(data.is_next_trig(), true); // VALID
		
		ManuallyDrop::drop(&mut data); // <<-- INVALID BEH, COUNTER += 1 (=1), COMBO DROP
	}
	
	println!("data: {:?}", &data); // <<-- INVALID BEH, COUNTER += 1 (=2)
	#[allow(unused_unsafe)] // to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
	let _data2 = unsafe { // <<-- INVALID BEH, COUNTER += 1 (=3)
		ManuallyDrop::take(&mut data)
	};
	
	#[cfg(feature = "support_count_trig")]
	assert_eq!(ManuallyDrop::get_count_trig_events(), 3);
}
