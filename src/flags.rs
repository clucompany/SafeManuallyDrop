
pub const IS_ASAFE_MODE: bool = {
	#[cfg(debug_assertions)] {
		true
	}
	#[cfg(not(debug_assertions))] {
		false
	}
};
