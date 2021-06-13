
use alloc::prelude::v1::Box;
use core::cell::Ref;
use core::cell::RefMut;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct StateManuallyDrop {
	// TODO: except AtomicTypes/UnsafeCell
	//state: RefCell<StateManuallyDropData>
	state: Box<RefCell<StateManuallyDropData>>
}

#[derive(Debug, Clone)]
pub enum StateManuallyDropData {
	DefState,
	
	TakeMode,
	DropMode,
	IntoInnerMode,
	
	IgnorePanicWhenDrop,
}

impl Default for StateManuallyDropData {
	#[inline(always)]
	fn default() -> Self {
		Self::DefState
	}
}

impl StateManuallyDropData {
	#[inline(always)]
	pub const fn is_def_state(&self) -> bool {
		match self {
			Self::DefState => true,
			_ => false
		}
	}
	
	#[inline(always)]
	pub /*const*/ fn def_state_fn<F: FnOnce(&Self) -> R, ERR: FnOnce(&Self) -> R, R>(&self, next: F, errf: ERR) -> R {
		match self {
			Self::DefState => next(self),
			_ => errf(self),
		}
	}
	
	#[inline(always)]
	pub /*const*/ fn def_mut_state_fn<F: FnOnce(&mut Self) -> R, ERR: FnOnce(&Self) -> R, R>(&mut self, next: F, errf: ERR) -> R {
		match self {
			Self::DefState => next(self),
			_ => errf(self),
		}
	}
}

impl StateManuallyDrop {
	#[inline(never)] // YES!
	pub fn new() -> Self {
		Self {
			state: Box::new(RefCell::new(Default::default()))
		}
	}
	
	#[inline(always)]
	pub fn is_def_mode(&self) -> bool {
		let read = self.__read();
		read.is_def_state()
	}
	
	#[inline(always)]
	fn __write(&self) -> RefMut<StateManuallyDropData> {
		self.state.borrow_mut()
	}
	
	#[inline(always)]
	fn __read(&self) -> Ref<StateManuallyDropData> {
		self.state.borrow()
	}
	
	#[inline]
	fn __safe_replace_mutstate<F: FnOnce(&mut StateManuallyDropData)>(&self, mut_state: &mut StateManuallyDropData, next: F) {
		mut_state.def_mut_state_fn(
			next,
			|write| {
				let astate: StateManuallyDropData = write.clone();
				drop(write);
				
				crate::undef_beh_nextpanic!(
					"SafeManuallyDrop, undef_beh, {:?} != {:?}", 
					StateManuallyDropData::default(), 
					astate
				)
			},
		);
	}
	
	pub fn to_dropmode_or_panic(&self) {
		let mut write = self.__write();
		
		self.__safe_replace_mutstate(
			&mut write,
			|write| *write = StateManuallyDropData::DropMode
		);
	}
	
	pub fn to_takemode_or_panic(&self) {
		let mut write = self.__write();
		
		self.__safe_replace_mutstate(
			&mut write,
			|write| *write = StateManuallyDropData::TakeMode
		);
	}
	
	pub fn to_ignore_panic_when_drop(&self) {
		let mut write = self.__write();
		
		self.__safe_replace_mutstate(
			&mut write,
			|write| *write = StateManuallyDropData::IgnorePanicWhenDrop
		);
	}
	
	pub fn to_intoinnermode_or_panic(&self) {
		let mut write = self.__write();
		
		self.__safe_replace_mutstate(
			&mut write,
			|write| *write = StateManuallyDropData::IntoInnerMode
		);
	}
	
	pub fn deref_or_panic(&self) {
		let read = self.__read();
		
		read.def_state_fn(
			|_read| {},
			|read| {
				let astate: StateManuallyDropData = read.clone();
				drop(read);
				
				crate::undef_beh_nextpanic!(
					"SafeManuallyDrop, undef_beh (deref), {:?} != {:?}", 
					StateManuallyDropData::default(), 
					astate
				)
			}
		);
	}
	
	pub fn exp_nodef_state<F: FnOnce()>(&self, fn_panic: F) {
		let read = self.__read();
		
		read.def_state_fn(
			|read| {
				let astate: StateManuallyDropData = read.clone();
				drop(read);
				
				fn_panic();
				
				crate::undef_beh_nextpanic!(
					"SafeManuallyDrop, undef_beh (exp_nodef_state), {:?} == {:?}",
					StateManuallyDropData::default(), 
					astate
				)
			},
			|_read| {}
		);
	}
}

impl Default for StateManuallyDrop {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}
