use crate::core::trig::TrigManuallyDrop;
use core::fmt::Arguments;

mod __internal_counter_logic {
	use core::sync::atomic::AtomicU32;
	use core::sync::atomic::Ordering;

	static COUNT_TRIG_SAFEMANUALLYDROP: AtomicU32 = AtomicU32::new(0);

	const DEF_SETORDERING: Ordering = Ordering::SeqCst;
	const DEF_GETORDERING: Ordering = Ordering::Relaxed;

	/// Manually add add to counter +1
	#[inline]
	pub unsafe fn trig_next_invalid_beh() {
		COUNT_TRIG_SAFEMANUALLYDROP.fetch_add(1, DEF_SETORDERING);
	}

	/// Get the number of times the undefined behavior was triggered.
	#[inline]
	pub fn get_count_trig_events() -> u32 {
		COUNT_TRIG_SAFEMANUALLYDROP.load(DEF_GETORDERING)
	}
}

/// A protected version of SafeManuallyDrop with a function to count the amount of undefined behavior of the ManuallyDrop logic.
/// The undefined behavior of CounterManuallyDrop will be the same as when using the standard ManuallyDrop.
pub type AlwaysSafeCounterManuallyDrop<T> =
	crate::beh::safe::SafeManuallyDrop<T, CounterTrigManuallyDrop>;

/// A secure or non-secure version of SafeManuallyDrop with a
/// function to count the undefined behavior of the ManuallyDrop logic.
/// The undefined behavior of CounterManuallyDrop will be the same as when
/// using the standard ManuallyDrop.
pub type AutoSafeCounterManuallyDrop<T> =
	crate::beh::auto::AutoSafeManuallyDrop<T, CounterTrigManuallyDrop>;

/// On undefined behavior, ManuallyDrop enables the undefined behavior,
/// but increments the global counter by +1 each time it detects undefined behavior.
pub enum CounterTrigManuallyDrop {}

impl TrigManuallyDrop for CounterTrigManuallyDrop {
	// #[inline(always)] ignore!,
	#[inline]
	fn trig_next_invalid_beh(_a: Arguments<'_>) -> trig_manuallydrop_returntype!() {
		unsafe { crate::core::trig::counter::trig_next_invalid_beh() }
	}
}

/// Manually add add to counter +1
#[inline]
pub unsafe fn trig_next_invalid_beh() {
	__internal_counter_logic::trig_next_invalid_beh();
}

/// Get the number of times the undefined behavior was triggered.
#[inline]
pub fn get_count_trig_events() -> u32 {
	__internal_counter_logic::get_count_trig_events()
}

impl AutoSafeCounterManuallyDrop<()> {
	/// Get the number of times the undefined behavior was triggered.
	#[inline(always)]
	pub fn get_count_trig_events() -> u32 {
		crate::core::trig::counter::get_count_trig_events()
	}
}

#[cfg(test)]
#[test]
fn test_counter_trig_manuallydrop() {
	use core::sync::atomic::AtomicU32;
	use core::sync::atomic::Ordering;

	const DEF_SETORDERING: Ordering = Ordering::SeqCst;
	const DEF_GETORDERING: Ordering = Ordering::Relaxed;

	static __TEST_COUNTER: AtomicU32 = AtomicU32::new(0);
	struct __Test;
	impl Drop for __Test {
		#[inline]
		fn drop(&mut self) {
			__TEST_COUNTER.fetch_add(1, DEF_SETORDERING);
		}
	}

	let mut check_data = AlwaysSafeCounterManuallyDrop::new(__Test);

	unsafe {
		// combo drop
		AlwaysSafeCounterManuallyDrop::drop(&mut check_data);
		assert_eq!(AlwaysSafeCounterManuallyDrop::get_count_trig_events(), 0);

		AlwaysSafeCounterManuallyDrop::drop(&mut check_data);
		assert_eq!(AlwaysSafeCounterManuallyDrop::get_count_trig_events(), 1);

		AlwaysSafeCounterManuallyDrop::drop(&mut check_data);
		assert_eq!(AlwaysSafeCounterManuallyDrop::get_count_trig_events(), 2);

		assert_eq!(
			__TEST_COUNTER.load(DEF_GETORDERING),
			AlwaysSafeCounterManuallyDrop::get_count_trig_events() + 1
		);
	}
}
