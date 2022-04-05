
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
		is_safe: false,
		is_always_compatible: true,
		is_maybe_compatible: true,
		is_repr_transparent: true,
	];
}

impl<T, Trig> Copy for UnsafeManuallyDrop<T, Trig> where T: ?Sized + Copy, Trig: TrigManuallyDrop {}

impl<T, Trig> Clone for UnsafeManuallyDrop<T, Trig> where T: ?Sized + Clone, Trig: TrigManuallyDrop {
	#[inline(always)]
	fn clone(&self) -> Self {
		let ref_value = &self.value as &T;
		let value = Clone::clone(ref_value);
		
		Self::new(value)
	}
}
