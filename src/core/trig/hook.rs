
use core::fmt::Arguments;
use crate::core::trig::TrigManuallyDrop;


/// Protected version of the SafeManuallyDrop with an execution 
/// function in case of undefined behavior of the ManuallyDrop logic.
pub type AlwaysSafeHookManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, HookFnTrigManuallyDrop>;

/// Protected or unprotected version of ManuallyDrop with function 
/// execution in case of undefined behavior of ManuallyDrop logic. 
pub type AutoSafeHookManuallyDrop<T> = crate::beh::auto::AutoSafeManuallyDrop<T, HookFnTrigManuallyDrop>;

/// The hook function to be executed in case of 
/// undefined behavior of the HookManuallyDrop.
pub type HookFunction = fn(Arguments) -> trig_manuallydrop_returntype!();

/// In case of undefined behavior of manual memory management, execute an external hook.
pub enum HookFnTrigManuallyDrop {}

/// Hook is a function to be executed in case of 
/// undefined behavior of the ManuallyDrop logic.
static mut HOOK: HookFunction = |args| {
	panic!("{}", args);
};

// UNSAFE_MODE TODO!!
/// Set a hook function to be executed in case of 
/// undefined behavior of ManuallyDrop logic.
#[inline(always)]
pub unsafe fn set_hook(function: HookFunction) {
	HOOK = function;
}

/// Get a hook function that will be executed in case of 
/// undefined behavior of the ManuallyDrop logic.
#[inline(always)]
pub fn take_hook() -> HookFunction {
	// TODO UNSAFE MODE
	unsafe { HOOK }
}

/// Execute a hook function that is always executed in case of 
/// undefined behavior of the ManuallyDrop logic.
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

/// ==============================
/// !!!May change in the future!!!
/// ==============================
/// You can quickly organize your anonymous hook function, just give it 
/// an empty struct as its type, give the struct the traits Default and FnOnce. 
/// Works only in a nightlight using certain flags.
/// 
/// TODO, Exp support <https://github.com/rust-lang/rust/issues/35121>
impl<F> TrigManuallyDrop for F where F: Default + FnOnce(Arguments<'_>) {
	#[inline(always)]
	fn trig_next_invalid_beh<'a>(a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		let function: F = Default::default();
		
		function(a); // <-- exp return: !
		panic!("Be sure to read the documentation an anonymous function should abort a thread, but it's not possible to do this in safe rust at the moment.");
	}
}
/*#[cfg(test)]
#[test]
fn test_anonym_hook() {
	#[derive(Default)]
	struct MyHookManuallyDrop();
	
	impl<'a> FnOnce(Arguments<'a>) for MyHookManuallyDrop {
		type Output = ();
		
		fn call_once(self, args: Args) -> Self::Output {
			
		}
	}
}*/

impl AutoSafeHookManuallyDrop<()> {
	/// Set a hook function to be executed in case of undefined behavior 
	/// of ManuallyDrop logic.
	#[inline(always)]
	pub unsafe fn set_hook(function: HookFunction) {
		crate::core::trig::hook::set_hook(function)
	}
	
	/// Get a hook function that will be executed in case of undefined behavior 
	/// of the ManuallyDrop logic.
	#[inline(always)]
	pub fn take_hook() -> HookFunction {
		crate::core::trig::hook::take_hook()
	}
	
	/// Execute a hook function that is always executed in case of undefined behavior 
	/// of the ManuallyDrop logic.
	#[inline(always)]
	pub fn run_hook<'a>(args: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		crate::core::trig::hook::run_hook(args)
	}
}

