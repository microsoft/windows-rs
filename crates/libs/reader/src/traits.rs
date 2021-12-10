/// # Safety
pub unsafe trait Pod {}

/// # Safety
pub unsafe trait CopyPod: Copy {}

unsafe impl CopyPod for u8 {}
unsafe impl CopyPod for u16 {}
unsafe impl CopyPod for u32 {}
unsafe impl CopyPod for u64 {}
unsafe impl CopyPod for i8 {}
unsafe impl CopyPod for i16 {}
unsafe impl CopyPod for i32 {}
unsafe impl CopyPod for i64 {}

pub trait View {
    fn view_as<T: Pod>(&self, cli_offset: u32) -> &T;
    fn view_as_slice_of<T: Pod>(&self, cli_offset: u32, len: u32) -> &[T];
    fn copy_as<T: Copy + CopyPod>(&self, cli_offset: u32) -> T;
    fn view_as_str(&self, cli_offset: u32) -> &[u8];
}
