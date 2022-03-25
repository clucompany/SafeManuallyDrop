
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

#[cfg(feature = "support_hookfn_trig")]
pub mod hook;

/// Implementation of behavior in case of detection of undefined manual memory management.
pub trait TrigManuallyDrop {
	fn trig_next_invalid_beh<'a>(a: Arguments<'a>) -> trig_manuallydrop_returntype!();
}

#[cfg(any(
	feature = "always_deftrig_panic",
	feature = "always_deftrig_hookfn",
	feature = "always_deftrig_count",
	feature = "always_deftrig_loop",
))]
#[path = "fix_deftrig.rs"]
pub mod current_deftrig;

#[cfg(all(
	not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop")
))]
#[path = "auto_detect_deftrig.rs"]
pub mod current_deftrig;

pub use current_deftrig::DefTrigManuallyDrop;
pub (crate) use current_deftrig::IS_AUTO_DETECT_DEFTRIG;
pub (crate) use current_deftrig::IS_INVALID_AUTO_DETECT_DEFTRIG;

pub mod r#loop;

#[doc(hidden)]
#[deprecated(since = "0.1.5", note = "Use `crate::core::trig::r#loop::EmptyLoopTrigManuallyDrop` instead")]
pub use crate::core::trig::r#loop::EmptyLoopTrigManuallyDrop as EmptyLoopTrigManuallyDrop;
