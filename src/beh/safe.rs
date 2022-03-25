
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

//impl<T> Copy for ManuallyDrop<T> where T: ?Sized + Copy {} TODO

impl<T, Trig> Clone for SafeManuallyDrop<T, Trig> where T: ?Sized + Clone, Trig: TrigManuallyDrop {
	#[inline(always)]
	fn clone(&self) -> Self {
		let ref_value: &T = self.value.deref();
		let state = self.state.clone();
		
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
		self.state.if_empty_then_run_trigfn::<Trig, _>(
			"expected ManuallyDrop::drop(&mut value)",
			|| unsafe {
				// What for? - >> to ignore miri errors allocate.
				UnsafeStdManuallyDrop::drop(&mut self.value);
			}
		);
	}
}
