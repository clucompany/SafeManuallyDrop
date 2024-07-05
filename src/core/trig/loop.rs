use crate::core::trig::TrigManuallyDrop;
use core::fmt::Arguments;

#[cfg(feature = "support_istrig_loop")]
mod _support_istrig_loop {
	use core::sync::atomic::AtomicBool;
	use core::sync::atomic::Ordering;

	pub static IS_TRIG_LOOPSAFEMANUALLYDROP: AtomicBool = AtomicBool::new(false);

	pub const DEF_SETORDERING: Ordering = Ordering::SeqCst;
	pub const DEF_GETORDERING: Ordering = Ordering::Relaxed;
}

//

/// The safe version of ManuallyDrop loops the current thread in case of undefined behavior,
/// and using the `support_istrig_loop` build flag, you can determine whether the
/// thread looped.
pub type AlwaysSafeEmptyLoopManuallyDrop<T> =
	crate::beh::safe::SafeManuallyDrop<T, EmptyLoopTrigManuallyDrop>;

/// The safe or unsafe version of ManuallyDrop loops the current thread in case
/// of undefined behavior, and with the build flag `support_istrig_loop` you
/// can determine if the thread is looped.
pub type AutoSafeEmptyLoopManuallyDrop<T> =
	crate::beh::auto::AutoSafeManuallyDrop<T, EmptyLoopTrigManuallyDrop>;

/// Starts looping the current thread on error, and with the
/// `support_istrig_loop` build flag you can determine if a thread is looped.
pub enum EmptyLoopTrigManuallyDrop {}

impl TrigManuallyDrop for EmptyLoopTrigManuallyDrop {
	#[inline]
	fn trig_next_invalid_beh(_a: Arguments<'_>) -> trig_manuallydrop_returntype!() {
		#[cfg(feature = "support_istrig_loop")]
		unsafe {
			crate::core::trig::r#loop::trig_next_invalid_beh();
		}

		#[allow(clippy::empty_loop)]
		#[inline(never)]
		#[cold]
		fn _cold_loop() -> trig_manuallydrop_returntype!() {
			loop {}
		}

		_cold_loop()
	}
}

/// Globally marks the presence state of a
/// looped thread when undefined behavior is detected.
#[cfg(feature = "support_istrig_loop")]
#[inline]
pub unsafe fn trig_next_invalid_beh() {
	_support_istrig_loop::IS_TRIG_LOOPSAFEMANUALLYDROP
		.store(true, _support_istrig_loop::DEF_SETORDERING);
}

/// Globally marks the presence state of a
/// looped thread when undefined behavior is detected.
#[cfg(not(feature = "support_istrig_loop"))]
#[inline]
pub const unsafe fn trig_next_invalid_beh() {}

/// Check if at least one thread has been globally
/// looped to avoid undefined behavior.
#[cfg(feature = "support_istrig_loop")]
#[inline]
pub fn is_trig_next_invalid_beh() -> bool {
	_support_istrig_loop::IS_TRIG_LOOPSAFEMANUALLYDROP.load(_support_istrig_loop::DEF_GETORDERING)
}

/// Check if at least one thread has been globally
/// looped to avoid undefined behavior.
/// (Because `support_istrig_loop` is disabled, this function will always return false.)
#[cfg(not(feature = "support_istrig_loop"))]
#[inline]
pub const fn is_trig_next_invalid_beh() -> bool {
	false
}

impl AutoSafeEmptyLoopManuallyDrop<()> {
	/// Check if at least one thread has been globally looped to avoid undefined behavior.
	#[inline(always)]
	pub fn is_trig_next_invalid_beh() -> bool {
		crate::core::trig::r#loop::is_trig_next_invalid_beh()
	}
}
