

/// Build test data
#[inline(never)]
fn build_new_test_vec() -> Vec<String> {
	let mut vec = Vec::new();
	vec.push("test".into());
	vec.push("test2".into());
	vec.push("test3".into());
	
	vec
}

mod panic_test_methods {
	use SafeManuallyDrop::ManuallyDrop;
	use core::ops::Deref;
	use super::build_new_test_vec;
	
	
	/// PANIC METHOD #1
	/// 1. COMBO DROP
	#[inline(never)]
	pub (crate) fn test_combo_drop(data: Vec<String>) {
		let mut control_drop = ManuallyDrop::new(data);
		
		let _e = control_drop.deref();
		{
			unsafe {
				ManuallyDrop::drop(&mut control_drop);
				
				assert_eq!(control_drop.is_maybe_next_panic(), true);
				// <<-- PANIC
				ManuallyDrop::drop(&mut control_drop);
			}
		}
	}
	
	/// PANIC METHOD #2
	/// 1. DROP + READ
	#[inline(never)]
	pub (crate) fn test_drop_and_read(data: Vec<String>) {
		let mut control_drop = ManuallyDrop::new(data);
		
		let _e = control_drop.deref();
		let _e = control_drop.deref();
		{
			unsafe {
				ManuallyDrop::drop(&mut control_drop);
			}
		}
		
		assert_eq!(control_drop.is_maybe_next_panic(), true);
		// <<-- PANIC
		let _e = control_drop.deref();
	}
	
	/// PANIC METHOD #3
	/// 1. READ + TAKE + READ
	#[inline(never)]
	pub (crate) fn test_take_and_read(data: Vec<String>) {
		let mut control_drop = ManuallyDrop::new(data);
		
		let _e = control_drop.deref();
		let _e = control_drop.deref();
		{
			let data = unsafe {
				ManuallyDrop::take(&mut control_drop)
			};
			assert_eq!(data, build_new_test_vec());
			drop(data);
		}
		
		assert_eq!(control_drop.is_maybe_next_panic(), true);
		// <<-- PANIC
		let _e = control_drop.deref();
	}
	
	/// PANIC METHOD #4
	/// 1. READ + TAKE + DROP
	#[inline(never)]
	pub (crate) fn test_take_and_drop(data: Vec<String>) {
		let mut control_drop = ManuallyDrop::new(data);
		
		let _e = control_drop.deref();
		let _e = control_drop.deref();
		{
			let data = unsafe {
				ManuallyDrop::take(&mut control_drop)
			};
			assert_eq!(data, build_new_test_vec());
			drop(data);
		}
		
		assert_eq!(control_drop.is_maybe_next_panic(), true);
		// <<-- PANIC
		unsafe {
			ManuallyDrop::drop(&mut control_drop);
		}
	}
	
	/// PANIC METHOD #5
	/// 1. READ + DROP + INTO_INNER
	#[inline(never)]
	pub (crate) fn test_drop_and_into_inner(data: Vec<String>) {
		let mut control_drop = ManuallyDrop::new(data);
		
		let _e = control_drop.deref();
		let _e = control_drop.deref();
		{
			unsafe {
				ManuallyDrop::drop(&mut control_drop)
			}
		}
		
		assert_eq!(control_drop.is_maybe_next_panic(), true);
		// <<-- PANIC
		let _data = ManuallyDrop::into_inner(control_drop);
	}
}

#[test]
fn test_panic_mode() {
	use SafeManuallyDrop::ManuallyDrop;
	assert_eq!(ManuallyDrop::is_safe_mode(), true);
	
	
	static mut PANIC_COUNTER: usize = 0;
	std::panic::set_hook(Box::new(|panic_info| {
		unsafe {
			PANIC_COUNTER += 1;
		}
		println!("panic_num: {}, panic: {:?}", unsafe { PANIC_COUNTER }, panic_info);
	}));
	
	
	// Start test panic methods
	// START POS
	let arr_fn: &[&'static fn(a: Vec<String>)] = &[
		(&(panic_test_methods::test_combo_drop as fn(a: Vec<String>))) as _,
		(&(panic_test_methods::test_drop_and_read as fn(a: Vec<String>))) as _,
		(&(panic_test_methods::test_take_and_read as fn(a: Vec<String>))) as _,
		(&(panic_test_methods::test_take_and_drop as fn(a: Vec<String>))) as _,
		(&(panic_test_methods::test_drop_and_into_inner as fn(a: Vec<String>))) as _,
	];
	
	for a in arr_fn.iter() {
		let e = std::thread::spawn(move || {
			let a = a;
			a(build_new_test_vec());
			
		}).join();
		assert_eq!(e.is_err(), true);
	}
	
	std::panic::set_hook(Box::new(|_| {}));
	assert_eq!(unsafe {PANIC_COUNTER}, arr_fn.len());
}

