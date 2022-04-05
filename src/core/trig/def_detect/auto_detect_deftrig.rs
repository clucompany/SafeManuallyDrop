
/*
	def = if:
		always_deftrig_panic: not exists AND
		always_deftrig_hookfn: not exists AND
		always_deftrig_count: not exists AND
		always_deftrig_loop: not exists THEN
	
		support_hookfn_trig -> Hook,	else:
		support_panic_trig -> Panic,	else:
		support_count_trig -> Count, else:
			Loop
*/

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(all(
	feature = "support_hookfn_trig"
))]
pub type DefTrigManuallyDrop = crate::core::trig::hook::HookFnTrigManuallyDrop;

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(all(
	not(feature = "support_hookfn_trig"),
	
	feature = "support_panic_trig"
))]
pub type DefTrigManuallyDrop = crate::core::trig::panic::PanicFnTrigManuallyDrop;

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(all(
	not(feature = "support_hookfn_trig"),
	not(feature = "support_panic_trig"),
	
	feature = "support_count_trig"
))]
pub type DefTrigManuallyDrop = crate::core::trig::count::CounterTrigManuallyDrop;

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(all(
	not(feature = "support_hookfn_trig"),
	not(feature = "support_panic_trig"),
	not(feature = "support_count_trig")
))]
pub type DefTrigManuallyDrop = crate::core::trig::r#loop::EmptyLoopTrigManuallyDrop;

/// Whether the default behavior autodetection was used for ManuallyDrop.
pub const IS_AUTO_DETECT_DEFTRIG: bool = true;
/// The build was done using all-features, the required behavior cannot be determined.
pub const IS_INVALID_AUTO_DETECT_DEFTRIG: bool = false;