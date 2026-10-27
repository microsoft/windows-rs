windows_link::link!("test.dll" "system" fn Register(callback : Option < unsafe extern "system" fn(*const Record, *mut u32) -> i32 >));
windows_link::link!("test.dll" "system" fn RegisterOptional(callback : Option < unsafe extern "system" fn() >, handle : CALLBACK));
pub type CALLBACK = Option<unsafe extern "system" fn()>;
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
    pub nested: Option<unsafe extern "system" fn(*const *const Point, *mut [Size; 2])>,
}
impl Default for Callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Record {
    pub value: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub cx: i32,
}
