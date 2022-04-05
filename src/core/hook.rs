
use core::fmt::Arguments;

/*
	=============
	!!ATTENTION!!
	=============
	Module is outdated.
	#[deprecated(since = "0.1.2", note = "Use `SafeManuallyDrop::core::trig::hook` instead")]
*/

/// The hook function to be executed in case of 
/// undefined behavior of the HookManuallyDrop.
#[doc(hidden)]
#[deprecated(since = "0.1.2", note = "Use `SafeManuallyDrop::core::trig::hook::HookFunction` instead")]
pub type HookFunction = fn(Arguments) -> !;

/// Hook is a function to be executed in case of 
/// undefined behavior of the ManuallyDrop logic.
#[doc(hidden)]
#[allow(deprecated)] // internal
#[deprecated(since = "0.1.2", note = "Use `SafeManuallyDrop::core::trig::hook::HOOK` instead")]
static mut HOOK: HookFunction = |args| {
	panic!("{}", args);
};

// UNSAFE_MODE TODO!!
/// Set a hook function to be executed in case of 
/// undefined behavior of ManuallyDrop logic.
#[doc(hidden)]
#[allow(deprecated)] // internal
#[deprecated(since = "0.1.2", note = "Use `SafeManuallyDrop::core::trig::hook::set_hook` instead")]
#[inline(always)]
pub unsafe fn set_hook(function: HookFunction) {
	HOOK = function;
}

/// Get a hook function that will be executed in case of 
/// undefined behavior of the ManuallyDrop logic.
#[doc(hidden)]
#[allow(deprecated)] // internal
#[deprecated(since = "0.1.2", note = "Use `SafeManuallyDrop::core::trig::hook::take_hook` instead")]
#[inline(always)]
pub fn take_hook() -> HookFunction {
	// TODO UNSAFE MODE
	unsafe { HOOK }
}

/// Execute a hook function that is always executed in case of 
/// undefined behavior of the ManuallyDrop logic.
#[doc(hidden)]
#[allow(deprecated)] // internal
#[deprecated(since = "0.1.2", note = "Use `SafeManuallyDrop::core::trig::hook::run_hook` instead")]
#[inline(always)]
pub fn run_hook(args: Arguments) -> ! {
	(take_hook())(args)
}

