#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AliasContainer {
    pub value: MixedAlias,
}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Aligned {
    pub value: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Container {
    pub values: [PackedMixed; 2],
    pub tail: u8,
}
impl Default for Container {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MixedAlias = PackedMixed;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NestedContainer {
    pub values: [[Container; 2]; 2],
}
impl Default for NestedContainer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct PackedMixed {
    pub tag: u8,
    #[doc(hidden)]
    pub __padding_1: [core::mem::MaybeUninit<u8>; 3],
    pub pointer: *mut core::ffi::c_void,
    #[doc(hidden)]
    pub __padding_2: [core::mem::MaybeUninit<u8>; 8],
    pub value: Aligned,
    pub tail: u8,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct PackedMixed {
    pub tag: u8,
    #[doc(hidden)]
    pub __padding_1: [core::mem::MaybeUninit<u8>; 3],
    pub pointer: __windows_bindgen_Unaligned<
        *mut core::ffi::c_void,
        {::core::mem::size_of::<*mut core::ffi::c_void>() },
    >,
    #[doc(hidden)]
    pub __padding_2: [core::mem::MaybeUninit<u8>; 4],
    pub value: Aligned,
    pub tail: u8,
}
impl Default for PackedMixed {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerOnly {
    pub value: *const PackedMixed,
}
#[doc = r" Unaligned native field storage. No references to the stored value are exposed."]
#[doc(hidden)]
#[repr(C)]
pub struct __windows_bindgen_Unaligned<T, const N: usize> {
    bytes: [core::mem::MaybeUninit<u8>; N],
    value: core::marker::PhantomData<T>,
}
impl<T, const N: usize> __windows_bindgen_Unaligned<T, N> {
    pub fn new(value: T) -> Self {
        let mut result = Self {
            bytes: [core::mem::MaybeUninit::uninit(); N],
            value: core::marker::PhantomData,
        };
        result.write(value);
        result
    }
    #[doc = r" Copies the stored value without creating an aligned reference."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Storage must contain a valid, initialized `T`. For non-`Copy` values,"]
    #[doc = r" the caller must prevent duplicated ownership and subsequent reads."]
    pub unsafe fn read(&self) -> T {
        assert_eq!(N,::core::mem::size_of::<T>());
        unsafe { self.bytes.as_ptr().cast::<T>().read_unaligned() }
    }
    #[doc = r" Writes a value without dropping the previous contents."]
    pub fn write(&mut self, value: T) {
        assert_eq!(N,::core::mem::size_of::<T>());
        unsafe {
            self.bytes.as_mut_ptr().cast::<T>().write_unaligned(value);
        }
    }
}
impl<T: Copy, const N: usize> Copy for __windows_bindgen_Unaligned<T, N> {}
impl<T: Copy, const N: usize> Clone for __windows_bindgen_Unaligned<T, N> {
    fn clone(&self) -> Self {
        *self
    }
}
