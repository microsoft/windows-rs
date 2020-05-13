/// RuntimeType is used to constrain WinRT generic types to WinRT types.
///
/// It is highly unlikely that users of winrt will ever need to implement this
/// trait for themselves
///
/// # Safety
///
/// A type should only implement RuntimeType if the associated `Abi` type is safe to pass
/// across FFI boundaries.
/// The type itself must also be zero initializable and safe to drop if all bits are zeroable.
/// RuntimeTypes must be safe to use in WinRT generics.
pub unsafe trait RuntimeType {
    type Abi;

    fn abi(&self) -> Self::Abi;
    fn set_abi(&mut self) -> *mut Self::Abi;

    fn from_abi(abi: &Self::Abi) -> &Self {
        unsafe { std::mem::transmute_copy(&abi) }
    }

    fn from_mut_abi(abi: &mut Self::Abi) -> &mut Self {
        unsafe { std::mem::transmute_copy(&abi) }
    }

    // TODO: this should be a const function returning &'static str
    // It is only used internally by ComInterface::iid() which should
    // itself be a const function.
    fn signature() -> String;
}

macro_rules! primitive_runtime_type {
    ($(($t:ty, $s:literal)),+) => {
        $(unsafe impl RuntimeType for $t {
            type Abi = Self;
            fn signature() -> String {
                $s.to_owned()
            }
            fn abi(&self) -> Self::Abi {
                *self
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self as *mut Self::Abi
            }
        })*
    };
}

primitive_runtime_type! {
    (bool, "b1"),
    (i8, "i1"),
    (u8, "u1"),
    (i16, "i2"),
    (u16, "u2"),
    (i32, "i4"),
    (u32, "u4"),
    (i64, "i8"),
    (u64, "u8"),
    (f32, "f4"),
    (f64, "f8")
}
