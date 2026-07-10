pub const DEVPKEY_Test_Sample: DEVPROPKEY = DEVPROPKEY {
    fmtid: GUID {
        data1: 0x26e3e2a2,
        data2: 0x1234,
        data3: 0x5678,
        data4: [154, 188, 222, 240, 18, 52, 86, 120],
    },
    pid: 26,
};
pub type DEVPROPGUID = GUID;
pub type DEVPROPID = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEVPROPKEY {
    pub fmtid: DEVPROPGUID,
    pub pid: DEVPROPID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub const PKEY_Test_Sample: PROPERTYKEY = PROPERTYKEY {
    fmtid: GUID {
        data1: 0x540b947e,
        data2: 0x8b40,
        data3: 0x45bc,
        data4: [168, 162, 106, 11, 137, 76, 189, 162],
    },
    pid: 100,
};
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROPERTYKEY {
    pub fmtid: GUID,
    pub pid: u32,
}
