pub const NDIS_OBJECT_TYPE_OID_REQUEST: u32 = 150;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NDIS_OID(pub u32);
pub type NDIS_REQUEST_TYPE = i32;
pub const NdisRequestClose: NDIS_REQUEST_TYPE = 4;
pub const NdisRequestGeneric1: NDIS_REQUEST_TYPE = 8;
pub const NdisRequestGeneric2: NDIS_REQUEST_TYPE = 9;
pub const NdisRequestGeneric3: NDIS_REQUEST_TYPE = 10;
pub const NdisRequestGeneric4: NDIS_REQUEST_TYPE = 11;
pub const NdisRequestOpen: NDIS_REQUEST_TYPE = 3;
pub const NdisRequestQueryInformation: NDIS_REQUEST_TYPE = 0;
pub const NdisRequestQueryStatistics: NDIS_REQUEST_TYPE = 2;
pub const NdisRequestReset: NDIS_REQUEST_TYPE = 7;
pub const NdisRequestSend: NDIS_REQUEST_TYPE = 5;
pub const NdisRequestSetInformation: NDIS_REQUEST_TYPE = 1;
pub const NdisRequestTransferData: NDIS_REQUEST_TYPE = 6;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDIS_OID(pub *mut u32);
impl PNDIS_OID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNDIS_OID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDIS_REQUEST_TYPE(pub *mut NDIS_REQUEST_TYPE);
impl PNDIS_REQUEST_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNDIS_REQUEST_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
