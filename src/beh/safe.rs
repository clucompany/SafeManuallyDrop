//! A safe version of the insecure manual control of freeing memory.

use crate::core::trig::TrigManuallyDrop;
use crate::macro_codegen::__codegen;
use crate::UnsafeStdManuallyDrop;
use core::marker::PhantomData;

/// A safe version of the insecure manual control of freeing memory.
// #[repr(transparent)]
pub struct SafeManuallyDrop<T, Trig>
where
	T: ?Sized,
	Trig: TrigManuallyDrop,
{
	state: StateManuallyDrop,
	_pp: PhantomData<Trig>,

	value: UnsafeStdManuallyDrop<T>,
}

__codegen! {
	@use;
	@impl SafeManuallyDrop {
		is_safe: true,
		is_always_compatible: false,
		is_maybe_compatible: true,
		is_repr_transparent: false,

		fn {
			/// Wrap a value to be manually dropped.
			new |value| {
				Self {
					value,
					state: StateManuallyDrop::EMPTY_STATE,
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

//impl<T> Copy for ManuallyDrop<T> where T: ?Sized + Copy {} TODO
impl<T, Trig> Drop for SafeManuallyDrop<T, Trig>
where
	T: ?Sized,
	Trig: TrigManuallyDrop,
{
	#[inline]
	fn drop(&mut self) {
		self.state.if_empty_then_run_trigfn::<Trig, _>(
			"expected ManuallyDrop::drop(&mut value)",
			|| unsafe {
				// What for? - >> to ignore miri errors allocate.
				UnsafeStdManuallyDrop::drop(&mut self.value);
			},
		);
	}
}
