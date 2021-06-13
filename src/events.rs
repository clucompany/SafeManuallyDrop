
/*pub fn undef_beh_nextpanic(a: &str) -> ! {
	
}*/

#[doc(hidden)]
#[macro_export]
macro_rules! undef_beh_nextpanic {
	[$($all: tt)*] => {
		panic!($($all)*);
	};
}
