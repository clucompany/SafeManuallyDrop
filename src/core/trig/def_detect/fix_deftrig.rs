/*
	def = if:
		always_deftrig_panic -> Panic else:
		always_deftrig_abort -> Abort else:
		always_deftrig_hookfn -> Hook else:
		always_deftrig_count -> Count else:
		always_deftrig_loop -> Loop
*/

#[cfg(all( // AUTO DEF (ALL-FEATURES)
	feature = "always_deftrig_hookfn",
	feature = "always_deftrig_panic",
	feature = "always_deftrig_abort",
	feature = "always_deftrig_count",
	feature = "always_deftrig_loop",
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::panic::PanicTrigManuallyDrop;

#[cfg(all(
	feature = "support_panic_trig",
	feature = "always_deftrig_panic",
	not(feature = "always_deftrig_hookfn"),
	//not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_abort"),
	not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop"),
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::panic::PanicTrigManuallyDrop;

#[cfg(all(
	feature = "support_abort_trig",
	feature = "always_deftrig_abort",
	not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_panic"),
	//not(feature = "always_deftrig_abort"),
	not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop"),
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::abort::AbortTrigManuallyDrop;

#[cfg(all(
	feature = "support_hookfn_trig",
	feature = "always_deftrig_hookfn",
	//not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_abort"),
	not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop"),
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::hook::HookFnTrigManuallyDrop;

#[cfg(all(
	feature = "support_count_trig",
	feature = "always_deftrig_count",
	not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_abort"),
	//not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop"),
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::counter::CounterTrigManuallyDrop;

#[cfg(all(
	feature = "always_deftrig_loop",
	not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_abort"),
	not(feature = "always_deftrig_count"),
	//not(feature = "always_deftrig_loop"),
))]
/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
pub type DefTrigManuallyDrop = crate::core::trig::r#loop::EmptyLoopTrigManuallyDrop;

/// Whether the default behavior autodetection was used for ManuallyDrop.
pub const BUILD_FLAG_AUTO_DETECT_DEFTRIG_ENABLED: bool = false;

/// The build was done using all-features, the required behavior cannot be determined.
pub const BUILD_FLAG_INVALID_AUTO_DETECT_DEFTRIG_ENABLED: bool = {
	#[cfg(all( // cargo check --all-features correct!
		feature = "always_deftrig_hookfn",
		feature = "always_deftrig_panic",
		feature = "always_deftrig_abort",
		feature = "always_deftrig_count",
		feature = "always_deftrig_loop"
	))]
	{
		true
	}
	#[cfg(not(all( // cargo check --all-features correct!
		feature = "always_deftrig_hookfn",
		feature = "always_deftrig_panic",
		feature = "always_deftrig_abort",
		feature = "always_deftrig_count",
		feature = "always_deftrig_loop"
	)))]
	{
		false
	}
};
