//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2022
//


#![allow(non_snake_case)]

#![no_std]

use crate::core::trig::DefTrigManuallyDrop;
use ::core as stdcore;

use crate::core::trig::TrigManuallyDrop;
pub use stdcore::mem::ManuallyDrop as UnsafeStdManuallyDrop;

/// The core of the library that defines the basic primitives.
pub mod core {
	pub mod state;
	#[macro_use]
	pub mod flags;
	
	#[doc(hidden)]
	#[deprecated(since = "0.1.2", note = "Use `crate::core::trig::hook` instead")]
	#[cfg(feature = "enable_deprecated_hook")]
	pub mod hook;
	pub mod trig;
}

/// Safe and insecure implementations of manual memory management.
pub mod beh {
	/*crate::cfg_if_safemode! {
		/// Insecure standard implementation of manual memory management.
		#if_not_safe(pub mod r#unsafe;)
		
		/// A safe version of the insecure manual control of freeing memory.
		#if_safe(pub mod safe;)
	}*/
	
	/// Insecure standard implementation of manual memory management.
	pub mod r#unsafe;
		
	/// A safe version of the insecure manual control of freeing memory.
	pub mod safe;
}

// Unsafe
/// Unprotected version of ManuallyDrop with backwards compatibility for SafeManuallyDrop features.
pub type AlwaysUnsafeManuallyDrop<T, Trig> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, Trig>;

// Safe
/// A protected version of SafeManuallyDrop with a function to execute a trigger function in case of undefined behavior of the ManuallyDrop logic.
pub type AlwaysSafeManuallyDrop<T, Trig> = crate::beh::safe::SafeManuallyDrop<T, Trig>;

/// A protected version of SafeManuallyDrop with a function to execute a panic in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_panic_trig")]
pub type PanicManuallyDrop<T> = crate::core::trig::panic::PanicManuallyDrop<T>;

/// Protected version of the SafeManuallyDrop with an execution function in case of undefined behavior of the ManuallyDrop logic.
#[cfg(feature = "support_hook_trig")]
pub type HookManuallyDrop<T> = crate::core::trig::hook::HookManuallyDrop<T>;

/// A protected version of SafeManuallyDrop with a function to count the amount of undefined behavior of the ManuallyDrop logic. 
/// The undefined behavior of CounterManuallyDrop will be the same as when using the standard ManuallyDrop.
#[cfg(feature = "support_count_trig")]
pub type CounterManuallyDrop<T> = crate::core::trig::counter::CounterManuallyDrop<T>;

cfg_if_safemode! {
	// Unsafe
	/// Depending on the build flag, a protected version of ManuallyDrop or an unprotected version of ManuallyDrop with a default trigger.
	#if_not_safe(pub type ManuallyDrop<T> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, DefTrigManuallyDrop>;)
	//#if_not_safe(type CurrentManuallyDrop<T, Trig> = crate::beh::r#unsafe::UnsafeManuallyDrop<T, Trig>;)
	//#if_not_safe(pub type AlwaysUnsafeManuallyDrop<T, Trig> = crate::beh::r#unsafe::SafeManuallyDrop<T, Trig>;)
	
	// Safe
	/// Depending on the build flag, a protected version of ManuallyDrop or an unprotected version of ManuallyDrop with a default trigger.
	#if_safe(pub type ManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, DefTrigManuallyDrop>;)
	//#if_safe(pub type AlwaysSafeManuallyDrop<T, Trig> = crate::beh::safe::SafeManuallyDrop<T, Trig>;)
	//#if_safe(pub type PanicManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, PanicTrigManuallyDrop>;)
	//#if_safe(pub type HookManuallyDrop<T> = crate::beh::safe::SafeManuallyDrop<T, HookFnTrigManuallyDrop>;)
	//#if_safe(type CurrentManuallyDrop<T, Trig> = crate::beh::safe::SafeManuallyDrop<T, Trig>;)
}


#[doc(hidden)]
#[macro_export]
macro_rules! __if_codegen {
	[
		if ( #true ) {
			$($all:tt)*
		} $(else {
			$($else:tt)*
		})*
	] => {
		$($all)*
	};
	[
		if ( #false ) {
			$($all:tt)*
		} $(else {
			$($else:tt)*
		})*
	] => {
		$( $($else)* )*
	};
}


#[doc(hidden)]
#[macro_export]
macro_rules! __codegen {
	[
		@use;
		$($($all:tt)+)?
	] => {
		use ::core as stdcore;
		use crate::core::state::StateManuallyDropData;
		#[allow(unused_imports)]
		use crate::core::state::StateManuallyDrop;
		use stdcore::ops::DerefMut;
		use stdcore::ops::Deref;
		
		$(
			$crate::__codegen! {
				$($all)+
			}
		)?
	};
	[
		#$current_type: tt [is_safe: $is_safe:tt];
		$($($all:tt)+)?
	] => {
		impl<T, Trig> $current_type<T, Trig> where Trig: TrigManuallyDrop {
			crate::__if_codegen! {
				if (#$is_safe) {
					// Safe
					#[inline(always)]
					pub /*const*/ fn new(value: T) -> Self {
						let value = UnsafeStdManuallyDrop::new(value);
						
						Self {
							value,
							state: StateManuallyDrop::empty(),
							_pp: PhantomData
						}
					}
				}else {
					// Unsafe
					#[inline(always)]
					pub /*const*/ fn new(value: T) -> Self {
						let value = UnsafeStdManuallyDrop::new(value);
						
						Self {
							value,
							_pp: PhantomData
						}
					}
				}
			}
			
			#[inline(always)]
			pub unsafe fn as_unsafestd_manuallydrop(&self) -> &UnsafeStdManuallyDrop<T> {
				&self.value
			}
			
			#[inline(always)]
			pub unsafe fn as_mut_unsafestd_manuallydrop(&mut self) -> &mut UnsafeStdManuallyDrop<T> {
				&mut self.value
			}
			
			#[inline]
			pub fn get_state(&self) -> Option<StateManuallyDropData> {
				crate::__if_codegen! {
					if (#$is_safe) {
						// Safe
						Some(self.state.read())
					}else {
						// Unsafe
						None
					}
				}
			}
			
			#[deprecated(since = "0.1.2", note = "Use `is_empty_state` instead")]
			#[inline(always)]
			pub fn is_def_state(&self) -> Option<bool> {
				self.is_empty_state()
			}
			
			#[inline]
			pub fn is_empty_state(&self) -> Option<bool> {
				crate::__if_codegen! {
					if (#$is_safe) {
						// Safe
						Some(self.state.is_empty())
					}else {
						// Unsafe
						None
					}
				}
			}
			
			#[inline(always)]
			pub fn as_ptr(&self) -> *const T {
				// TODO, VALID?, Exp: ManuallyDrop::as_ptr
				&*(self.value.deref() as &T)
			}
			
			#[inline(always)]
			pub fn as_mut_ptr(&mut self) -> *mut T {
				// TODO, VALID?, Exp: ManuallyDrop::as_mut_ptr
				&mut *(self.value.deref_mut() as &mut T)
			}
			
			#[inline(always)]
			pub fn as_value(&self) -> &T {
				&self.value
			}
			
			#[inline(always)]
			pub fn as_mut_value(&mut self) -> &mut T {
				&mut self.value
			}
			
			#[inline(always)]
			pub /*const*/ fn into_inner(slot: $current_type<T, Trig>) -> T {
				let core_inner = Self::into_core_inner(slot);
					
				UnsafeStdManuallyDrop::into_inner(core_inner)
			}
			
			crate::__if_codegen! {
				if (#$is_safe) {
					// Safe
					#[inline(always)]
					pub /*const*/ fn into_core_inner(slot: $current_type<T, Trig>) -> UnsafeStdManuallyDrop<T> {
						slot.state.to_intoinnermode_or_trig::<Trig>();
						
						// analog UnsafeManuallyDrop::take
						let result: UnsafeStdManuallyDrop<T> = unsafe {
							stdcore::ptr::read(&slot.value)
						};
						
						let _ignore_drop = UnsafeStdManuallyDrop::new(slot);
						result
					}
				}else {
					// Unsafe
					#[inline(always)]
					pub /*const*/ fn into_core_inner(slot: $current_type<T, Trig>) -> UnsafeStdManuallyDrop<T> {
						slot.value
					}
				}
			}
			
			#[inline(always)]
			pub unsafe fn take(slot: &mut $current_type<T, Trig>) -> T {
				crate::__if_codegen! {
					if (#$is_safe) {
						slot.state.to_takemode_or_trig::<Trig>();
					}
				}
				
				UnsafeStdManuallyDrop::take(&mut slot.value)
			}
			
			#[deprecated(since = "0.1.2", note = "Use `is_next_panic` instead")]
			#[inline(always)]
			pub fn is_maybe_next_panic(&self) -> bool {
				self.is_next_trig()
			}
			
			#[deprecated(since = "0.1.2", note = "Use `is_next_trig` instead")]
			#[inline(always)]
			pub fn is_next_panic(&self) -> bool {
				self.is_next_trig()
			}
			
			#[inline(always)]
			pub fn is_next_trig(&self) -> bool {
				crate::__if_codegen! {
					if (#$is_safe) {
						return self.state.is_next_trig();
					}else {
						return false;
					}
				}
			}
			
			#[deprecated(since = "0.1.2", note = "Use `is_next_trig_optionresult` instead")]
			#[inline(always)]
			pub fn is_next_panic_optionresult(&self) -> Option<bool> {
				self.is_next_trig_optionresult()
			}
			
			#[inline(always)]
			pub fn is_next_trig_optionresult(&self) -> Option<bool> {
				crate::__if_codegen! {
					if (#$is_safe) {
						return Some(self.state.is_next_trig());
					}else {
						return None;
					}
				}
			}
			
			#[inline(always)]
			pub unsafe fn ignore_drop(&self) {
				crate::__if_codegen! {
					if (#$is_safe) {
						self.state.to_ignore_trig_when_drop::<Trig>();
					}
				}
			}
		}
		
		impl<T, Trig> $current_type<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
			#[inline(always)]
			pub unsafe fn drop(slot: &mut $current_type<T, Trig>) {
				crate::__if_codegen! {
					if (#$is_safe) {
						slot.state.to_dropmode_or_trig::<Trig>();
					}
				}
				
				UnsafeStdManuallyDrop::drop(&mut slot.value)
			}
		}
		
		impl<Trig> $current_type<(), Trig> where Trig: TrigManuallyDrop {
			#[inline(always)]
			pub /*const*/ fn is_safe_mode() -> bool {
				crate::core::flags::IS_SAFE_MODE
			}
		}
		
		impl<T, Trig> Deref for $current_type<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
			type Target = T;
			
			#[inline(always)]
			fn deref(&self) -> &T {
				crate::__if_codegen! {
					if (#$is_safe) {
						self.state.deref_or_trig::<Trig>();
					}
				}
				
				&self.value
			}
		}
		 
		impl<T, Trig> DerefMut for $current_type<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
			#[inline(always)]
			fn deref_mut(&mut self) -> &mut T {
				crate::__if_codegen! {
					if (#$is_safe) {
						self.state.deref_or_trig::<Trig>();
					}
				}
				
				&mut self.value
			}
		}
		
		$(
			$crate::__codegen! {
				$($all)+
			}
		)?
	};
	
	//[] => {}
}





