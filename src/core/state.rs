
#[cfg(feature = "support_panic_trig")]
use crate::core::trig::panic::PanicTrigManuallyDrop;
use crate::TrigManuallyDrop;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateManuallyDropData {
	Empty = 1,
	
	//
	TakeModeTrig = 5,
	DropModeTrig = 15,
	IntoInnerModeTrig = 25,
	
	IgnoreTrigWhenDrop = 30,
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
	#[inline(always)]
	pub const fn into(self) -> u8 {
		self as _
	}
	
	#[deprecated(since = "0.1.2", note = "Use `from_or_empty` instead")]
	#[inline]
	pub /*const*/ fn from_or_default(a: u8) -> Self {
		Self::from_or_empty(a)
	}
	
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
	
	#[inline(always)]
	const fn __empty() -> Self {
		Self::Empty
	}
	
	#[deprecated(since = "0.1.2", note = "Use `empty` instead")]
	#[inline(always)]
	pub const fn default() -> Self {
		Self::empty()
	}
	
	#[inline(always)]
	pub const fn empty() -> Self {
		Self::__empty()
	}
	
	#[inline(always)]
	pub const fn no_panic_state() -> Self {
		Self::empty()
	}
	
	#[inline(always)]
	pub /*const*/ fn is_valid_byte_fn<F: FnOnce() -> R, FE: FnOnce() -> R, R>(a: u8, next: F, errf: FE) -> R {
		match a {
			a if a == Self::Empty as _ ||
			
				a == Self::TakeModeTrig as _ ||
				a == Self::DropModeTrig as _ ||
				a == Self::IntoInnerModeTrig as _ ||
				
				a == Self::IgnoreTrigWhenDrop as _ => next(),
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
	
	#[deprecated(since = "0.1.2", note = "Use `is_next_panic` instead")]
	#[inline(always)]
	pub const fn is_next_panic(&self) -> bool {
		self.is_next_trig()
	}
	
	#[inline(always)]
	pub const fn is_next_trig(&self) -> bool {
		match self {
			StateManuallyDropData::Empty => false,
			_ => true,
		}
	}
	
	#[deprecated(since = "0.1.2", note = "Use `is_empty` instead")]
	#[inline(always)]
	pub const fn is_default(&self) -> bool {
		self.is_empty()
	}
	
	#[inline(always)]
	pub const fn is_empty(&self) -> bool {
		match self {
			StateManuallyDropData::Empty => true,
			_ => false,
		}
	}
}

impl StateManuallyDrop {
	#[inline(always)]
	pub /*const*/ fn empty() -> Self {
		let sself = Self {
			state: AtomicU8::new(StateManuallyDropData::empty() as _)
		};
		debug_assert_eq!(sself.is_empty(), true);
		debug_assert_eq!(sself.is_next_trig(), false);
		
		sself
	}
	
	#[deprecated(since = "0.1.2", note = "Use `is_trig_mode` instead")]
	#[inline(always)]
	pub /*const*/ fn default() -> Self {
		Self::empty()
	}
	
	#[deprecated(since = "0.1.2", note = "Use `is_ignore_trig_mode` instead")]
	#[inline(always)]
	pub fn is_def_mode(&self) -> bool {
		self.is_empty()
	}
	
	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.read().is_empty()
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
	fn __force_write(&self, a: StateManuallyDropData) -> StateManuallyDropData {
		let byte = self.state.swap(a as _, WRITE_ORDERING_METHOD);
		unsafe {
			StateManuallyDropData::force_from(byte)
		}
	}
	
	#[inline]
	fn __safe_replace_mutstate<Trig: TrigManuallyDrop>(&self, new_state: StateManuallyDropData) {
		debug_assert_eq!(new_state.is_next_trig(), true);
		
		let old_state = self.__force_write(new_state);
		
		// COMBO REPLACE STATE -> ERR
		if old_state.is_next_trig() {
			Trig::trig_next_invalid_beh(
				format_args!(
					"SafeManuallyDrop, undef_beh (combo_replace_state), SafeManuallyDrop::empty() != {:?}", 
					old_state
				)
			);
		}
	}
	
	#[deprecated(since = "0.1.2", note = "Use `to_dropmode_or_trig` instead")]
	#[cfg(feature = "support_panic_trig")]
	#[inline(always)]
	pub fn to_dropmode_or_panic(&self) {
		self.to_dropmode_or_trig::<PanicTrigManuallyDrop>()
	}
	
	#[inline(always)]
	pub fn to_dropmode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::DropModeTrig
		);
		
		debug_assert_eq!(self.is_next_trig(), true);
	}
	
	#[deprecated(since = "0.1.2", note = "Use `to_takemode_or_trig` instead")]
	#[cfg(feature = "support_panic_trig")]
	#[inline(always)]
	pub fn to_takemode_or_panic(&self) {
		self.to_takemode_or_trig::<PanicTrigManuallyDrop>()
	}
	
	#[inline(always)]
	pub fn to_takemode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::TakeModeTrig
		);
		
		debug_assert_eq!(self.is_next_trig(), true);
	}
	
	#[deprecated(since = "0.1.2", note = "Use `to_ignore_trig_when_drop` instead")]
	#[cfg(feature = "support_panic_trig")]
	#[inline(always)]
	pub fn to_ignore_panic_when_drop(&self) {
		self.to_ignore_trig_when_drop::<PanicTrigManuallyDrop>()
	}
	
	#[inline(always)]
	pub fn to_ignore_trig_when_drop<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::IgnoreTrigWhenDrop
		);
		
		debug_assert_eq!(self.is_next_trig(), true);
	}
	
	#[deprecated(since = "0.1.2", note = "Use `to_intoinnermode_or_trig` instead")]
	#[cfg(feature = "support_panic_trig")]
	#[inline(always)]
	pub fn to_intoinnermode_or_panic(&self) {
		self.to_intoinnermode_or_trig::<PanicTrigManuallyDrop>()
	}
	
	#[inline(always)]
	pub fn to_intoinnermode_or_trig<Trig: TrigManuallyDrop>(&self) {
		self.__safe_replace_mutstate::<Trig>(
			StateManuallyDropData::IntoInnerModeTrig
		);
		
		debug_assert_eq!(self.is_next_trig(), true);
	}
	
	#[deprecated(since = "0.1.2", note = "Use `deref_or_trig` instead")]
	#[cfg(feature = "support_panic_trig")]
	pub fn deref_or_panic<F: FnOnce()>(&self, fn_panic: F) {
		type Trig = PanicTrigManuallyDrop;
		let a_state = self.read();
		
		if a_state.is_next_trig() {
			fn_panic();
			
			Trig::trig_next_invalid_beh(
				format_args!(
					"SafeManuallyDrop, undef_beh (deref_or_panic), SafeManuallyDrop::no_panic_state() != {:?}",
					a_state
				)
			)
		}
	}
	
	pub fn deref_or_trig<Trig: TrigManuallyDrop>(&self) {
		let a_state = self.read();
		
		if a_state.is_next_trig() {
			Trig::trig_next_invalid_beh(
				format_args!(
					"SafeManuallyDrop, undef_beh (deref_or_trig), SafeManuallyDrop::no_panic_state() != {:?}",
					a_state
				)
			)
		}
	}
	
	#[deprecated(since = "0.1.2", note = "Use `if_empty_then_run_trigfn` instead")]
	#[cfg(feature = "support_panic_trig")]
	pub fn if_def_state_then_run_panicfn<F: FnOnce()>(&self, fn_panic: F) {
		type Trig = PanicTrigManuallyDrop;
		let a_state = self.read();
		
		if a_state.is_empty() {
			fn_panic();
			
			Trig::trig_next_invalid_beh(
				format_args!(
					"SafeManuallyDrop, undef_beh (exp_def_state), SafeManuallyDrop::default\\empty() == {:?}",
					a_state
				)
			)
		}
	}
	
	pub fn if_empty_then_run_trigfn<Trig: TrigManuallyDrop, F: FnOnce()>(&self, fn_trig: F) {
		let a_state = self.read();
		
		if a_state.is_empty() {
			fn_trig();
			
			Trig::trig_next_invalid_beh(
				format_args!(
					"SafeManuallyDrop, undef_beh (exp_def_state), SafeManuallyDrop::empty() == {:?}",
					a_state
				)
			)
		}
	}
	
	#[deprecated(since = "0.1.2", note = "Use `is_next_trig` instead")]
	#[inline(always)]
	pub fn is_next_panic(&self) -> bool {
		self.is_next_trig()
	}
	
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

#[cfg(test)]
#[test]
fn test_state() {
	use crate::core::trig::EmptyLoopTrigManuallyDrop;

	let state = StateManuallyDrop::empty();
	assert_eq!(state.is_empty(), true);
	assert_eq!(state.is_next_trig(), false);
	
	state.deref_or_trig::<EmptyLoopTrigManuallyDrop>(); // ok
}
