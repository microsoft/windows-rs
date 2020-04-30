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
}

macro_rules! primitive_runtime_type {
    ($($t:ty),+) => {
        $(unsafe impl RuntimeType for $t {
            type Abi = Self;
            fn abi(&self) -> Self::Abi {
                *self
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self as *mut Self::Abi
            }
        })*
    };
}

primitive_runtime_type! { bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64 }
