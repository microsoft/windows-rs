impl ::core::default::Default for CLCTL_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLCTL_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLCTL_CODES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLRES_CALLBACK_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for CLRES_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for CLRES_V1_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for CLRES_V2_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for CLRES_V3_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for CLRES_V4_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUADMEX_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUADMEX_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUADMEX_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_AFFINITYRULE_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_AFFINITYRULE_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_AFFINITYRULE_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_CLUSTER_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_CLUSTER_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_CLUSTER_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_GROUPSET_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_GROUPSET_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_GROUPSET_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_GROUP_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_GROUP_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_GROUP_CODES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.GetTickCount64 == other.GetTickCount64 && self.GetSystemTime == other.GetSystemTime && self.NodeId == other.NodeId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT").field("GetTickCount64", &self.GetTickCount64).field("GetSystemTime", &self.GetSystemTime).field("NodeId", &self.NodeId).finish()
    }
}
impl ::core::default::Default for CLUSCTL_NETINTERFACE_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_NETINTERFACE_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_NETINTERFACE_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_NETWORK_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_NETWORK_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_NETWORK_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_NODE_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_NODE_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_NODE_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_RESOURCE_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_RESOURCE_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_RESOURCE_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwVersion == other.dwVersion && self.eReason == other.eReason
    }
}
impl ::core::cmp::Eq for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {}
impl ::core::fmt::Debug for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT").field("dwSize", &self.dwSize).field("dwVersion", &self.dwVersion).field("eReason", &self.eReason).finish()
    }
}
impl ::core::default::Default for CLUSCTL_RESOURCE_TYPE_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSCTL_RESOURCE_TYPE_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSCTL_RESOURCE_TYPE_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.guidPoolFilter == other.guidPoolFilter
    }
}
impl ::core::cmp::Eq for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {}
impl ::core::fmt::Debug for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT").field("dwFlags", &self.dwFlags).field("guidPoolFilter", &self.guidPoolFilter).finish()
    }
}
impl ::core::default::Default for CLUSGROUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSGROUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSGROUP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSPROP_BINARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CLUSPROP_BUFFER_HELPER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_DWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUSPROP_FILETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_FTSET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_IPADDR_ENABLENETBIOS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSPROP_IPADDR_ENABLENETBIOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSPROP_IPADDR_ENABLENETBIOS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSPROP_LARGE_INTEGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_PARTITION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_PIFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSPROP_PIFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSPROP_PIFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSPROP_REQUIRED_DEPENDENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_RESOURCE_CLASS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for CLUSPROP_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_SYNTAX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_SZ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_ULARGE_INTEGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSPROP_WORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSTERSET_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTERSET_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTERSET_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTERVERSIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTERVERSIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersionInfoSize == other.dwVersionInfoSize && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.BuildNumber == other.BuildNumber && self.szVendorId == other.szVendorId && self.szCSDVersion == other.szCSDVersion && self.dwClusterHighestVersion == other.dwClusterHighestVersion && self.dwClusterLowestVersion == other.dwClusterLowestVersion && self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for CLUSTERVERSIONINFO {}
impl ::core::fmt::Debug for CLUSTERVERSIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTERVERSIONINFO")
            .field("dwVersionInfoSize", &self.dwVersionInfoSize)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("BuildNumber", &self.BuildNumber)
            .field("szVendorId", &self.szVendorId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("dwClusterHighestVersion", &self.dwClusterHighestVersion)
            .field("dwClusterLowestVersion", &self.dwClusterLowestVersion)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::default::Default for CLUSTERVERSIONINFO_NT4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTERVERSIONINFO_NT4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersionInfoSize == other.dwVersionInfoSize && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.BuildNumber == other.BuildNumber && self.szVendorId == other.szVendorId && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::core::cmp::Eq for CLUSTERVERSIONINFO_NT4 {}
impl ::core::fmt::Debug for CLUSTERVERSIONINFO_NT4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTERVERSIONINFO_NT4").field("dwVersionInfoSize", &self.dwVersionInfoSize).field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("BuildNumber", &self.BuildNumber).field("szVendorId", &self.szVendorId).field("szCSDVersion", &self.szCSDVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUSTER_AVAILABILITY_SET_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUSTER_AVAILABILITY_SET_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwUpdateDomains == other.dwUpdateDomains && self.dwFaultDomains == other.dwFaultDomains && self.bReserveSpareNode == other.bReserveSpareNode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUSTER_AVAILABILITY_SET_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUSTER_AVAILABILITY_SET_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_AVAILABILITY_SET_CONFIG").field("dwVersion", &self.dwVersion).field("dwUpdateDomains", &self.dwUpdateDomains).field("dwFaultDomains", &self.dwFaultDomains).field("bReserveSpareNode", &self.bReserveSpareNode).finish()
    }
}
impl ::core::default::Default for CLUSTER_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_BATCH_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command && self.dwOptions == other.dwOptions && self.wzName == other.wzName && self.lpData == other.lpData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for CLUSTER_BATCH_COMMAND {}
impl ::core::fmt::Debug for CLUSTER_BATCH_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_BATCH_COMMAND").field("Command", &self.Command).field("dwOptions", &self.dwOptions).field("wzName", &self.wzName).field("lpData", &self.lpData).field("cbData", &self.cbData).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_CLUSTER_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_CLUSTER_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_CLUSTER_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_GROUPSET_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_GROUPSET_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_GROUPSET_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_GROUP_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_GROUP_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_GROUP_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_NETINTERFACE_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_NETINTERFACE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NETINTERFACE_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_NETWORK_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_NETWORK_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NETWORK_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_NODE_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_NODE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NODE_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_QUORUM_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_QUORUM_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_QUORUM_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_REGISTRY_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_REGISTRY_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_REGISTRY_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_RESOURCE_TYPE_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_RESOURCE_TYPE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_RESOURCE_TYPE_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_RESOURCE_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_RESOURCE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_RESOURCE_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_SHARED_VOLUME_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_SHARED_VOLUME_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_SHARED_VOLUME_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CHANGE_SPACEPORT_V2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CHANGE_SPACEPORT_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_SPACEPORT_V2").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CLOUD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CLOUD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CLOUD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CONTROL_OBJECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CONTROL_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CONTROL_OBJECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_CREATE_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_CREATE_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.groupType == other.groupType
    }
}
impl ::core::cmp::Eq for CLUSTER_CREATE_GROUP_INFO {}
impl ::core::fmt::Debug for CLUSTER_CREATE_GROUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_CREATE_GROUP_INFO").field("dwVersion", &self.dwVersion).field("groupType", &self.groupType).finish()
    }
}
impl ::core::default::Default for CLUSTER_CSV_VOLUME_FAULT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_CSV_VOLUME_FAULT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_CSV_VOLUME_FAULT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_ENUM_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_ENUM_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwType == other.dwType && self.cbId == other.cbId && self.lpszId == other.lpszId && self.cbName == other.cbName && self.lpszName == other.lpszName
    }
}
impl ::core::cmp::Eq for CLUSTER_ENUM_ITEM {}
impl ::core::fmt::Debug for CLUSTER_ENUM_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_ENUM_ITEM").field("dwVersion", &self.dwVersion).field("dwType", &self.dwType).field("cbId", &self.cbId).field("lpszId", &self.lpszId).field("cbName", &self.cbName).field("lpszName", &self.lpszName).finish()
    }
}
impl ::core::default::Default for CLUSTER_GROUP_AUTOFAILBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_GROUP_AUTOFAILBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_AUTOFAILBACK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_GROUP_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_GROUP_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_GROUP_ENUM_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_GROUP_ENUM_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.lpszId == other.lpszId && self.cbName == other.cbName && self.lpszName == other.lpszName && self.state == other.state && self.cbOwnerNode == other.cbOwnerNode && self.lpszOwnerNode == other.lpszOwnerNode && self.dwFlags == other.dwFlags && self.cbProperties == other.cbProperties && self.pProperties == other.pProperties && self.cbRoProperties == other.cbRoProperties && self.pRoProperties == other.pRoProperties
    }
}
impl ::core::cmp::Eq for CLUSTER_GROUP_ENUM_ITEM {}
impl ::core::fmt::Debug for CLUSTER_GROUP_ENUM_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_GROUP_ENUM_ITEM")
            .field("dwVersion", &self.dwVersion)
            .field("cbId", &self.cbId)
            .field("lpszId", &self.lpszId)
            .field("cbName", &self.cbName)
            .field("lpszName", &self.lpszName)
            .field("state", &self.state)
            .field("cbOwnerNode", &self.cbOwnerNode)
            .field("lpszOwnerNode", &self.lpszOwnerNode)
            .field("dwFlags", &self.dwFlags)
            .field("cbProperties", &self.cbProperties)
            .field("pProperties", &self.pProperties)
            .field("cbRoProperties", &self.cbRoProperties)
            .field("pRoProperties", &self.pRoProperties)
            .finish()
    }
}
impl ::core::default::Default for CLUSTER_GROUP_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_GROUP_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_GROUP_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_GROUP_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_HEALTH_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_HEALTH_FAULT {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.ErrorType == other.ErrorType && self.ErrorCode == other.ErrorCode && self.Description == other.Description && self.Provider == other.Provider && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for CLUSTER_HEALTH_FAULT {}
impl ::core::fmt::Debug for CLUSTER_HEALTH_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_HEALTH_FAULT").field("Id", &self.Id).field("ErrorType", &self.ErrorType).field("ErrorCode", &self.ErrorCode).field("Description", &self.Description).field("Provider", &self.Provider).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for CLUSTER_HEALTH_FAULT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_HEALTH_FAULT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.numFaults == other.numFaults && self.faults == other.faults
    }
}
impl ::core::cmp::Eq for CLUSTER_HEALTH_FAULT_ARRAY {}
impl ::core::fmt::Debug for CLUSTER_HEALTH_FAULT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_HEALTH_FAULT_ARRAY").field("numFaults", &self.numFaults).field("faults", &self.faults).finish()
    }
}
impl ::core::default::Default for CLUSTER_IP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_IP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.lpszIpAddress == other.lpszIpAddress && self.dwPrefixLength == other.dwPrefixLength
    }
}
impl ::core::cmp::Eq for CLUSTER_IP_ENTRY {}
impl ::core::fmt::Debug for CLUSTER_IP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_IP_ENTRY").field("lpszIpAddress", &self.lpszIpAddress).field("dwPrefixLength", &self.dwPrefixLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUSTER_MEMBERSHIP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUSTER_MEMBERSHIP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.HasQuorum == other.HasQuorum && self.UpnodesSize == other.UpnodesSize && self.Upnodes == other.Upnodes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUSTER_MEMBERSHIP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUSTER_MEMBERSHIP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_MEMBERSHIP_INFO").field("HasQuorum", &self.HasQuorum).field("UpnodesSize", &self.UpnodesSize).field("Upnodes", &self.Upnodes).finish()
    }
}
impl ::core::default::Default for CLUSTER_MGMT_POINT_RESTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_MGMT_POINT_RESTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_MGMT_POINT_RESTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_MGMT_POINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_MGMT_POINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_MGMT_POINT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NETINTERFACE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NETINTERFACE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NETINTERFACE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NETWORK_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NETWORK_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NETWORK_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NETWORK_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NETWORK_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NETWORK_ROLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NETWORK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NETWORK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NETWORK_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NODE_DRAIN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NODE_DRAIN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_DRAIN_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NODE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NODE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NODE_RESUME_FAILBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NODE_RESUME_FAILBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_RESUME_FAILBACK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NODE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NODE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NODE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NODE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_NOTIFICATIONS_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_NOTIFICATIONS_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_NOTIFICATIONS_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_PROPERTY_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_PROPERTY_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_PROPERTY_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_PROPERTY_SYNTAX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_PROPERTY_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_PROPERTY_SYNTAX").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_PROPERTY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_QUORUM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_QUORUM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_QUORUM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_QUORUM_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_QUORUM_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_QUORUM_VALUE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_READ_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_READ_BATCH_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command && self.dwOptions == other.dwOptions && self.wzSubkeyName == other.wzSubkeyName && self.wzValueName == other.wzValueName && self.lpData == other.lpData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for CLUSTER_READ_BATCH_COMMAND {}
impl ::core::fmt::Debug for CLUSTER_READ_BATCH_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_READ_BATCH_COMMAND").field("Command", &self.Command).field("dwOptions", &self.dwOptions).field("wzSubkeyName", &self.wzSubkeyName).field("wzValueName", &self.wzValueName).field("lpData", &self.lpData).field("cbData", &self.cbData).finish()
    }
}
impl ::core::default::Default for CLUSTER_REG_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_REG_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_REG_COMMAND").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_APPLICATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_APPLICATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_APPLICATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_ENUM_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_RESOURCE_ENUM_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.lpszId == other.lpszId && self.cbName == other.cbName && self.lpszName == other.lpszName && self.cbOwnerGroupName == other.cbOwnerGroupName && self.lpszOwnerGroupName == other.lpszOwnerGroupName && self.cbOwnerGroupId == other.cbOwnerGroupId && self.lpszOwnerGroupId == other.lpszOwnerGroupId && self.cbProperties == other.cbProperties && self.pProperties == other.pProperties && self.cbRoProperties == other.cbRoProperties && self.pRoProperties == other.pRoProperties
    }
}
impl ::core::cmp::Eq for CLUSTER_RESOURCE_ENUM_ITEM {}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_ENUM_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_RESOURCE_ENUM_ITEM")
            .field("dwVersion", &self.dwVersion)
            .field("cbId", &self.cbId)
            .field("lpszId", &self.lpszId)
            .field("cbName", &self.cbName)
            .field("lpszName", &self.lpszName)
            .field("cbOwnerGroupName", &self.cbOwnerGroupName)
            .field("lpszOwnerGroupName", &self.lpszOwnerGroupName)
            .field("cbOwnerGroupId", &self.cbOwnerGroupId)
            .field("lpszOwnerGroupId", &self.lpszOwnerGroupId)
            .field("cbProperties", &self.cbProperties)
            .field("pProperties", &self.pProperties)
            .field("cbRoProperties", &self.cbRoProperties)
            .field("pRoProperties", &self.pRoProperties)
            .finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_RESTART_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_RESTART_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_RESTART_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_STATE_CHANGE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_STATE_CHANGE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_STATE_CHANGE_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_RESOURCE_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_RESOURCE_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_TYPE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_ROLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_ROLE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_ROLE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_ROLE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_SETUP_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_SETUP_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_SETUP_PHASE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_SETUP_PHASE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_SETUP_PHASE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_SETUP_PHASE_SEVERITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_SETUP_PHASE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_SETUP_PHASE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_SETUP_PHASE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUSTER_SET_PASSWORD_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUSTER_SET_PASSWORD_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.NodeId == other.NodeId && self.SetAttempted == other.SetAttempted && self.ReturnStatus == other.ReturnStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUSTER_SET_PASSWORD_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUSTER_SET_PASSWORD_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_SET_PASSWORD_STATUS").field("NodeId", &self.NodeId).field("SetAttempted", &self.SetAttempted).field("ReturnStatus", &self.ReturnStatus).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_BACKUP_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_BACKUP_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_BACKUP_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.NewVolumeName == other.NewVolumeName && self.NewVolumeGuid == other.NewVolumeGuid
    }
}
impl ::core::cmp::Eq for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME").field("NewVolumeName", &self.NewVolumeName).field("NewVolumeGuid", &self.NewVolumeGuid).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.NewVolumeName == other.NewVolumeName
    }
}
impl ::core::cmp::Eq for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME").field("NewVolumeName", &self.NewVolumeName).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.szVolumeName == other.szVolumeName && self.szNodeName == other.szNodeName && self.VolumeState == other.VolumeState
    }
}
impl ::core::cmp::Eq for CLUSTER_SHARED_VOLUME_STATE_INFO {}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_SHARED_VOLUME_STATE_INFO").field("szVolumeName", &self.szVolumeName).field("szNodeName", &self.szNodeName).field("VolumeState", &self.VolumeState).finish()
    }
}
impl ::core::default::Default for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.szVolumeName == other.szVolumeName && self.szNodeName == other.szNodeName && self.VolumeState == other.VolumeState && self.szVolumeFriendlyName == other.szVolumeFriendlyName && self.RedirectedIOReason == other.RedirectedIOReason && self.VolumeRedirectedIOReason == other.VolumeRedirectedIOReason
    }
}
impl ::core::cmp::Eq for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {}
impl ::core::fmt::Debug for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_SHARED_VOLUME_STATE_INFO_EX").field("szVolumeName", &self.szVolumeName).field("szNodeName", &self.szNodeName).field("VolumeState", &self.VolumeState).field("szVolumeFriendlyName", &self.szVolumeFriendlyName).field("RedirectedIOReason", &self.RedirectedIOReason).field("VolumeRedirectedIOReason", &self.VolumeRedirectedIOReason).finish()
    }
}
impl ::core::default::Default for CLUSTER_STORAGENODE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_STORAGENODE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_STORAGENODE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_UPGRADE_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSTER_UPGRADE_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSTER_UPGRADE_PHASE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSTER_VALIDATE_CSV_FILENAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_VALIDATE_CSV_FILENAME {
    fn eq(&self, other: &Self) -> bool {
        self.szFileName == other.szFileName
    }
}
impl ::core::cmp::Eq for CLUSTER_VALIDATE_CSV_FILENAME {}
impl ::core::fmt::Debug for CLUSTER_VALIDATE_CSV_FILENAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_VALIDATE_CSV_FILENAME").field("szFileName", &self.szFileName).finish()
    }
}
impl ::core::default::Default for CLUSTER_VALIDATE_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_VALIDATE_DIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.szPath == other.szPath
    }
}
impl ::core::cmp::Eq for CLUSTER_VALIDATE_DIRECTORY {}
impl ::core::fmt::Debug for CLUSTER_VALIDATE_DIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_VALIDATE_DIRECTORY").field("szPath", &self.szPath).finish()
    }
}
impl ::core::default::Default for CLUSTER_VALIDATE_NETNAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_VALIDATE_NETNAME {
    fn eq(&self, other: &Self) -> bool {
        self.szNetworkName == other.szNetworkName
    }
}
impl ::core::cmp::Eq for CLUSTER_VALIDATE_NETNAME {}
impl ::core::fmt::Debug for CLUSTER_VALIDATE_NETNAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_VALIDATE_NETNAME").field("szNetworkName", &self.szNetworkName).finish()
    }
}
impl ::core::default::Default for CLUSTER_VALIDATE_PATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_VALIDATE_PATH {
    fn eq(&self, other: &Self) -> bool {
        self.szPath == other.szPath
    }
}
impl ::core::cmp::Eq for CLUSTER_VALIDATE_PATH {}
impl ::core::fmt::Debug for CLUSTER_VALIDATE_PATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_VALIDATE_PATH").field("szPath", &self.szPath).finish()
    }
}
impl ::core::default::Default for CLUS_AFFINITY_RULE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUS_AFFINITY_RULE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUS_AFFINITY_RULE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUS_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUS_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUS_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUS_CHKDSK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_CHKDSK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionNumber == other.PartitionNumber && self.ChkdskState == other.ChkdskState && self.FileIdCount == other.FileIdCount && self.FileIdList == other.FileIdList
    }
}
impl ::core::cmp::Eq for CLUS_CHKDSK_INFO {}
impl ::core::fmt::Debug for CLUS_CHKDSK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_CHKDSK_INFO").field("PartitionNumber", &self.PartitionNumber).field("ChkdskState", &self.ChkdskState).field("FileIdCount", &self.FileIdCount).field("FileIdList", &self.FileIdList).finish()
    }
}
impl ::core::default::Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.FileServerName == other.FileServerName
    }
}
impl ::core::cmp::Eq for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {}
impl ::core::fmt::Debug for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT").field("FileServerName", &self.FileServerName).finish()
    }
}
impl ::core::default::Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.FileServerName == other.FileServerName
    }
}
impl ::core::cmp::Eq for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {}
impl ::core::fmt::Debug for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT").field("FileServerName", &self.FileServerName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InMaintenance == other.InMaintenance && self.VolumeName == other.VolumeName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_CSV_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_CSV_MAINTENANCE_MODE_INFO").field("InMaintenance", &self.InMaintenance).field("VolumeName", &self.VolumeName).finish()
    }
}
impl ::core::default::Default for CLUS_CSV_VOLUME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_CSV_VOLUME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeOffset == other.VolumeOffset && self.PartitionNumber == other.PartitionNumber && self.FaultState == other.FaultState && self.BackupState == other.BackupState && self.szVolumeFriendlyName == other.szVolumeFriendlyName && self.szVolumeName == other.szVolumeName
    }
}
impl ::core::cmp::Eq for CLUS_CSV_VOLUME_INFO {}
impl ::core::fmt::Debug for CLUS_CSV_VOLUME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_CSV_VOLUME_INFO").field("VolumeOffset", &self.VolumeOffset).field("PartitionNumber", &self.PartitionNumber).field("FaultState", &self.FaultState).field("BackupState", &self.BackupState).field("szVolumeFriendlyName", &self.szVolumeFriendlyName).field("szVolumeName", &self.szVolumeName).finish()
    }
}
impl ::core::default::Default for CLUS_CSV_VOLUME_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_CSV_VOLUME_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeOffset == other.VolumeOffset && self.szVolumeName == other.szVolumeName && self.szRootPath == other.szRootPath
    }
}
impl ::core::cmp::Eq for CLUS_CSV_VOLUME_NAME {}
impl ::core::fmt::Debug for CLUS_CSV_VOLUME_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_CSV_VOLUME_NAME").field("VolumeOffset", &self.VolumeOffset).field("szVolumeName", &self.szVolumeName).field("szRootPath", &self.szRootPath).finish()
    }
}
impl ::core::default::Default for CLUS_DISK_NUMBER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_DISK_NUMBER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DiskNumber == other.DiskNumber && self.BytesPerSector == other.BytesPerSector
    }
}
impl ::core::cmp::Eq for CLUS_DISK_NUMBER_INFO {}
impl ::core::fmt::Debug for CLUS_DISK_NUMBER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_DISK_NUMBER_INFO").field("DiskNumber", &self.DiskNumber).field("BytesPerSector", &self.BytesPerSector).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_DNN_LEADER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_DNN_LEADER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.IsOnline == other.IsOnline && self.IsFileServerPresent == other.IsFileServerPresent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_DNN_LEADER_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_DNN_LEADER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_DNN_LEADER_STATUS").field("IsOnline", &self.IsOnline).field("IsFileServerPresent", &self.IsFileServerPresent).finish()
    }
}
impl ::core::default::Default for CLUS_DNN_SODAFS_CLONE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_DNN_SODAFS_CLONE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.NodeId == other.NodeId && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for CLUS_DNN_SODAFS_CLONE_STATUS {}
impl ::core::fmt::Debug for CLUS_DNN_SODAFS_CLONE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_DNN_SODAFS_CLONE_STATUS").field("NodeId", &self.NodeId).field("Status", &self.Status).finish()
    }
}
impl ::core::default::Default for CLUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUS_FORCE_QUORUM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_FORCE_QUORUM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwNodeBitMask == other.dwNodeBitMask && self.dwMaxNumberofNodes == other.dwMaxNumberofNodes && self.multiszNodeList == other.multiszNodeList
    }
}
impl ::core::cmp::Eq for CLUS_FORCE_QUORUM_INFO {}
impl ::core::fmt::Debug for CLUS_FORCE_QUORUM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_FORCE_QUORUM_INFO").field("dwSize", &self.dwSize).field("dwNodeBitMask", &self.dwNodeBitMask).field("dwMaxNumberofNodes", &self.dwMaxNumberofNodes).field("multiszNodeList", &self.multiszNodeList).finish()
    }
}
impl ::core::default::Default for CLUS_FTSET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_FTSET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwRootSignature == other.dwRootSignature && self.dwFtType == other.dwFtType
    }
}
impl ::core::cmp::Eq for CLUS_FTSET_INFO {}
impl ::core::fmt::Debug for CLUS_FTSET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_FTSET_INFO").field("dwRootSignature", &self.dwRootSignature).field("dwFtType", &self.dwFtType).finish()
    }
}
impl ::core::default::Default for CLUS_GROUP_START_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUS_GROUP_START_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUS_GROUP_START_SETTING").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_MAINTENANCE_MODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InMaintenance == other.InMaintenance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_MAINTENANCE_MODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_MAINTENANCE_MODE_INFO").field("InMaintenance", &self.InMaintenance).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_MAINTENANCE_MODE_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_MAINTENANCE_MODE_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.InMaintenance == other.InMaintenance && self.MaintainenceModeType == other.MaintainenceModeType && self.InternalState == other.InternalState && self.Signature == other.Signature
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_MAINTENANCE_MODE_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_MAINTENANCE_MODE_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_MAINTENANCE_MODE_INFOEX").field("InMaintenance", &self.InMaintenance).field("MaintainenceModeType", &self.MaintainenceModeType).field("InternalState", &self.InternalState).field("Signature", &self.Signature).finish()
    }
}
impl ::core::default::Default for CLUS_NETNAME_IP_INFO_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_NETNAME_IP_INFO_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.NodeId == other.NodeId && self.AddressSize == other.AddressSize && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for CLUS_NETNAME_IP_INFO_ENTRY {}
impl ::core::fmt::Debug for CLUS_NETNAME_IP_INFO_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_NETNAME_IP_INFO_ENTRY").field("NodeId", &self.NodeId).field("AddressSize", &self.AddressSize).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.NumEntries == other.NumEntries && self.IpInfo == other.IpInfo
    }
}
impl ::core::cmp::Eq for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {}
impl ::core::fmt::Debug for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL").field("szName", &self.szName).field("NumEntries", &self.NumEntries).field("IpInfo", &self.IpInfo).finish()
    }
}
impl ::core::default::Default for CLUS_NETNAME_PWD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_NETNAME_PWD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Password == other.Password && self.CreatingDC == other.CreatingDC && self.ObjectGuid == other.ObjectGuid
    }
}
impl ::core::cmp::Eq for CLUS_NETNAME_PWD_INFO {}
impl ::core::fmt::Debug for CLUS_NETNAME_PWD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_NETNAME_PWD_INFO").field("Flags", &self.Flags).field("Password", &self.Password).field("CreatingDC", &self.CreatingDC).field("ObjectGuid", &self.ObjectGuid).finish()
    }
}
impl ::core::default::Default for CLUS_NETNAME_PWD_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_NETNAME_PWD_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Password == other.Password && self.CreatingDC == other.CreatingDC && self.ObjectGuid == other.ObjectGuid
    }
}
impl ::core::cmp::Eq for CLUS_NETNAME_PWD_INFOEX {}
impl ::core::fmt::Debug for CLUS_NETNAME_PWD_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_NETNAME_PWD_INFOEX").field("Flags", &self.Flags).field("Password", &self.Password).field("CreatingDC", &self.CreatingDC).field("ObjectGuid", &self.ObjectGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_NETNAME_VS_TOKEN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_NETNAME_VS_TOKEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessID == other.ProcessID && self.DesiredAccess == other.DesiredAccess && self.InheritHandle == other.InheritHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_NETNAME_VS_TOKEN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_NETNAME_VS_TOKEN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_NETNAME_VS_TOKEN_INFO").field("ProcessID", &self.ProcessID).field("DesiredAccess", &self.DesiredAccess).field("InheritHandle", &self.InheritHandle).finish()
    }
}
impl ::core::default::Default for CLUS_PARTITION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_PARTITION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.szDeviceName == other.szDeviceName && self.szVolumeLabel == other.szVolumeLabel && self.dwSerialNumber == other.dwSerialNumber && self.rgdwMaximumComponentLength == other.rgdwMaximumComponentLength && self.dwFileSystemFlags == other.dwFileSystemFlags && self.szFileSystem == other.szFileSystem
    }
}
impl ::core::cmp::Eq for CLUS_PARTITION_INFO {}
impl ::core::fmt::Debug for CLUS_PARTITION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_PARTITION_INFO").field("dwFlags", &self.dwFlags).field("szDeviceName", &self.szDeviceName).field("szVolumeLabel", &self.szVolumeLabel).field("dwSerialNumber", &self.dwSerialNumber).field("rgdwMaximumComponentLength", &self.rgdwMaximumComponentLength).field("dwFileSystemFlags", &self.dwFileSystemFlags).field("szFileSystem", &self.szFileSystem).finish()
    }
}
impl ::core::default::Default for CLUS_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_PARTITION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.szDeviceName == other.szDeviceName && self.szVolumeLabel == other.szVolumeLabel && self.dwSerialNumber == other.dwSerialNumber && self.rgdwMaximumComponentLength == other.rgdwMaximumComponentLength && self.dwFileSystemFlags == other.dwFileSystemFlags && self.szFileSystem == other.szFileSystem && self.TotalSizeInBytes == other.TotalSizeInBytes && self.FreeSizeInBytes == other.FreeSizeInBytes && self.DeviceNumber == other.DeviceNumber && self.PartitionNumber == other.PartitionNumber && self.VolumeGuid == other.VolumeGuid
    }
}
impl ::core::cmp::Eq for CLUS_PARTITION_INFO_EX {}
impl ::core::fmt::Debug for CLUS_PARTITION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_PARTITION_INFO_EX")
            .field("dwFlags", &self.dwFlags)
            .field("szDeviceName", &self.szDeviceName)
            .field("szVolumeLabel", &self.szVolumeLabel)
            .field("dwSerialNumber", &self.dwSerialNumber)
            .field("rgdwMaximumComponentLength", &self.rgdwMaximumComponentLength)
            .field("dwFileSystemFlags", &self.dwFileSystemFlags)
            .field("szFileSystem", &self.szFileSystem)
            .field("TotalSizeInBytes", &self.TotalSizeInBytes)
            .field("FreeSizeInBytes", &self.FreeSizeInBytes)
            .field("DeviceNumber", &self.DeviceNumber)
            .field("PartitionNumber", &self.PartitionNumber)
            .field("VolumeGuid", &self.VolumeGuid)
            .finish()
    }
}
impl ::core::default::Default for CLUS_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_PARTITION_INFO_EX2 {
    fn eq(&self, other: &Self) -> bool {
        self.GptPartitionId == other.GptPartitionId && self.szPartitionName == other.szPartitionName && self.EncryptionFlags == other.EncryptionFlags
    }
}
impl ::core::cmp::Eq for CLUS_PARTITION_INFO_EX2 {}
impl ::core::fmt::Debug for CLUS_PARTITION_INFO_EX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_PARTITION_INFO_EX2").field("GptPartitionId", &self.GptPartitionId).field("szPartitionName", &self.szPartitionName).field("EncryptionFlags", &self.EncryptionFlags).finish()
    }
}
impl ::core::default::Default for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.resourceState == other.resourceState && self.szProviderId == other.szProviderId
    }
}
impl ::core::cmp::Eq for CLUS_PROVIDER_STATE_CHANGE_INFO {}
impl ::core::fmt::Debug for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_PROVIDER_STATE_CHANGE_INFO").field("dwSize", &self.dwSize).field("resourceState", &self.resourceState).field("szProviderId", &self.szProviderId).finish()
    }
}
impl ::core::default::Default for CLUS_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLUS_RESSUBCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUS_RESSUBCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUS_RESSUBCLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUS_RESSUBCLASS_NETWORK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUS_RESSUBCLASS_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUS_RESSUBCLASS_NETWORK").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUS_RESSUBCLASS_STORAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUS_RESSUBCLASS_STORAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUS_RESSUBCLASS_STORAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUS_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.InMaintenance == other.InMaintenance && self.ExtraParameterSize == other.ExtraParameterSize && self.ExtraParameter == other.ExtraParameter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_SET_MAINTENANCE_MODE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_SET_MAINTENANCE_MODE_INPUT").field("InMaintenance", &self.InMaintenance).field("ExtraParameterSize", &self.ExtraParameterSize).field("ExtraParameter", &self.ExtraParameter).finish()
    }
}
impl ::core::default::Default for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.BackupState == other.BackupState && self.DelayTimerInSecs == other.DelayTimerInSecs && self.VolumeName == other.VolumeName
    }
}
impl ::core::cmp::Eq for CLUS_SHARED_VOLUME_BACKUP_MODE {}
impl ::core::fmt::Debug for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_SHARED_VOLUME_BACKUP_MODE").field("BackupState", &self.BackupState).field("DelayTimerInSecs", &self.DelayTimerInSecs).field("VolumeName", &self.VolumeName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_STARTING_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_STARTING_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bForm == other.bForm && self.bFirst == other.bFirst
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_STARTING_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_STARTING_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_STARTING_PARAMS").field("dwSize", &self.dwSize).field("bForm", &self.bForm).field("bFirst", &self.bFirst).finish()
    }
}
impl ::core::default::Default for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn eq(&self, other: &Self) -> bool {
        self.AvailDrivelettersMask == other.AvailDrivelettersMask
    }
}
impl ::core::cmp::Eq for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {}
impl ::core::fmt::Debug for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS").field("AvailDrivelettersMask", &self.AvailDrivelettersMask).finish()
    }
}
impl ::core::default::Default for CLUS_STORAGE_REMAP_DRIVELETTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_STORAGE_REMAP_DRIVELETTER {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentDriveLetterMask == other.CurrentDriveLetterMask && self.TargetDriveLetterMask == other.TargetDriveLetterMask
    }
}
impl ::core::cmp::Eq for CLUS_STORAGE_REMAP_DRIVELETTER {}
impl ::core::fmt::Debug for CLUS_STORAGE_REMAP_DRIVELETTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_STORAGE_REMAP_DRIVELETTER").field("CurrentDriveLetterMask", &self.CurrentDriveLetterMask).field("TargetDriveLetterMask", &self.TargetDriveLetterMask).finish()
    }
}
impl ::core::default::Default for CLUS_STORAGE_SET_DRIVELETTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUS_STORAGE_SET_DRIVELETTER {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionNumber == other.PartitionNumber && self.DriveLetterMask == other.DriveLetterMask
    }
}
impl ::core::cmp::Eq for CLUS_STORAGE_SET_DRIVELETTER {}
impl ::core::fmt::Debug for CLUS_STORAGE_SET_DRIVELETTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_STORAGE_SET_DRIVELETTER").field("PartitionNumber", &self.PartitionNumber).field("DriveLetterMask", &self.DriveLetterMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLUS_WORKER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLUS_WORKER {
    fn eq(&self, other: &Self) -> bool {
        self.hThread == other.hThread && self.Terminate == other.Terminate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLUS_WORKER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLUS_WORKER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUS_WORKER").field("hThread", &self.hThread).field("Terminate", &self.Terminate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATE_CLUSTER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATE_CLUSTER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.lpszClusterName == other.lpszClusterName && self.cNodes == other.cNodes && self.ppszNodeNames == other.ppszNodeNames && self.cIpEntries == other.cIpEntries && self.pIpEntries == other.pIpEntries && self.fEmptyCluster == other.fEmptyCluster && self.managementPointType == other.managementPointType && self.managementPointResType == other.managementPointResType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATE_CLUSTER_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATE_CLUSTER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_CLUSTER_CONFIG").field("dwVersion", &self.dwVersion).field("lpszClusterName", &self.lpszClusterName).field("cNodes", &self.cNodes).field("ppszNodeNames", &self.ppszNodeNames).field("cIpEntries", &self.cIpEntries).field("pIpEntries", &self.pIpEntries).field("fEmptyCluster", &self.fEmptyCluster).field("managementPointType", &self.managementPointType).field("managementPointResType", &self.managementPointResType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATE_CLUSTER_NAME_ACCOUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATE_CLUSTER_NAME_ACCOUNT {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.lpszClusterName == other.lpszClusterName && self.dwFlags == other.dwFlags && self.pszUserName == other.pszUserName && self.pszPassword == other.pszPassword && self.pszDomain == other.pszDomain && self.managementPointType == other.managementPointType && self.managementPointResType == other.managementPointResType && self.bUpgradeVCOs == other.bUpgradeVCOs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATE_CLUSTER_NAME_ACCOUNT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATE_CLUSTER_NAME_ACCOUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_CLUSTER_NAME_ACCOUNT").field("dwVersion", &self.dwVersion).field("lpszClusterName", &self.lpszClusterName).field("dwFlags", &self.dwFlags).field("pszUserName", &self.pszUserName).field("pszPassword", &self.pszPassword).field("pszDomain", &self.pszDomain).field("managementPointType", &self.managementPointType).field("managementPointResType", &self.managementPointResType).field("bUpgradeVCOs", &self.bUpgradeVCOs).finish()
    }
}
impl ::core::default::Default for FAILURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FAILURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAILURE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILESHARE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILESHARE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Change == other.Change && self.ShareName == other.ShareName
    }
}
impl ::core::cmp::Eq for FILESHARE_CHANGE {}
impl ::core::fmt::Debug for FILESHARE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILESHARE_CHANGE").field("Change", &self.Change).field("ShareName", &self.ShareName).finish()
    }
}
impl ::core::default::Default for FILESHARE_CHANGE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILESHARE_CHANGE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILESHARE_CHANGE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILESHARE_CHANGE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILESHARE_CHANGE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumEntries == other.NumEntries && self.ChangeEntry == other.ChangeEntry
    }
}
impl ::core::cmp::Eq for FILESHARE_CHANGE_LIST {}
impl ::core::fmt::Debug for FILESHARE_CHANGE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILESHARE_CHANGE_LIST").field("NumEntries", &self.NumEntries).field("ChangeEntry", &self.ChangeEntry).finish()
    }
}
impl ::core::default::Default for GET_OPERATION_CONTEXT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_OPERATION_CONTEXT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Type == other.Type && self.Priority == other.Priority
    }
}
impl ::core::cmp::Eq for GET_OPERATION_CONTEXT_PARAMS {}
impl ::core::fmt::Debug for GET_OPERATION_CONTEXT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_OPERATION_CONTEXT_PARAMS").field("Size", &self.Size).field("Version", &self.Version).field("Type", &self.Type).field("Priority", &self.Priority).finish()
    }
}
impl ::core::default::Default for GROUP_FAILURE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_FAILURE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFailoverAttemptsRemaining == other.dwFailoverAttemptsRemaining && self.dwFailoverPeriodRemaining == other.dwFailoverPeriodRemaining
    }
}
impl ::core::cmp::Eq for GROUP_FAILURE_INFO {}
impl ::core::fmt::Debug for GROUP_FAILURE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_FAILURE_INFO").field("dwFailoverAttemptsRemaining", &self.dwFailoverAttemptsRemaining).field("dwFailoverPeriodRemaining", &self.dwFailoverPeriodRemaining).finish()
    }
}
impl ::core::default::Default for GROUP_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_FAILURE_INFO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for GROUP_FAILURE_INFO_BUFFER {}
impl ::core::fmt::Debug for GROUP_FAILURE_INFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_FAILURE_INFO_BUFFER").field("dwVersion", &self.dwVersion).field("Info", &self.Info).finish()
    }
}
impl ::core::default::Default for GRP_PLACEMENT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRP_PLACEMENT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRP_PLACEMENT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterDataInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterDataInfo {}
impl ::core::fmt::Debug for IGetClusterDataInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterDataInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterGroupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterGroupInfo {}
impl ::core::fmt::Debug for IGetClusterGroupInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterGroupInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterNetInterfaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterNetInterfaceInfo {}
impl ::core::fmt::Debug for IGetClusterNetInterfaceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterNetInterfaceInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterNetworkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterNetworkInfo {}
impl ::core::fmt::Debug for IGetClusterNetworkInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterNetworkInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterNodeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterNodeInfo {}
impl ::core::fmt::Debug for IGetClusterNodeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterNodeInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterObjectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterObjectInfo {}
impl ::core::fmt::Debug for IGetClusterObjectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterObjectInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterResourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterResourceInfo {}
impl ::core::fmt::Debug for IGetClusterResourceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterResourceInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetClusterUIInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetClusterUIInfo {}
impl ::core::fmt::Debug for IGetClusterUIInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetClusterUIInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusCryptoKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusCryptoKeys {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusCryptoKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusCryptoKeys").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusDisk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusDisk {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusDisk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusDisk").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusDisks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusDisks {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusDisks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusDisks").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNetInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNetInterface {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNetInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNetInterface").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNetInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNetInterfaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNetInterfaces").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNetwork").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNetworkNetInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNetworkNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNetworkNetInterfaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNetworkNetInterfaces").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNetworks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNetworks {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNetworks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNetworks").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNode {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNode").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNodeNetInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNodeNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNodeNetInterfaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNodeNetInterfaces").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusNodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusNodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusNodes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusPartition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusPartition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusPartition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusPartition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusPartitionEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusPartitionEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusPartitionEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusPartitionEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionEx {
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Flags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DeviceName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VolumeLabel(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VolumeLabel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SerialNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SerialNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MaximumComponentLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MaximumComponentLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FileSystemFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FileSystemFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FileSystem(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FileSystem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusPartitions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusPartitions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusPartitions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusPartitions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusPropertyValue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusPropertyValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusPropertyValue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusPropertyValueData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusPropertyValueData {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusPropertyValueData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusPropertyValueData").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusPropertyValues {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusPropertyValues {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusPropertyValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusPropertyValues").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusRefObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusRefObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusRefObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusRefObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusRegistryKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusRegistryKeys {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusRegistryKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusRegistryKeys").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResDependencies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResDependencies {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResDependencies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResDependencies").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResDependents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResDependents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResDependents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResDependents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResGroupPreferredOwnerNodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResGroupPreferredOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResGroupPreferredOwnerNodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResGroupPreferredOwnerNodes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResGroupResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResGroupResources {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResGroupResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResGroupResources").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResGroups {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResGroups").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResPossibleOwnerNodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResPossibleOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResPossibleOwnerNodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResPossibleOwnerNodes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResType {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResTypePossibleOwnerNodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResTypePossibleOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResTypePossibleOwnerNodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResTypePossibleOwnerNodes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResTypeResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResTypeResources {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResTypeResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResTypeResources").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResTypes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResTypes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResTypes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResource {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResource").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusResources {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusResources").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusScsiAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusScsiAddress {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusScsiAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusScsiAddress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusVersion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusVersion {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusVersion").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISCluster {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISCluster {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISCluster {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCluster").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISClusterNames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISClusterNames {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISClusterNames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISClusterNames").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISDomainNames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISDomainNames {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISDomainNames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISDomainNames").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWCContextMenuCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWCContextMenuCallback {}
impl ::core::fmt::Debug for IWCContextMenuCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWCContextMenuCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWCPropertySheetCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWCPropertySheetCallback {}
impl ::core::fmt::Debug for IWCPropertySheetCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWCPropertySheetCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWCWizard97Callback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWCWizard97Callback {}
impl ::core::fmt::Debug for IWCWizard97Callback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWCWizard97Callback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWCWizardCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWCWizardCallback {}
impl ::core::fmt::Debug for IWCWizardCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWCWizardCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWEExtendContextMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWEExtendContextMenu {}
impl ::core::fmt::Debug for IWEExtendContextMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWEExtendContextMenu").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWEExtendPropertySheet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWEExtendPropertySheet {}
impl ::core::fmt::Debug for IWEExtendPropertySheet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWEExtendPropertySheet").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWEExtendWizard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWEExtendWizard {}
impl ::core::fmt::Debug for IWEExtendWizard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWEExtendWizard").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWEExtendWizard97 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWEExtendWizard97 {}
impl ::core::fmt::Debug for IWEExtendWizard97 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWEExtendWizard97").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWEInvokeCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWEInvokeCommand {}
impl ::core::fmt::Debug for IWEInvokeCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWEInvokeCommand").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOG_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOG_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOG_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MAINTENANCE_MODE_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MAINTENANCE_MODE_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAINTENANCE_MODE_TYPE_ENUM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONITOR_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONITOR_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.LastUpdate == other.LastUpdate && self.State == other.State && self.ActiveResource == other.ActiveResource && self.ResmonStop == other.ResmonStop
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONITOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONITOR_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITOR_STATE").field("LastUpdate", &self.LastUpdate).field("State", &self.State).field("ActiveResource", &self.ActiveResource).field("ResmonStop", &self.ResmonStop).finish()
    }
}
impl ::core::default::Default for NODE_CLUSTER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NODE_CLUSTER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NODE_CLUSTER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NOTIFY_FILTER_AND_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NOTIFY_FILTER_AND_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.dwObjectType == other.dwObjectType && self.FilterFlags == other.FilterFlags
    }
}
impl ::core::cmp::Eq for NOTIFY_FILTER_AND_TYPE {}
impl ::core::fmt::Debug for NOTIFY_FILTER_AND_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFY_FILTER_AND_TYPE").field("dwObjectType", &self.dwObjectType).field("FilterFlags", &self.FilterFlags).finish()
    }
}
impl ::core::default::Default for NodeUtilizationInfoElement {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NodeUtilizationInfoElement {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.AvailableMemory == other.AvailableMemory && self.AvailableMemoryAfterReclamation == other.AvailableMemoryAfterReclamation
    }
}
impl ::core::cmp::Eq for NodeUtilizationInfoElement {}
impl ::core::fmt::Debug for NodeUtilizationInfoElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NodeUtilizationInfoElement").field("Id", &self.Id).field("AvailableMemory", &self.AvailableMemory).field("AvailableMemoryAfterReclamation", &self.AvailableMemoryAfterReclamation).finish()
    }
}
impl ::core::default::Default for PLACEMENT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PLACEMENT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLACEMENT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for POST_UPGRADE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POST_UPGRADE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.newMajorVersion == other.newMajorVersion && self.newUpgradeVersion == other.newUpgradeVersion && self.oldMajorVersion == other.oldMajorVersion && self.oldUpgradeVersion == other.oldUpgradeVersion && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for POST_UPGRADE_VERSION_INFO {}
impl ::core::fmt::Debug for POST_UPGRADE_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POST_UPGRADE_VERSION_INFO").field("newMajorVersion", &self.newMajorVersion).field("newUpgradeVersion", &self.newUpgradeVersion).field("oldMajorVersion", &self.oldMajorVersion).field("oldUpgradeVersion", &self.oldUpgradeVersion).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for PaxosTagCStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PaxosTagCStruct {
    fn eq(&self, other: &Self) -> bool {
        self.__padding__PaxosTagVtable == other.__padding__PaxosTagVtable
            && self.__padding__NextEpochVtable == other.__padding__NextEpochVtable
            && self.__padding__NextEpoch_DateTimeVtable == other.__padding__NextEpoch_DateTimeVtable
            && self.NextEpoch_DateTime_ticks == other.NextEpoch_DateTime_ticks
            && self.NextEpoch_Value == other.NextEpoch_Value
            && self.__padding__BoundryNextEpoch == other.__padding__BoundryNextEpoch
            && self.__padding__EpochVtable == other.__padding__EpochVtable
            && self.__padding__Epoch_DateTimeVtable == other.__padding__Epoch_DateTimeVtable
            && self.Epoch_DateTime_ticks == other.Epoch_DateTime_ticks
            && self.Epoch_Value == other.Epoch_Value
            && self.__padding__BoundryEpoch == other.__padding__BoundryEpoch
            && self.Sequence == other.Sequence
            && self.__padding__BoundrySequence == other.__padding__BoundrySequence
    }
}
impl ::core::cmp::Eq for PaxosTagCStruct {}
impl ::core::fmt::Debug for PaxosTagCStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PaxosTagCStruct")
            .field("__padding__PaxosTagVtable", &self.__padding__PaxosTagVtable)
            .field("__padding__NextEpochVtable", &self.__padding__NextEpochVtable)
            .field("__padding__NextEpoch_DateTimeVtable", &self.__padding__NextEpoch_DateTimeVtable)
            .field("NextEpoch_DateTime_ticks", &self.NextEpoch_DateTime_ticks)
            .field("NextEpoch_Value", &self.NextEpoch_Value)
            .field("__padding__BoundryNextEpoch", &self.__padding__BoundryNextEpoch)
            .field("__padding__EpochVtable", &self.__padding__EpochVtable)
            .field("__padding__Epoch_DateTimeVtable", &self.__padding__Epoch_DateTimeVtable)
            .field("Epoch_DateTime_ticks", &self.Epoch_DateTime_ticks)
            .field("Epoch_Value", &self.Epoch_Value)
            .field("__padding__BoundryEpoch", &self.__padding__BoundryEpoch)
            .field("Sequence", &self.Sequence)
            .field("__padding__BoundrySequence", &self.__padding__BoundrySequence)
            .finish()
    }
}
impl ::core::default::Default for RESDLL_CONTEXT_OPERATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESDLL_CONTEXT_OPERATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESDLL_CONTEXT_OPERATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RESOURCE_EXIT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESOURCE_EXIT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESOURCE_EXIT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RESOURCE_FAILURE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESOURCE_FAILURE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwRestartAttemptsRemaining == other.dwRestartAttemptsRemaining && self.dwRestartPeriodRemaining == other.dwRestartPeriodRemaining
    }
}
impl ::core::cmp::Eq for RESOURCE_FAILURE_INFO {}
impl ::core::fmt::Debug for RESOURCE_FAILURE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESOURCE_FAILURE_INFO").field("dwRestartAttemptsRemaining", &self.dwRestartAttemptsRemaining).field("dwRestartPeriodRemaining", &self.dwRestartPeriodRemaining).finish()
    }
}
impl ::core::default::Default for RESOURCE_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESOURCE_FAILURE_INFO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for RESOURCE_FAILURE_INFO_BUFFER {}
impl ::core::fmt::Debug for RESOURCE_FAILURE_INFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESOURCE_FAILURE_INFO_BUFFER").field("dwVersion", &self.dwVersion).field("Info", &self.Info).finish()
    }
}
impl ::core::default::Default for RESOURCE_MONITOR_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESOURCE_MONITOR_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESOURCE_MONITOR_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESOURCE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESOURCE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ResourceState == other.ResourceState && self.CheckPoint == other.CheckPoint && self.WaitHint == other.WaitHint && self.EventHandle == other.EventHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESOURCE_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESOURCE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESOURCE_STATUS").field("ResourceState", &self.ResourceState).field("CheckPoint", &self.CheckPoint).field("WaitHint", &self.WaitHint).field("EventHandle", &self.EventHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESOURCE_STATUS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESOURCE_STATUS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ResourceState == other.ResourceState && self.CheckPoint == other.CheckPoint && self.EventHandle == other.EventHandle && self.ApplicationSpecificErrorCode == other.ApplicationSpecificErrorCode && self.Flags == other.Flags && self.WaitHint == other.WaitHint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESOURCE_STATUS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESOURCE_STATUS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESOURCE_STATUS_EX").field("ResourceState", &self.ResourceState).field("CheckPoint", &self.CheckPoint).field("EventHandle", &self.EventHandle).field("ApplicationSpecificErrorCode", &self.ApplicationSpecificErrorCode).field("Flags", &self.Flags).field("WaitHint", &self.WaitHint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.isTerminalFailure == other.isTerminalFailure && self.restartPeriodRemaining == other.restartPeriodRemaining
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESOURCE_TERMINAL_FAILURE_INFO_BUFFER").field("isTerminalFailure", &self.isTerminalFailure).field("restartPeriodRemaining", &self.restartPeriodRemaining).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESUTIL_FILETIME_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESUTIL_FILETIME_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Default == other.Default && self.Minimum == other.Minimum && self.Maximum == other.Maximum
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESUTIL_FILETIME_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESUTIL_FILETIME_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESUTIL_FILETIME_DATA").field("Default", &self.Default).field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).finish()
    }
}
impl ::core::default::Default for RESUTIL_LARGEINT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESUTIL_LARGEINT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Default == other.Default && self.Minimum == other.Minimum && self.Maximum == other.Maximum
    }
}
impl ::core::cmp::Eq for RESUTIL_LARGEINT_DATA {}
impl ::core::fmt::Debug for RESUTIL_LARGEINT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESUTIL_LARGEINT_DATA").field("Default", &self.Default).field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESUTIL_PROPERTY_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RESUTIL_ULARGEINT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESUTIL_ULARGEINT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Default == other.Default && self.Minimum == other.Minimum && self.Maximum == other.Maximum
    }
}
impl ::core::cmp::Eq for RESUTIL_ULARGEINT_DATA {}
impl ::core::fmt::Debug for RESUTIL_ULARGEINT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESUTIL_ULARGEINT_DATA").field("Default", &self.Default).field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).finish()
    }
}
impl ::core::default::Default for ResourceUtilizationInfoElement {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ResourceUtilizationInfoElement {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalNumaId == other.PhysicalNumaId && self.CurrentMemory == other.CurrentMemory
    }
}
impl ::core::cmp::Eq for ResourceUtilizationInfoElement {}
impl ::core::fmt::Debug for ResourceUtilizationInfoElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ResourceUtilizationInfoElement").field("PhysicalNumaId", &self.PhysicalNumaId).field("CurrentMemory", &self.CurrentMemory).finish()
    }
}
impl ::core::default::Default for SR_DISK_REPLICATION_ELIGIBLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SR_DISK_REPLICATION_ELIGIBLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SR_DISK_REPLICATION_ELIGIBLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SR_REPLICATED_DISK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SR_REPLICATED_DISK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SR_REPLICATED_DISK_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.ReplicationGroupName == other.ReplicationGroupName && self.Description == other.Description && self.LogPath == other.LogPath && self.MaxLogSizeInBytes == other.MaxLogSizeInBytes && self.LogType == other.LogType && self.ReplicationMode == other.ReplicationMode && self.MinimumPartnersInSync == other.MinimumPartnersInSync && self.EnableWriteConsistency == other.EnableWriteConsistency && self.EnableEncryption == other.EnableEncryption && self.CertificateThumbprint == other.CertificateThumbprint && self.VolumeNameCount == other.VolumeNameCount && self.VolumeNames == other.VolumeNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP")
            .field("ReplicationGroupName", &self.ReplicationGroupName)
            .field("Description", &self.Description)
            .field("LogPath", &self.LogPath)
            .field("MaxLogSizeInBytes", &self.MaxLogSizeInBytes)
            .field("LogType", &self.LogType)
            .field("ReplicationMode", &self.ReplicationMode)
            .field("MinimumPartnersInSync", &self.MinimumPartnersInSync)
            .field("EnableWriteConsistency", &self.EnableWriteConsistency)
            .field("EnableEncryption", &self.EnableEncryption)
            .field("CertificateThumbprint", &self.CertificateThumbprint)
            .field("VolumeNameCount", &self.VolumeNameCount)
            .field("VolumeNames", &self.VolumeNames)
            .finish()
    }
}
impl ::core::default::Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Result == other.Result && self.ErrorString == other.ErrorString
    }
}
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {}
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT").field("Result", &self.Result).field("ErrorString", &self.ErrorString).finish()
    }
}
impl ::core::default::Default for SR_RESOURCE_TYPE_DISK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_DISK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Reason == other.Reason && self.DiskGuid == other.DiskGuid
    }
}
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_DISK_INFO {}
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_DISK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_DISK_INFO").field("Reason", &self.Reason).field("DiskGuid", &self.DiskGuid).finish()
    }
}
impl ::core::default::Default for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.DiskInfo == other.DiskInfo
    }
}
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {}
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT").field("Count", &self.Count).field("DiskInfo", &self.DiskInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn eq(&self, other: &Self) -> bool {
        self.DataDiskGuid == other.DataDiskGuid && self.IncludeOfflineDisks == other.IncludeOfflineDisks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS").field("DataDiskGuid", &self.DataDiskGuid).field("IncludeOfflineDisks", &self.IncludeOfflineDisks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn eq(&self, other: &Self) -> bool {
        self.DataDiskGuid == other.DataDiskGuid && self.IncludeAvailableStoargeDisks == other.IncludeAvailableStoargeDisks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS").field("DataDiskGuid", &self.DataDiskGuid).field("IncludeAvailableStoargeDisks", &self.IncludeAvailableStoargeDisks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn eq(&self, other: &Self) -> bool {
        self.SourceDataDiskGuid == other.SourceDataDiskGuid && self.TargetReplicationGroupGuid == other.TargetReplicationGroupGuid && self.SkipConnectivityCheck == other.SkipConnectivityCheck && self.IncludeOfflineDisks == other.IncludeOfflineDisks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS").field("SourceDataDiskGuid", &self.SourceDataDiskGuid).field("TargetReplicationGroupGuid", &self.TargetReplicationGroupGuid).field("SkipConnectivityCheck", &self.SkipConnectivityCheck).field("IncludeOfflineDisks", &self.IncludeOfflineDisks).finish()
    }
}
impl ::core::default::Default for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.ClusterDiskResourceGuid == other.ClusterDiskResourceGuid && self.ReplicationGroupId == other.ReplicationGroupId && self.ReplicationGroupName == other.ReplicationGroupName
    }
}
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_REPLICATED_DISK {}
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_REPLICATED_DISK").field("Type", &self.Type).field("ClusterDiskResourceGuid", &self.ClusterDiskResourceGuid).field("ReplicationGroupId", &self.ReplicationGroupId).field("ReplicationGroupName", &self.ReplicationGroupName).finish()
    }
}
impl ::core::default::Default for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.ReplicatedDisks == other.ReplicatedDisks
    }
}
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {}
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT").field("Count", &self.Count).field("ReplicatedDisks", &self.ReplicatedDisks).finish()
    }
}
impl ::core::default::Default for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.PartitionArray == other.PartitionArray
    }
}
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {}
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY").field("Count", &self.Count).field("PartitionArray", &self.PartitionArray).finish()
    }
}
impl ::core::default::Default for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionOffset == other.PartitionOffset && self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {}
impl ::core::fmt::Debug for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO").field("PartitionOffset", &self.PartitionOffset).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::core::default::Default for VM_RESDLL_CONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VM_RESDLL_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VM_RESDLL_CONTEXT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WitnessTagHelper {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WitnessTagHelper {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.paxosToValidate == other.paxosToValidate
    }
}
impl ::core::cmp::Eq for WitnessTagHelper {}
impl ::core::fmt::Debug for WitnessTagHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WitnessTagHelper").field("Version", &self.Version).field("paxosToValidate", &self.paxosToValidate).finish()
    }
}
impl ::core::default::Default for WitnessTagUpdateHelper {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WitnessTagUpdateHelper {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.paxosToSet == other.paxosToSet && self.paxosToValidate == other.paxosToValidate
    }
}
impl ::core::cmp::Eq for WitnessTagUpdateHelper {}
impl ::core::fmt::Debug for WitnessTagUpdateHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WitnessTagUpdateHelper").field("Version", &self.Version).field("paxosToSet", &self.paxosToSet).field("paxosToValidate", &self.paxosToValidate).finish()
    }
}
