
use core::marker::PhantomData;
use crate::UnsafeStdManuallyDrop;
use crate::core::trig::TrigManuallyDrop;

/// Insecure standard implementation of manual memory management.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnsafeManuallyDrop<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
	_pp: PhantomData<Trig>,
	value: UnsafeStdManuallyDrop<T>,
}

crate::__codegen! {
	@use;
	#UnsafeManuallyDrop		[is_safe: false];
}
