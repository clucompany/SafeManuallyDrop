//Copyright 2022-2024 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2022-2024
//

/*!

A safe version of ManuallyDrop with various features and options to track undefined behavior when working with ManuallyDrop.

# Use

### 1. easy

```rust,should_panic
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

		// to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
		#[allow(unused_unsafe)]
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

### 2. EasyStruct

```rust
// 1. In production code, it is recommended to use AutoSafe instead of AlwaysSafe,
// this will eliminate unnecessary checks in the release build, but leave
// them in the test build.
//
// 2. It is generally recommended to use Panic or Abort as a trigger for undefined behavior.
//
use SafeManuallyDrop::AlwaysSafePanicManuallyDrop as ManuallyDrop;

#[derive(Default, Debug)]
struct ControlDrop(usize);

// Properly created and validated MyLogicData structure.
#[derive(Default)]
struct MyLogicData {
	data: ManuallyDrop<ControlDrop>
}

impl MyLogicData {
	/// Exceptional logic. As a result, the original value will always be returned.
	pub fn ignore_mylogic_and_getdata(mut self) -> ControlDrop {
		// Note that you can only `take` once, any further operation with
		// ManuallyDrop will cause a panic.
		let data = unsafe {
			ManuallyDrop::take(&mut self.data)
		};

		// ManuallyDrop::forget analog forget(self).
		ManuallyDrop::forget(self);

		/*
			data logic
		*/

		data
	}
}

impl Drop for MyLogicData {
	fn drop(&mut self) {
		/*
			def logic
		*/
		println!("MyLogicData, indata: {:?}", self.data);

		/*
			Notification
			1. `ManuallyDrop` always requires it to be freed when it is no longer needed.
			2. Once `ManuallyDrop` is freed, you will not be able to read data from it
			3. You cannot drop `ManuallyDrop` twice.
			...

			You can remove the `unsafe` flags if you don't use the `always_compatible_stdapi` flag.
		*/
		unsafe {
			ManuallyDrop::drop(&mut self.data);
		}
	}
}

fn main() {
	{
		// run my logic
		let indata = MyLogicData::default();
		drop(indata);

		// This case will just make the logic default by executing the code in drop.
	}
	{
		// ignore_mylogic
		let indata = MyLogicData::default();
		let cd_data = indata.ignore_mylogic_and_getdata();

		println!("ignore_mylogic: {:?}", cd_data);

		// In this case, the standard reset logic is eliminated and another
		// specific principle is used, which is embedded in the function with data return.
	}
}
```

### 3. hook

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

	// to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
	#[allow(unused_unsafe)]
	unsafe {
		assert_eq!(data.is_next_trig(), false); // VALID
		ManuallyDrop::drop(&mut data); // VALID
		assert_eq!(data.is_next_trig(), true); // VALID

		// <<-- HOOK
		ManuallyDrop::drop(&mut data); // INVALID, COMBO DROP
	}
}
```

### 4. counter

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

	// to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
	#[allow(unused_unsafe)]
	let _data2 = unsafe { // <<-- INVALID BEH, COUNTER += 1 (=3)
		// !!! Trying to get the freed value
		ManuallyDrop::take(&mut data)
	};

	#[cfg(feature = "support_count_trig")]
	assert_eq!(ManuallyDrop::get_count_trig_events(), 3); // <-- The number of times the undefined behavior was triggered.
}
```

### 1. PlugAndPlay (Minimal, Panic)
```rust,ignore
[dependencies.SafeManuallyDrop]
version = "1.0.3"
default-features = false
features = [
	"always_check_in_case_debug_assertions",

	#"always_compatible_stdapi",

	"support_panic_trig",
	"always_deftrig_panic"
]
```

### 2. PlugAndPlay (Minimal, Abort)
```rust,ignore
[dependencies.SafeManuallyDrop]
version = "1.0.3"
default-features = false
features = [
	"always_check_in_case_debug_assertions",

	#"always_compatible_stdapi",

	"support_abort_trig",
	"always_deftrig_abort"
]
```

### 3. PlugAndPlay (Minimal, Hook)
```rust,ignore
[dependencies.SafeManuallyDrop]
version = "1.0.3"
default-features = false
features = [
	"always_check_in_case_debug_assertions",

	#"always_compatible_stdapi",

	"support_hookfn_trig",
	"always_deftrig_hookfn"
]
```

# cargo.toml -> features

```rust,ignore
// Flags:
//
// ManuallyDrop and AutoManuallyDrop are always type safe and are automatically
// checked on use if the debug_assertions flag is enabled (the flag is automatically
// enabled if test build, debug build, or env: CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true).
//
// (Also, AlwaysSafeManuallyDrop is always checked for safety when it is used, regardless of the flags.)
"always_check_in_case_debug_assertions",

// ManuallyDrop and AutoManuallyDrop are always checked when used,
// regardless of external flags.
//
// (Also, AlwaysSafeManuallyDrop is always checked for safety when it is used, regardless of the flags.)
//"always_safe_manuallydrop",

// Enable additional internal checks of the SafeManuallyDrop library when
// the debug_assertions flag is enabled (does not depend on the always_check_in_case_debug_assertions
// and always_safe_manuallydrop options). This flag type only applies to internal
// library function checks, it is independent of ManuallyDrop and its valid or invalid usage.
//
// "allow_extended_debug_assertions",

# Preserve unsafe fn flags even if functions are safe
# (may be required for additional compatibility with the standard API)
"always_compatible_stdapi",

// Always create a modular table of library flags used in the build.
// (crate::core::flags)
"flags_table",

// Trigs:
//
// Ability to determine if an empty loop trigger has been executed.
"support_istrig_loop",

// Support for PanicManuallyDrop, in case of undefined behavior
// of ManuallyDrop there will be a panic.
"support_panic_trig",

// Support for AbortManuallyDrop, in case of undefined behavior
// of ManuallyDrop there will be a abort. (Note that this feature requires std.)
//"support_abort_trig",

// HookManuallyDrop support, in case of undefined HookManuallyDrop behavior,
// the hook function will be called.
"support_hookfn_trig",

// Support for CounterManuallyDrop, in case of undefined behavior,
// CounterManuallyDrop will add +1 to the counter.
//"support_count_trig",

// The behavior for the simple AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop type will always
// cause a panic in case of undefined behavior.
//"always_deftrig_panic",

// The behavior for the simple AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop type will always
// cause a abort in case of undefined behavior.
//"always_deftrig_abort",

// The behavior for the simple AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop type will always
// call the hook function in case of undefined behavior.
"always_deftrig_hookfn",

// The behavior for the simple AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop type will always call
// the +1 counter function in case of undefined behavior.
//"always_deftrig_count",

// The behavior for the simple type AutoSafeManuallyDrop/AlwaysSafeManuallyDrop/ManuallyDrop will always call
// the eternal loop function in case of undefined behavior.
//"always_deftrig_loop"
```

*/

#![allow(non_snake_case)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::let_and_return)]
#![allow(clippy::needless_if)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "support_abort_trig"), no_std)]

use crate::beh::auto::cfg_if_safemode;

/// The insecure standard version of ManuallyDrop
// rustfmt::skip why?: the oddity forces fmt to convert ::core to core, breaking the library.
#[rustfmt::skip]
pub use ::core::mem::ManuallyDrop as UnsafeStdManuallyDrop;

/// The core of the library that defines the basic primitives.
pub mod core {
	pub mod state;

	#[cfg_attr(docsrs, doc(cfg(feature = "flags_table")))]
	#[cfg(any(test, feature = "flags_table"))]
	pub mod flags;

	#[cfg_attr(docsrs, doc(cfg(feature = "flags_table")))]
	#[cfg(not(any(test, feature = "flags_table")))]
	pub mod flags {
		/// Whether a table of build flags to use was created when the library was compiled.
		pub const BUILD_FLAG_TABLE_CREATED: bool = false;
	}

	/// Implementation of behavior in case of detection of
	/// undefined manual memory management.
	pub mod trig;
}

/// Internal code generation
mod macro_codegen;

/// Internal extended_debug_assertions
mod extended_debug_assertions;

/// Safe and insecure implementations of manual memory management.
pub mod beh {
	pub mod auto;
	pub mod safe;
	pub mod r#unsafe;
}

// PANIC
/// A protected version of ManuallyDrop with a function to
/// execute a panic in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_panic_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_panic_trig")))]
pub type AlwaysSafePanicManuallyDrop<T> = crate::core::trig::panic::AlwaysSafePanicManuallyDrop<T>;

/// A secure or non-secure version of ManuallyDrop with a function to trigger
/// a panic in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_panic_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_panic_trig")))]
pub type AutoSafePanicManuallyDrop<T> = crate::core::trig::panic::AutoSafePanicManuallyDrop<T>;

// ABORT
/// A protected version of ManuallyDrop with a function to
/// execute a abort in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_abort_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_abort_trig")))]
pub type AlwaysSafeAbortManuallyDrop<T> = crate::core::trig::abort::AlwaysSafeAbortManuallyDrop<T>;

/// A secure or non-secure version of ManuallyDrop with a function to trigger
/// a abort in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_abort_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_abort_trig")))]
pub type AutoSafeAbortManuallyDrop<T> = crate::core::trig::abort::AutoSafeAbortManuallyDrop<T>;

// HOOK
/// Protected or unprotected version of ManuallyDrop with function
/// execution in case of undefined behavior of ManuallyDrop logic.
#[cfg(feature = "support_hookfn_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_hookfn_trig")))]
pub type AlwaysSafeHookManuallyDrop<T> = crate::core::trig::hook::AlwaysSafeHookManuallyDrop<T>;

/// Protected or unprotected version of ManuallyDrop with function
/// execution in case of undefined behavior of ManuallyDrop logic.
#[cfg(feature = "support_hookfn_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_hookfn_trig")))]
pub type AutoSafeHookManuallyDrop<T> = crate::core::trig::hook::AutoSafeHookManuallyDrop<T>;

// COUNTER
/// A protected version of SafeManuallyDrop with a function to count
/// the amount of undefined behavior of the ManuallyDrop logic.
/// The undefined behavior of CounterManuallyDrop will be the same
/// as when using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_count_trig")))]
pub type AlwaysSafeCounterManuallyDrop<T> =
	crate::core::trig::counter::AlwaysSafeCounterManuallyDrop<T>;

/// A secure or non-secure version of SafeManuallyDrop with a
/// function to count the undefined behavior of the ManuallyDrop logic.
/// The undefined behavior of CounterManuallyDrop will be the same as when
/// using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
#[cfg_attr(docsrs, doc(cfg(feature = "support_count_trig")))]
pub type AutoSafeCounterManuallyDrop<T> =
	crate::core::trig::counter::AutoSafeCounterManuallyDrop<T>;

// EMPTY
/// The safe version of ManuallyDrop loops the current thread in case of undefined behavior,
/// and using the `support_istrig_loop` build flag, you can determine whether the
/// thread looped.
pub type AlwaysSafeEmptyLoopManuallyDrop<T> =
	crate::core::trig::r#loop::AlwaysSafeEmptyLoopManuallyDrop<T>;

/// The safe or unsafe version of ManuallyDrop loops the current thread in case
/// of undefined behavior, and with the build flag `support_istrig_loop` you
/// can determine if the thread is looped.
pub type AutoSafeEmptyLoopManuallyDrop<T> =
	crate::core::trig::r#loop::AutoSafeEmptyLoopManuallyDrop<T>;

// AUTO
/// Depending on the build flag, a protected version of ManuallyDrop or
/// an unprotected version of ManuallyDrop with a default trigger.
///
/// features:
/// ```text
/// if always_safe_manuallydrop | ( always_check_in_case_debug_assertions && debug_assertions ) -> SafeManuallyDrop
/// else -> UnsafeManuallyDrop
/// ```
pub type AutoSafeManuallyDrop<T> =
	crate::beh::auto::AutoSafeManuallyDrop<T, crate::core::trig::DefTrigManuallyDrop>;

// ALWAYS_UNSAFE
/// Unprotected version of ManuallyDrop with backwards compatibility
/// for SafeManuallyDrop features.
pub type AlwaysUnsafeManuallyDrop<T, Trig> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, Trig>;

// ALWAYS_SAFE
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
	pub const SAFE_MODE: bool = cfg_if_safemode! {
		#if_safe() {
			true
		}else {
			false
		}
	};

	/// Depending on the build flag, a protected version of ManuallyDrop or
	/// an unprotected version of ManuallyDrop with a default trigger.
	#[inline(always)]
	pub const fn is_safe_mode() -> bool {
		Self::SAFE_MODE
	}
}
