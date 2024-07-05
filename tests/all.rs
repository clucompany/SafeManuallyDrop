use SafeManuallyDrop::ManuallyDrop;

/// Build test data
#[allow(clippy::vec_init_then_push)] // why?: this is only needed for internal tests, the logic can be traced, there should be no other meaning here
fn build_new_test_vec() -> Vec<String> {
	let mut vec = Vec::with_capacity(3);
	vec.push("test".into());
	vec.push("test2".into());
	vec.push("test3".into());

	vec
}

#[cfg(feature = "support_panic_trig")]
#[allow(unused_unsafe)]
mod panic_test_methods {
	use super::build_new_test_vec;
	use core::ops::Deref;
	use SafeManuallyDrop::AlwaysSafePanicManuallyDrop as PanicManuallyDrop;

	/// PANIC METHOD #1
	/// 1. COMBO DROP
	#[inline(never)]
	pub(crate) fn test_combo_drop(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID
		let _e = control_drop.deref(); // VALID

		{
			unsafe {
				PanicManuallyDrop::drop(&mut control_drop); // VALID

				assert!(control_drop.is_next_trig()); // VALID
									  // <<-- PANIC
				PanicManuallyDrop::drop(&mut control_drop); // INVALID
			}
		}
	}

	/// PANIC METHOD #2
	/// 1. DROP + READ
	#[inline(never)]
	pub(crate) fn test_drop_and_read(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID
		let _e = control_drop.deref(); // VALID
		let _e = control_drop.deref(); // VALID
		{
			unsafe {
				PanicManuallyDrop::drop(&mut control_drop); // VALID
			}
		}

		assert!(control_drop.is_next_trig()); // VALID
									  // <<-- PANIC
		let _e = control_drop.deref(); // INVALID
	}

	/// PANIC METHOD #3
	/// 1. READ + TAKE + READ
	#[inline(never)]
	pub(crate) fn test_take_and_read(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID

		let _e = control_drop.deref(); // VALID
		let _e = control_drop.deref(); // VALID
		{
			let data = unsafe {
				PanicManuallyDrop::take(&mut control_drop) // VALID
			};
			assert_eq!(data, build_new_test_vec()); // VALID
			drop(data); // VALID
		}
		assert!(control_drop.is_next_trig()); // VALID

		// <<-- PANIC
		let _e = control_drop.deref(); // INVALID
	}

	/// PANIC METHOD #4
	/// 1. READ + TAKE + DROP
	#[inline(never)]
	pub(crate) fn test_take_and_drop(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID

		let _e = control_drop.deref(); // VALID
		let _e = control_drop.deref(); // VALID
		{
			let data = unsafe {
				PanicManuallyDrop::take(&mut control_drop) // VALID
			};
			assert_eq!(data, build_new_test_vec()); // VALID
			drop(data); // VALID
		}
		assert!(control_drop.is_next_trig()); // VALID

		// <<-- PANIC
		unsafe {
			PanicManuallyDrop::drop(&mut control_drop); // INVALID
		}
	}

	/// PANIC METHOD #5
	/// 1. READ + DROP + INTO_INNER
	#[inline(never)]
	pub(crate) fn test_drop_and_into_inner(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID

		let _e = control_drop.deref(); // VALID
		let _e = control_drop.deref(); // VALID
		{
			unsafe {
				PanicManuallyDrop::drop(&mut control_drop) // VALID
			}
		}
		assert!(control_drop.is_next_trig()); // VALID

		// <<-- PANIC
		let _data = PanicManuallyDrop::into_inner(control_drop); // INVALID
	}

	/// PANIC METHOD #5
	/// 1. AutoPanic when drop
	#[inline(never)]
	pub(crate) fn test_expmanualdrop(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID

		unsafe {
			PanicManuallyDrop::drop(&mut control_drop); // VALID
		}
		assert!(control_drop.is_next_trig()); // VALID

		// <<-- PANIC
		let _e = control_drop.deref(); // INVALID
	}

	// IGNORE PANIC
	#[inline(never)]
	pub(crate) fn test_expmanualdrop2_ignorepanic(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID

		unsafe {
			PanicManuallyDrop::drop(&mut control_drop); // VALID
		}

		assert!(control_drop.is_next_trig()); // VALID
	}

	// IGNORE PANIC
	#[inline(never)]
	pub(crate) fn test_expmanualdrop3_ignorepanic(data: Vec<String>) {
		let mut control_drop = PanicManuallyDrop::new(data); // VALID

		let data = unsafe {
			PanicManuallyDrop::take(&mut control_drop) // VALID
		};
		assert!(control_drop.is_next_trig()); // VALID

		//
		drop(control_drop); // VALID
		drop(data); // VALID
	}
}

#[cfg(all(test, feature = "support_panic_trig"))]
#[test]
fn test_panic_mode() {
	use std::sync::{Mutex, MutexGuard};

	static PANIC_COUNTER: Mutex<usize> = Mutex::new(0);
	/// (Let's just shorten the code, it doesn't make much sense for testing.)
	fn panic_counter<R>(rw: impl FnOnce(MutexGuard<usize>) -> R) -> R {
		let result = rw(match PANIC_COUNTER.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		});

		result
	}

	std::panic::set_hook(Box::new(|panic_info| {
		panic_counter(|mut rw| *rw += 1);

		println!(
			"#[test_trigger, num: {}] {:?}, OK.",
			panic_counter(|rw| *rw),
			panic_info
		);
	}));

	// Start test panic methods
	// START POS
	// why?: this is only needed for internal tests, the logic can be traced, there should be no other meaning here
	#[allow(clippy::type_complexity)]
	let arr_fn: &[(bool, fn(a: Vec<String>))] = &[
		(true, panic_test_methods::test_combo_drop as _),
		(true, panic_test_methods::test_drop_and_read as _),
		(true, panic_test_methods::test_take_and_read as _),
		(true, panic_test_methods::test_take_and_drop as _),
		(true, panic_test_methods::test_drop_and_into_inner as _),
		(true, panic_test_methods::test_expmanualdrop as _),
		(
			false,
			panic_test_methods::test_expmanualdrop2_ignorepanic as _,
		),
		(
			false,
			panic_test_methods::test_expmanualdrop3_ignorepanic as _,
		),
	];

	let mut c_ignore_panic = 0;
	for (is_err, ..) in arr_fn.iter() {
		if !is_err {
			c_ignore_panic += 1;
		}
	}

	for (is_err, function) in arr_fn.iter() {
		let e = std::thread::spawn(|| {
			let function = *function;
			let new_data = build_new_test_vec();

			(function)(new_data);
		})
		.join();

		assert_eq!(&e.is_err(), is_err);
	}

	std::panic::set_hook(Box::new(|_| {}));
	assert_eq!(panic_counter(|rw| *rw), arr_fn.len() - c_ignore_panic);
}

#[test]
fn test_thread_drop() {
	let a = ManuallyDrop::new(vec![1]);

	let is_ok = std::thread::spawn(move || {
		let mut a = a;
		#[allow(unused_unsafe)]
		unsafe {
			ManuallyDrop::drop(&mut a);
		}

		true
	})
	.join();

	assert!(is_ok.is_ok());
	let ok = is_ok.unwrap();
	assert!(ok);
}
