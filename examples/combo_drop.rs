
use SafeManuallyDrop::ManuallyDrop;
use std::ops::Deref;

fn main() {
	assert_eq!(SafeManuallyDrop::core::flags::IS_SAFE_MODE, true);
	
	let data = vec![1, 2, 3, 4];
	let mut control_drop = ManuallyDrop::new(data);
		
	let _e = control_drop.deref();
	{
		unsafe {
			assert_eq!(control_drop.is_next_trig(), false);
			ManuallyDrop::drop(&mut control_drop);
			
			assert_eq!(control_drop.is_next_trig(), true);
			// <<-- PANIC
			ManuallyDrop::drop(&mut control_drop);
		}
	}	
}
