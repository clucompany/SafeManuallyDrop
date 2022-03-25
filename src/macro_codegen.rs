
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
		use crate::core::state::StateManuallyDropData;
		#[allow(unused_imports)]
		use crate::core::state::StateManuallyDrop;
		use ::core::ops::DerefMut;
		use ::core::ops::Deref;
		use ::core::fmt::Debug;
		use ::core::hash::Hash;
		
		$(
			$crate::__codegen! {
				$($all)+
			}
		)?
	};
	[
		#$current_type: tt [
			is_safe: $is_safe:tt
		];
		$($($all:tt)+)?
	] => {
		impl<T, Trig> $current_type<T, Trig> where Trig: TrigManuallyDrop {
			// Safe
			/// Wrap a value to be manually dropped.
			#[inline(always)]
			pub /*const*/ fn new(value: T) -> Self {
				let value = UnsafeStdManuallyDrop::new(value);
				
				crate::__if_codegen! {
					if (#$is_safe) {
						// Safe
						Self {
							value,
							state: StateManuallyDrop::empty(),
							_pp: PhantomData
						}
					} else {
						// Unsafe
						Self {
							value,
							_pp: PhantomData
						}
					}
				}
			}
			
			/// Forgets value (similar to core::mem::forget), if you need to forget a 
			/// value in ManuallyData use ignore_drop() instead of this function.
			#[inline(always)]
			pub fn forget(value: T) {
				let sself = Self::new(value);
				unsafe {
					sself.ignore_drop();
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
			
			/// Is ManuallyDrop a wrapper with values, or is it actually a transparent 
			/// value with no false data. 
			#[inline(always)]
			pub fn is_repr_transparent(&self) -> bool {
				crate::__if_codegen! {
					if (#$is_safe) {
						// Safe
						false
					} else {
						// Unsafe
						true
					}
				}
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
			
			/// Checking if a trigger that defines undefined behavior will fire.
			/// Some(true) - means that the state is empty and you can work with the value later.
			/// Some(false) means that the value has already been converted by some method, and further work with the value will cause an undefined behavior trigger.
			/// None - This version of ManuallyDrop is stateless, so defining undefined behavior is not possible.
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
				&*(self.deref() as &T)
			}
			
			#[inline(always)]
			pub fn as_mut_ptr(&mut self) -> *mut T {
				// TODO, VALID?, Exp: ManuallyDrop::as_mut_ptr
				&mut *(self.deref_mut() as &mut T)
			}
			
			#[inline(always)]
			pub fn as_value(&self) -> &T {
				self.deref()
			}
			
			#[inline(always)]
			pub fn as_mut_value(&mut self) -> &mut T {
				self.deref_mut()
			}
			
			/// Safe or insecure version of ManuallyDrop.
			#[inline(always)]
			pub /*const*/ fn is_safe_type(&self) -> bool {
				crate::__if_codegen! {
					if (#$is_safe) {
						// Safe
						true
					} else {
						// Unsafe
						false
					}
				}
			}
			
			/// Extracts the value from the ManuallyDrop container.
			#[inline(always)]
			pub /*const*/ fn into_inner(slot: $current_type<T, Trig>) -> T {
				let core_inner = Self::into_core_inner(slot);
					
				UnsafeStdManuallyDrop::into_inner(core_inner)
			}
			
			#[inline(always)]
			pub /*const*/ fn into_core_inner(slot: $current_type<T, Trig>) -> UnsafeStdManuallyDrop<T> {
				crate::__if_codegen! {
					if (#$is_safe) {
						slot.state.to_intoinnermode_or_trig::<Trig>();
						
						// analog UnsafeManuallyDrop::take
						let result: UnsafeStdManuallyDrop<T> = unsafe {
							::core::ptr::read(&slot.value)
						};
						
						let _ignore_drop = UnsafeStdManuallyDrop::new(slot);
						result
					}else {
						slot.value
					}
				}
			}
			
			/// Takes the value from the ManuallyDrop<T> container out.
			#[inline(always)]
			pub unsafe fn take(slot: &mut $current_type<T, Trig>) -> T {
				crate::__if_codegen! {
					if (#$is_safe) {
						slot.state.to_takemode_or_trig::<Trig>();
					}
				}
				
				UnsafeStdManuallyDrop::take(&mut slot.value)
			}
			
			/// Resets the ManuallyDrop state to its original state and returns the previous state.
			#[inline(always)]
			pub unsafe fn get_state_and_reset(&self) -> Option<StateManuallyDropData> {
				crate::__if_codegen! {
					if (#$is_safe) {
						Some(self.state.get_and_reset())
					} else {
						None
					}
				}
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
			
			/// Checking if a trigger that defines undefined behavior will fire. 
			/// false - means the state is empty and you can work with the value in the future.
			/// - true means the value has already been converted by some method.
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
			
			/// Checking if a trigger that defines undefined behavior will fire.
			/// Some(false) - means that the state is empty and you can work with the value later.
			/// Some(true) means that the value has already been converted by some method, and further work with the value will cause an undefined behavior trigger.
			/// None - This version of ManuallyDrop is stateless, so defining undefined behavior is not possible.
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
			
			/// The version of mem::forget is adapted for safe and insecure ManuallyDrop.
			#[inline(always)]
			pub unsafe fn ignore_drop(&self) {
				crate::__if_codegen! {
					if (#$is_safe) {
						self.state.to_ignore_trig_when_drop::<Trig>();
					}
				}
			}
			
			/// Manually drops the contained value.
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
		
		impl<T, Trig> Deref for $current_type<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
			type Target = T;
			
			#[inline(always)]
			fn deref(&self) -> &T {
				crate::__if_codegen! {
					if (#$is_safe) {
						self.state.deref_or_trig::<Trig>();
					}
				}
				
				self.value.deref()
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
				
				self.value.deref_mut()
			}
		}
		
		impl<T, Trig> Default for $current_type<T, Trig> where T: ?Sized + Default, Trig: TrigManuallyDrop {
			#[inline(always)]
			fn default() -> Self {
				Self::new(Default::default())
			}
		}
		
		impl<T, Trig, Rhs> PartialEq<Rhs> for $current_type<T, Trig> where T: ?Sized + PartialEq<Rhs>, Trig: TrigManuallyDrop {
			#[inline]
			fn eq(&self, a: &Rhs) -> bool {
				let value: &T = self.deref();
				PartialEq::<Rhs>::eq(value, a)
			}
			
			#[inline]
			fn ne(&self, a: &Rhs) -> bool {
				let value: &T = self.deref();
				PartialEq::<Rhs>::ne(value, a)
			}
		}
		
		impl<T, Trig> Eq for $current_type<T, Trig> where T: Eq + PartialEq<$current_type<T, Trig>>, Trig: TrigManuallyDrop {
			#[inline]
			fn assert_receiver_is_total_eq(&self) {
				let value: &T = self.deref();
				Eq::assert_receiver_is_total_eq(value)
			}
		}
		
		impl<T, Trig> Ord for $current_type<T, Trig> where T: Ord + PartialOrd<$current_type<T, Trig>>, Trig: TrigManuallyDrop {
			#[inline]
			fn cmp(&self, a: &Self) -> core::cmp::Ordering {
				let value: &T = self.deref();
				Ord::cmp(value, a)
			}
		}
		
		impl<T, Trig, Rhs> PartialOrd<Rhs> for $current_type<T, Trig> where T: ?Sized + PartialOrd<Rhs>, Trig: TrigManuallyDrop {
			#[inline]
			fn partial_cmp(&self, a: &Rhs) -> Option<core::cmp::Ordering> {
				let value: &T = self.deref();
				PartialOrd::partial_cmp(value, a)
			}
		}
		
		impl<T, Trig> Hash for $current_type<T, Trig> where T: ?Sized + Hash, Trig: TrigManuallyDrop {
			#[inline]
			fn hash<H>(&self, a: &mut H) where H: core::hash::Hasher {
				let value: &T = self.deref();
				Hash::hash(value, a)
			}
		}
		
		impl<T, Trig> Debug for $current_type<T, Trig> where T: ?Sized + Debug, Trig: TrigManuallyDrop {
			#[inline(always)]
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
				let value: &T = self.deref();
				Debug::fmt(value, f)
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
