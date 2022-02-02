
#[macro_export]
#[doc(hidden)]
macro_rules! cfg_if_safemode {
	[ #if_safe() { $($all:tt)* } $( else $($else_all:tt)* )? /*$($macros_data:tt)**/ ] => {
		{
			#[cfg(
				any(
					feature = "always_safe_manuallydrop",
					all(feature = "always_check_in_case_debug_assertions", debug_assertions),
				)
			)] {
				$($all)*
			}
			
			$(
				#[cfg(not(
					any(
						feature = "always_safe_manuallydrop",
						all(feature = "always_check_in_case_debug_assertions", debug_assertions),
					)
				))] {
					$($else_all)*
				}
			)?
		}
		
		/*$crate::cfg_if_safemode! {
			$($macros_data)*
		}*/
	};
	
	[
		$(#[$($meta:tt)*])*
		#if_safe ( $($all:tt)* )  $($macros_data:tt)*
	] => {
		#[cfg(
			any(
				feature = "always_safe_manuallydrop",
				all(feature = "always_check_in_case_debug_assertions", debug_assertions),
			)
		)]
			$(#[$($meta)*])*
			$($all)*
		
		$crate::cfg_if_safemode! {
			$($macros_data)*
		}
	};
	
	[
		$(#[$($meta:tt)*])*
		#if_not_safe ( $($all:tt)* )  $($macros_data:tt)*
	] => {
		#[cfg(
			not(
				any(
					feature = "always_safe_manuallydrop",
					all(feature = "always_check_in_case_debug_assertions", debug_assertions),
				)
			)
		)] 
			$(#[$($meta)*])*
			$($all)*
		
		$crate::cfg_if_safemode! {
			$($macros_data)*
		}
	};
	
	[] => {};
	[ #if_safe { $($all:tt)* } ] => {
		{
			#[cfg(
				any(
					feature = "always_safe_manuallydrop",
					all(feature = "always_check_in_case_debug_assertions", debug_assertions)
				)
			)] {
				$($all)*
			}
		}
	};
}

crate::cfg_if_safemode! {
	#if_not_safe (pub const IS_SAFE_MODE: bool = false;)
	#if_safe (pub const IS_SAFE_MODE: bool = true;)
}

#[cfg(feature = "support_panic_trig")]
pub const SUPPORT_PANIC_TRIG: bool = true;

#[cfg(not(feature = "support_panic_trig"))]
pub const SUPPORT_PANIC_TRIG: bool = false;

#[cfg(feature = "support_hook_trig")]
pub const SUPPORT_HOOK_TRIG: bool = true;

#[cfg(not(feature = "support_hook_trig"))]
pub const SUPPORT_HOOK_TRIG: bool = false;

#[cfg(feature = "support_count_trig")]
pub const SUPPORT_COUNT_TRIG: bool = true;

#[cfg(not(feature = "support_count_trig"))]
pub const SUPPORT_COUNT_TRIG: bool = false;

pub const SUPPORT_EMPTY_TRIG: bool = true;

#[cfg(test)]
#[test]
fn test_flag_is_safe_mode() {
	#[allow(unused_assignments)]
	let mut is_checked_c = 0;
	
	#[cfg(feature = "always_safe_manuallydrop")] {
		assert_eq!(IS_SAFE_MODE, true);
		
		//#[allow(unused_assignments)] // error[E0658]: attributes on expressions are experimental
		is_checked_c = 1;
	}
	if is_checked_c != 1 {} // fix error[E0658]: attributes on expressions are experimental
	
	#[cfg( all(feature = "always_check_in_case_debug_assertions", debug_assertions) )] {
		assert_eq!(IS_SAFE_MODE, true);
		
		is_checked_c = 1;
	}
	
	#[cfg(not(
		any(
			all(feature = "always_check_in_case_debug_assertions", debug_assertions),
			feature = "always_safe_manuallydrop"
		)
	))] {
		assert_eq!(IS_SAFE_MODE, false);
		
		is_checked_c = is_checked_c + 1;
	}
	
	assert_eq!(is_checked_c, 1);
}