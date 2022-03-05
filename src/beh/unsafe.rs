
use core::marker::PhantomData;
use crate::UnsafeStdManuallyDrop;
use crate::core::trig::TrigManuallyDrop;

/// Insecure standard implementation of manual memory management.
#[repr(transparent)]
pub struct UnsafeManuallyDrop<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
	_pp: PhantomData<Trig>,
	value: UnsafeStdManuallyDrop<T>,
}

crate::__codegen! {
	@use;
	#UnsafeManuallyDrop [
		is_safe: false
	];
}

impl<T, Trig> UnsafeManuallyDrop<T, Trig> where T: ?Sized + Clone, Trig: TrigManuallyDrop {
	/// For tests only, resets the MannualuDrop state to the initial state
	#[inline(always)]
	pub unsafe fn flush(&mut self) -> StateManuallyDropData {
		StateManuallyDropData::empty()
	}
}

impl<T, Trig> Copy for UnsafeManuallyDrop<T, Trig> where T: ?Sized + Copy, Trig: TrigManuallyDrop {}

impl<T, Trig> Clone for UnsafeManuallyDrop<T, Trig> where T: ?Sized + Clone, Trig: TrigManuallyDrop {
	#[inline(always)]
	fn clone(&self) -> Self {
		Self::new(
			Clone::clone(&self.value)
		)
	}
}
