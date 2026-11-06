#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Mixed {
    pub tag: u8,
    pub bits: __windows_bindgen_Unaligned<i32, {::core::mem::size_of::<i32>() }>,
    pub pointer: __windows_bindgen_Unaligned<*mut u32, {::core::mem::size_of::<*mut u32>() }>,
    pub nested: __windows_bindgen_Unaligned<Padded, {::core::mem::size_of::<Padded>() }>,
}
impl Default for Mixed {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Mixed {
    pub unsafe fn Delta(&self) -> i32 {
        (((((unsafe { self.bits.read() }) as u32) & 31) as i32) << 27) >> 27
    }
    pub unsafe fn set_Delta(&mut self, value: i32) {
        self.bits = __windows_bindgen_Unaligned::new(
            ((((unsafe { self.bits.read() }) as u32) & !31) | ((value as u32) & 31)) as i32,
        );
    }
    pub unsafe fn Flag(&self) -> bool {
        ((((unsafe { self.bits.read() }) as u32) >> 5) & 1) != 0
    }
    pub unsafe fn set_Flag(&mut self, value: bool) {
        self.bits = __windows_bindgen_Unaligned::new(
            ((((unsafe { self.bits.read() }) as u32) & !(1 << 5)) | (((value as u32) & 1) << 5))
                as i32,
        );
    }
}
#[repr(C, align(2))]
#[derive(Clone, Copy)]
pub union MixedUnion {
    pub bits: __windows_bindgen_Unaligned<i32, {::core::mem::size_of::<i32>() }>,
    pub pointer: __windows_bindgen_Unaligned<*mut u32, {::core::mem::size_of::<*mut u32>() }>,
}
impl Default for MixedUnion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl MixedUnion {
    pub unsafe fn Delta(&self) -> i32 {
        (((((unsafe { self.bits.read() }) as u32) & 31) as i32) << 27) >> 27
    }
    pub unsafe fn set_Delta(&mut self, value: i32) {
        self.bits = __windows_bindgen_Unaligned::new(
            ((((unsafe { self.bits.read() }) as u32) & !31) | ((value as u32) & 31)) as i32,
        );
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct Packed {
    pub tag: u8,
    pub bits: i32,
    pub pointer: *mut u32,
}
impl Packed {
    pub fn Delta(&self) -> i32 {
        (self.bits << 27) >> 27
    }
    pub fn set_Delta(&mut self, value: i32) {
        self.bits = ((self.bits as u32 & !31) | (value as u32 & 31)) as i32;
    }
    pub fn Flag(&self) -> bool {
        (self.bits >> 5) & 1 != 0
    }
    pub fn set_Flag(&mut self, value: bool) {
        self.bits = (self.bits & !(1 << 5)) | ((value as i32) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Padded {
    pub tag: u8,
    pub value: u32,
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
