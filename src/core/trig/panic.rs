
use core::fmt::Arguments;
use crate::core::trig::TrigManuallyDrop;

/// A protected version of SafeManuallyDrop with a function to execute a panic in case of undefined behavior of the ManuallyDrop logic.
pub type PanicManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, PanicTrigManuallyDrop>;

/// In case of undefined behavior of manual memory management, perform a normal panic.
pub enum PanicTrigManuallyDrop {}

impl TrigManuallyDrop for PanicTrigManuallyDrop {
	#[inline(always)]
	fn trig_next_invalid_beh<'a>(a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		panic!("{}", a);
	}
}
