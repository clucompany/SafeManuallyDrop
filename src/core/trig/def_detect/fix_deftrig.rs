
/*
	def = if:
		always_deftrig_panic -> Panic else:
		always_deftrig_hookfn -> Hook else:
		always_deftrig_count -> Count else:
		always_deftrig_loop -> Loop
*/

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(
	any(
		all(
			feature = "support_hookfn_trig",
			feature = "always_deftrig_hookfn",
			
			//not(feature = "always_deftrig_hookfn"),
			not(feature = "always_deftrig_panic"),
			not(feature = "always_deftrig_count"),
			not(feature = "always_deftrig_loop"),
		),
		all( // cargo check --all-features correct!
			feature = "always_deftrig_hookfn",
			feature = "always_deftrig_panic",
			feature = "always_deftrig_count",
			feature = "always_deftrig_loop"
		)
	)
)]
pub type DefTrigManuallyDrop = crate::core::trig::hook::HookFnTrigManuallyDrop;

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(all(
	feature = "support_panic_trig",
	feature = "always_deftrig_panic",
	
	not(feature = "always_deftrig_hookfn"),
	//not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop"),
))]
pub type DefTrigManuallyDrop = crate::core::trig::panic::PanicFnTrigManuallyDrop;

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(all(
	feature = "support_count_trig",
	feature = "always_deftrig_count",
	
	not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_panic"),
	//not(feature = "always_deftrig_count"),
	not(feature = "always_deftrig_loop"),
))]
pub type DefTrigManuallyDrop = crate::core::trig::count::CounterTrigManuallyDrop;

/// Trigger is the default function that will be executed in case of undefined behavior of protected ManuallyDrop.
#[cfg(all(
	feature = "always_deftrig_loop",
	
	not(feature = "always_deftrig_hookfn"),
	not(feature = "always_deftrig_panic"),
	not(feature = "always_deftrig_count"),
	//not(feature = "always_deftrig_loop")
))]
pub type DefTrigManuallyDrop = crate::core::trig::r#loop::EmptyLoopTrigManuallyDrop;

/// Whether the default behavior autodetection was used for ManuallyDrop.
pub const IS_AUTO_DETECT_DEFTRIG: bool = false;

/// The build was done using all-features, the required behavior cannot be determined.
pub const IS_INVALID_AUTO_DETECT_DEFTRIG: bool = {
	#[cfg(all( // cargo check --all-features correct!
		feature = "always_deftrig_hookfn",
		feature = "always_deftrig_panic",
		feature = "always_deftrig_count",
		feature = "always_deftrig_loop"
	))] {
		true
	}
	#[cfg(not(all( // cargo check --all-features correct!
		feature = "always_deftrig_hookfn",
		feature = "always_deftrig_panic",
		feature = "always_deftrig_count",
		feature = "always_deftrig_loop"
	)))] {
		false
	}
};
