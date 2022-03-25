
use core::fmt::Arguments;
use crate::core::trig::TrigManuallyDrop;

/// In case of undefined behavior of manual memory management, execute an external hook.
pub enum HookFnTrigManuallyDrop {}
/// Protected version of the SafeManuallyDrop with an execution function in case of undefined behavior of the ManuallyDrop logic.
pub type HookManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, HookFnTrigManuallyDrop>;
/// The hook function to be executed in case of undefined behavior of the HookManuallyDrop.
pub type HookFunction = fn(Arguments) -> trig_manuallydrop_returntype!();

/// Hook is a function to be executed in case of undefined behavior of the ManuallyDrop logic.
static mut HOOK: HookFunction = |args| {
	panic!("{}", args);
};

// UNSAFE_MODE TODO!!
/// Set a hook function to be executed in case of undefined behavior of ManuallyDrop logic.
#[inline(always)]
pub unsafe fn set_hook(function: HookFunction) {
	HOOK = function;
}

/// Get a hook function that will be executed in case of undefined behavior of the ManuallyDrop logic.
#[inline(always)]
pub fn take_hook() -> HookFunction {
	// TODO UNSAFE MODE
	unsafe { HOOK }
}

/// Execute a hook function that is always executed in case of undefined behavior of the ManuallyDrop logic.
#[inline(always)]
pub fn run_hook<'a>(args: Arguments<'a>) -> trig_manuallydrop_returntype!() {
	let trig_fn = take_hook();
	trig_fn(args)
}

impl TrigManuallyDrop for HookFnTrigManuallyDrop {
	#[inline(always)]
	fn trig_next_invalid_beh<'a>(a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		crate::core::trig::hook::run_hook(a);
	}
}

impl HookManuallyDrop<()> {
	/// Set a hook function to be executed in case of undefined behavior of ManuallyDrop logic.
	#[inline(always)]
	pub unsafe fn set_hook(function: HookFunction) {
		crate::core::trig::hook::set_hook(function)
	}
	
	/// Get a hook function that will be executed in case of undefined behavior of the ManuallyDrop logic.
	#[inline(always)]
	pub fn take_hook() -> HookFunction {
		crate::core::trig::hook::take_hook()
	}
	
	/// Execute a hook function that is always executed in case of undefined behavior of the ManuallyDrop logic.
	#[inline(always)]
	pub fn run_hook<'a>(args: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		crate::core::trig::hook::run_hook(args)
	}
}
