pub const NDIS_DEFAULT_PORT_NUMBER: NDIS_PORT_NUMBER = NDIS_PORT_NUMBER(0);
pub const NDIS_MAXIMUM_PORTS: u32 = 16777216;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NDIS_PORT_NUMBER(pub u32);
pub type NDIS_PORT_TYPE = i32;
pub const NdisPortType8021xSupplicant: NDIS_PORT_TYPE = 3;
pub const NdisPortTypeBridge: NDIS_PORT_TYPE = 1;
pub const NdisPortTypeMax: NDIS_PORT_TYPE = 4;
pub const NdisPortTypeRasConnection: NDIS_PORT_TYPE = 2;
pub const NdisPortTypeUndefined: NDIS_PORT_TYPE = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDIS_PORT_NUMBER(pub *mut u32);
impl PNDIS_PORT_NUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNDIS_PORT_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDIS_PORT_TYPE(pub *mut NDIS_PORT_TYPE);
impl PNDIS_PORT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNDIS_PORT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
