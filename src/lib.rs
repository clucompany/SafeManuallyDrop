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

// #Ulin Project 2022
//

/*!

A safe version of ManuallyDrop with various features and options to track undefined behavior when working with ManuallyDrop.

# Use

### 1. easy

```should_panic
use SafeManuallyDrop::ManuallyDrop;
use std::ops::Deref;

fn main() {
	/*
		ManuallyDrop - Depending on the build flag, a protected version of ManuallyDrop 
		or an unprotected version of ManuallyDrop with a default trigger. 
	*/
	if ManuallyDrop::is_safe_mode() {
		// ManuallyDrop is protected, let's do the standard behavior ManuallyDrop
		// but at the end we'll make the undefined behavior ManuallyDrop.
		
		// 
		let mut data = ManuallyDrop::new(vec![1, 2, 3, 4]);
		println!("data: {:?}", data.deref());
		
		#[allow(unused_unsafe)] // to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
		unsafe {
			assert_eq!(data.is_next_trig(), false); // VALID
			ManuallyDrop::drop(&mut data); // VALID
			assert_eq!(data.is_next_trig(), true); // VALID
			
			// <<-- PANIC
			/*
				thread 'main' panicked at 'Undefined behavior when using 
				ManuallyDrop(combo_replace_manudropstate), instead of the expected default 
				state, the current state: DropModeTrig.', src/core/trig/hook.rs:14:5
			*/
			ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
		}
	}else {
		println!("#[0] ManuallyDrop is an alias for AutoSafeManuallyDrop, ");
		println!("#[1] ManuallyDrop in the release build has no protection by default,");
		println!("#[2] if ManuallyDrop is not protected it will be the same as in std.");
		println!("#[3] To run the protected version, use `cargo run --example easy` or ");
		println!("`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=\"true\" cargo run --example easy --release`");
		println!();
		println!("Or use concrete types instead of auto (AutoSafeManuallyDrop, AutoSafePanicManuallyDrop, AutoSafeHookManuallyDrop, AutoSafeCounterManuallyDrop, AlwaysSafeManuallyDrop, AlwaysSafePanicManuallyDrop, AlwaysSafeHookManuallyDrop, AlwaysSafeCounterManuallyDrop) specific data types with specific behavior.");
	}
}
```

### 2. hook

```rust
use std::ops::Deref;

// For better performance, we recommend using AutoSafeHookManuallyDrop instead 
// of AlwaysSafeHookManuallyDrop. The AutoSafeHookManuallyDrop type depends on 
// the type of build, debug or release will be with the safe or insecure version
// of ManuallyDrop.
use SafeManuallyDrop::AlwaysSafeHookManuallyDrop as ManuallyDrop;

fn main() {
	unsafe {
		ManuallyDrop::set_hook(|args| {
			println!("!!!{:?}", args);
			
			for _ in 0..3 {
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}
			
			println!("exit");
			std::process::exit(0x0100);
		});
	}
	
	let mut data = ManuallyDrop::new(vec![1, 2, 3, 4]);
	println!("data: {:?}", data.deref());
	
	#[allow(unused_unsafe)] // to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
	unsafe {
		assert_eq!(data.is_next_trig(), false); // VALID
		ManuallyDrop::drop(&mut data); // VALID
		assert_eq!(data.is_next_trig(), true); // VALID
		
		// <<-- HOOK
		ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
	}
}
```

### 3. counter

```rust
// Let me remind you that CounterManuallyDrop by behavior allows undefined 
// behavior in the same way as ManuallyDrop, but, unlike ManuallyDrop, 
// Counter keeps a counter of the number of undefined behavior triggers.

// !!!!
// CounterManuallyDrop is experimental and changes the behavior of 
// the trigger trait for all types.

#[cfg(feature = "support_count_trig")]
use SafeManuallyDrop::AutoSafeCounterManuallyDrop as ManuallyDrop;

#[cfg(not(feature = "support_count_trig"))]
use SafeManuallyDrop::ManuallyDrop;

use std::ops::Deref;

#[allow(unreachable_code)]
fn main() {
	#[cfg(not(feature = "support_count_trig"))] {
		println!("To run the example, a build with feature: support_count_trig is required,");
		println!("exp: cargo run --example counter --all-features");
		println!("end.");
		
		return;
	}
	
	let mut data = ManuallyDrop::new(&[1, 2, 3, 4]);
	println!("data: {:?}", data.deref());
	
	#[allow(unused_unsafe)] // feature !always_compatible_stdapi
	unsafe {
		assert_eq!(data.is_next_trig(), false); // VALID, triggers never fired
		
		// =================
		// !!! ATTENTION !!!
		// =================
		// Procedure:
		// 1. Free up memory and try to read it
		// 2. Re-free memory
		ManuallyDrop::drop(&mut data); // VALID
		assert_eq!(data.is_next_trig(), true); // VALID, counter trigger worked.
		
		ManuallyDrop::drop(&mut data); // <<-- INVALID BEH, COUNTER += 1 (=1), COMBO DROP
	}
	
	// !!! Reading an already freed value
	println!("data: {:?}", &data); // <<-- INVALID BEH, COUNTER += 1 (=2)
	
	#[allow(unused_unsafe)] // to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
	let _data2 = unsafe { // <<-- INVALID BEH, COUNTER += 1 (=3)
		// !!! Trying to get the freed value
		ManuallyDrop::take(&mut data)
	};
	
	#[cfg(feature = "support_count_trig")]
	assert_eq!(ManuallyDrop::get_count_trig_events(), 3); // <-- The number of times the undefined behavior was triggered.
}
```

# cargo.toml -> features

```ignore
// The ManuallyDrop type is always SafeManuallyDrop if the debug_assertions flag 
// is active (test build, debug build).
"always_check_in_case_debug_assertions"

// The AutoSafeManuallyDrop/ManuallyDrop type is always SafeManuallyDrop, 
// i.e. with traceable behavior.
#"always_safe_manuallydrop"

// For compatibility with older software, create a separate crate::core::hook 
// instead of the new crate::core::trig::hook.
"enable_deprecated_hook"

// Mark functions as unsafe even if they are safe 
// for std API compatibility.
"always_compatible_stdapi"

// Ability to determine if an empty loop trigger has been executed.
"support_istrig_loop"

// Support for PanicManuallyDrop, in case of undefined behavior 
// of PanicManuallyDrop there will be a panic.
"support_panic_trig"

// HookManuallyDrop support, in case of undefined HookManuallyDrop behavior, 
// the hook function will be called.
"support_hookfn_trig"

// Support for CounterManuallyDrop, in case of undefined behavior, 
// CounterManuallyDrop will add +1 to the counter.
#"support_count_trig"

// The behavior for the simple AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop type will always 
// cause a panic in case of undefined behavior.
#"always_deftrig_panic"

// The behavior for the simple AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop type will always 
// call the hook function in case of undefined behavior.
#"always_deftrig_hookfn"

// The behavior for the simple AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop type will always call 
// the +1 counter function in case of undefined behavior.
#"always_deftrig_count"

// The behavior for the simple type AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop will always call 
// the eternal loop function in case of undefined behavior.
#"always_deftrig_loop"

// INFO:
// If the behavior for the general AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop is not fixed, 
// the behavior will be determined according to the following scheme:
// 
// 	always_deftrig_panic not exists AND
// 	always_deftrig_hookfn not exists AND
// 	always_deftrig_count not exists AND
// 	always_deftrig_loop not exists THEN
// 
// 	support_hookfn_trig -> Hook,	else:
// 	support_panic_trig -> Panic,	else:
// 	support_count_trig -> Count,	else:
// 		Loop
// 
```

*/

// =============
// !!ATTENTION!!
// =============
// Anything related to deprecated features 
// will be removed in a future regression release.
//
// 0.1.0
// 0.1.2
// ... unk
// 0.1.5
// 0.1.6
// 0.1.7
// 0.1.8 <-- current
// 1.0.0 <--
//

#![allow(non_snake_case)]

#![no_std]

/// The insecure standard version of ManuallyDrop
pub use ::core::mem::ManuallyDrop as UnsafeStdManuallyDrop;

/// The core of the library that defines the basic primitives.
pub mod core {
	/// Safe States for ManuallyDrop
	pub mod state;
	
	/// Flags used when building this library
	//#[macro_use]
	pub mod flags;
	
	/// Protected version of the SafeManuallyDrop with an execution 
	/// function in case of undefined behavior of the ManuallyDrop logic.
	#[doc(hidden)]
	#[deprecated(since = "0.1.2", note = "Use `SafeManuallyDrop::core::trig::hook` instead")]
	#[cfg(feature = "enable_deprecated_hook")]
	pub mod hook;
	
	/// Implementation of behavior in case of detection of 
	/// undefined manual memory management.
	pub mod trig;
}

/// Internal code generation
#[doc(hidden)]
mod macro_codegen;

#[doc(hidden)]
#[deprecated(since = "0.1.5", note = "Use `SafeManuallyDrop::core::state` instead")]
pub use crate::core::state as state;

#[doc(hidden)]
#[deprecated(since = "0.1.5", note = "Use `SafeManuallyDrop::core::hook` instead")]
#[allow(deprecated)]
#[cfg(feature = "enable_deprecated_hook")]
pub use crate::core::hook as hook;

#[doc(hidden)]
#[deprecated(since = "0.1.5", note = "Use `SafeManuallyDrop::core::flags` instead")]
pub use crate::core::flags as flags;

/// Safe and insecure implementations of manual memory management.
pub mod beh {
	/// Insecure standard implementation of manual memory management.
	pub mod r#unsafe;
		
	/// A safe version of the insecure manual control of freeing memory.
	pub mod safe;
	
	/// Depending on the build flag, a protected version of ManuallyDrop 
	/// or an unprotected version of ManuallyDrop.
	pub mod auto;
}

/// A secure or non-secure version of ManuallyDrop with a function to trigger 
/// a panic in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_panic_trig")]
#[doc(hidden)]
#[deprecated(since = "0.1.7", note = "Use `SafeManuallyDrop::AlwaysSafePanicManuallyDrop` instead")]
pub type PanicManuallyDrop<T> = AlwaysSafePanicManuallyDrop<T>;
/// A secure or non-secure version of ManuallyDrop with a function to trigger 
/// a panic in case of undefined behavior of the ManuallyDrop logic.

#[cfg(feature = "support_panic_trig")]
pub type AlwaysSafePanicManuallyDrop<T> = crate::core::trig::panic::AlwaysSafePanicManuallyDrop<T>;


/// A secure or non-secure version of ManuallyDrop with a function to trigger 
/// a panic in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_panic_trig")]
pub type AutoSafePanicManuallyDrop<T> = crate::core::trig::panic::AutoSafePanicManuallyDrop<T>;

/// Protected or unprotected version of ManuallyDrop with function 
/// execution in case of undefined behavior of ManuallyDrop logic. 
#[cfg(feature = "support_hookfn_trig")]
#[doc(hidden)]
#[deprecated(since = "0.1.7", note = "Use `SafeManuallyDrop::AlwaysSafeHookManuallyDrop` instead")]
pub type HookManuallyDrop<T> = AlwaysSafeHookManuallyDrop<T>;

/// Protected or unprotected version of ManuallyDrop with function 
/// execution in case of undefined behavior of ManuallyDrop logic. 
#[cfg(feature = "support_hookfn_trig")]
pub type AlwaysSafeHookManuallyDrop<T> = crate::core::trig::hook::AlwaysSafeHookManuallyDrop<T>;


/// Protected or unprotected version of ManuallyDrop with function 
/// execution in case of undefined behavior of ManuallyDrop logic. 
#[cfg(feature = "support_hookfn_trig")]
pub type AutoSafeHookManuallyDrop<T> = crate::core::trig::hook::AutoSafeHookManuallyDrop<T>;

/// A protected version of SafeManuallyDrop with a function to count 
/// the amount of undefined behavior of the ManuallyDrop logic. 
/// The undefined behavior of CounterManuallyDrop will be the same 
/// as when using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
#[doc(hidden)]
#[deprecated(since = "0.1.7", note = "Use `SafeManuallyDrop::AlwaysSafeCounterManuallyDrop` instead")]
pub type CounterManuallyDrop<T> = crate::core::trig::counter::AlwaysSafeCounterManuallyDrop<T>;

/// A protected version of SafeManuallyDrop with a function to count 
/// the amount of undefined behavior of the ManuallyDrop logic. 
/// The undefined behavior of CounterManuallyDrop will be the same 
/// as when using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
pub type AlwaysSafeCounterManuallyDrop<T> = crate::core::trig::counter::AlwaysSafeCounterManuallyDrop<T>;


/// A secure or non-secure version of SafeManuallyDrop with a 
/// function to count the undefined behavior of the ManuallyDrop logic. 
/// The undefined behavior of CounterManuallyDrop will be the same as when 
/// using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
pub type AutoSafeCounterManuallyDrop<T> = crate::core::trig::counter::AutoSafeCounterManuallyDrop<T>;

/// The safe version of ManuallyDrop loops the current thread in case of undefined behavior, 
/// and using the `support_istrig_loop` build flag, you can determine whether the 
/// thread looped. 
#[doc(hidden)]
#[deprecated(since = "0.1.7", note = "Use `SafeManuallyDrop::AlwaysSafeEmptyLoopManuallyDrop` instead")]
pub type EmptyLoopManuallyDrop<T> = crate::core::trig::r#loop::AlwaysSafeEmptyLoopManuallyDrop<T>;

/// The safe version of ManuallyDrop loops the current thread in case of undefined behavior, 
/// and using the `support_istrig_loop` build flag, you can determine whether the 
/// thread looped. 
pub type AlwaysSafeEmptyLoopManuallyDrop<T> = crate::core::trig::r#loop::AlwaysSafeEmptyLoopManuallyDrop<T>;

/// The safe or unsafe version of ManuallyDrop loops the current thread in case 
/// of undefined behavior, and with the build flag `support_istrig_loop` you 
/// can determine if the thread is looped.
pub type AutoSafeEmptyLoopManuallyDrop<T> = crate::core::trig::r#loop::AutoSafeEmptyLoopManuallyDrop<T>;

/// Depending on the build flag, a protected version of ManuallyDrop or 
/// an unprotected version of ManuallyDrop with a default trigger.
/// 
/// features:
/// ```ignore
/// if always_safe_manuallydrop | ( always_check_in_case_debug_assertions && debug_assertions ) -> SafeManuallyDrop
/// else -> UnsafeManuallyDrop
/// ```
pub type AutoSafeManuallyDrop<T> = crate::beh::auto::AutoSafeManuallyDrop<T, crate::core::trig::DefTrigManuallyDrop>;

// Unsafe
/// Unprotected version of ManuallyDrop with backwards compatibility 
/// for SafeManuallyDrop features.
pub type AlwaysUnsafeManuallyDrop<T, Trig> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, Trig>;

// Safe
/// A protected version of SafeManuallyDrop with a function to execute 
/// a trigger function in case of undefined behavior of the ManuallyDrop logic.
pub type AlwaysSafeManuallyDrop<T, Trig> = crate::beh::safe::SafeManuallyDrop<T, Trig>;


/// Depending on the build flag, a protected version of ManuallyDrop or 
/// an unprotected version of ManuallyDrop with a default trigger. 
/// (!! It is an alias to AutoSafeManuallyDrop, the type is needed for clarity 
/// and compatibility in codes)
pub type ManuallyDrop<T> = AutoSafeManuallyDrop<T>;

impl AutoSafeManuallyDrop<()> {
	/// Depending on the build flag, a protected version of ManuallyDrop or 
	/// an unprotected version of ManuallyDrop with a default trigger.
	#[inline(always)]
	pub const fn is_safe_mode() -> bool {
		cfg_if_safemode! {
			#if_safe() {
				true
			}else {
				false
			}
		}
	}
}
