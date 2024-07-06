//! AtomicStates for ManuallyDrop

use crate::core::trig::TrigManuallyDrop;
use crate::extended_debug_assertions::extended_debug_assertions;
use core::fmt::Debug;
use core::fmt::Display;
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;

/// Atomic safe states for ManuallyDrop
#[repr(transparent)]
pub struct StateManuallyDrop {
	state: AtomicU8,
}

impl Clone for StateManuallyDrop {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			state: AtomicU8::new(self.__read_byte()),
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
	const READ_ORDERING_METHOD: Ordering = Ordering::SeqCst;
	const WRITE_ORDERING_METHOD: Ordering = Ordering::SeqCst; //

	/// Convert state to byte
	#[inline(always)]
	pub const fn into(self) -> u8 {
		self as _
	}

	/// Create a state from a byte, in case of an error,
	/// return the default state.
	#[inline]
	pub fn from_or_empty(a: u8) -> Self {
		Self::validate_with_fns(a, || unsafe { Self::unchecked_from(a) }, Self::empty)
	}

	/// Create a state from a byte, return None on error.
	#[inline]
	pub fn from(a: u8) -> Option<Self> {
		Self::validate_with_fns(
			a,
			|| {
				let sself = unsafe { Self::unchecked_from(a) };

				Some(sself)
			},
			|| None,
		)
	}

	/// Create default state
	#[inline(always)]
	pub const fn empty() -> Self {
		Self::Empty
	}

	/// Create default state
	#[inline(always)]
	pub const fn no_panic_state() -> Self {
		Self::empty()
	}

	/// Validates a status byte and executes a function based on the result.
	///
	/// Returns the result of the next function if the byte is valid,
	/// otherwise returns the result of the errf function.
	pub fn validate_with_fns<R>(a: u8, next: impl FnOnce() -> R, errf: impl FnOnce() -> R) -> R {
		match a {
			a if a == Self::Empty as _
				|| a == Self::TakeModeTrig as _
				|| a == Self::DropModeTrig as _
				|| a == Self::IntoInnerModeTrig as _
				|| a == Self::IgnoreTrigWhenDrop as _ =>
			{
				next()
			}
			_ => errf(),
		}
	}

	/// Create a state from a byte quickly and without checks
	/// (important, the byte is checked anyway, but only in a debug build)
	#[inline]
	pub unsafe fn unchecked_from(a: u8) -> Self {
		extended_debug_assertions!(Self::validate_with_fns(a, || true, || false), true);

		Self::__unchecked_from(a)
	}

	/// Create a state from a byte quickly and without checks
	/// (important, the byte is checked anyway, but only in a debug build)
	#[inline]
	const fn __unchecked_from(a: u8) -> Self {
		// as u8: It's not really needed here, but it allows me to control code regression in the transmutation functions.
		#[allow(clippy::unnecessary_cast)]
		let result: StateManuallyDropData = unsafe {
			// safe, u8 -> StateManuallyDropData (enum)
			core::mem::transmute(a as u8)
		};
		result
	}

	/// Determining if a trigger should be executed
	#[inline]
	pub const fn is_next_trig(&self) -> bool {
		!matches!(self, StateManuallyDropData::Empty)
	}

	/// Whether the current state is like a new unused object.
	#[inline]
	pub const fn is_empty(&self) -> bool {
		matches!(self, StateManuallyDropData::Empty)
	}
}

impl StateManuallyDrop {
	// clippy::declare_interior_mutable_const why?: This constant is only used for initialization, no one is going to constantly access it for use.
	#[allow(clippy::declare_interior_mutable_const)]
	/// Empty state, needed only for some implementations of const functions.
	pub const EMPTY_STATE: StateManuallyDrop = StateManuallyDrop::__empty();

	/// Create default state
	#[inline]
	pub fn empty() -> Self {
		let sself = Self::EMPTY_STATE;

		extended_debug_assertions!(sself.is_empty(), true);
		extended_debug_assertions!(sself.is_next_trig(), false);

		sself
	}

	/// Create default state
	#[inline]
	const fn __empty() -> Self {
		Self {
			state: AtomicU8::new(StateManuallyDropData::empty() as _),
		}
	}

	/// Whether the current state is like a new unused object.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.read().is_empty()
	}

	/// Getting the status byte of the current ManuallyDrop.
	#[inline]
	fn __read_byte(&self) -> u8 {
		self.state.load(StateManuallyDropData::READ_ORDERING_METHOD)
	}

	/// Getting the status of the current ManuallyDrop.
	#[inline]
	pub fn read(&self) -> StateManuallyDropData {
		let byte = self.__read_byte();

		unsafe { StateManuallyDropData::unchecked_from(byte) }
	}

	/// Quick substitution of the state of the current ManuallyDrop
	/// (note that the previous state of ManuallyDrop is returned)
	#[inline]
	fn __force_write(&self, a: StateManuallyDropData) -> StateManuallyDropData {
		let byte = self
			.state
			.swap(a as _, StateManuallyDropData::WRITE_ORDERING_METHOD);

		unsafe { StateManuallyDropData::unchecked_from(byte) }
	}

	/// Resets the ManuallyDrop state to the initial state
	pub unsafe fn get_and_reset(&self) -> StateManuallyDropData {
		let old_value = self.__force_write(StateManuallyDropData::Empty);
		extended_debug_assertions!(self.is_empty(), true);
		extended_debug_assertions!(self.is_next_trig(), false);

		old_value
	}

	/// Function to safely replace the state of the ManuallyDrop trigger
	/// definer (note that the new state must fire on validation)
	#[inline]
	fn __safe_replace_mutstate<Trig: TrigManuallyDrop>(&self, new_state: StateManuallyDropData) {
		extended_debug_assertions!(new_state.is_next_trig(), true);

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
		self.__safe_replace_mutstate::<Trig>(StateManuallyDropData::DropModeTrig);

		extended_debug_assertions!(self.is_next_trig(), true);
	}

	/// Change the state of ManuallyDrop to the state of the released value,
	/// or execute the trigger function if the current state was not empty.
	#[inline]
	pub fn to_takemode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(StateManuallyDropData::TakeModeTrig);

		extended_debug_assertions!(self.is_next_trig(), true);
	}

	/// Change the ManuallyDrop state to ignore freeing the value, or execute the
	/// trigger function if the current state was not empty.
	#[inline]
	pub fn to_ignore_trig_when_drop<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(StateManuallyDropData::IgnoreTrigWhenDrop);

		extended_debug_assertions!(self.is_next_trig(), true);
	}

	/// Change the state of ManuallyDrop to the state of the released value, or execute
	/// the trigger function if the current state was not empty.
	#[inline]
	pub fn to_intoinnermode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(StateManuallyDropData::IntoInnerModeTrig);

		extended_debug_assertions!(self.is_next_trig(), true);
	}

	/// Check the state of ManuallyDrop for a readable state, or execute a trigger
	/// function if the current state was not empty.
	#[inline]
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
	pub fn if_empty_then_run_trigfn<Trig: TrigManuallyDrop, F: FnOnce()>(
		&self,
		exp_str: &'static str,
		fn_trig: F,
	) {
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
	#[inline]
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
mod tests {
	use crate::core::{
		state::{StateManuallyDrop, StateManuallyDropData},
		trig::panic::PanicTrigManuallyDrop,
	};

	#[test]
	fn test_state() {
		let state = StateManuallyDrop::empty();
		assert!(state.is_empty());
		assert!(!state.is_next_trig());

		state.deref_or_trig::<PanicTrigManuallyDrop>(); // ok
	}

	#[test]
	fn test_const_empty_state() {
		let state = StateManuallyDrop::EMPTY_STATE; // Copy

		assert!(state.is_empty());
		assert!(!state.is_next_trig());

		state.deref_or_trig::<PanicTrigManuallyDrop>(); // ok
	}

	#[test]
	fn test_reset() {
		let state = StateManuallyDrop::empty();

		assert!(state.is_empty());
		assert!(!state.is_next_trig());

		state.deref_or_trig::<PanicTrigManuallyDrop>(); // ok
		state.to_dropmode_or_trig::<PanicTrigManuallyDrop>();

		assert!(!state.is_empty());
		assert!(state.is_next_trig());

		let old_state = unsafe { state.get_and_reset() };
		assert!(state.is_empty());
		assert!(!state.is_next_trig());
		assert!(!old_state.is_empty());
		assert!(old_state.is_next_trig());
		assert_eq!(old_state, StateManuallyDropData::DropModeTrig);
	}
}
