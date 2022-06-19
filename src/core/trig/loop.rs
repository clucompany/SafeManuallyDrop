
use core::fmt::Arguments;
use crate::core::trig::TrigManuallyDrop;

#[cfg(feature = "support_istrig_loop")]
mod _support_istrig_loop {
	use core::sync::atomic::Ordering;
	use core::sync::atomic::AtomicBool;

	pub static IS_TRIG_LOOPSAFEMANUALLYDROP: AtomicBool = AtomicBool::new(false);

	pub const DEF_SETORDERING: Ordering = Ordering::SeqCst;
	pub const DEF_GETORDERING: Ordering = Ordering::Relaxed;
}

//

/// The safe version of ManuallyDrop loops the current thread in case of undefined behavior, 
/// and using the `support_istrig_loop` build flag, you can determine whether the 
/// thread looped. 
pub type AlwaysSafeEmptyLoopManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, EmptyLoopTrigManuallyDrop>;

/// The safe or unsafe version of ManuallyDrop loops the current thread in case 
/// of undefined behavior, and with the build flag `support_istrig_loop` you 
/// can determine if the thread is looped.
pub type AutoSafeEmptyLoopManuallyDrop<T> = crate::beh::auto::AutoSafeManuallyDrop<T, EmptyLoopTrigManuallyDrop>;

/// Starts looping the current thread on error, and with the 
/// `support_istrig_loop` build flag you can determine if a thread is looped.
pub enum EmptyLoopTrigManuallyDrop {}

impl TrigManuallyDrop for EmptyLoopTrigManuallyDrop {
	#[inline(always)]
	fn trig_next_invalid_beh<'a>(_a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		#[cfg(feature = "support_istrig_loop")] {
			unsafe {
				crate::core::trig::r#loop::trig_next_invalid_beh()
			}
		}
		
		loop {}
	}
}

#[cfg(feature = "support_istrig_loop")]
#[inline]
pub unsafe fn trig_next_invalid_beh() {
	_support_istrig_loop::IS_TRIG_LOOPSAFEMANUALLYDROP.store(
		true, 
		_support_istrig_loop::DEF_SETORDERING
	);
}

/// Get the number of times the undefined behavior was triggered.
#[cfg(feature = "support_istrig_loop")]
#[inline]
pub fn is_trig_next_invalid_beh() -> bool {
	_support_istrig_loop::IS_TRIG_LOOPSAFEMANUALLYDROP.load(
		_support_istrig_loop::DEF_GETORDERING
	)
}

/// Get the number of times the undefined behavior was triggered.
#[cfg(not(feature = "support_istrig_loop"))]
#[inline]
pub fn is_trig_next_invalid_beh() -> bool {
	false
}


impl AutoSafeEmptyLoopManuallyDrop<()> {
	/// Get the number of times the undefined behavior was triggered.
	#[inline(always)]
	pub fn is_trig_next_invalid_beh() -> bool {
		crate::core::trig::r#loop::is_trig_next_invalid_beh()
	}
}

