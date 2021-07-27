
use core::fmt::Debug;
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU8;

const READ_ORDERING_METHOD: Ordering = Ordering::SeqCst;
const WRITE_ORDERING_METHOD: Ordering = Ordering::SeqCst; // 

#[repr(transparent)]
pub struct StateManuallyDrop {
	state: AtomicU8,
}

impl Clone for StateManuallyDrop {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			state: AtomicU8::new(self.__read_byte())
		}
	}
}

impl Debug for StateManuallyDrop {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("StateManuallyDrop")
		.field("state", &self.read())
		.finish()
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StateManuallyDropData {
	DefState = 1,
	
	TakeMode = 5,
	DropMode = 15,
	IntoInnerMode = 25,
	
	IgnorePanicWhenDrop = 30,
}

impl From<u8> for StateManuallyDropData {
	#[inline]
	fn from(a: u8) -> Self {
		StateManuallyDropData::from_or_default(a)
	}
}

impl Default for StateManuallyDropData {
	#[inline(always)]
	fn default() -> Self {
		StateManuallyDropData::default()
	}
}

impl StateManuallyDropData {
	#[inline(always)]
	pub const fn into(self) -> u8 {
		self as _
	}
	
	#[inline]
	pub /*const*/ fn from_or_default(a: u8) -> Self {
		Self::is_valid_byte_fn(
			a, 
			|| unsafe {
				Self::force_from(a)
			},
			|| Self::default()
		)
	}
	
	#[inline]
	pub /*const*/ fn from(a: u8) -> Option<Self> {
		Self::is_valid_byte_fn(
			a, 
			|| Some(unsafe {
				Self::force_from(a)
			}),
			|| None
		)
	}
	
	#[inline(always)]
	pub const fn default() -> Self {
		Self::DefState
	}
	
	#[inline(always)]
	pub /*const*/ fn is_valid_byte_fn<F: FnOnce() -> R, FE: FnOnce() -> R, R>(a: u8, next: F, errf: FE) -> R {
		match a {
			a if a == Self::DefState as _ ||
			
				a == Self::TakeMode as _ ||
				a == Self::DropMode as _ ||
				a == Self::IntoInnerMode as _ ||
				
				a == Self::IgnorePanicWhenDrop as _ => next(),
			_ => errf()
		}
	}
	
	#[inline]
	pub fn is_valid_byte(a: u8) -> bool {
		Self::is_valid_byte_fn(
			a, 
			|| true,
			|| false,
		)
	}
	
	#[inline(always)]
	pub unsafe fn force_from(a: u8) -> Self {
		debug_assert_eq!(
			Self::is_valid_byte(a),
			true
		);
		
		#[allow(unused_unsafe)]
		let result: Self = unsafe {
			core::mem::transmute(a as u8)
		};
		result
	}
}

impl StateManuallyDrop {
	#[inline(always)]
	pub const fn default() -> Self {
		Self {
			state: AtomicU8::new(StateManuallyDropData::default() as _)
		}
	}
	
	#[inline(always)]
	pub fn is_def_mode(&self) -> bool {
		self.read() == StateManuallyDropData::default() as _
	}
	
	#[inline(always)]
	fn __read_byte(&self) -> u8 {
		self.state.load(READ_ORDERING_METHOD)
	}
	
	#[inline(always)]
	pub fn read(&self) -> StateManuallyDropData {
		let byte = self.__read_byte();
		unsafe {
			StateManuallyDropData::force_from(byte)
		}
	}
	
	#[inline(always)]
	fn __write(&self, a: StateManuallyDropData) -> StateManuallyDropData {
		let byte = self.state.swap(a as _, WRITE_ORDERING_METHOD);
		unsafe {
			StateManuallyDropData::force_from(byte)
		}
	}
	
	#[inline]
	fn __safe_replace_mutstate(&self, new_state: StateManuallyDropData) {
		let a_state = self.__write(new_state);
		
		let def_state = StateManuallyDropData::default();
		match a_state == def_state {
			true => {},
			false => crate::panic::run_hook(
				format_args!(
					"SafeManuallyDrop, undef_beh, {:?} != {:?}", 
					def_state,
					a_state
				)
			),
		}
	}
	
	#[inline(always)]
	pub fn to_dropmode_or_panic(&self) {
		self.__safe_replace_mutstate(
			StateManuallyDropData::DropMode
		);
	}
	
	#[inline(always)]
	pub fn to_takemode_or_panic(&self) {
		self.__safe_replace_mutstate(
			StateManuallyDropData::TakeMode
		);
	}
	
	#[inline(always)]
	pub fn to_ignore_panic_when_drop(&self) {
		self.__safe_replace_mutstate(
			StateManuallyDropData::IgnorePanicWhenDrop
		);
	}
	
	#[inline(always)]
	pub fn to_intoinnermode_or_panic(&self) {
		self.__safe_replace_mutstate(
			StateManuallyDropData::IntoInnerMode
		);
	}
	
	pub fn deref_or_panic<F: FnOnce()>(&self, fn_panic: F) {
		let a_state = self.read();
		
		let def_state = StateManuallyDropData::default();
		if a_state != def_state {
			fn_panic();
			
			crate::panic::run_hook(
				format_args!(
					"SafeManuallyDrop, undef_beh (deref_or_panic), {:?} == {:?}",
					def_state,
					a_state
				)
			)
		}
	}
	
	pub fn exp_def_state_and_panic<F: FnOnce()>(&self, fn_panic: F) {
		let a_state = self.read();
		
		let def_state = StateManuallyDropData::default();
		if a_state == def_state {
			fn_panic();
			
			crate::panic::run_hook(
				format_args!(
					"SafeManuallyDrop, undef_beh (exp_def_state), {:?} == {:?}",
					def_state,
					a_state
				)
			)
		}
	}
}

impl Default for StateManuallyDrop {
	#[inline(always)]
	fn default() -> Self {
		StateManuallyDrop::default()
	}
}
