
use core::fmt::Arguments;

/// wait stable https://github.com/rust-lang/rust/issues/35121
#[cfg(not(feature = "support_count_trig"))]
#[macro_export]
#[doc(hidden)]
macro_rules! trig_manuallydrop_returntype {
	[] => {
		!
	};
}

#[cfg(feature = "support_count_trig")]
#[macro_export]
#[doc(hidden)]
macro_rules! trig_manuallydrop_returntype {
	[] => {
		()
	};
}

#[cfg(feature = "support_panic_trig")]
pub mod panic;
#[cfg(feature = "support_count_trig")]
pub mod counter;

#[cfg(feature = "support_hook_trig")]
pub mod hook;

/// Implementation of behavior in case of detection of undefined manual memory management.
pub trait TrigManuallyDrop {
	fn trig_next_invalid_beh<'a>(a: Arguments<'a>) -> trig_manuallydrop_returntype!();
}

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(feature = "support_hook_trig")]
pub type DefTrigManuallyDrop = crate::core::trig::hook::HookFnTrigManuallyDrop;

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(not(feature = "support_hook_trig"))]
pub type DefTrigManuallyDrop = EmptyLoopTrigManuallyDrop;

pub enum EmptyLoopTrigManuallyDrop {}

impl TrigManuallyDrop for EmptyLoopTrigManuallyDrop {
	#[inline(always)]
	fn trig_next_invalid_beh<'a>(_a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		loop {}
	}
}

