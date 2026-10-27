windows_link::link!("test.dll" "system" fn Register(callback : Option < unsafe extern "system" fn(*const Record, *mut u32) -> i32 >));
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Callbacks {
    pub c: Option<unsafe extern "C" fn(i32, *const u8) -> i32>,
    pub system: Option<unsafe extern "system" fn()>,
    pub array: [Option<unsafe extern "system" fn(Record)>; 2],
    pub factory: Option<
        unsafe extern "C" fn(
            Option<unsafe extern "system" fn(i32)>,
        ) -> Option<unsafe extern "C" fn() -> Record>,
    >,
}
impl Default for Callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Record {
    pub value: i32,
}
