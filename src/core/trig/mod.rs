
use core::fmt::Arguments;

// TODO! Wait stable https://github.com/rust-lang/rust/issues/35121
#[cfg(not(feature = "support_count_trig"))]
#[macro_export]
#[doc(hidden)]
macro_rules! trig_manuallydrop_returntype {
	[] => {
		!
	};
}

// Due to the fact that CounterManuallyDrop assumes the continuation
// of the execution of invalid code, it is required to exclude 
// the program interruption optimization. 
#[cfg(feature = "support_count_trig")]
#[macro_export]
#[doc(hidden)]
macro_rules! trig_manuallydrop_returntype {
	[] => {
		()
	};
}

/// A protected version of SafeManuallyDrop with a function to execute a panic 
/// in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_panic_trig")]
pub mod panic;

/// A protected version of SafeManuallyDrop with a function to count 
/// the amount of undefined behavior of the ManuallyDrop logic. 
/// The undefined behavior of CounterManuallyDrop will be the same as 
/// when using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
pub mod counter;

/// Protected version of the SafeManuallyDrop with an execution 
/// function in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_hookfn_trig")]
pub mod hook;

/// Implementation of behavior in case of detection of 
/// undefined manual memory management.
pub trait TrigManuallyDrop {
	/// Implementation of behavior in case of detection of 
	/// undefined manual memory management.
	fn trig_next_invalid_beh<'a>(a: Arguments<'a>) -> trig_manuallydrop_returntype!();
}

#[cfg(any(
	feature = "always_deftrig_panic",
	feature = "always_deftrig_hookfn",
	feature = "always_deftrig_count",
	feature = "always_deftrig_loop",
))]
#[path = "def_detect/fix_deftrig.rs"]
/// Default trigger for ManuallyDrop type.
pub mod current_deftrig;

#[cfg(all(
	not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop")
))]
#[path = "def_detect/auto_detect_deftrig.rs"]
/// Default trigger for ManuallyDrop type.
pub mod current_deftrig;

/// Trigger is the default function that will be executed in case of 
/// undefined behavior of protected ManuallyDrop.
pub use current_deftrig::DefTrigManuallyDrop;

/// Whether the default behavior autodetection was used for ManuallyDrop.
#[doc(hidden)]
#[cfg(any(test, feature = "always_build_flagstable"))]
pub (crate) use current_deftrig::IS_AUTO_DETECT_DEFTRIG;

/// The build was done using all-features, the required behavior cannot be determined.
#[doc(hidden)]
#[cfg(any(test, feature = "always_build_flagstable"))]
pub (crate) use current_deftrig::IS_INVALID_AUTO_DETECT_DEFTRIG;

/// The safe version of ManuallyDrop loops the current thread in case of undefined behavior, 
/// and using the `support_istrig_loop` build flag, you can determine whether the 
/// thread looped. 
pub mod r#loop;

