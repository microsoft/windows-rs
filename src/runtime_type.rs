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

    // TODO: code gen hard-coded values for signature where it is known and for generic types use a 
    // std::sync::Once to initialize the value when needed.
    // Make sure it produces the correct GUID before switching ComInterface over to providing an iid() method.
    fn signature() -> &'static str {
        panic!(); // TODO: drop when everyone has a signature
    }
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
