
#[cfg(debug_assertions)]
pub const IS_ASAFE_MODE: bool = true;

#[cfg(not(debug_assertions))]
pub const IS_ASAFE_MODE: bool = false;
