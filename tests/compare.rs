use SafeManuallyDrop::AlwaysSafePanicManuallyDrop as SafeLibManuallyDrop;
use SafeManuallyDrop::UnsafeStdManuallyDrop;

#[test]
fn compare_api() {
	let mut safe_liba = SafeLibManuallyDrop::new(10usize);
	let mut safe_liba2 = SafeLibManuallyDrop::new(10usize);
	let mut unsafe_stda = UnsafeStdManuallyDrop::new(10usize);

	assert_eq!(safe_liba, safe_liba2);
	assert_eq!(safe_liba, 10usize);
	assert_eq!(safe_liba2, 10usize);
	assert_eq!(safe_liba, unsafe_stda);

	unsafe {
		SafeLibManuallyDrop::drop(&mut safe_liba);
		SafeLibManuallyDrop::drop(&mut safe_liba2);
		UnsafeStdManuallyDrop::drop(&mut unsafe_stda);
	}
}
