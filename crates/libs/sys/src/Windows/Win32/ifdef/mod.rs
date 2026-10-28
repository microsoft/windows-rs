pub const IFI_UNSPECIFIED: NET_IFINDEX = 0;
pub const IF_ADMINISTRATIVE_DEMANDDIAL: IF_ADMINISTRATIVE_STATE = 2;
pub const IF_ADMINISTRATIVE_DISABLED: IF_ADMINISTRATIVE_STATE = 0;
pub const IF_ADMINISTRATIVE_ENABLED: IF_ADMINISTRATIVE_STATE = 1;
pub type IF_ADMINISTRATIVE_STATE = i32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type IF_COUNTED_STRING = IF_COUNTED_STRING_LH;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IF_COUNTED_STRING_LH {
    pub Length: super::USHORT,
    pub String: [super::WCHAR; 257],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IF_COUNTED_STRING_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IF_INDEX = u32;
#[cfg(feature = "shared")]
pub type IF_LUID = NET_LUID_LH;
pub const IF_MAX_PHYS_ADDRESS_LENGTH: i32 = 32;
pub const IF_MAX_STRING_SIZE: i32 = 256;
pub type IF_OPER_STATUS = i32;
#[cfg(feature = "shared")]
pub type IF_PHYSICAL_ADDRESS = IF_PHYSICAL_ADDRESS_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IF_PHYSICAL_ADDRESS_LH {
    pub Length: super::USHORT,
    pub Address: [super::UCHAR; 32],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NDIS_INTERFACE_INFORMATION {
    pub ifOperStatus: NET_IF_OPER_STATUS,
    pub ifOperStatusFlags: super::ULONG,
    pub MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    pub MediaDuplexState: NET_IF_MEDIA_DUPLEX_STATE,
    pub ifMtu: super::ULONG,
    pub ifPromiscuousMode: super::BOOLEAN,
    pub ifDeviceWakeUpEnable: super::BOOLEAN,
    pub XmitLinkSpeed: super::ULONG64,
    pub RcvLinkSpeed: super::ULONG64,
    pub ifLastChange: super::ULONG64,
    pub ifCounterDiscontinuityTime: super::ULONG64,
    pub ifInUnknownProtos: super::ULONG64,
    pub ifInDiscards: super::ULONG64,
    pub ifInErrors: super::ULONG64,
    pub ifHCInOctets: super::ULONG64,
    pub ifHCInUcastPkts: super::ULONG64,
    pub ifHCInMulticastPkts: super::ULONG64,
    pub ifHCInBroadcastPkts: super::ULONG64,
    pub ifHCOutOctets: super::ULONG64,
    pub ifHCOutUcastPkts: super::ULONG64,
    pub ifHCOutMulticastPkts: super::ULONG64,
    pub ifHCOutBroadcastPkts: super::ULONG64,
    pub ifOutErrors: super::ULONG64,
    pub ifOutDiscards: super::ULONG64,
    pub ifHCInUcastOctets: super::ULONG64,
    pub ifHCInMulticastOctets: super::ULONG64,
    pub ifHCInBroadcastOctets: super::ULONG64,
    pub ifHCOutUcastOctets: super::ULONG64,
    pub ifHCOutMulticastOctets: super::ULONG64,
    pub ifHCOutBroadcastOctets: super::ULONG64,
    pub CompartmentId: NET_IF_COMPARTMENT_ID,
    pub SupportedStatistics: super::ULONG,
}
pub type NET_IFINDEX = u32;
pub const NET_IFINDEX_UNSPECIFIED: NET_IFINDEX = 0;
pub const NET_IFLUID_UNSPECIFIED: i32 = 0;
pub type NET_IFTYPE = u16;
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
#[cfg(feature = "shared")]
pub type NET_IF_ALIAS = NET_IF_ALIAS_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_IF_ALIAS_LH {
    pub ifAliasLength: super::USHORT,
    pub ifAliasOffset: super::USHORT,
}
pub type NET_IF_COMPARTMENT_ID = u32;
pub const NET_IF_COMPARTMENT_ID_PRIMARY: NET_IF_COMPARTMENT_ID = 1;
pub const NET_IF_COMPARTMENT_ID_UNSPECIFIED: NET_IF_COMPARTMENT_ID = 0;
pub type NET_IF_COMPARTMENT_SCOPE = u32;
pub const NET_IF_COMPARTMENT_SCOPE_ALL: NET_IF_COMPARTMENT_SCOPE = 4294967295;
pub const NET_IF_COMPARTMENT_SCOPE_UNSPECIFIED: NET_IF_COMPARTMENT_SCOPE = 0;
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
#[cfg(feature = "shared")]
pub const NET_IF_LINK_SPEED_UNKNOWN: super::ULONG64 = 18446744073709551615;
pub type NET_IF_MEDIA_CONNECT_STATE = i32;
pub type NET_IF_MEDIA_DUPLEX_STATE = i32;
pub type NET_IF_NETWORK_GUID = windows_sys::core::GUID;
pub type NET_IF_OBJECT_ID = u32;
pub const NET_IF_OID_COMPARTMENT_ID: i32 = 2;
pub const NET_IF_OID_IF_ALIAS: i32 = 1;
pub const NET_IF_OID_IF_ENTRY: i32 = 4;
pub const NET_IF_OID_NETWORK_GUID: i32 = 3;
pub type NET_IF_OPER_STATUS = i32;
pub const NET_IF_OPER_STATUS_DORMANT: NET_IF_OPER_STATUS = 5;
pub const NET_IF_OPER_STATUS_DORMANT_LOW_POWER: i32 = 8;
pub const NET_IF_OPER_STATUS_DORMANT_PAUSED: i32 = 4;
pub const NET_IF_OPER_STATUS_DOWN: NET_IF_OPER_STATUS = 2;
pub const NET_IF_OPER_STATUS_DOWN_NOT_AUTHENTICATED: i32 = 1;
pub const NET_IF_OPER_STATUS_DOWN_NOT_MEDIA_CONNECTED: i32 = 2;
pub const NET_IF_OPER_STATUS_LOWER_LAYER_DOWN: NET_IF_OPER_STATUS = 7;
pub const NET_IF_OPER_STATUS_NOT_PRESENT: NET_IF_OPER_STATUS = 6;
pub const NET_IF_OPER_STATUS_TESTING: NET_IF_OPER_STATUS = 3;
pub const NET_IF_OPER_STATUS_UNKNOWN: NET_IF_OPER_STATUS = 4;
pub const NET_IF_OPER_STATUS_UP: NET_IF_OPER_STATUS = 1;
#[cfg(feature = "shared")]
pub type NET_IF_RCV_ADDRESS = NET_IF_RCV_ADDRESS_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_IF_RCV_ADDRESS_LH {
    pub ifRcvAddressType: NET_IF_RCV_ADDRESS_TYPE,
    pub ifRcvAddressLength: super::USHORT,
    pub ifRcvAddressOffset: super::USHORT,
}
pub type NET_IF_RCV_ADDRESS_TYPE = i32;
pub const NET_IF_RCV_ADDRESS_TYPE_NON_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = 3;
pub const NET_IF_RCV_ADDRESS_TYPE_OTHER: NET_IF_RCV_ADDRESS_TYPE = 1;
pub const NET_IF_RCV_ADDRESS_TYPE_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = 2;
#[cfg(feature = "shared")]
pub type NET_LUID = NET_LUID_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NET_LUID_LH {
    pub Value: super::ULONG64,
    pub Info: NET_LUID_LH_0,
}
#[cfg(feature = "shared")]
impl Default for NET_LUID_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_LUID_LH_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
pub type NET_PHYSICAL_LOCATION = NET_PHYSICAL_LOCATION_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_PHYSICAL_LOCATION_LH {
    pub BusNumber: super::ULONG,
    pub SlotNumber: super::ULONG,
    pub FunctionNumber: super::ULONG,
}
pub const NET_SITEID_MAXSYSTEM: i32 = 268435455;
pub const NET_SITEID_MAXUSER: i32 = 134217727;
pub const NET_SITEID_UNSPECIFIED: i32 = 0;
#[cfg(feature = "shared")]
pub const NIIF_BUS_NUMBER_UNKNOWN: super::ULONG = 4294967295;
pub const NIIF_FILTER_INTERFACE: i32 = 2;
#[cfg(feature = "shared")]
pub const NIIF_FUNCTION_NUMBER_UNKNOWN: super::ULONG = 4294967295;
pub const NIIF_HARDWARE_INTERFACE: i32 = 1;
pub const NIIF_NDIS_ENDPOINT_INTERFACE: i32 = 64;
pub const NIIF_NDIS_ISCSI_INTERFACE: i32 = 128;
pub const NIIF_NDIS_RESERVED1: i32 = 4;
pub const NIIF_NDIS_RESERVED2: i32 = 8;
pub const NIIF_NDIS_RESERVED3: i32 = 16;
pub const NIIF_NDIS_RESERVED4: i32 = 256;
pub const NIIF_NDIS_WDM_INTERFACE: i32 = 32;
#[cfg(feature = "shared")]
pub const NIIF_SLOT_NUMBER_UNKNOWN: super::ULONG = 4294967295;
#[cfg(feature = "shared")]
pub const NIIF_WAN_TUNNEL_TYPE_UNKNOWN: super::ULONG = 4294967295;
pub type PIF_ADMINISTRATIVE_STATE = *mut IF_ADMINISTRATIVE_STATE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIF_COUNTED_STRING = *mut IF_COUNTED_STRING_LH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIF_COUNTED_STRING_LH = *mut IF_COUNTED_STRING_LH;
pub type PIF_INDEX = *mut u32;
#[cfg(feature = "shared")]
pub type PIF_LUID = *mut NET_LUID_LH;
#[cfg(feature = "shared")]
pub type PIF_PHYSICAL_ADDRESS = *mut IF_PHYSICAL_ADDRESS_LH;
#[cfg(feature = "shared")]
pub type PIF_PHYSICAL_ADDRESS_LH = *mut IF_PHYSICAL_ADDRESS_LH;
#[cfg(feature = "shared")]
pub type PNDIS_INTERFACE_INFORMATION = *mut NDIS_INTERFACE_INFORMATION;
pub type PNET_IFINDEX = *mut u32;
pub type PNET_IFTYPE = *mut u16;
pub type PNET_IF_ACCESS_TYPE = *mut NET_IF_ACCESS_TYPE;
pub type PNET_IF_ADMIN_STATUS = *mut NET_IF_ADMIN_STATUS;
#[cfg(feature = "shared")]
pub type PNET_IF_ALIAS = *mut NET_IF_ALIAS_LH;
#[cfg(feature = "shared")]
pub type PNET_IF_ALIAS_LH = *mut NET_IF_ALIAS_LH;
pub type PNET_IF_COMPARTMENT_ID = *mut u32;
pub type PNET_IF_COMPARTMENT_SCOPE = *mut u32;
pub type PNET_IF_CONNECTION_TYPE = *mut NET_IF_CONNECTION_TYPE;
pub type PNET_IF_DIRECTION_TYPE = *mut NET_IF_DIRECTION_TYPE;
pub type PNET_IF_MEDIA_CONNECT_STATE = *mut NET_IF_MEDIA_CONNECT_STATE;
pub type PNET_IF_MEDIA_DUPLEX_STATE = *mut NET_IF_MEDIA_DUPLEX_STATE;
pub type PNET_IF_NETWORK_GUID = *mut windows_sys::core::GUID;
pub type PNET_IF_OBJECT_ID = *mut u32;
pub type PNET_IF_OPER_STATUS = *mut NET_IF_OPER_STATUS;
#[cfg(feature = "shared")]
pub type PNET_IF_RCV_ADDRESS = *mut NET_IF_RCV_ADDRESS_LH;
#[cfg(feature = "shared")]
pub type PNET_IF_RCV_ADDRESS_LH = *mut NET_IF_RCV_ADDRESS_LH;
pub type PNET_IF_RCV_ADDRESS_TYPE = *mut NET_IF_RCV_ADDRESS_TYPE;
#[cfg(feature = "shared")]
pub type PNET_LUID = *mut NET_LUID_LH;
#[cfg(feature = "shared")]
pub type PNET_LUID_LH = *mut NET_LUID_LH;
#[cfg(feature = "shared")]
pub type PNET_PHYSICAL_LOCATION = *mut NET_PHYSICAL_LOCATION_LH;
#[cfg(feature = "shared")]
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
