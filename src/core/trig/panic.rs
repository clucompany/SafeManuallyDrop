use crate::core::trig::TrigManuallyDrop;
use core::fmt::Arguments;

/// A protected version of ManuallyDrop with a function to
/// execute a panic in case of undefined behavior of the ManuallyDrop logic.
pub type AlwaysSafePanicManuallyDrop<T> =
	crate::beh::safe::SafeManuallyDrop<T, PanicTrigManuallyDrop>;

/// A secure or non-secure version of ManuallyDrop with a function to trigger
/// a panic in case of undefined behavior of the ManuallyDrop logic.
pub type AutoSafePanicManuallyDrop<T> =
	crate::beh::auto::AutoSafeManuallyDrop<T, PanicTrigManuallyDrop>;

/// In case of undefined behavior of manual memory management,
/// perform a normal panic.
pub enum PanicTrigManuallyDrop {}

impl TrigManuallyDrop for PanicTrigManuallyDrop {
	// Just a cold version of panic
	#[inline(never)]
	#[cold]
	fn trig_next_invalid_beh(a: Arguments<'_>) -> trig_manuallydrop_returntype!() {
		panic!("{}", a);
	}
}
