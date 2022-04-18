
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
	@impl UnsafeManuallyDrop {
		is_safe: false,
		is_always_compatible: true,
		is_maybe_compatible: true,
		is_repr_transparent: true,
		
		fn {
			/// Wrap a value to be manually dropped.
			new |value| {
				Self {
					value,
					_pp: PhantomData
				}
			}
			
			as_unsafestd_manuallydrop |sself| {
				&sself.value
			}
			
			as_mut_unsafestd_manuallydrop |sself| {
				&mut sself.value
			}
			
			/// Get reference to value. Always unprotected!
			force_as_value |sself| {
				&sself.value
			}
			
			/// Get a mutable reference to a value. Always unprotected!
			force_as_mut_value |sself| {
				&mut sself.value
			}
		}
	}
}

impl<T, Trig> Copy for UnsafeManuallyDrop<T, Trig> where T: ?Sized + Copy, Trig: TrigManuallyDrop {}
