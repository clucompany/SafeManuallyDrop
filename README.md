# SafeManuallyDrop
[![CI](https://github.com/clucompany/SafeManuallyDrop/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/SafeManuallyDrop/actions/workflows/CI.yml)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/SafeManuallyDrop)](https://crates.io/crates/SafeManuallyDrop)
[![Documentation](https://docs.rs/SafeManuallyDrop/badge.svg)](https://docs.rs/SafeManuallyDrop)

A safe version of ManuallyDrop with various features and options to track undefined behavior when working with ManuallyDrop.

# Use

### 1. easy

```rust
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

```
// The ManuallyDrop type is always SafeManuallyDrop if the debug_assertions flag 
// is active (test build, debug build).
"always_check_in_case_debug_assertions"

// The AutoSafeManuallyDrop/ManuallyDrop type is always SafeManuallyDrop, 
// i.e. with traceable behavior.
#"always_safe_manuallydrop"

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

# License

Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
