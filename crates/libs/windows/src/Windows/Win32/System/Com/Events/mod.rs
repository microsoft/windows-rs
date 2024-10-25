pub const EOC_DeletedObject: EOC_ChangeType = 2i32;
pub const EOC_ModifiedObject: EOC_ChangeType = 1i32;
pub const EOC_NewObject: EOC_ChangeType = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EOC_ChangeType(pub i32);
impl windows_core::TypeKind for EOC_ChangeType {
    type TypeKind = windows_core::CopyType;
}
pub const CEventClass: windows_core::GUID = windows_core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
pub const CEventPublisher: windows_core::GUID = windows_core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
pub const CEventSubscription: windows_core::GUID = windows_core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
pub const CEventSystem: windows_core::GUID = windows_core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: windows_core::BSTR,
    pub partitionId: windows_core::BSTR,
    pub applicationId: windows_core::BSTR,
    pub reserved: [windows_core::GUID; 10],
}
impl Default for COMEVENTSYSCHANGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COMEVENTSYSCHANGEINFO {
    type TypeKind = windows_core::CopyType;
}
pub const EventObjectChange: windows_core::GUID = windows_core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
pub const EventObjectChange2: windows_core::GUID = windows_core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
