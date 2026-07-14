pub const IpDadStateDeprecated: NL_DAD_STATE = 3;
pub const IpDadStateDuplicate: NL_DAD_STATE = 2;
pub const IpDadStateInvalid: NL_DAD_STATE = 0;
pub const IpDadStatePreferred: NL_DAD_STATE = 4;
pub const IpDadStateTentative: NL_DAD_STATE = 1;
pub const IpPrefixOriginDhcp: NL_PREFIX_ORIGIN = 3;
pub const IpPrefixOriginManual: NL_PREFIX_ORIGIN = 1;
pub const IpPrefixOriginOther: NL_PREFIX_ORIGIN = 0;
pub const IpPrefixOriginRouterAdvertisement: NL_PREFIX_ORIGIN = 4;
pub const IpPrefixOriginUnchanged: NL_PREFIX_ORIGIN = 16;
pub const IpPrefixOriginWellKnown: NL_PREFIX_ORIGIN = 2;
pub const IpSuffixOriginDhcp: NL_SUFFIX_ORIGIN = 3;
pub const IpSuffixOriginLinkLayerAddress: NL_SUFFIX_ORIGIN = 4;
pub const IpSuffixOriginManual: NL_SUFFIX_ORIGIN = 1;
pub const IpSuffixOriginOther: NL_SUFFIX_ORIGIN = 0;
pub const IpSuffixOriginRandom: NL_SUFFIX_ORIGIN = 5;
pub const IpSuffixOriginUnchanged: NL_SUFFIX_ORIGIN = 16;
pub const IpSuffixOriginWellKnown: NL_SUFFIX_ORIGIN = 2;
pub const LinkLocalAlwaysOff: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = 0;
pub const LinkLocalAlwaysOn: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = 2;
pub const LinkLocalDelayed: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = 1;
pub const LinkLocalUnchanged: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = -1;
pub const MIB_IPPROTO_BBN: NL_ROUTE_PROTOCOL = 12;
pub const MIB_IPPROTO_BGP: NL_ROUTE_PROTOCOL = 14;
pub const MIB_IPPROTO_CISCO: NL_ROUTE_PROTOCOL = 11;
pub const MIB_IPPROTO_DHCP: NL_ROUTE_PROTOCOL = 19;
pub const MIB_IPPROTO_DVMRP: NL_ROUTE_PROTOCOL = 17;
pub const MIB_IPPROTO_EGP: NL_ROUTE_PROTOCOL = 5;
pub const MIB_IPPROTO_EIGRP: NL_ROUTE_PROTOCOL = 16;
pub const MIB_IPPROTO_ES_IS: NL_ROUTE_PROTOCOL = 10;
pub const MIB_IPPROTO_GGP: NL_ROUTE_PROTOCOL = 6;
pub const MIB_IPPROTO_HELLO: NL_ROUTE_PROTOCOL = 7;
pub const MIB_IPPROTO_ICMP: NL_ROUTE_PROTOCOL = 4;
pub const MIB_IPPROTO_IDPR: NL_ROUTE_PROTOCOL = 15;
pub const MIB_IPPROTO_IS_IS: NL_ROUTE_PROTOCOL = 9;
pub const MIB_IPPROTO_LOCAL: NL_ROUTE_PROTOCOL = 2;
pub const MIB_IPPROTO_NETMGMT: NL_ROUTE_PROTOCOL = 3;
pub const MIB_IPPROTO_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = 10002;
pub const MIB_IPPROTO_NT_STATIC: NL_ROUTE_PROTOCOL = 10006;
pub const MIB_IPPROTO_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = 10007;
pub const MIB_IPPROTO_OSPF: NL_ROUTE_PROTOCOL = 13;
pub const MIB_IPPROTO_OTHER: NL_ROUTE_PROTOCOL = 1;
pub const MIB_IPPROTO_RIP: NL_ROUTE_PROTOCOL = 8;
pub const MIB_IPPROTO_RPL: NL_ROUTE_PROTOCOL = 18;
pub const NET_IF_CURRENT_SESSION: u32 = 4294967295;
pub type NL_ADDRESS_TYPE = i32;
pub type NL_BANDWIDTH_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NL_BANDWIDTH_INFORMATION {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: bool,
}
pub type NL_DAD_STATE = i32;
pub type NL_INTERFACE_NETWORK_CATEGORY_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NL_INTERFACE_OFFLOAD_ROD {
    pub _bitfield: bool,
}
pub type NL_LINK_LOCAL_ADDRESS_BEHAVIOR = i32;
pub const NL_MAX_METRIC_COMPONENT: u32 = 2147483647;
pub type NL_NEIGHBOR_STATE = i32;
pub type NL_NETWORK_CATEGORY = i32;
pub type NL_NETWORK_CONNECTIVITY_COST_HINT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NL_NETWORK_CONNECTIVITY_HINT {
    pub ConnectivityLevel: NL_NETWORK_CONNECTIVITY_LEVEL_HINT,
    pub ConnectivityCost: NL_NETWORK_CONNECTIVITY_COST_HINT,
    pub ApproachingDataLimit: bool,
    pub OverDataLimit: bool,
    pub Roaming: bool,
}
pub type NL_NETWORK_CONNECTIVITY_LEVEL_HINT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NL_PATH_BANDWIDTH_ROD {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: bool,
}
pub type NL_PREFIX_ORIGIN = i32;
pub type NL_ROUTER_DISCOVERY_BEHAVIOR = i32;
pub type NL_ROUTE_ORIGIN = i32;
pub type NL_ROUTE_PROTOCOL = i32;
pub type NL_SUFFIX_ORIGIN = i32;
pub const NetworkCategoryDomainAuthenticated: NL_NETWORK_CATEGORY = 2;
pub const NetworkCategoryPrivate: NL_NETWORK_CATEGORY = 1;
pub const NetworkCategoryPublic: NL_NETWORK_CATEGORY = 0;
pub const NetworkCategoryUnchanged: NL_NETWORK_CATEGORY = -1;
pub const NetworkCategoryUnknown: NL_NETWORK_CATEGORY = -1;
pub const NetworkConnectivityCostHintFixed: NL_NETWORK_CONNECTIVITY_COST_HINT = 2;
pub const NetworkConnectivityCostHintUnknown: NL_NETWORK_CONNECTIVITY_COST_HINT = 0;
pub const NetworkConnectivityCostHintUnrestricted: NL_NETWORK_CONNECTIVITY_COST_HINT = 1;
pub const NetworkConnectivityCostHintVariable: NL_NETWORK_CONNECTIVITY_COST_HINT = 3;
pub const NetworkConnectivityLevelHintConstrainedInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 4;
pub const NetworkConnectivityLevelHintHidden: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 5;
pub const NetworkConnectivityLevelHintInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 3;
pub const NetworkConnectivityLevelHintLocalAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 2;
pub const NetworkConnectivityLevelHintNone: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 1;
pub const NetworkConnectivityLevelHintUnknown: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 0;
pub const NlatAnycast: NL_ADDRESS_TYPE = 2;
pub const NlatBroadcast: NL_ADDRESS_TYPE = 4;
pub const NlatInvalid: NL_ADDRESS_TYPE = 5;
pub const NlatMulticast: NL_ADDRESS_TYPE = 3;
pub const NlatUnicast: NL_ADDRESS_TYPE = 1;
pub const NlatUnspecified: NL_ADDRESS_TYPE = 0;
pub const NlbwDisabled: NL_BANDWIDTH_FLAG = 0;
pub const NlbwEnabled: NL_BANDWIDTH_FLAG = 1;
pub const NlbwUnchanged: NL_BANDWIDTH_FLAG = -1;
pub const NldsDeprecated: NL_DAD_STATE = 3;
pub const NldsDuplicate: NL_DAD_STATE = 2;
pub const NldsInvalid: NL_DAD_STATE = 0;
pub const NldsPreferred: NL_DAD_STATE = 4;
pub const NldsTentative: NL_DAD_STATE = 1;
pub const NlincCategoryStateMax: NL_INTERFACE_NETWORK_CATEGORY_STATE = 4;
pub const NlincCategoryUnknown: NL_INTERFACE_NETWORK_CATEGORY_STATE = 0;
pub const NlincDomainAuthenticated: NL_INTERFACE_NETWORK_CATEGORY_STATE = 3;
pub const NlincPrivate: NL_INTERFACE_NETWORK_CATEGORY_STATE = 2;
pub const NlincPublic: NL_INTERFACE_NETWORK_CATEGORY_STATE = 1;
pub const NlnsDelay: NL_NEIGHBOR_STATE = 3;
pub const NlnsIncomplete: NL_NEIGHBOR_STATE = 1;
pub const NlnsMaximum: NL_NEIGHBOR_STATE = 7;
pub const NlnsPermanent: NL_NEIGHBOR_STATE = 6;
pub const NlnsProbe: NL_NEIGHBOR_STATE = 2;
pub const NlnsReachable: NL_NEIGHBOR_STATE = 5;
pub const NlnsStale: NL_NEIGHBOR_STATE = 4;
pub const NlnsUnreachable: NL_NEIGHBOR_STATE = 0;
pub const NlpoDhcp: u32 = 3;
pub const NlpoManual: u32 = 1;
pub const NlpoOther: u32 = 0;
pub const NlpoRouterAdvertisement: u32 = 4;
pub const NlpoWellKnown: u32 = 2;
pub const Nlro6to4: NL_ROUTE_ORIGIN = 4;
pub const NlroDHCP: NL_ROUTE_ORIGIN = 2;
pub const NlroManual: NL_ROUTE_ORIGIN = 0;
pub const NlroRouterAdvertisement: NL_ROUTE_ORIGIN = 3;
pub const NlroWellKnown: NL_ROUTE_ORIGIN = 1;
pub const NlsoDhcp: NL_SUFFIX_ORIGIN = 3;
pub const NlsoLinkLayerAddress: NL_SUFFIX_ORIGIN = 4;
pub const NlsoManual: NL_SUFFIX_ORIGIN = 1;
pub const NlsoOther: NL_SUFFIX_ORIGIN = 0;
pub const NlsoRandom: NL_SUFFIX_ORIGIN = 5;
pub const NlsoWellKnown: NL_SUFFIX_ORIGIN = 2;
pub type PNL_ADDRESS_TYPE = *mut NL_ADDRESS_TYPE;
pub type PNL_BANDWIDTH_FLAG = *mut NL_BANDWIDTH_FLAG;
pub type PNL_BANDWIDTH_INFORMATION = *mut NL_BANDWIDTH_INFORMATION;
pub type PNL_INTERFACE_NETWORK_CATEGORY_STATE = *mut NL_INTERFACE_NETWORK_CATEGORY_STATE;
pub type PNL_INTERFACE_OFFLOAD_ROD = *mut NL_INTERFACE_OFFLOAD_ROD;
pub type PNL_NEIGHBOR_STATE = *mut NL_NEIGHBOR_STATE;
pub type PNL_NETWORK_CATEGORY = *mut NL_NETWORK_CATEGORY;
pub type PNL_PATH_BANDWIDTH_ROD = *mut NL_PATH_BANDWIDTH_ROD;
pub type PNL_ROUTE_ORIGIN = *mut NL_ROUTE_ORIGIN;
pub type PNL_ROUTE_PROTOCOL = *mut NL_ROUTE_PROTOCOL;
pub const PROTO_IP_BBN: NL_ROUTE_PROTOCOL = 12;
pub const PROTO_IP_BGP: NL_ROUTE_PROTOCOL = 14;
pub const PROTO_IP_CISCO: NL_ROUTE_PROTOCOL = 11;
pub const PROTO_IP_DHCP: NL_ROUTE_PROTOCOL = 19;
pub const PROTO_IP_DVMRP: NL_ROUTE_PROTOCOL = 17;
pub const PROTO_IP_EGP: NL_ROUTE_PROTOCOL = 5;
pub const PROTO_IP_EIGRP: NL_ROUTE_PROTOCOL = 16;
pub const PROTO_IP_ES_IS: NL_ROUTE_PROTOCOL = 10;
pub const PROTO_IP_GGP: NL_ROUTE_PROTOCOL = 6;
pub const PROTO_IP_HELLO: NL_ROUTE_PROTOCOL = 7;
pub const PROTO_IP_ICMP: NL_ROUTE_PROTOCOL = 4;
pub const PROTO_IP_IDPR: NL_ROUTE_PROTOCOL = 15;
pub const PROTO_IP_IS_IS: NL_ROUTE_PROTOCOL = 9;
pub const PROTO_IP_LOCAL: NL_ROUTE_PROTOCOL = 2;
pub const PROTO_IP_NETMGMT: NL_ROUTE_PROTOCOL = 3;
pub const PROTO_IP_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = 10002;
pub const PROTO_IP_NT_STATIC: NL_ROUTE_PROTOCOL = 10006;
pub const PROTO_IP_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = 10007;
pub const PROTO_IP_OSPF: NL_ROUTE_PROTOCOL = 13;
pub const PROTO_IP_OTHER: NL_ROUTE_PROTOCOL = 1;
pub const PROTO_IP_RIP: NL_ROUTE_PROTOCOL = 8;
pub const PROTO_IP_RPL: NL_ROUTE_PROTOCOL = 18;
pub const RouteProtocolBbn: NL_ROUTE_PROTOCOL = 12;
pub const RouteProtocolBgp: NL_ROUTE_PROTOCOL = 14;
pub const RouteProtocolCisco: NL_ROUTE_PROTOCOL = 11;
pub const RouteProtocolDhcp: NL_ROUTE_PROTOCOL = 19;
pub const RouteProtocolDvmrp: NL_ROUTE_PROTOCOL = 17;
pub const RouteProtocolEgp: NL_ROUTE_PROTOCOL = 5;
pub const RouteProtocolEigrp: NL_ROUTE_PROTOCOL = 16;
pub const RouteProtocolEsIs: NL_ROUTE_PROTOCOL = 10;
pub const RouteProtocolGgp: NL_ROUTE_PROTOCOL = 6;
pub const RouteProtocolHello: NL_ROUTE_PROTOCOL = 7;
pub const RouteProtocolIcmp: NL_ROUTE_PROTOCOL = 4;
pub const RouteProtocolIdpr: NL_ROUTE_PROTOCOL = 15;
pub const RouteProtocolIsIs: NL_ROUTE_PROTOCOL = 9;
pub const RouteProtocolLocal: NL_ROUTE_PROTOCOL = 2;
pub const RouteProtocolNetMgmt: NL_ROUTE_PROTOCOL = 3;
pub const RouteProtocolOspf: NL_ROUTE_PROTOCOL = 13;
pub const RouteProtocolOther: NL_ROUTE_PROTOCOL = 1;
pub const RouteProtocolRip: NL_ROUTE_PROTOCOL = 8;
pub const RouteProtocolRpl: NL_ROUTE_PROTOCOL = 18;
pub const RouterDiscoveryDhcp: NL_ROUTER_DISCOVERY_BEHAVIOR = 2;
pub const RouterDiscoveryDisabled: NL_ROUTER_DISCOVERY_BEHAVIOR = 0;
pub const RouterDiscoveryEnabled: NL_ROUTER_DISCOVERY_BEHAVIOR = 1;
pub const RouterDiscoveryUnchanged: NL_ROUTER_DISCOVERY_BEHAVIOR = -1;
