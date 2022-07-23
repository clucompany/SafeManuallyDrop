
#[cfg(all(test, feature = "support_panic_trig"))]
use crate::core::trig::panic::PanicTrigManuallyDrop;
use core::fmt::Display;
use crate::core::trig::TrigManuallyDrop;
use core::fmt::Debug;
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU8;

const READ_ORDERING_METHOD: Ordering = Ordering::SeqCst;
const WRITE_ORDERING_METHOD: Ordering = Ordering::SeqCst; // 

/// Atomic safe states for ManuallyDrop
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

impl Display for StateManuallyDrop {
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		Display::fmt(&self.read(), f)
	}
}

/// Safe States for ManuallyDrop
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateManuallyDropData {
	/// It is allowed to convert ManuallyDrop to 
	/// anything without calling a trigger.
	Empty = 1,
	
	/// With the take function, the value is forgotten, subsequent work 
	/// with ManuallyDrop will necessarily call the trigger.
	TakeModeTrig = 5,
	/// With the drop function, the value is released, subsequent work 
	/// with ManuallyDrop will necessarily call the trigger.
	DropModeTrig = 15,
	/// With the into_inner function, the value is cast, subsequent work 
	/// with ManuallyDrop will definitely find the trigger.
	IntoInnerModeTrig = 25,
	
	/// (unsafe/manual_behavior) ManuallyDrop must be forgotten, subsequent work 
	/// with ManuallyDrop will definitely call the trigger.
	IgnoreTrigWhenDrop = 30,
}

impl Display for StateManuallyDropData {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		let str = match self {
			Self::Empty => "Empty",
			
			Self::TakeModeTrig => "TakeModeTrig",
			Self::DropModeTrig => "DropModeTrig",
			Self::IntoInnerModeTrig => "IntoInnerModeTrig",
			
			Self::IgnoreTrigWhenDrop => "IgnoreTrigWhenDrop",
		};
		
		Display::fmt(str, f)
	}
}

impl From<u8> for StateManuallyDropData {
	#[inline(always)]
	fn from(a: u8) -> Self {
		StateManuallyDropData::from_or_empty(a)
	}
}

impl Default for StateManuallyDropData {
	#[inline(always)]
	fn default() -> Self {
		Self::empty()
	}
}

impl StateManuallyDropData {
	/// Convert state to byte
	#[inline(always)]
	pub const fn into(self) -> u8 {
		self as _
	}
	
	/// Create a state from a byte, in case of an error, 
	/// return the default state.
	#[inline]
	pub /*const*/ fn from_or_empty(a: u8) -> Self {
		Self::is_valid_byte_fn(
			a, 
			|| unsafe {
				Self::force_from(a)
			},
			|| Self::empty()
		)
	}
	
	/// Create a state from a byte, return None on error.
	#[inline]
	pub /*const*/ fn from(a: u8) -> Option<Self> {
		Self::is_valid_byte_fn(
			a, 
			|| {
				let sself = unsafe {
					Self::force_from(a)
				};
				
				Some(sself)
			},
			|| None
		)
	}
	
	/// Create default state
	#[inline(always)]
	const fn __empty() -> Self {
		let sself = Self::Empty;
		sself
	}
	
	/// Create default state
	#[inline(always)]
	pub const fn empty() -> Self {
		Self::__empty()
	}
	
	/// Create default state
	#[inline(always)]
	pub const fn no_panic_state() -> Self {
		Self::empty()
	}
	
	/// Generic Status Byte Validation Function
	#[inline(always)]
	pub /*const*/ fn is_valid_byte_fn<R>(a: u8, next: impl FnOnce() -> R, errf: impl FnOnce() -> R) -> R {
		match a {
			a if a == Self::Empty as _ ||
			
				a == Self::TakeModeTrig as _ ||
				a == Self::DropModeTrig as _ ||
				a == Self::IntoInnerModeTrig as _ ||
				
				a == Self::IgnoreTrigWhenDrop as _ => next(),
			_ => errf()
		}
	}
	
	/// General function to check the status byte, 
	/// return false on error, true on success.
	#[inline]
	pub fn is_valid_byte(a: u8) -> bool {
		Self::is_valid_byte_fn(
			a, 
			|| true,
			|| false,
		)
	}
	
	/// Create a state from a byte quickly and without checks 
	/// (important, the byte is checked anyway, but only in a debug build)
	#[inline(always)]
	pub unsafe fn force_from(a: u8) -> Self {
		crate::__fullinternal_debug_assertions!(
			Self::is_valid_byte(a),
			true
		);
		
		Self::__force_form(a)
	}
	
	/// Create a state from a byte quickly and without checks 
	/// (important, the byte is checked anyway, but only in a debug build)
	#[inline(always)]
	const fn __force_form(a: u8) -> Self {
		let result: StateManuallyDropData = unsafe {
			// safe, u8 -> StateManuallyDropData (enum)
			core::mem::transmute(a as u8)
		};
		result
	}
	
	/// Determining if a trigger should be executed
	#[inline(always)]
	pub const fn is_next_trig(&self) -> bool {
		match self {
			StateManuallyDropData::Empty => false,
			_ => true,
		}
	}
	
	/// Whether the current state is like a new unused object.
	#[inline(always)]
	pub const fn is_empty(&self) -> bool {
		match self {
			StateManuallyDropData::Empty => true,
			_ => false,
		}
	}
}

/// Empty state, needed only for some implementations of const functions.
pub const EMPTY_STATE: StateManuallyDrop = StateManuallyDrop::__empty();

impl StateManuallyDrop {
	/// Create default state
	#[inline(always)]
	pub /*const*/ fn empty() -> Self {
		let sself = Self::__empty();
		
		crate::__fullinternal_debug_assertions!(sself.is_empty(), true);
		crate::__fullinternal_debug_assertions!(sself.is_next_trig(), false);
		
		sself
	}
	
	/// Create default state
	#[inline(always)]
	const fn __empty() -> Self {
		Self {
			state: AtomicU8::new(StateManuallyDropData::empty() as _)
		}
	}
	
	/// Whether the current state is like a new unused object.
	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.read().is_empty()
	}
	
	/// Getting the status byte of the current ManuallyDrop.
	#[inline(always)]
	fn __read_byte(&self) -> u8 {
		self.state.load(READ_ORDERING_METHOD)
	}
	
	/// Getting the status of the current ManuallyDrop.
	#[inline(always)]
	pub fn read(&self) -> StateManuallyDropData {
		let byte = self.__read_byte();
		unsafe {
			StateManuallyDropData::force_from(byte)
		}
	}
	
	/// Quick substitution of the state of the current ManuallyDrop 
	/// (note that the previous state of ManuallyDrop is returned)
	#[inline(always)]
	fn __force_write(&self, a: StateManuallyDropData) -> StateManuallyDropData {
		let byte = self.state.swap(a as _, WRITE_ORDERING_METHOD);
		unsafe {
			StateManuallyDropData::force_from(byte)
		}
	}
	
	/// Resets the ManuallyDrop state to the initial state
	pub unsafe fn get_and_reset(&self) -> StateManuallyDropData {
		let old_value = self.__force_write(
			StateManuallyDropData::Empty
		);
		crate::__fullinternal_debug_assertions!(self.is_empty(), true);
		crate::__fullinternal_debug_assertions!(self.is_next_trig(), false);
		
		old_value
	}
	
	/// Function to safely replace the state of the ManuallyDrop trigger 
	/// definer (note that the new state must fire on validation)
	#[inline]
	fn __safe_replace_mutstate<Trig: TrigManuallyDrop>(&self, new_state: StateManuallyDropData) {
		crate::__fullinternal_debug_assertions!(new_state.is_next_trig(), true);
		
		let old_state = self.__force_write(new_state);
		
		// COMBO REPLACE STATE -> ERR
		if old_state.is_next_trig() {
			Trig::trig_next_invalid_beh(
				format_args!(
					"Undefined behavior when using ManuallyDrop(combo_replace_manudropstate), instead of the expected default state, the current state: {:?}.", 
					old_state
				)
			);
		}
	}
	
	/// Change the ManuallyDrop state to a panicked state, or execute a trigger 
	/// function if the current state was not empty.
	#[inline(always)]
	pub fn to_dropmode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::DropModeTrig
		);
		
		crate::__fullinternal_debug_assertions!(self.is_next_trig(), true);
	}
	
	/// Change the state of ManuallyDrop to the state of the released value, 
	/// or execute the trigger function if the current state was not empty.
	#[inline(always)]
	pub fn to_takemode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::TakeModeTrig
		);
		
		crate::__fullinternal_debug_assertions!(self.is_next_trig(), true);
	}
	
	/// Change the ManuallyDrop state to ignore freeing the value, or execute the 
	/// trigger function if the current state was not empty.
	#[inline(always)]
	pub fn to_ignore_trig_when_drop<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::IgnoreTrigWhenDrop
		);
		
		crate::__fullinternal_debug_assertions!(self.is_next_trig(), true);
	}
	
	/// Change the state of ManuallyDrop to the state of the released value, or execute 
	/// the trigger function if the current state was not empty.
	#[inline(always)]
	pub fn to_intoinnermode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::IntoInnerModeTrig
		);
		
		crate::__fullinternal_debug_assertions!(self.is_next_trig(), true);
	}
	
	/// Check the state of ManuallyDrop for a readable state, or execute a trigger 
	/// function if the current state was not empty.
	#[inline(always)]
	pub fn deref_or_trig<Trig: TrigManuallyDrop>(&self) {
		let a_state = self.read();
		
		if a_state.is_next_trig() {
			Trig::trig_next_invalid_beh(
				format_args!(
					"Undefined behavior when using ManuallyDrop.deref(), instead of the expected default state, the current state: {:?}.",
					a_state
				)
			)
		}
	}
	
	/// Check if the ManuallyDrop state is empty, or execute the trigger function if 
	/// the current state was not empty.
	pub fn if_empty_then_run_trigfn<Trig: TrigManuallyDrop, F: FnOnce()>(&self, exp_str: &'static str, fn_trig: F) {
		let a_state = self.read();
		
		if a_state.is_empty() {
			fn_trig();
			
			Trig::trig_next_invalid_beh(
				format_args!(
					"Undefined behavior when using ManuallyDrop ({}), state should not be default, current state is {:?}.",
					exp_str,
					a_state
				)
			)
		}
	}
	
	/// Determining if a trigger should be executed
	#[inline(always)]
	pub fn is_next_trig(&self) -> bool {
		self.read().is_next_trig()
	}
}

impl Default for StateManuallyDrop {
	#[inline(always)]
	fn default() -> Self {
		StateManuallyDrop::empty()
	}
}

#[cfg(all(test, feature = "support_panic_trig"))]
#[test]
fn test_state() {
	let state = StateManuallyDrop::empty();
	assert_eq!(state.is_empty(), true);
	assert_eq!(state.is_next_trig(), false);
	
	state.deref_or_trig::<PanicTrigManuallyDrop>(); // ok
}

#[cfg(all(test, feature = "support_panic_trig"))]
#[test]
fn test_const_empty_state() {
	let state = EMPTY_STATE; // Copy
	assert_eq!(state.is_empty(), true);
	assert_eq!(state.is_next_trig(), false);
	
	state.deref_or_trig::<PanicTrigManuallyDrop>(); // ok
}


#[cfg(all(test, feature = "support_panic_trig"))]
#[test]
fn test_reset() {
	let state = StateManuallyDrop::empty();
	assert_eq!(state.is_empty(), true);
	assert_eq!(state.is_next_trig(), false);
	
	state.deref_or_trig::<PanicTrigManuallyDrop>(); // ok
	state.to_dropmode_or_trig::<PanicTrigManuallyDrop>();
	
	assert_eq!(state.is_empty(), false);
	assert_eq!(state.is_next_trig(), true);
	
	let old_state = unsafe {
		state.get_and_reset()
	};
	assert_eq!(state.is_empty(), true);
	assert_eq!(state.is_next_trig(), false);
	assert_eq!(old_state.is_empty(), false);
	assert_eq!(old_state.is_next_trig(), true);
	assert_eq!(old_state, StateManuallyDropData::DropModeTrig);
}

