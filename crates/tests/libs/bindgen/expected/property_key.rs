pub const DEVPKEY_Test_Sample: DEVPROPKEY = DEVPROPKEY {
    fmtid: windows_core::GUID::from_u128(0x26e3e2a2_1234_5678_9abc_def012345678),
    pid: DEVPROPID(26),
};
pub type DEVPROPGUID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DEVPROPID(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVPROPKEY {
    pub fmtid: DEVPROPGUID,
    pub pid: DEVPROPID,
}
pub const PKEY_Test_Sample: PROPERTYKEY = PROPERTYKEY {
    fmtid: windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2),
    pid: 100,
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPERTYKEY {
    pub fmtid: windows_core::GUID,
    pub pid: u32,
}
