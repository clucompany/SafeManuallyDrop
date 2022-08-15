# SafeManuallyDrop
[![CI](https://github.com/clucompany/SafeManuallyDrop/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/SafeManuallyDrop/actions/workflows/CI.yml)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/SafeManuallyDrop)](https://crates.io/crates/SafeManuallyDrop)
[![Documentation](https://docs.rs/SafeManuallyDrop/badge.svg)](https://docs.rs/SafeManuallyDrop)

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
	
	#[allow(unused_unsafe)] // to avoid warning if the always_compatible_stdapi flag is not used (can be removed)
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
// "allow_fullinternal_debug_assertions",

# Preserve unsafe fn flags even if functions are safe 
# (may be required for additional compatibility with the standard API)
"always_compatible_stdapi",

// Always create a modular table of library flags used in the build.
// (crate::core::flags)
"always_build_flagstable",

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

# License

Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
