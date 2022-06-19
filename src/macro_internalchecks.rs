
/// Asserts that a boolean expression is `true` at runtime.
///
/// This will invoke the [`panic!`] macro if the provided expression cannot be
/// evaluated to `true` at runtime.
///
/// Like [`assert!`], this macro also has a second version, where a custom panic
/// message can be provided.
///
/// # Uses
///
/// Unlike [`assert!`], `debug_assert!` statements are only enabled in non
/// optimized builds by default. An optimized build will not execute
/// `debug_assert!` statements unless `-C debug-assertions` is passed to the
/// compiler. This makes `debug_assert!` useful for checks that are too
/// expensive to be present in a release build but may be helpful during
/// development. The result of expanding `debug_assert!` is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to keep
/// running, which might have unexpected consequences but does not introduce
/// unsafety as long as this only happens in safe code. The performance cost
/// of assertions, however, is not measurable in general. Replacing [`assert!`]
/// with `debug_assert!` is thus only encouraged after thorough profiling, and
/// more importantly, only in safe code!
/// 
/// Source: rust std debug_assert.
#[doc(hidden)]
#[macro_export]
#[cfg(all(
	feature = "allow_fullinternal_debug_assertions",
	debug_assertions
))]
macro_rules! __fullinternal_debug_assertions {
	( $($all:tt)* ) => {
		assert_eq!($($all)*)
	}
}

/// Asserts that a boolean expression is `true` at runtime.
///
/// This will invoke the [`panic!`] macro if the provided expression cannot be
/// evaluated to `true` at runtime.
///
/// Like [`assert!`], this macro also has a second version, where a custom panic
/// message can be provided.
///
/// # Uses
///
/// Unlike [`assert!`], `debug_assert!` statements are only enabled in non
/// optimized builds by default. An optimized build will not execute
/// `debug_assert!` statements unless `-C debug-assertions` is passed to the
/// compiler. This makes `debug_assert!` useful for checks that are too
/// expensive to be present in a release build but may be helpful during
/// development. The result of expanding `debug_assert!` is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to keep
/// running, which might have unexpected consequences but does not introduce
/// unsafety as long as this only happens in safe code. The performance cost
/// of assertions, however, is not measurable in general. Replacing [`assert!`]
/// with `debug_assert!` is thus only encouraged after thorough profiling, and
/// more importantly, only in safe code!
///
/// Source: rust std debug_assert.
#[doc(hidden)]
#[macro_export]
#[cfg(not(all(
	feature = "allow_fullinternal_debug_assertions",
	debug_assertions
)))]
macro_rules! __fullinternal_debug_assertions {
	( $($all:tt)* ) => {
		
	}
}
