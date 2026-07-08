pub type Callback = Option<unsafe extern "system" fn(value: u32) -> u32>;
pub type CdeclCallback = Option<unsafe extern "C" fn(value: u32) -> u32>;
pub type FastcallCallback = Option<unsafe extern "system" fn(value: u32) -> u32>;
