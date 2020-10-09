/// A "Plain Ol' Data" structure that represents any type that
/// can be viewed from any probably aligned and sized buffer of
/// bytes in a well defined way. This means that the representation
/// of the type in memory must be stable and it must not contain
/// any invariant constraints.
///
/// ## Examples
///
/// A `u32` is a `Pod` because any [u8; 4] can be viewed as a `u32` safely.
/// A `bool` is _not_ a `Pod` because it must either be a `0` or `1` in memory
pub(crate) unsafe trait Pod {}

/// A Pod type that is also safe to copy.
///
/// In addition to the same safety properties as Pod types, this type must be able to
/// be zeroed, many that it is valid to represent this type in memory as all 0s.
pub(crate) unsafe trait CopyPod: Copy {}

unsafe impl CopyPod for u8 {}
unsafe impl CopyPod for u16 {}
unsafe impl CopyPod for u32 {}
unsafe impl CopyPod for u64 {}
unsafe impl CopyPod for i8 {}
unsafe impl CopyPod for i16 {}
unsafe impl CopyPod for i32 {}
unsafe impl CopyPod for i64 {}

pub(crate) trait View {
    fn view_as<T: Pod>(&self, cli_offset: u32) -> &T;
    fn view_as_slice_of<T: Pod>(&self, cli_offset: u32, len: u32) -> &[T];
    fn copy_as<T: Copy + CopyPod>(&self, cli_offset: u32) -> T;
    fn view_as_str(&self, cli_offset: u32) -> &[u8];
}

pub trait Decode {
    fn decode(code: u32, file: u16) -> Self;
}
