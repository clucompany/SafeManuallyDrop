
use core::fmt::Arguments;
use crate::core::trig::TrigManuallyDrop;

/// A protected version of ManuallyDrop with a function to 
/// execute a abort in case of undefined behavior of the ManuallyDrop logic.
pub type AlwaysSafeAbortManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, AbortTrigManuallyDrop>;

/// A secure or non-secure version of ManuallyDrop with a function to trigger 
/// a abort in case of undefined behavior of the ManuallyDrop logic.
pub type AutoSafeAbortManuallyDrop<T> = crate::beh::auto::AutoSafeManuallyDrop<T, AbortTrigManuallyDrop>;

/// In case of undefined behavior of manual memory management, 
/// perform a normal abort.
pub enum AbortTrigManuallyDrop {}

impl TrigManuallyDrop for AbortTrigManuallyDrop {
	#[inline(always)]
	fn trig_next_invalid_beh<'a>(a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		use std::io::Write;
		
		{
			let mut lock = std::io::stderr().lock();
			let _e = write!(lock, "{}\n", a);
			let _e = lock.flush();
		}
		
		std::process::abort();
	}
}
