#[repr(C)]
#[derive(Clone, Copy)]
pub struct External {
    pub values: [Named_0; 2],
}
impl Default for External {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Matrix {
    pub values: [[Named_0; 2]; 2],
}
impl Default for Matrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Named {
    pub values: [Named_0; 2],
}
impl Default for Named {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Named_0 {
    pub first: u8,
    #[doc(hidden)]
    pub __padding_1: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let alignment =::core::mem::align_of::<u8>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<u8>();
        let natural =::core::mem::align_of::<u32>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub second: u32,
}
impl Default for Named_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Outer {
    pub tag: u8,
    pub value: Outer_0,
}
impl Default for Outer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Outer_0 {
    pub first: u8,
    #[doc(hidden)]
    pub __padding_1: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let alignment =::core::mem::align_of::<u8>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<u8>();
        let natural =::core::mem::align_of::<u32>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub second: u32,
}
impl Default for Outer_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerOnly {
    pub value: *mut PointerOnly_0,
}
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct PointerOnly_0 {
    #[doc(hidden)]
    pub __padding_0: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let natural =::core::mem::align_of::<u32>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub value: u32,
}
impl Default for PointerOnly_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
