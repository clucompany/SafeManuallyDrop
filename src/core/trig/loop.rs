
use core::fmt::Arguments;
use crate::core::trig::TrigManuallyDrop;

pub enum EmptyLoopTrigManuallyDrop {}

impl TrigManuallyDrop for EmptyLoopTrigManuallyDrop {
	#[inline(always)]
	fn trig_next_invalid_beh<'a>(_a: Arguments<'a>) -> trig_manuallydrop_returntype!() {
		loop {}
	}
}
