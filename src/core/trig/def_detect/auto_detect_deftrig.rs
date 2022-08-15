
/*
	def = if:
		always_deftrig_panic: not exists AND
		always_deftrig_abort: not exists AND
		always_deftrig_hookfn: not exists AND
		always_deftrig_count: not exists AND
		always_deftrig_loop: not exists THEN
	
		support_hookfn_trig -> Hook,	else:
		support_panic_trig -> Panic,	else:
		support_count_trig -> Count, else:
			Loop
*/

#[cfg(all( // AUTO DEF (ALL-FEATURES)
	feature = "support_hookfn_trig",
	feature = "support_abort_trig",
	feature = "support_panic_trig",
	feature = "support_count_trig"
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::panic::PanicTrigManuallyDrop;


#[cfg(all(
	feature = "support_panic_trig",
	
	not(feature = "support_hookfn_trig"),
	not(feature = "support_abort_trig"),
	//not(feature = "support_panic_trig"),
	not(feature = "support_count_trig")
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::panic::PanicTrigManuallyDrop;
	
#[cfg(all(
	feature = "support_abort_trig",
	
	not(feature = "support_hookfn_trig"),
	//not(feature = "support_abort_trig"),
	not(feature = "support_panic_trig"),
	not(feature = "support_count_trig")
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::abort::AbortTrigManuallyDrop;
	
#[cfg(all(
	feature = "support_hookfn_trig",
	
	//not(feature = "support_hookfn_trig"),
	not(feature = "support_abort_trig"),
	not(feature = "support_panic_trig"),
	not(feature = "support_count_trig")
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::hook::HookFnTrigManuallyDrop;
	
#[cfg(all(
	feature = "support_count_trig",
	
	not(feature = "support_hookfn_trig"),
	not(feature = "support_abort_trig"),
	not(feature = "support_panic_trig"),
	//not(feature = "support_count_trig"),
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::counter::CounterTrigManuallyDrop;
	
#[cfg(all(
	not(feature = "support_hookfn_trig"),
	not(feature = "support_abort_trig"),
	not(feature = "support_panic_trig"),
	not(feature = "support_count_trig")
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::r#loop::EmptyLoopTrigManuallyDrop;
	

/// Whether the default behavior autodetection was used for ManuallyDrop.
pub const IS_AUTO_DETECT_DEFTRIG: bool = true;
/// The build was done using all-features, the required behavior cannot be determined.
pub const IS_INVALID_AUTO_DETECT_DEFTRIG: bool = false;
