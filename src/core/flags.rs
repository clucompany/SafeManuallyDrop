//! Flags used when building this library

/// Whether a table of build flags to use was created
/// when the library was compiled.
pub const BUILD_FLAG_TABLE_CREATED: bool = true;

/// Depending on the build flag, a protected version of ManuallyDrop or
/// an unprotected version of ManuallyDrop with a default trigger.
pub const SAFE_MANUALLYDROP_ENABLED: bool = crate::ManuallyDrop::is_safe_mode();

/// Whether the library build flag was used to support panic_trig.
pub const BUILD_FLAG_PANIC_TRIGGER_ENABLED: bool = {
	#[cfg(feature = "support_panic_trig")]
	{
		true
	}

	#[cfg(not(feature = "support_panic_trig"))]
	{
		false
	}
};

/// Whether the library build flag was used to support hookfn_trig.
pub const BUILD_FLAG_HOOKFN_TRIGGER_ENABLED: bool = {
	#[cfg(feature = "support_hookfn_trig")]
	{
		true
	}

	#[cfg(not(feature = "support_hookfn_trig"))]
	{
		false
	}
};

/// Whether the library build flag was used to support abort_trig.
pub const BUILD_FLAG_ABORT_TRIGGER_ENABLED: bool = {
	#[cfg(feature = "support_abort_trig")]
	{
		true
	}

	#[cfg(not(feature = "support_abort_trig"))]
	{
		false
	}
};

/// Whether the library build flag was used to support count_trig.
pub const BUILD_FLAG_COUNT_TRIGGER_ENABLED: bool = {
	#[cfg(feature = "support_count_trig")]
	{
		true
	}

	#[cfg(not(feature = "support_count_trig"))]
	{
		false
	}
};

/// Whether the library build flag was used to support loop_trig.
pub const BUILD_FLAG_LOOP_TRIGGER_ENABLED: bool = true;

/// Ability to determine if an empty loop trigger has been executed.
pub const BUILD_FLAG_LOOP_IS_TRIGGER_ENABLED: bool = {
	#[cfg(feature = "support_istrig_loop")]
	{
		true
	}

	#[cfg(not(feature = "support_istrig_loop"))]
	{
		false
	}
};

/// Enable additional internal checks of the SafeManuallyDrop library when
/// the debug_assertions flag is enabled (does not depend on the always_check_in_case_debug_assertions
/// and always_safe_manuallydrop options).
pub const BUILD_FLAG_EXTENDED_DEBUG_ASSERTIONS_ENABLED: bool = {
	#[cfg(feature = "allow_extended_debug_assertions")]
	{
		true
	}

	#[cfg(not(feature = "allow_extended_debug_assertions"))]
	{
		false
	}
};

/// Whether the default behavior autodetection was used for ManuallyDrop.
pub const BUILD_FLAG_AUTO_DETECT_DEFTRIG_ENABLED: bool =
	crate::core::trig::BUILD_FLAG_AUTO_DETECT_DEFTRIG_ENABLED;

/// If the build was done using "all functions" (cargo test/doc/build --all-features), the required behavior in a safe mandrop cannot be determined,
/// if this flag is active, EmptyLoopTrigManuallyDrop will be used.
pub const BUILD_FLAG_INVALID_AUTO_DETECT_DEFTRIG_ENABLED: bool =
	crate::core::trig::BUILD_FLAG_INVALID_AUTO_DETECT_DEFTRIG_ENABLED;

#[cfg(test)]
#[test]
fn test_flag_is_safe_mode() {
	#[allow(unused_assignments)]
	let mut is_checked_c = 0;

	#[cfg(feature = "always_safe_manuallydrop")]
	{
		assert_eq!(SAFE_MANUALLYDROP_ENABLED, true);

		//#[allow(unused_assignments)] // error[E0658]: attributes on expressions are experimental
		is_checked_c = 1;
	}
	if is_checked_c != 1 {} // fix error[E0658]: attributes on expressions are experimental

	#[cfg(all(feature = "always_check_in_case_debug_assertions", debug_assertions))]
	{
		// clippy::assertions_on_constants why? it's part of this test, it's okay.
		#![allow(clippy::assertions_on_constants)]
		assert!(SAFE_MANUALLYDROP_ENABLED);

		is_checked_c = 1;
	}

	#[cfg(not(any(
		all(feature = "always_check_in_case_debug_assertions", debug_assertions),
		feature = "always_safe_manuallydrop"
	)))]
	{
		assert_eq!(SAFE_MANUALLYDROP_ENABLED, false);

		is_checked_c = is_checked_c + 1;
	}

	assert_eq!(is_checked_c, 1);
}
