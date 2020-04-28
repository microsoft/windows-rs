/// RuntimeType is used to constrain WinRT generic types to WinRT types
pub trait RuntimeType {
    type Abi;

    fn abi(&self) -> Self::Abi;
    fn set_abi(&mut self) -> *mut Self::Abi;
}

macro_rules! primitive_runtime_type {
    ($($t:ty),+) => {
        $(impl RuntimeType for $t {
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
