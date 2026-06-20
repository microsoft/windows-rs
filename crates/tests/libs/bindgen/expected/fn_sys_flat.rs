windows_link::link!("test.dll" "system" fn SysFlatFunction(s : *const Struct) -> i32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
