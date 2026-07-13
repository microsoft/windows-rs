pub const VDSBusType1394: VDS_STORAGE_BUS_TYPE = 4;
pub const VDSBusTypeAta: VDS_STORAGE_BUS_TYPE = 3;
pub const VDSBusTypeAtapi: VDS_STORAGE_BUS_TYPE = 2;
pub const VDSBusTypeFibre: VDS_STORAGE_BUS_TYPE = 6;
pub const VDSBusTypeFileBackedVirtual: VDS_STORAGE_BUS_TYPE = 15;
pub const VDSBusTypeMax: VDS_STORAGE_BUS_TYPE = 14;
pub const VDSBusTypeMaxReserved: VDS_STORAGE_BUS_TYPE = 127;
pub const VDSBusTypeMmc: VDS_STORAGE_BUS_TYPE = 13;
pub const VDSBusTypeNVMe: VDS_STORAGE_BUS_TYPE = 17;
pub const VDSBusTypeRAID: VDS_STORAGE_BUS_TYPE = 8;
pub const VDSBusTypeSas: VDS_STORAGE_BUS_TYPE = 10;
pub const VDSBusTypeSata: VDS_STORAGE_BUS_TYPE = 11;
pub const VDSBusTypeScm: VDS_STORAGE_BUS_TYPE = 18;
pub const VDSBusTypeScsi: VDS_STORAGE_BUS_TYPE = 1;
pub const VDSBusTypeSd: VDS_STORAGE_BUS_TYPE = 12;
pub const VDSBusTypeSpaces: VDS_STORAGE_BUS_TYPE = 16;
pub const VDSBusTypeSsa: VDS_STORAGE_BUS_TYPE = 5;
pub const VDSBusTypeUfs: VDS_STORAGE_BUS_TYPE = 19;
pub const VDSBusTypeUnknown: VDS_STORAGE_BUS_TYPE = 0;
pub const VDSBusTypeUsb: VDS_STORAGE_BUS_TYPE = 7;
pub const VDSBusTypeVirtual: VDS_STORAGE_BUS_TYPE = 14;
pub const VDSBusTypeiScsi: VDS_STORAGE_BUS_TYPE = 9;
pub const VDSStorageIdCodeSetAscii: VDS_STORAGE_IDENTIFIER_CODE_SET = 2;
pub const VDSStorageIdCodeSetBinary: VDS_STORAGE_IDENTIFIER_CODE_SET = 1;
pub const VDSStorageIdCodeSetReserved: VDS_STORAGE_IDENTIFIER_CODE_SET = 0;
pub const VDSStorageIdCodeSetUtf8: VDS_STORAGE_IDENTIFIER_CODE_SET = 3;
pub const VDSStorageIdTypeEUI64: VDS_STORAGE_IDENTIFIER_TYPE = 2;
pub const VDSStorageIdTypeFCPHName: VDS_STORAGE_IDENTIFIER_TYPE = 3;
pub const VDSStorageIdTypeLogicalUnitGroup: VDS_STORAGE_IDENTIFIER_TYPE = 6;
pub const VDSStorageIdTypeMD5LogicalUnitIdentifier: VDS_STORAGE_IDENTIFIER_TYPE = 7;
pub const VDSStorageIdTypePortRelative: VDS_STORAGE_IDENTIFIER_TYPE = 4;
pub const VDSStorageIdTypeScsiNameString: VDS_STORAGE_IDENTIFIER_TYPE = 8;
pub const VDSStorageIdTypeTargetPortGroup: VDS_STORAGE_IDENTIFIER_TYPE = 5;
pub const VDSStorageIdTypeVendorId: VDS_STORAGE_IDENTIFIER_TYPE = 1;
pub const VDSStorageIdTypeVendorSpecific: VDS_STORAGE_IDENTIFIER_TYPE = 0;
pub const VDS_IA_FCFS: VDS_INTERCONNECT_ADDRESS_TYPE = 1;
pub const VDS_IA_FCPH: VDS_INTERCONNECT_ADDRESS_TYPE = 2;
pub const VDS_IA_FCPH3: VDS_INTERCONNECT_ADDRESS_TYPE = 3;
pub const VDS_IA_MAC: VDS_INTERCONNECT_ADDRESS_TYPE = 4;
pub const VDS_IA_SCSI: VDS_INTERCONNECT_ADDRESS_TYPE = 5;
pub const VDS_IA_UNKNOWN: VDS_INTERCONNECT_ADDRESS_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VDS_INTERCONNECT {
    pub m_addressType: VDS_INTERCONNECT_ADDRESS_TYPE,
    pub m_cbPort: u32,
    pub m_pbPort: *mut u8,
    pub m_cbAddress: u32,
    pub m_pbAddress: *mut u8,
}
impl Default for VDS_INTERCONNECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_INTERCONNECT_ADDRESS_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VDS_LUN_INFORMATION {
    pub m_version: u32,
    pub m_DeviceType: u8,
    pub m_DeviceTypeModifier: u8,
    pub m_bCommandQueueing: windows_core::BOOL,
    pub m_BusType: VDS_STORAGE_BUS_TYPE,
    pub m_szVendorId: *mut i8,
    pub m_szProductId: *mut i8,
    pub m_szProductRevision: *mut i8,
    pub m_szSerialNumber: *mut i8,
    pub m_diskSignature: windows_core::GUID,
    pub m_deviceIdDescriptor: VDS_STORAGE_DEVICE_ID_DESCRIPTOR,
    pub m_cInterconnects: u32,
    pub m_rgInterconnects: *mut VDS_INTERCONNECT,
}
impl Default for VDS_LUN_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_STORAGE_BUS_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    pub m_version: u32,
    pub m_cIdentifiers: u32,
    pub m_rgIdentifiers: *mut VDS_STORAGE_IDENTIFIER,
}
impl Default for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VDS_STORAGE_IDENTIFIER {
    pub m_CodeSet: VDS_STORAGE_IDENTIFIER_CODE_SET,
    pub m_Type: VDS_STORAGE_IDENTIFIER_TYPE,
    pub m_cbIdentifier: u32,
    pub m_rgbIdentifier: *mut u8,
}
impl Default for VDS_STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_STORAGE_IDENTIFIER_CODE_SET = i32;
pub type VDS_STORAGE_IDENTIFIER_TYPE = i32;
pub const VER_VDS_LUN_INFORMATION: u32 = 1;
