//Copyright 2021 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2021
//


#![allow(non_snake_case)]

#![no_std]

extern crate alloc;

use alloc::prelude::v1::Box;
use core::hash::Hash;
use crate::state::StateManuallyDrop;
use core::ops::DerefMut;
use core::ops::Deref;
pub use core::mem::ManuallyDrop as UnsafeManuallyDrop;

pub type SafeManuallyDrop<T> = ManuallyDrop<T>;

mod state;
pub mod flags;
mod events;

// Unsafe
#[cfg(not(debug_assertions))]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ManuallyDrop<T> where T: ?Sized {
	value: UnsafeManuallyDrop<T>
}

// SafeTest
#[cfg(debug_assertions)]
#[derive(/*Copy,*/ Clone, Debug)] /*Copy, :((*/
pub struct ManuallyDrop<T> where T: ?Sized {
	value: Box<UnsafeManuallyDrop<T>>, // why box? -->> T: ?Sized
	state: StateManuallyDrop,
}

#[cfg(debug_assertions)]
impl<T> Default for ManuallyDrop<T> where T: ?Sized + Default {
	#[inline(always)]
	fn default() -> Self {
		Self::new(Default::default())
	}
}

#[cfg(debug_assertions)]
impl<T, Rhs> PartialEq<Rhs> for ManuallyDrop<T> where T: ?Sized + PartialEq<Rhs> {
	#[inline]
	fn eq(&self, a: &Rhs) -> bool {
		let value: &T = self.value.deref();
		PartialEq::<Rhs>::eq(value, a)
	}
	
	#[inline]
	fn ne(&self, a: &Rhs) -> bool {
		let value: &T = self.value.deref();
		PartialEq::<Rhs>::ne(value, a)
	}
}

#[cfg(debug_assertions)]
impl<T> Eq for ManuallyDrop<T> where T: ?Sized + Eq + PartialEq<Self> {
	#[inline]
	fn assert_receiver_is_total_eq(&self) {
		let value: &T = self.value.deref();
		Eq::assert_receiver_is_total_eq(value)
	}
}

#[cfg(debug_assertions)]
impl<T, Rhs> PartialOrd<Rhs> for ManuallyDrop<T> where T: ?Sized + PartialOrd<Rhs> {
	#[inline]
	fn partial_cmp(&self, a: &Rhs) -> Option<core::cmp::Ordering> {
		let value: &T = self.value.deref();
		PartialOrd::partial_cmp(value, a)
	}
}

#[cfg(debug_assertions)]
impl<T> Ord for ManuallyDrop<T> where T: ?Sized + Eq + PartialEq<Self> + PartialOrd<Self> + Ord {
	#[inline]
	fn cmp(&self, a: &Self) -> core::cmp::Ordering {
		let value: &T = self.value.deref();
		Ord::cmp(value, a)
	}
}

#[cfg(debug_assertions)]
impl<T> Hash for ManuallyDrop<T> where T: ?Sized + Hash {
	#[inline]
	fn hash<H>(&self, a: &mut H) where H: core::hash::Hasher {
		let value: &T = self.value.deref();
		Hash::hash(value, a)
	}
}

impl<T> ManuallyDrop<T> {
	#[cfg(debug_assertions)]
	#[inline(always)]
	pub /*const*/ fn new(value: T) -> ManuallyDrop<T> {
		let value = Box::new(UnsafeManuallyDrop::new(value));
		
		ManuallyDrop { 
			value,
			state: StateManuallyDrop::default(),
		}
	}
	
	#[cfg(not(debug_assertions))]
	#[inline(always)]
	pub const fn new(value: T) -> ManuallyDrop<T> {
		let value = UnsafeManuallyDrop::new(value);
		
		ManuallyDrop { 
			value,
			state: StateManuallyDrop::default(),
		}
	}
	
	
	#[cfg(not(debug_assertions))]
	#[inline(always)]
	pub /*const*/ fn into_inner(slot: ManuallyDrop<T>) -> T {
		let value = slot.value;
		
		UnsafeManuallyDrop::into_inner(
			value
		)
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	pub /*const*/ fn into_inner(slot: ManuallyDrop<T>) -> T {
		slot.state.to_intoinnermode_or_panic();
		
		// why? ->> (box_into_inner #80437)
		let value = {
			let value: UnsafeManuallyDrop<T> = unsafe {
				core::ptr::read(slot.value.deref() as _)
			};
			
			// Ignore T, T == ManuallyDrop
			drop(slot);
			
			value
		};
		
		UnsafeManuallyDrop::into_inner(
			value
		)
	}
	
	#[cfg(not(debug_assertions))]
	pub const fn into_core_inner(slot: ManuallyDrop<T>) -> UnsafeManuallyDrop<T> {
		slot.value
	}
	
	#[cfg(debug_assertions)]
	#[inline(always)]
	pub /*const*/ fn into_core_inner(slot: ManuallyDrop<T>) -> UnsafeManuallyDrop<T> {
		slot.state.to_intoinnermode_or_panic();
		
		// why? ->> (box_into_inner #80437)
		let value = {
			let value: UnsafeManuallyDrop<T> = unsafe {
				core::ptr::read(slot.value.deref() as _)
			};
			
			// Ignore T, T == ManuallyDrop
			drop(slot);
			
			value
		};
		
		value
	}
	
	#[inline(always)]
	pub unsafe fn take(slot: &mut ManuallyDrop<T>) -> T {
		#[cfg(debug_assertions)] {
			slot.state.to_takemode_or_panic();
		}
		
		UnsafeManuallyDrop::take(&mut slot.value)
	}
	
	#[cfg(debug_assertions)]
	pub fn is_maybe_next_panic(&self) -> bool {
		!self.state.is_def_mode()
	}
}

impl<T> ManuallyDrop<T> where T: ?Sized {
	#[inline(always)]
	pub unsafe fn drop(slot: &mut ManuallyDrop<T>) {
		#[cfg(debug_assertions)] {
			slot.state.to_dropmode_or_panic();
		}
		
		UnsafeManuallyDrop::drop(&mut slot.value)
	}
}

impl ManuallyDrop<()> {
	#[inline(always)]
	pub const fn is_safe_mode() -> bool {
		crate::flags::IS_ASAFE_MODE
	}
}

impl<T> Deref for ManuallyDrop<T> where T: ?Sized {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T {
		#[cfg(debug_assertions)] {
			self.state.deref_or_panic();
		}
		
		&self.value
	}
}
 
impl<T> DerefMut for ManuallyDrop<T> where T: ?Sized {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T {
		#[cfg(debug_assertions)] {
			self.state.deref_or_panic();
		}
		
		&mut self.value
	}
}
