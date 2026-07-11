pub const IFI_UNSPECIFIED: u32 = 0;
pub const IF_ADMINISTRATIVE_DEMANDDIAL: IF_ADMINISTRATIVE_STATE = 2;
pub const IF_ADMINISTRATIVE_DISABLED: IF_ADMINISTRATIVE_STATE = 0;
pub const IF_ADMINISTRATIVE_ENABLED: IF_ADMINISTRATIVE_STATE = 1;
pub type IF_ADMINISTRATIVE_STATE = i32;
pub type IF_COUNTED_STRING = IF_COUNTED_STRING_LH;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IF_COUNTED_STRING_LH {
    pub Length: u16,
    pub String: [u16; 257],
}
impl Default for IF_COUNTED_STRING_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IF_INDEX = NET_IFINDEX;
pub type IF_LUID = NET_LUID;
pub const IF_MAX_PHYS_ADDRESS_LENGTH: u32 = 32;
pub const IF_MAX_STRING_SIZE: u32 = 256;
pub type IF_OPER_STATUS = i32;
pub type IF_PHYSICAL_ADDRESS = IF_PHYSICAL_ADDRESS_LH;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IF_PHYSICAL_ADDRESS_LH {
    pub Length: u16,
    pub Address: [u8; 32],
}
impl Default for IF_PHYSICAL_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IfOperStatusDormant: IF_OPER_STATUS = 5;
pub const IfOperStatusDown: IF_OPER_STATUS = 2;
pub const IfOperStatusLowerLayerDown: IF_OPER_STATUS = 7;
pub const IfOperStatusNotPresent: IF_OPER_STATUS = 6;
pub const IfOperStatusTesting: IF_OPER_STATUS = 3;
pub const IfOperStatusUnknown: IF_OPER_STATUS = 4;
pub const IfOperStatusUp: IF_OPER_STATUS = 1;
pub const MediaConnectStateConnected: NET_IF_MEDIA_CONNECT_STATE = 1;
pub const MediaConnectStateDisconnected: NET_IF_MEDIA_CONNECT_STATE = 2;
pub const MediaConnectStateUnknown: NET_IF_MEDIA_CONNECT_STATE = 0;
pub const MediaDuplexStateFull: NET_IF_MEDIA_DUPLEX_STATE = 2;
pub const MediaDuplexStateHalf: NET_IF_MEDIA_DUPLEX_STATE = 1;
pub const MediaDuplexStateUnknown: NET_IF_MEDIA_DUPLEX_STATE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDIS_INTERFACE_INFORMATION {
    pub ifOperStatus: NET_IF_OPER_STATUS,
    pub ifOperStatusFlags: u32,
    pub MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    pub MediaDuplexState: NET_IF_MEDIA_DUPLEX_STATE,
    pub ifMtu: u32,
    pub ifPromiscuousMode: bool,
    pub ifDeviceWakeUpEnable: bool,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub ifLastChange: u64,
    pub ifCounterDiscontinuityTime: u64,
    pub ifInUnknownProtos: u64,
    pub ifInDiscards: u64,
    pub ifInErrors: u64,
    pub ifHCInOctets: u64,
    pub ifHCInUcastPkts: u64,
    pub ifHCInMulticastPkts: u64,
    pub ifHCInBroadcastPkts: u64,
    pub ifHCOutOctets: u64,
    pub ifHCOutUcastPkts: u64,
    pub ifHCOutMulticastPkts: u64,
    pub ifHCOutBroadcastPkts: u64,
    pub ifOutErrors: u64,
    pub ifOutDiscards: u64,
    pub ifHCInUcastOctets: u64,
    pub ifHCInMulticastOctets: u64,
    pub ifHCInBroadcastOctets: u64,
    pub ifHCOutUcastOctets: u64,
    pub ifHCOutMulticastOctets: u64,
    pub ifHCOutBroadcastOctets: u64,
    pub CompartmentId: NET_IF_COMPARTMENT_ID,
    pub SupportedStatistics: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NET_IFINDEX(pub u32);
pub const NET_IFINDEX_UNSPECIFIED: u32 = 0;
pub const NET_IFLUID_UNSPECIFIED: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NET_IFTYPE(pub u16);
pub const NET_IF_ACCESS_BROADCAST: NET_IF_ACCESS_TYPE = 2;
pub const NET_IF_ACCESS_LOOPBACK: NET_IF_ACCESS_TYPE = 1;
pub const NET_IF_ACCESS_MAXIMUM: NET_IF_ACCESS_TYPE = 5;
pub const NET_IF_ACCESS_POINT_TO_MULTI_POINT: NET_IF_ACCESS_TYPE = 4;
pub const NET_IF_ACCESS_POINT_TO_POINT: NET_IF_ACCESS_TYPE = 3;
pub type NET_IF_ACCESS_TYPE = i32;
pub type NET_IF_ADMIN_STATUS = i32;
pub const NET_IF_ADMIN_STATUS_DOWN: NET_IF_ADMIN_STATUS = 2;
pub const NET_IF_ADMIN_STATUS_TESTING: NET_IF_ADMIN_STATUS = 3;
pub const NET_IF_ADMIN_STATUS_UP: NET_IF_ADMIN_STATUS = 1;
pub type NET_IF_ALIAS = NET_IF_ALIAS_LH;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NET_IF_ALIAS_LH {
    pub ifAliasLength: u16,
    pub ifAliasOffset: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NET_IF_COMPARTMENT_ID(pub u32);
pub const NET_IF_COMPARTMENT_ID_PRIMARY: NET_IF_COMPARTMENT_ID = NET_IF_COMPARTMENT_ID(1);
pub const NET_IF_COMPARTMENT_ID_UNSPECIFIED: NET_IF_COMPARTMENT_ID = NET_IF_COMPARTMENT_ID(0);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NET_IF_COMPARTMENT_SCOPE(pub u32);
pub const NET_IF_COMPARTMENT_SCOPE_ALL: NET_IF_COMPARTMENT_SCOPE = NET_IF_COMPARTMENT_SCOPE(4294967295);
pub const NET_IF_COMPARTMENT_SCOPE_UNSPECIFIED: NET_IF_COMPARTMENT_SCOPE = NET_IF_COMPARTMENT_SCOPE(0);
pub const NET_IF_CONNECTION_DEDICATED: NET_IF_CONNECTION_TYPE = 1;
pub const NET_IF_CONNECTION_DEMAND: NET_IF_CONNECTION_TYPE = 3;
pub const NET_IF_CONNECTION_MAXIMUM: NET_IF_CONNECTION_TYPE = 4;
pub const NET_IF_CONNECTION_PASSIVE: NET_IF_CONNECTION_TYPE = 2;
pub type NET_IF_CONNECTION_TYPE = i32;
pub const NET_IF_DIRECTION_MAXIMUM: NET_IF_DIRECTION_TYPE = 3;
pub const NET_IF_DIRECTION_RECEIVEONLY: NET_IF_DIRECTION_TYPE = 2;
pub const NET_IF_DIRECTION_SENDONLY: NET_IF_DIRECTION_TYPE = 1;
pub const NET_IF_DIRECTION_SENDRECEIVE: NET_IF_DIRECTION_TYPE = 0;
pub type NET_IF_DIRECTION_TYPE = i32;
pub const NET_IF_LINK_SPEED_UNKNOWN: i32 = -1;
pub type NET_IF_MEDIA_CONNECT_STATE = i32;
pub type NET_IF_MEDIA_DUPLEX_STATE = i32;
pub type NET_IF_NETWORK_GUID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NET_IF_OBJECT_ID(pub u32);
pub const NET_IF_OID_COMPARTMENT_ID: u32 = 2;
pub const NET_IF_OID_IF_ALIAS: u32 = 1;
pub const NET_IF_OID_IF_ENTRY: u32 = 4;
pub const NET_IF_OID_NETWORK_GUID: u32 = 3;
pub type NET_IF_OPER_STATUS = i32;
pub const NET_IF_OPER_STATUS_DORMANT: NET_IF_OPER_STATUS = 5;
pub const NET_IF_OPER_STATUS_DORMANT_LOW_POWER: u32 = 8;
pub const NET_IF_OPER_STATUS_DORMANT_PAUSED: u32 = 4;
pub const NET_IF_OPER_STATUS_DOWN: NET_IF_OPER_STATUS = 2;
pub const NET_IF_OPER_STATUS_DOWN_NOT_AUTHENTICATED: u32 = 1;
pub const NET_IF_OPER_STATUS_DOWN_NOT_MEDIA_CONNECTED: u32 = 2;
pub const NET_IF_OPER_STATUS_LOWER_LAYER_DOWN: NET_IF_OPER_STATUS = 7;
pub const NET_IF_OPER_STATUS_NOT_PRESENT: NET_IF_OPER_STATUS = 6;
pub const NET_IF_OPER_STATUS_TESTING: NET_IF_OPER_STATUS = 3;
pub const NET_IF_OPER_STATUS_UNKNOWN: NET_IF_OPER_STATUS = 4;
pub const NET_IF_OPER_STATUS_UP: NET_IF_OPER_STATUS = 1;
pub type NET_IF_RCV_ADDRESS = NET_IF_RCV_ADDRESS_LH;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NET_IF_RCV_ADDRESS_LH {
    pub ifRcvAddressType: NET_IF_RCV_ADDRESS_TYPE,
    pub ifRcvAddressLength: u16,
    pub ifRcvAddressOffset: u16,
}
pub type NET_IF_RCV_ADDRESS_TYPE = i32;
pub const NET_IF_RCV_ADDRESS_TYPE_NON_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = 3;
pub const NET_IF_RCV_ADDRESS_TYPE_OTHER: NET_IF_RCV_ADDRESS_TYPE = 1;
pub const NET_IF_RCV_ADDRESS_TYPE_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = 2;
pub type NET_LUID = NET_LUID_LH;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NET_LUID_LH {
    pub Value: u64,
    pub Info: NET_LUID_LH_0,
}
impl Default for NET_LUID_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NET_LUID_LH_0 {
    pub _bitfield: u64,
}
pub type NET_PHYSICAL_LOCATION = NET_PHYSICAL_LOCATION_LH;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NET_PHYSICAL_LOCATION_LH {
    pub BusNumber: u32,
    pub SlotNumber: u32,
    pub FunctionNumber: u32,
}
pub const NET_SITEID_MAXSYSTEM: u32 = 268435455;
pub const NET_SITEID_MAXUSER: u32 = 134217727;
pub const NET_SITEID_UNSPECIFIED: u32 = 0;
pub const NIIF_BUS_NUMBER_UNKNOWN: i32 = -1;
pub const NIIF_FILTER_INTERFACE: u32 = 2;
pub const NIIF_FUNCTION_NUMBER_UNKNOWN: i32 = -1;
pub const NIIF_HARDWARE_INTERFACE: u32 = 1;
pub const NIIF_NDIS_ENDPOINT_INTERFACE: u32 = 64;
pub const NIIF_NDIS_ISCSI_INTERFACE: u32 = 128;
pub const NIIF_NDIS_RESERVED1: u32 = 4;
pub const NIIF_NDIS_RESERVED2: u32 = 8;
pub const NIIF_NDIS_RESERVED3: u32 = 16;
pub const NIIF_NDIS_RESERVED4: u32 = 256;
pub const NIIF_NDIS_WDM_INTERFACE: u32 = 32;
pub const NIIF_SLOT_NUMBER_UNKNOWN: i32 = -1;
pub const NIIF_WAN_TUNNEL_TYPE_UNKNOWN: i32 = -1;
pub type PIF_ADMINISTRATIVE_STATE = *mut IF_ADMINISTRATIVE_STATE;
pub type PIF_COUNTED_STRING = *mut IF_COUNTED_STRING;
pub type PIF_COUNTED_STRING_LH = *mut IF_COUNTED_STRING_LH;
pub type PIF_INDEX = *mut NET_IFINDEX;
pub type PIF_LUID = *mut NET_LUID;
pub type PIF_PHYSICAL_ADDRESS = *mut IF_PHYSICAL_ADDRESS;
pub type PIF_PHYSICAL_ADDRESS_LH = *mut IF_PHYSICAL_ADDRESS_LH;
pub type PNDIS_INTERFACE_INFORMATION = *mut NDIS_INTERFACE_INFORMATION;
pub type PNET_IFINDEX = *mut u32;
pub type PNET_IFTYPE = *mut u16;
pub type PNET_IF_ACCESS_TYPE = *mut NET_IF_ACCESS_TYPE;
pub type PNET_IF_ADMIN_STATUS = *mut NET_IF_ADMIN_STATUS;
pub type PNET_IF_ALIAS = *mut NET_IF_ALIAS;
pub type PNET_IF_ALIAS_LH = *mut NET_IF_ALIAS_LH;
pub type PNET_IF_COMPARTMENT_ID = *mut u32;
pub type PNET_IF_COMPARTMENT_SCOPE = *mut u32;
pub type PNET_IF_CONNECTION_TYPE = *mut NET_IF_CONNECTION_TYPE;
pub type PNET_IF_DIRECTION_TYPE = *mut NET_IF_DIRECTION_TYPE;
pub type PNET_IF_MEDIA_CONNECT_STATE = *mut NET_IF_MEDIA_CONNECT_STATE;
pub type PNET_IF_MEDIA_DUPLEX_STATE = *mut NET_IF_MEDIA_DUPLEX_STATE;
pub type PNET_IF_NETWORK_GUID = *mut windows_core::GUID;
pub type PNET_IF_OBJECT_ID = *mut u32;
pub type PNET_IF_OPER_STATUS = *mut NET_IF_OPER_STATUS;
pub type PNET_IF_RCV_ADDRESS = *mut NET_IF_RCV_ADDRESS;
pub type PNET_IF_RCV_ADDRESS_LH = *mut NET_IF_RCV_ADDRESS_LH;
pub type PNET_IF_RCV_ADDRESS_TYPE = *mut NET_IF_RCV_ADDRESS_TYPE;
pub type PNET_LUID = *mut NET_LUID;
pub type PNET_LUID_LH = *mut NET_LUID_LH;
pub type PNET_PHYSICAL_LOCATION = *mut NET_PHYSICAL_LOCATION;
pub type PNET_PHYSICAL_LOCATION_LH = *mut NET_PHYSICAL_LOCATION_LH;
pub type PTUNNEL_TYPE = *mut TUNNEL_TYPE;
pub type TUNNEL_TYPE = i32;
pub const TUNNEL_TYPE_6TO4: TUNNEL_TYPE = 11;
pub const TUNNEL_TYPE_DIRECT: TUNNEL_TYPE = 2;
pub const TUNNEL_TYPE_IPHTTPS: TUNNEL_TYPE = 15;
pub const TUNNEL_TYPE_ISATAP: TUNNEL_TYPE = 13;
pub const TUNNEL_TYPE_NONE: TUNNEL_TYPE = 0;
pub const TUNNEL_TYPE_OTHER: TUNNEL_TYPE = 1;
pub const TUNNEL_TYPE_TEREDO: TUNNEL_TYPE = 14;
