
/// Conditional code generation in case 
/// the structure is safe or unsafe 
#[doc(hidden)]
#[macro_export]
macro_rules! __if_codegen {
	[
		if ( #true ) {
			$($all:tt)*
		} $(else {
			$($else:tt)*
		})?
	] => {
		$( $all )?
	};
	
	[
		if ( #false ) {
			$($all:tt)*
		} $(else {
			$($else:tt)*
		})?
	] => {
		$( $($else)* )?
	};
	
	($($all:tt)*) => {
		compile_error!(
			concat!(
				"Please check the spelling of the macro, body: '",
				stringify!($($all)*),
				"'"
			)
		);
	}
}

/// Conditional code generation if standard library compatibility 
/// is enabled and whether the framework is safe or unsafe.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "always_compatible_stdapi")]
macro_rules! __codegen_compatible_stdapi_ornot {
	(
		#if_compatible_stdapi_and_safeapi (#is_always_compatible: true || ( #is_feature && #is_maybe_compatible: $_is_maybe_compatible:tt)) {
			$($true_code:tt)*
		} $(else { $( $false_code:tt)* })?
	) => {
		$($true_code)* // <-- TRUE
	};
	(
		#if_compatible_stdapi_and_safeapi (#is_always_compatible: $_is_always_compatible:tt || ( #is_feature && #is_maybe_compatible: true)) {
			$($true_code:tt)*
		} $(else { $( $false_code:tt)* })?
	) => {
		$($true_code)* // <-- TRUE
	};
	(
		#if_compatible_stdapi_and_safeapi (#is_always_compatible: $_is_always_compatible:tt || ( #is_feature && #is_maybe_compatible: false)) {
			$($true_code:tt)*
		} $(else { $( $false_code:tt)* })?
	) => {
		$( $($false_code)* )? // <-- FALSE
	};
	
	($($all:tt)*) => {
		compile_error!(
			concat!(
				"Please check the spelling of the macro, body: '",
				stringify!($($all)*),
				"'"
			)
		);
	}
}

/// Conditional code generation if standard library compatibility 
/// is enabled and whether the framework is safe or unsafe.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "always_compatible_stdapi"))]
macro_rules! __codegen_compatible_stdapi_ornot {
	(
		#if_compatible_stdapi_and_safeapi (#is_always_compatible: true || ( #is_feature && #is_maybe_compatible: $_is_maybe_compatible:tt)) {
			$($true_code:tt)*
		} $(else { $( $false_code:tt)* })?
	) => {
		$($true_code)* // <-- TRUE
	};
	(
		#if_compatible_stdapi_and_safeapi (#is_always_compatible: $_is_always_compatible:tt || ( #is_feature && #is_maybe_compatible: true)) {
			$($true_code:tt)*
		} $(else { $( $false_code:tt)* })?
	) => {
		$( $($false_code)* )? // <-- FALSE
	};
	(
		#if_compatible_stdapi_and_safeapi (#is_always_compatible: $_is_always_compatible:tt || ( #is_feature && #is_maybe_compatible: false)) {
			$($true_code:tt)*
		} $(else { $( $false_code:tt)* })?
	) => {
		$( $($false_code)* )? // <-- FALSE
	};
	
	($($all:tt)*) => {
		compile_error!(
			concat!(
				"Please check the spelling of the macro, body: '",
				stringify!($($all)*),
				"'"
			)
		);
	}
}

/// Internal code generation is needed more for compatibility 
/// and bypassing trait restrictions.
#[doc(hidden)]
#[macro_export]
macro_rules! __codegen {
	[
		@use;
		$(
			$($all:tt)+
		)?
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
		@impl $current_type: tt {
			is_safe: $is_safe: tt,
			is_always_compatible: $is_always_compatible: tt,
			is_maybe_compatible: $is_maybe_compatible: tt,
			is_repr_transparent: $is_repr_transparent: tt,
			
			fn {
				$(#[$($__tt:tt)*])? // ignore comments
				new |$new_value_fn:ident| $new_fn: block
				
				$(#[$($___tt:tt)*])? // ignore comments
				as_unsafestd_manuallydrop |$sself_as_unsafestd_manuallydrop: ident| $as_unsafestd_manuallydrop: block
				$(#[$($____tt:tt)*])? // ignore comments
				as_mut_unsafestd_manuallydrop |$sself_as_mut_unsafestd_manuallydrop: ident| $as_mut_unsafestd_manuallydrop: block
				
				$(#[$($_____tt:tt)*])? // ignore comments
				force_as_value |$sself_force_as_value: ident| $force_as_value_fn: block
				$(#[$($______tt:tt)*])? // ignore comments
				force_as_mut_value |$sself_force_as_mut_value: ident| $force_as_mut_value_fn: block
			}
		}
		$(
			$($all:tt)+
		)?
	] => {
		impl<T, Trig> $current_type<T, Trig> where Trig: TrigManuallyDrop {
			/// Wrap a value to be manually dropped.
			#[inline]
			pub /*const*/ fn new(value: T) -> Self {
				let value = UnsafeStdManuallyDrop::new(value);
				
				unsafe {
					Self::from_std(value)
				}
			}
			
			/// Wrap a value to be manually dropped. 
			/// Unsafe because the UnsafeStdManuallyDrop input argument is in an undefined state.
			#[inline]
			pub /*const*/ unsafe fn from_std($new_value_fn: UnsafeStdManuallyDrop<T>) -> Self {
				$new_fn
			}
			
			/// Forgets value (similar to core::mem::forget), if you need to forget a 
			/// value in ManuallyData use ignore_drop() instead of this function.
			#[inline(always)]
			pub fn forget(value: T) {
				let sself = Self::new(value);
				
				#[allow(unused_unsafe)]
				unsafe {
					sself.ignore_drop();
				}
			}
			
			/// Extracts the value from the ManuallyDrop container.
			#[inline]
			pub /*const*/ fn into_inner(slot: $current_type<T, Trig>) -> T {
				$crate::__if_codegen! {
					if (#$is_safe) {
						slot.state.to_intoinnermode_or_trig::<Trig>();
						
						// into_inner
						let mut slot = slot;
						
						let value: T = unsafe {
							UnsafeStdManuallyDrop::take(
								slot.as_mut_unsafestd_manuallydrop()
							)
						};
						
						let _ignore_drop = UnsafeStdManuallyDrop::new(slot);
						value
					}else {
						UnsafeStdManuallyDrop::into_inner(slot.value)
					}
				}
			}
			
			/// Extracts the value from the ManuallyDrop container.
			#[inline]
			pub /*const*/ fn into_core_inner(slot: $current_type<T, Trig>) -> UnsafeStdManuallyDrop<T> {
				$crate::__if_codegen! {
					if (#$is_safe) {
						slot.state.to_intoinnermode_or_trig::<Trig>();
						
						// analog UnsafeManuallyDrop::take
						let mandrop: UnsafeStdManuallyDrop<T> = unsafe {
							::core::ptr::read(slot.as_unsafestd_manuallydrop())
						};
						
						let _ignore_drop = UnsafeStdManuallyDrop::new(slot);
						mandrop
					}else {
						slot.value
					}
				}
			}
			
			// TODO! duplication of code, it could have been solved if the rust would allow 
			// it to be done somehow differently, but at this stage it’s the only way.
			$crate::__codegen_compatible_stdapi_ornot! {
				#if_compatible_stdapi_and_safeapi (#is_always_compatible: $is_always_compatible || ( #is_feature && #is_maybe_compatible: $is_maybe_compatible)) {
					/// Takes the value from the ManuallyDrop<T> container out.
					#[inline(always)]
					pub unsafe fn take(slot: &mut $current_type<T, Trig>) -> T {
						$crate::__if_codegen! {
							if (#$is_safe) {
								slot.state.to_takemode_or_trig::<Trig>();
							}
						}
						
						UnsafeStdManuallyDrop::take(&mut slot.value)
					}
				} else {
					/// Takes the value from the ManuallyDrop<T> container out.
					#[inline(always)]
					pub fn take(slot: &mut $current_type<T, Trig>) -> T {
						$crate::__if_codegen! {
							if (#$is_safe) {
								slot.state.to_takemode_or_trig::<Trig>();
							}
						}
						
						unsafe { // library provides security guarantees
							UnsafeStdManuallyDrop::take(&mut slot.value)
						}
					}
				}
			}
			
			
		}
		
		impl<T, Trig> $current_type<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
			// TODO! duplication of code, it could have been solved if the rust would allow 
			// it to be done somehow differently, but at this stage it’s the only way.
			$crate::__codegen_compatible_stdapi_ornot! {
				#if_compatible_stdapi_and_safeapi (#is_always_compatible: $is_always_compatible || ( #is_feature && #is_maybe_compatible: $is_maybe_compatible)) {
					/// Get reference to value.
					#[inline(always)]
					pub unsafe fn as_value(&self) -> &T {
						$crate::__if_codegen! {
							if (#$is_safe) {
								self.state.deref_or_trig::<Trig>();
							}
						}
						
						self.force_as_value()
					}
					
					/// Get a mutable reference to a value.
					#[inline(always)]
					pub unsafe fn as_mut_value(&mut self) -> &mut T {
						$crate::__if_codegen! {
							if (#$is_safe) {
								self.state.deref_or_trig::<Trig>();
							}
						}
						
						self.force_as_mut_value()
					}
					
					/// PSEUDO SAFE! In safe ManuallyDrop, this function prevents undefined behavior at run time, 
					/// but the resulting raw pointer is not protected and does not depend on lifetime 
					/// and may be dangling.
					#[inline(always)]
					pub unsafe fn as_ptr(&self) -> *const T {
						// TODO, VALID?, Exp: ManuallyDrop::as_ptr
						self.as_value() as _
					}
					
					/// PSEUDO SAFE! In safe ManuallyDrop, this function prevents undefined behavior at run time, 
					/// but the resulting raw pointer is not protected and does not depend on lifetime 
					/// and may be dangling.
					#[inline(always)]
					pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
						// TODO, VALID?, Exp: ManuallyDrop::as_mut_ptr
						self.as_mut_value() as _
					}
					
					/// Manually drops the contained value.
					#[inline(always)]
					pub unsafe fn drop(slot: &mut $current_type<T, Trig>) {
						$crate::__if_codegen! {
							if (#$is_safe) {
								slot.state.to_dropmode_or_trig::<Trig>();
							}
						}
						
						UnsafeStdManuallyDrop::drop(&mut slot.value)
					}
					
					/// Note that the safe ManuallyDrop checks to see if the value is freed when the safe ManuallyDrop struct dies.
					/// The version of mem::forget is adapted for safe and insecure ManuallyDrop.
					#[inline(always)]
					pub unsafe fn ignore_drop(&self) {
						$crate::__if_codegen! {
							if (#$is_safe) {
								self.state.to_ignore_trig_when_drop::<Trig>();
							}
						}
					}
				} else {
					/// Get reference to value.
					#[inline(always)]
					pub fn as_value(&self) -> &T {
						$crate::__if_codegen! {
							if (#$is_safe) {
								self.state.deref_or_trig::<Trig>();
							}
						}
						
						unsafe {
							self.force_as_value()
						}
					}
					
					/// Get a mutable reference to a value.
					#[inline(always)]
					pub fn as_mut_value(&mut self) -> &mut T {
						$crate::__if_codegen! {
							if (#$is_safe) {
								self.state.deref_or_trig::<Trig>();
							}
						}
						
						unsafe {
							self.force_as_mut_value()
						}
					}
					
					/// PSEUDO SAFE! In safe ManuallyDrop, this function prevents undefined behavior at run time, 
					/// but the resulting raw pointer is not protected and does not depend on lifetime 
					/// and may be dangling.
					#[inline(always)]
					pub fn as_ptr(&self) -> *const T {
						// TODO, VALID?, Exp: ManuallyDrop::as_ptr
						self.as_value() as _
					}
					
					/// PSEUDO SAFE! In safe ManuallyDrop, this function prevents undefined behavior at run time, 
					/// but the resulting raw pointer is not protected and does not depend on lifetime 
					/// and may be dangling.
					#[inline(always)]
					pub fn as_mut_ptr(&mut self) -> *mut T {
						// TODO, VALID?, Exp: ManuallyDrop::as_mut_ptr
						self.as_mut_value() as _
					}
					
					/// Manually drops the contained value.
					#[inline(always)]
					pub fn drop(slot: &mut $current_type<T, Trig>) {
						$crate::__if_codegen! {
							if (#$is_safe) {
								slot.state.to_dropmode_or_trig::<Trig>();
							}
						}
						
						unsafe { // library provides security guarantees
							UnsafeStdManuallyDrop::drop(&mut slot.value)
						}
					}
					
					/// Note that the safe ManuallyDrop checks to see if the value is freed when the safe ManuallyDrop struct dies.
					/// The version of mem::forget is adapted for safe and insecure ManuallyDrop.
					#[inline(always)]
					pub fn ignore_drop(&self) {
						$crate::__if_codegen! {
							if (#$is_safe) {
								self.state.to_ignore_trig_when_drop::<Trig>();
							}
						}
					}
				}
				
			}
			
			
			
			#[inline(always)]
			pub unsafe fn as_unsafestd_manuallydrop(&self) -> &UnsafeStdManuallyDrop<T> {
				let $sself_as_unsafestd_manuallydrop = self;
				
				$as_unsafestd_manuallydrop
			}
			
			#[inline(always)]
			pub unsafe fn as_mut_unsafestd_manuallydrop(&mut self) -> &mut UnsafeStdManuallyDrop<T> {
				let $sself_as_mut_unsafestd_manuallydrop = self;
				
				$as_mut_unsafestd_manuallydrop
			}
			
			/// Is ManuallyDrop a wrapper with values, or is it actually a transparent 
			/// value with no false data. 
			#[inline(always)]
			pub fn is_repr_transparent(&self) -> bool {
				$is_repr_transparent
			}
			
			#[inline]
			pub fn get_state(&self) -> Option<StateManuallyDropData> {
				$crate::__if_codegen! {
					if (#$is_safe) {
						// Safe
						Some(self.state.read())
					}else {
						// Unsafe
						None
					}
				}
			}
			
			/// Checking if a trigger that defines undefined behavior will fire.
			/// Some(true) - means that the state is empty and you can work with the value later.
			/// Some(false) means that the value has already been converted by some method, and 
			/// further work with the value will cause an undefined behavior trigger.
			/// None - This version of ManuallyDrop is stateless, so defining undefined behavior 
			/// is not possible.
			#[inline]
			pub fn is_empty_state(&self) -> Option<bool> {
				$crate::__if_codegen! {
					if (#$is_safe) {
						// Safe
						Some(self.state.is_empty())
					}else {
						// Unsafe
						None
					}
				}
			}
			
			/// Get a raw pointer to a value. The call is always insecure.
			#[inline(always)]
			pub unsafe fn force_as_ptr(&self) -> *const T {
				// TODO, VALID?, Exp: ManuallyDrop::as_ptr
				self.force_as_value() as _
			}
			
			/// Get a raw mut pointer to a value. The call is always insecure.
			#[inline(always)]
			pub unsafe fn force_as_mut_ptr(&mut self) -> *mut T {
				// TODO, VALID?, Exp: ManuallyDrop::as_mut_ptr
				self.force_as_mut_value() as _
			}

			/// Get reference to value. Always unprotected!
			#[inline(always)]
			pub unsafe fn force_as_value(&self) -> &T {
				let $sself_force_as_value = self;
				
				$force_as_value_fn
			}
			
			/// Get a mutable reference to a value. Always unprotected!
			#[inline(always)]
			pub unsafe fn force_as_mut_value(&mut self) -> &mut T {
				let $sself_force_as_mut_value = self;
				
				$force_as_mut_value_fn
			}
			
			
			/// Safe or insecure version of ManuallyDrop.
			#[inline(always)]
			pub /*const*/ fn is_safe_type(&self) -> bool {
				$is_safe
			}
			
			/// Resets the ManuallyDrop state to its original state and returns the previous state.
			#[inline(always)]
			pub unsafe fn get_state_and_reset(&self) -> Option<StateManuallyDropData> {
				$crate::__if_codegen! {
					if (#$is_safe) {
						Some(self.state.get_and_reset())
					} else {
						None
					}
				}
			}
			
			/// Checking if a trigger that defines undefined behavior will fire. 
			/// false - means the state is empty and you can work with the value in the future.
			/// - true means the value has already been converted by some method.
			#[inline(always)]
			pub fn is_next_trig(&self) -> bool {
				$crate::__if_codegen! {
					if (#$is_safe) {
						return self.state.is_next_trig();
					}else {
						return false;
					}
				}
			}
			
			/// Checking if a trigger that defines undefined behavior will fire.
			/// Some(false) - means that the state is empty and you can work with the value later.
			/// Some(true) means that the value has already been converted by some method, and further work with the value will cause an undefined behavior trigger.
			/// None - This version of ManuallyDrop is stateless, so defining undefined behavior is not possible.
			#[inline(always)]
			pub fn is_next_trig_optionresult(&self) -> Option<bool> {
				$crate::__if_codegen! {
					if (#$is_safe) {
						return Some(self.state.is_next_trig());
					}else {
						return None;
					}
				}
			}
			
		}
		
		impl<T, Trig> Clone for $current_type<T, Trig> where T: ?Sized + Clone, Trig: TrigManuallyDrop {
			#[inline(always)]
			fn clone(&self) -> Self {
				let ref_value: &T = self.deref();
				let value = Clone::clone(ref_value);
				
				Self::new(value)
			}
		}
		
		impl<T, Trig> Deref for $current_type<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
			type Target = T;
			
			#[inline(always)]
			fn deref(&self) -> &T {
				#[allow(unused_unsafe)]
				unsafe {
					self.as_value()
				}
			}
		}
		 
		impl<T, Trig> DerefMut for $current_type<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
			#[inline(always)]
			fn deref_mut(&mut self) -> &mut T {
				#[allow(unused_unsafe)]
				unsafe {
					self.as_mut_value()
				}
			}
		}
		
		impl<T, Trig> Default for $current_type<T, Trig> where T: ?Sized + Default, Trig: TrigManuallyDrop {
			#[inline(always)]
			fn default() -> Self {
				let value = Default::default();
				Self::new(value)
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
		
		impl<T, Trig> From<T> for $current_type<T, Trig> where T: Sized, Trig: TrigManuallyDrop {
			#[inline(always)]
			fn from(a: T) -> Self {
				$current_type::new(a)
			}
		}
		
		$(
			$crate::__codegen! {
				$($all)+
			}
		)?
	};
	
	($($all:tt)*) => {
		compile_error!(
			concat!(
				"Please check the spelling of the macro, body: '",
				stringify!($($all)*),
				"'"
			)
		);
	}
}
