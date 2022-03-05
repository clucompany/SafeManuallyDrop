
use crate::UnsafeStdManuallyDrop;
use crate::core::trig::TrigManuallyDrop;
use core::marker::PhantomData;

/// A safe version of the insecure manual control of freeing memory.
// #[repr(transparent)]
pub struct SafeManuallyDrop<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
	state: StateManuallyDrop,
	_pp: PhantomData<Trig>,
	value: UnsafeStdManuallyDrop<T>,
}

crate::__codegen! {
	@use;
	#SafeManuallyDrop [
		is_safe: true
	];
}

impl<T, Trig> SafeManuallyDrop<T, Trig> where T: ?Sized + Clone, Trig: TrigManuallyDrop {
	/// For tests only, resets the MannualuDrop state to the initial state
	#[inline(always)]
	pub unsafe fn flush(&mut self) -> StateManuallyDropData {
		self.state.flush()
	}
}

//impl<T> Copy for ManuallyDrop<T> where T: ?Sized + Copy {} TODO

impl<T, Trig> Clone for SafeManuallyDrop<T, Trig> where T: ?Sized + Clone, Trig: TrigManuallyDrop {
	#[inline(always)]
	fn clone(&self) -> Self {
		let state = self.state.clone();
		let ref_value: &T = self.value.deref();
		
		Self {
			state,
			value: UnsafeStdManuallyDrop::new(Clone::clone(ref_value)),
			_pp: PhantomData,
		}
	}
}

impl<T, Trig> Drop for SafeManuallyDrop<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
	#[inline]
	fn drop(&mut self) {		
		/*enum __HideTrig {}
		impl TrigManuallyDrop for __HideTrig {
			fn trig_next_invalid_beh<'a>(a: core::fmt::Arguments<'a>) -> ! {
				Trig::trig_next_invalid_beh(a)
			}
		}*/
		
		self.state.if_empty_then_run_trigfn::<Trig, _>(
			"expected ManuallyDrop::drop(&mut value)",
			|| unsafe {
				// What for? - >> to ignore miri errors allocate.
				UnsafeStdManuallyDrop::drop(&mut self.value);
			}
		);
	}
}
