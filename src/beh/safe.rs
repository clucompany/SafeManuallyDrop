
use core::hash::Hash;
use crate::UnsafeStdManuallyDrop;
use crate::core::trig::TrigManuallyDrop;
use core::marker::PhantomData;

/// A safe version of the insecure manual control of freeing memory.
// #[repr(transparent)]
//#[derive(/*Copy,*/ Clone, Debug)]
#[derive(/*Copy,*/ Clone, Debug/*, Default, PartialEq, Eq, PartialOrd, Ord, Hash*/)]
pub struct SafeManuallyDrop<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
	pub (crate) state: StateManuallyDrop,
	pub (crate) _pp: PhantomData<Trig>,
	pub (crate) value: UnsafeStdManuallyDrop<T>,
}

crate::__codegen! {
	@use;
	#SafeManuallyDrop		[is_safe: true];
}

//impl<T> Copy for ManuallyDrop<T> where T: ?Sized + Copy {} TODO

impl<T, Trig> Default for SafeManuallyDrop<T, Trig> where T: ?Sized + Default, Trig: TrigManuallyDrop {
	#[inline(always)]
	fn default() -> Self {
		Self::new(
			Default::default()
		)
	}
}

impl<T, Trig, Rhs> PartialEq<Rhs> for SafeManuallyDrop<T, Trig> where T: ?Sized + PartialEq<Rhs>, Trig: TrigManuallyDrop {
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

impl<T, Trig> Eq for SafeManuallyDrop<T, Trig> where T: Eq + PartialEq<SafeManuallyDrop<T, Trig>>, Trig: TrigManuallyDrop {
	#[inline]
	fn assert_receiver_is_total_eq(&self) {
		let value: &T = self.value.deref();
		Eq::assert_receiver_is_total_eq(value)
	}
}

impl<T, Trig> Ord for SafeManuallyDrop<T, Trig> where T: Ord + PartialOrd<SafeManuallyDrop<T, Trig>>, Trig: TrigManuallyDrop {
	#[inline]
	fn cmp(&self, a: &Self) -> core::cmp::Ordering {
		let value: &T = self.value.deref();
		Ord::cmp(value, a)
	}
}

impl<T, Trig, Rhs> PartialOrd<Rhs> for SafeManuallyDrop<T, Trig> where T: ?Sized + PartialOrd<Rhs>, Trig: TrigManuallyDrop {
	#[inline]
	fn partial_cmp(&self, a: &Rhs) -> Option<core::cmp::Ordering> {
		let value: &T = self.value.deref();
		PartialOrd::partial_cmp(value, a)
	}
}

impl<T, Trig> Hash for SafeManuallyDrop<T, Trig> where T: ?Sized + Hash, Trig: TrigManuallyDrop {
	#[inline]
	fn hash<H>(&self, a: &mut H) where H: core::hash::Hasher {
		let value: &T = self.value.deref();
		Hash::hash(value, a)
	}
}

impl<T, Trig> Drop for SafeManuallyDrop<T, Trig> where T: ?Sized, Trig: TrigManuallyDrop {
	#[inline]
	fn drop(&mut self) {
		let ref mut value = self.value;
		let ref state = self.state;
		
		/*enum __HideTrig {}
		impl TrigManuallyDrop for __HideTrig {
			fn trig_next_invalid_beh<'a>(a: core::fmt::Arguments<'a>) -> ! {
				Trig::trig_next_invalid_beh(a)
			}
		}*/
		
		state.if_empty_then_run_trigfn::<Trig, _>(
			|| unsafe {
				// What for? - >> to ignore miri errors allocate.
				UnsafeStdManuallyDrop::drop(value);
			}
		);
	}
}
