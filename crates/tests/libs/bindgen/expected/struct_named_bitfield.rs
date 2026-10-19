pub type Alias = Byte;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Byte(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Enumerated {
    pub _bitfield: Kind,
}
impl Enumerated {
    pub fn Signed(&self) -> i32 {
        (self._bitfield << 27) >> 27
    }
    pub fn set_Signed(&mut self, value: i32) {
        self._bitfield = ((self._bitfield as u32 & !31) | (value as u32 & 31)) as i32;
    }
    pub fn Flag(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Flag(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as i32) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Full {
    pub _bitfield: Alias,
}
impl Full {
    pub fn Value(&self) -> u8 {
        self._bitfield.0
    }
    pub fn set_Value(&mut self, value: u8) {
        self._bitfield = Byte(value);
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FullPointer {
    pub unsigned: usize,
    pub signed: isize,
}
#[cfg(target_arch = "x86")]
impl FullPointer {
    pub fn Unsigned(&self) -> usize {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target storage width");
        (self.unsigned) & 4294967295
    }
    pub fn set_Unsigned(&mut self, value: usize) {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target storage width");
        self.unsigned = (self.unsigned & !4294967295) | (value & 4294967295);
    }
    pub fn Signed(&self) -> isize {
        const _: () = assert!(isize::BITS >= 32, "bitfield exceeds target storage width");
        ((((self.signed as usize) & 4294967295) as isize) << (isize::BITS - 32))
            >> (isize::BITS - 32)
    }
    pub fn set_Signed(&mut self, value: isize) {
        const _: () = assert!(isize::BITS >= 32, "bitfield exceeds target storage width");
        self.signed =
            (((self.signed as usize) & !4294967295) | ((value as usize) & 4294967295)) as isize;
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FullPointer {
    pub unsigned: usize,
    pub signed: isize,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl FullPointer {
    pub fn Unsigned(&self) -> usize {
        const _: () = assert!(usize::BITS >= 64, "bitfield exceeds target storage width");
        self.unsigned
    }
    pub fn set_Unsigned(&mut self, value: usize) {
        const _: () = assert!(usize::BITS >= 64, "bitfield exceeds target storage width");
        self.unsigned = value;
    }
    pub fn Signed(&self) -> isize {
        const _: () = assert!(isize::BITS >= 64, "bitfield exceeds target storage width");
        self.signed
    }
    pub fn set_Signed(&mut self, value: isize) {
        const _: () = assert!(isize::BITS >= 64, "bitfield exceeds target storage width");
        self.signed = value;
    }
}
pub type Kind = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Logical {
    pub _bitfield: windows_core::BOOL,
}
impl Logical {
    pub fn Signed(&self) -> i32 {
        ((((self._bitfield.0 as u32) & 31) as i32) << 27) >> 27
    }
    pub fn set_Signed(&mut self, value: i32) {
        self._bitfield =
            windows_core::BOOL((((self._bitfield.0 as u32) & !31) | ((value as u32) & 31)) as i32);
    }
    pub fn Flag(&self) -> bool {
        (((self._bitfield.0 as u32) >> 5) & 1) != 0
    }
    pub fn set_Flag(&mut self, value: bool) {
        self._bitfield = windows_core::BOOL(
            (((self._bitfield.0 as u32) & !(1 << 5)) | (((value as u32) & 1) << 5)) as i32,
        );
    }
}
pub const One: Kind = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Pointer {
    pub _bitfield: usize,
}
impl Pointer {
    pub fn Low(&self) -> usize {
        const _: () = assert!(usize::BITS >= 4, "bitfield exceeds target storage width");
        (self._bitfield) & 15
    }
    pub fn set_Low(&mut self, value: usize) {
        const _: () = assert!(usize::BITS >= 4, "bitfield exceeds target storage width");
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn High(&self) -> usize {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target storage width");
        (self._bitfield >> 4) & 268435455
    }
    pub fn set_High(&mut self, value: usize) {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target storage width");
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerMember {
    pub storage: u64,
}
impl PointerMember {
    pub fn Value(&self) -> isize {
        const _: () = assert!(isize::BITS >= 32, "bitfield exceeds target member width");
        ((((self.storage >> 16) & 4294967295) as isize) << (isize::BITS - 32)) >> (isize::BITS - 32)
    }
    pub fn set_Value(&mut self, value: isize) {
        const _: () = assert!(isize::BITS >= 32, "bitfield exceeds target member width");
        self.storage = (self.storage & !(4294967295 << 16)) | (((value as u64) & 4294967295) << 16);
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScopedKind(pub i16);
impl ScopedKind {
    pub const Zero: Self = Self(0);
    pub const Negative: Self = Self(-1);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ScopedUnion {
    pub _bitfield: ScopedKind,
    pub All: i16,
}
impl ScopedUnion {
    pub fn Signed(&self) -> i16 {
        (((((unsafe { self._bitfield }).0 as u16) & 31) as i16) << 11) >> 11
    }
    pub fn set_Signed(&mut self, value: i16) {
        self._bitfield = ScopedKind(
            ((((unsafe { self._bitfield }).0 as u16) & !31) | ((value as u16) & 31)) as i16,
        );
    }
    pub fn Flag(&self) -> bool {
        ((((unsafe { self._bitfield }).0 as u16) >> 5) & 1) != 0
    }
    pub fn set_Flag(&mut self, value: bool) {
        self._bitfield = ScopedKind(
            ((((unsafe { self._bitfield }).0 as u16) & !(1 << 5)) | (((value as u16) & 1) << 5))
                as i16,
        );
    }
}
impl Default for ScopedUnion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SignedPointer {
    pub _bitfield: isize,
}
impl SignedPointer {
    pub fn Low(&self) -> isize {
        const _: () = assert!(isize::BITS >= 4, "bitfield exceeds target storage width");
        ((((self._bitfield as usize) & 15) as isize) << (isize::BITS - 4)) >> (isize::BITS - 4)
    }
    pub fn set_Low(&mut self, value: isize) {
        const _: () = assert!(isize::BITS >= 4, "bitfield exceeds target storage width");
        self._bitfield = (((self._bitfield as usize) & !15) | ((value as usize) & 15)) as isize;
    }
    pub fn High(&self) -> isize {
        const _: () = assert!(isize::BITS >= 32, "bitfield exceeds target storage width");
        (((((self._bitfield as usize) >> 4) & 268435455) as isize) << (isize::BITS - 28))
            >> (isize::BITS - 28)
    }
    pub fn set_High(&mut self, value: isize) {
        const _: () = assert!(isize::BITS >= 32, "bitfield exceeds target storage width");
        self._bitfield = (((self._bitfield as usize) & !(268435455 << 4))
            | (((value as usize) & 268435455) << 4)) as isize;
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Typed {
    pub storage: Alias,
}
impl Typed {
    pub fn Signed(&self) -> i8 {
        ((((self.storage.0) & 15) as i8) << 4) >> 4
    }
    pub fn set_Signed(&mut self, value: i8) {
        self.storage = Byte((self.storage.0 & !15) | ((value as u8) & 15));
    }
    pub fn Flag(&self) -> bool {
        ((self.storage.0 >> 4) & 1) != 0
    }
    pub fn set_Flag(&mut self, value: bool) {
        self.storage = Byte((self.storage.0 & !(1 << 4)) | (((value as u8) & 1) << 4));
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WideMember {
    pub storage: usize,
}
impl WideMember {
    pub fn Value(&self) -> u64 {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target storage width");
        ((self.storage) & 4294967295) as u64
    }
    pub fn set_Value(&mut self, value: u64) {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target storage width");
        self.storage = (self.storage & !4294967295) | ((value as usize) & 4294967295);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Wrapped {
    pub _bitfield: Alias,
}
impl Wrapped {
    pub fn Low(&self) -> u8 {
        (self._bitfield.0) & 7
    }
    pub fn set_Low(&mut self, value: u8) {
        self._bitfield = Byte((self._bitfield.0 & !7) | (value & 7));
    }
    pub fn High(&self) -> u8 {
        (self._bitfield.0 >> 3) & 15
    }
    pub fn set_High(&mut self, value: u8) {
        self._bitfield = Byte((self._bitfield.0 & !(15 << 3)) | ((value & 15) << 3));
    }
    pub fn Flag(&self) -> bool {
        (self._bitfield.0 >> 7) != 0
    }
    pub fn set_Flag(&mut self, value: bool) {
        self._bitfield = Byte((self._bitfield.0 & !(1 << 7)) | (((value as u8) & 1) << 7));
    }
}
pub const Zero: Kind = 0;
