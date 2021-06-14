
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU8;

const READ_ORDERING_METHOD: Ordering = Ordering::SeqCst;
const WRITE_ORDERING_METHOD: Ordering = Ordering::SeqCst; // 

#[repr(transparent)]
#[derive(Debug)]
pub struct StateManuallyDrop {
	state: AtomicU8,
}

impl Clone for StateManuallyDrop {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			state: AtomicU8::new(self.__read())
		}
	}
}

const DEF_STATE: u8 = 1;
const TAKE_MODE_STATE: u8 = 2;
const DROP_MODE_STATE: u8 = 3;
const INTO_INNER_MODE_STATE: u8 = 4;
const IGNORE_PANIC_WHEN_DROP_STATE: u8 = 5;

#[repr(u8)]
#[derive(Debug)]
pub enum StateManuallyDropData {
	DefState = DEF_STATE,
	
	TakeMode = TAKE_MODE_STATE,
	DropMode = DROP_MODE_STATE,
	IntoInnerMode = INTO_INNER_MODE_STATE,
	
	IgnorePanicWhenDrop = IGNORE_PANIC_WHEN_DROP_STATE,
}

impl From<u8> for StateManuallyDropData {
	#[inline]
	fn from(a: u8) -> Self {
		match a {
			a if a == DEF_STATE => Self::DefState,
			a if a == TAKE_MODE_STATE => Self::TakeMode,
			a if a == DROP_MODE_STATE => Self::DropMode,
			a if a == INTO_INNER_MODE_STATE => Self::IntoInnerMode,
			a if a == IGNORE_PANIC_WHEN_DROP_STATE => Self::IgnorePanicWhenDrop,
			
			_ => Self::DefState
		}
	}
}

impl Default for StateManuallyDropData {
	#[inline(always)]
	fn default() -> Self {
		Self::DefState
	}
}

impl StateManuallyDrop {
	#[inline(never)] // YES!
	pub const fn new() -> Self {
		Self {
			state: AtomicU8::new(DEF_STATE)
		}
	}
	
	#[inline(always)]
	pub fn is_def_mode(&self) -> bool {
		self.__read() == DEF_STATE
	}

	#[inline(always)]
	fn __read(&self) -> u8 {
		self.state.load(READ_ORDERING_METHOD)
	}
	
	#[inline(always)]
	fn __write(&self, a: u8) -> u8 {
		self.state.swap(a, WRITE_ORDERING_METHOD)
	}
	
	#[inline]
	fn __safe_replace_mutstate(&self, new_state: u8) {
		let astate = self.__write(new_state);
		match astate == DEF_STATE {
			true => {},
			false => {
				crate::undef_beh_nextpanic!(
					"SafeManuallyDrop, undef_beh, {:?} != {:?}", 
					StateManuallyDropData::default(),
					StateManuallyDropData::from(astate)
				)
			},
		}
	}
	
	#[inline(always)]
	pub fn to_dropmode_or_panic(&self) {
		self.__safe_replace_mutstate(DROP_MODE_STATE);
	}
	
	#[inline(always)]
	pub fn to_takemode_or_panic(&self) {
		self.__safe_replace_mutstate(TAKE_MODE_STATE);
	}
	
	#[inline(always)]
	pub fn to_ignore_panic_when_drop(&self) {
		self.__safe_replace_mutstate(IGNORE_PANIC_WHEN_DROP_STATE);
	}
	
	#[inline(always)]
	pub fn to_intoinnermode_or_panic(&self) {
		self.__safe_replace_mutstate(INTO_INNER_MODE_STATE);
	}
	
	pub fn deref_or_panic(&self) {
		let astate = self.__read();
		
		if astate != DEF_STATE {
			//fn_panic();
			
			crate::undef_beh_nextpanic!(
				"SafeManuallyDrop, undef_beh (deref_or_panic), {:?} == {:?}",
				StateManuallyDropData::default(),
				StateManuallyDropData::from(astate)
			)
		}
	}
	
	pub fn exp_def_state_and_panic<F: FnOnce()>(&self, fn_panic: F) {
		let astate = self.__read();
		
		if astate == DEF_STATE {
			fn_panic();
			
			crate::undef_beh_nextpanic!(
				"SafeManuallyDrop, undef_beh (exp_def_state), {:?} == {:?}",
				StateManuallyDropData::default(),
				StateManuallyDropData::from(astate)
			)
		}
	}
}

impl Default for StateManuallyDrop {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}
