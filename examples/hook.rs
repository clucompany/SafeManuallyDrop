
use SafeManuallyDrop::HookManuallyDrop as ManuallyDrop;
use std::ops::Deref;

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
	
	unsafe {
		assert_eq!(data.is_next_trig(), false); // VALID
		ManuallyDrop::drop(&mut data); // VALID
		assert_eq!(data.is_next_trig(), true); // VALID
		
		// <<-- HOOK
		ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
	}
}
