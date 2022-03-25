//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2022
//

// =============
// !!ATTENTION!!
// =============
// Anything related to deprecated features 
// will be removed in a future regression release.
//

#![allow(non_snake_case)]

#![no_std]

pub use ::core::mem::ManuallyDrop as UnsafeStdManuallyDrop;
use crate::core::trig::DefTrigManuallyDrop;

/// The core of the library that defines the basic primitives.
pub mod core {
	pub mod state;
	#[macro_use]
	pub mod flags;
	
	#[doc(hidden)]
	#[deprecated(since = "0.1.2", note = "Use `crate::core::trig::hook` instead")]
	#[cfg(feature = "enable_deprecated_hook")]
	pub mod hook;
	pub mod trig;
}

mod macro_codegen;

#[doc(hidden)]
#[deprecated(since = "0.1.5", note = "Use `crate::core::state` instead")]
pub use crate::core::state as state;

#[doc(hidden)]
#[deprecated(since = "0.1.5", note = "Use `crate::core::hook` instead")]
#[allow(deprecated)]
pub use crate::core::hook as hook;

#[doc(hidden)]
#[deprecated(since = "0.1.5", note = "Use `crate::core::flags` instead")]
pub use crate::core::flags as flags;

/// Safe and insecure implementations of manual memory management.
pub mod beh {
	/// Insecure standard implementation of manual memory management.
	pub mod r#unsafe;
		
	/// A safe version of the insecure manual control of freeing memory.
	pub mod safe;
}

// Unsafe
/// Unprotected version of ManuallyDrop with backwards compatibility 
/// for SafeManuallyDrop features.
pub type AlwaysUnsafeManuallyDrop<T, Trig> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, Trig>;

// Safe
/// A protected version of SafeManuallyDrop with a function to execute 
/// a trigger function in case of undefined behavior of the ManuallyDrop logic.
pub type AlwaysSafeManuallyDrop<T, Trig> = crate::beh::safe::SafeManuallyDrop<T, Trig>;

/// A protected version of SafeManuallyDrop with a function to execute 
/// a panic in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_panic_trig")]
pub type PanicManuallyDrop<T> = crate::core::trig::panic::PanicManuallyDrop<T>;

/// Protected version of the SafeManuallyDrop with an execution 
/// function in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_hookfn_trig")]
pub type HookManuallyDrop<T> = crate::core::trig::hook::HookManuallyDrop<T>;

/// A protected version of SafeManuallyDrop with a function to count 
/// the amount of undefined behavior of the ManuallyDrop logic. 
/// The undefined behavior of CounterManuallyDrop will be the same 
/// as when using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
pub type CounterManuallyDrop<T> = crate::core::trig::counter::CounterManuallyDrop<T>;

cfg_if_safemode! {
	// Unsafe
	/// Depending on the build flag, a protected version of ManuallyDrop or 
	/// an unprotected version of ManuallyDrop with a default trigger.
	#if_not_safe(pub type ManuallyDrop<T> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, DefTrigManuallyDrop>;)

	// Safe
	/// Depending on the build flag, a protected version of ManuallyDrop or 
	/// an unprotected version of ManuallyDrop with a default trigger.
	#if_safe(pub type ManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, DefTrigManuallyDrop>;)
}

impl ManuallyDrop<()> {
	/// Depending on the build flag, a protected version of ManuallyDrop or 
	/// an unprotected version of ManuallyDrop with a default trigger.
	#[inline(always)]
	pub const fn is_safe_mode() -> bool {
		crate::core::flags::IS_SAFE_MODE
	}
}
