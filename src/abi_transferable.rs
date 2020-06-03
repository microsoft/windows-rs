/// Types that are safe to transfer over a WinRT API boundary.
///
/// # Safety
/// Implementing types only have associated `Abi` types that are
/// safe to transfer over a WinRT boundary. Implementing types
/// must also be exactly equivalent to their associated types
/// in layout and abi such that it is safe to transmute between the
/// two types
pub unsafe trait AbiTransferable {
    type Abi;

    fn get_abi(&self) -> Self::Abi;
    fn set_abi(&mut self) -> *mut Self::Abi;

    fn from_abi(abi: &Self::Abi) -> &Self {
        // This must be safe for the implementing type to
        // correctly implement this trait
        unsafe { std::mem::transmute_copy(&abi) }
    }

    fn from_mut_abi(abi: &mut Self::Abi) -> &mut Self {
        // This must be safe for the implementing type to
        // correctly implement this trait
        unsafe { std::mem::transmute_copy(&abi) }
    }
}

macro_rules! primitive_transferable_type {
    ($($t:ty),+) => {
        $(unsafe impl AbiTransferable for $t {
            type Abi = Self;
            fn get_abi(&self) -> Self::Abi {
                *self
            }
            fn set_abi(&mut self) -> *mut Self::Abi {
                self as *mut Self::Abi
            }
        })*
    };
}

primitive_transferable_type! {
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
