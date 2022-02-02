
use core::fmt::Arguments;

pub type HookFunction = fn(Arguments) -> !;

static mut HOOK: HookFunction = |args| {
	panic!("{}", args);
};

// UNSAFE_MODE TODO!!
#[inline(always)]
pub unsafe fn set_hook(function: HookFunction) {
	HOOK = function;
}

#[inline(always)]
pub fn take_hook() -> HookFunction {
	// TODO UNSAFE MODE
	unsafe { HOOK }
}

#[inline(always)]
pub fn run_hook(args: Arguments) -> ! {
	(take_hook())(args)
}

