
use core::fmt::Arguments;
use crate::core::trig::TrigManuallyDrop;

crate::cfg_if_safemode! {
	#if_safe( use core::sync::atomic::AtomicU32; )
	#if_safe( use core::sync::atomic::Ordering; )
	
	#if_safe( static COUNT_TRIG_SAFEMANUALLYDROP: AtomicU32 = AtomicU32::new(0); )
	#if_safe( const DEF_SETORDERING: Ordering = Ordering::SeqCst; )
	#if_safe( const DEF_GETORDERING: Ordering = Ordering::Relaxed; )
}

/// A protected version of SafeManuallyDrop with a function to count the amount of undefined behavior of the ManuallyDrop logic. 
/// The undefined behavior of CounterManuallyDrop will be the same as when using the standard ManuallyDrop.
pub type CounterManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, CounterTrigManuallyDrop>;

pub enum CounterTrigManuallyDrop {}

impl TrigManuallyDrop for CounterTrigManuallyDrop {
	#[inline]
	fn trig_next_invalid_beh<'a>(_a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		COUNT_TRIG_SAFEMANUALLYDROP.fetch_add(1, DEF_SETORDERING);
	}
}

/// Get the number of times the undefined behavior was triggered.
#[inline]
pub fn get_count_trig_events() -> u32 {
	COUNT_TRIG_SAFEMANUALLYDROP.load(DEF_GETORDERING)
}

impl CounterManuallyDrop<()> {
	/// Get the number of times the undefined behavior was triggered.
	#[inline(always)]
	pub fn get_count_trig_events() -> u32 {
		crate::core::trig::counter::get_count_trig_events()
	}
}

#[cfg(test)]
#[test]
fn test_counter_trig_manuallydrop() {
	static __TEST_COUNTER: AtomicU32 = AtomicU32::new(0);
	struct __Test { }
	impl Drop for __Test {
		#[inline]
		fn drop(&mut self) {
			__TEST_COUNTER.fetch_add(1, DEF_SETORDERING);
		}
	}
	
	let mut check_data = CounterManuallyDrop::new(__Test {
		
	});
	
	unsafe { // combo drop
		CounterManuallyDrop::drop(&mut check_data);
		assert_eq!(CounterManuallyDrop::get_count_trig_events(), 0);
		
		CounterManuallyDrop::drop(&mut check_data);
		assert_eq!(CounterManuallyDrop::get_count_trig_events(), 1);
		
		CounterManuallyDrop::drop(&mut check_data);
		assert_eq!(CounterManuallyDrop::get_count_trig_events(), 2);
		
		assert_eq!(
			__TEST_COUNTER.load(DEF_GETORDERING),
			CounterManuallyDrop::get_count_trig_events() + 1
		);
	}
}
