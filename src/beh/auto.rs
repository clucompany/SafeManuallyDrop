
/// An internal macro that replaces many built-in assembly safety checks for 
/// the default ManuallyDrop type.
/// 
///	example0:
///	```rust
///	cfg_if_safemode! {
///		// Unsafe
///		// Depending on the build flag, a protected version of ManuallyDrop or 
///		// an unprotected version of ManuallyDrop with a default trigger.
///		#if_not_safe(pub type ManuallyDrop<T> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, crate::core::trig::DefTrigManuallyDrop>;)
///
///		// Safe
///		// Depending on the build flag, a protected version of ManuallyDrop or 
///		// an unprotected version of ManuallyDrop with a default trigger.
///		#if_safe(pub type ManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, crate::core::trig::DefTrigManuallyDrop>;)
///	}
///	```
/// 
///	example1:
///	```rust
///	// Depending on the build flag, a protected version of ManuallyDrop or 
///	// an unprotected version of ManuallyDrop with a default trigger.
///	#[inline(always)]
///	pub const fn is_safe_mode() -> bool {
///		cfg_if_safemode! {
///			#if_safe() {
///				true
///			}else {
///				false
///			}
///		}
///	}
///	```
/// 
#[doc(hidden)]
#[macro_export]
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

// TODO, MANYCOPYCODE
crate::cfg_if_safemode! {
	// Unsafe
	/// Depending on the build flag, a protected version of ManuallyDrop or 
	/// an unprotected version of ManuallyDrop with a default trigger.
	/// 
	/// features:
	/// ```
	/// if always_safe_manuallydrop | ( always_check_in_case_debug_assertions && debug_assertions ) -> SafeManuallyDrop
	/// else -> UnsafeManuallyDrop
	/// ```
	/// 
	/// current: 
	/// ```
	/// UnsafeManuallyDrop
	/// ```
	#if_not_safe(pub type AutoSafeManuallyDrop<T, Trig> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, Trig>;)

	// Safe
	/// Depending on the build flag, a protected version of ManuallyDrop or 
	/// an unprotected version of ManuallyDrop with a default trigger.
	/// 
	/// features:
	/// ```
	/// if always_safe_manuallydrop | ( always_check_in_case_debug_assertions && debug_assertions ) -> SafeManuallyDrop
	/// else -> UnsafeManuallyDrop
	/// ```
	/// 
	/// current: 
	/// ```
	/// SafeManuallyDrop
	/// ```
	#if_safe(pub type AutoSafeManuallyDrop<T, Trig> = crate::beh::safe::SafeManuallyDrop<T, Trig>;)
}
