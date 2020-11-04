pub unsafe trait GetAbi: Sized {
    type Abi;
    unsafe fn get_abi(&self) -> Self::Abi;
}

pub unsafe trait SetAbi: Sized {
    type Abi;
    unsafe fn set_abi(&mut self) -> Self::Abi;
}

macro_rules! primitive_abi_types {
    ($($t:ty),+) => {$(
        unsafe impl GetAbi for $t {
            type Abi = Self;
            unsafe fn get_abi(&self) -> Self {
                *self
            }
        }
        unsafe impl SetAbi for $t {
            type Abi = *mut Self;

            unsafe fn set_abi(&mut self) -> *mut Self {
                self
            }
        }
    )*};
}

primitive_abi_types! {
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    f32,
    f64
}
