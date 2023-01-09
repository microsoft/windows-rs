impl ::core::default::Default for ACCESS_REASON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACCESS_REASON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCESS_REASON_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ACTIVATION_CONTEXT_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATION_CONTEXT_INFO_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ALERT_SYSTEM_SEV {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ALERT_SYSTEM_SEV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ALERT_SYSTEM_SEV").field(&self.0).finish()
    }
}
impl ::core::default::Default for ANON_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ANON_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Sig1 == other.Sig1 && self.Sig2 == other.Sig2 && self.Version == other.Version && self.Machine == other.Machine && self.TimeDateStamp == other.TimeDateStamp && self.ClassID == other.ClassID && self.SizeOfData == other.SizeOfData
    }
}
impl ::core::cmp::Eq for ANON_OBJECT_HEADER {}
impl ::core::fmt::Debug for ANON_OBJECT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ANON_OBJECT_HEADER").field("Sig1", &self.Sig1).field("Sig2", &self.Sig2).field("Version", &self.Version).field("Machine", &self.Machine).field("TimeDateStamp", &self.TimeDateStamp).field("ClassID", &self.ClassID).field("SizeOfData", &self.SizeOfData).finish()
    }
}
impl ::core::default::Default for ANON_OBJECT_HEADER_BIGOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ANON_OBJECT_HEADER_BIGOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.Sig1 == other.Sig1 && self.Sig2 == other.Sig2 && self.Version == other.Version && self.Machine == other.Machine && self.TimeDateStamp == other.TimeDateStamp && self.ClassID == other.ClassID && self.SizeOfData == other.SizeOfData && self.Flags == other.Flags && self.MetaDataSize == other.MetaDataSize && self.MetaDataOffset == other.MetaDataOffset && self.NumberOfSections == other.NumberOfSections && self.PointerToSymbolTable == other.PointerToSymbolTable && self.NumberOfSymbols == other.NumberOfSymbols
    }
}
impl ::core::cmp::Eq for ANON_OBJECT_HEADER_BIGOBJ {}
impl ::core::fmt::Debug for ANON_OBJECT_HEADER_BIGOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ANON_OBJECT_HEADER_BIGOBJ")
            .field("Sig1", &self.Sig1)
            .field("Sig2", &self.Sig2)
            .field("Version", &self.Version)
            .field("Machine", &self.Machine)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("ClassID", &self.ClassID)
            .field("SizeOfData", &self.SizeOfData)
            .field("Flags", &self.Flags)
            .field("MetaDataSize", &self.MetaDataSize)
            .field("MetaDataOffset", &self.MetaDataOffset)
            .field("NumberOfSections", &self.NumberOfSections)
            .field("PointerToSymbolTable", &self.PointerToSymbolTable)
            .field("NumberOfSymbols", &self.NumberOfSymbols)
            .finish()
    }
}
impl ::core::default::Default for ANON_OBJECT_HEADER_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ANON_OBJECT_HEADER_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Sig1 == other.Sig1 && self.Sig2 == other.Sig2 && self.Version == other.Version && self.Machine == other.Machine && self.TimeDateStamp == other.TimeDateStamp && self.ClassID == other.ClassID && self.SizeOfData == other.SizeOfData && self.Flags == other.Flags && self.MetaDataSize == other.MetaDataSize && self.MetaDataOffset == other.MetaDataOffset
    }
}
impl ::core::cmp::Eq for ANON_OBJECT_HEADER_V2 {}
impl ::core::fmt::Debug for ANON_OBJECT_HEADER_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ANON_OBJECT_HEADER_V2").field("Sig1", &self.Sig1).field("Sig2", &self.Sig2).field("Version", &self.Version).field("Machine", &self.Machine).field("TimeDateStamp", &self.TimeDateStamp).field("ClassID", &self.ClassID).field("SizeOfData", &self.SizeOfData).field("Flags", &self.Flags).field("MetaDataSize", &self.MetaDataSize).field("MetaDataOffset", &self.MetaDataOffset).finish()
    }
}
impl ::core::default::Default for APPCOMMAND_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPCOMMAND_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPCOMMAND_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for APPLICATIONLAUNCH_SETTING_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APPLICATIONLAUNCH_SETTING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.ActivationTime == other.ActivationTime && self.Flags == other.Flags && self.ButtonInstanceID == other.ButtonInstanceID
    }
}
impl ::core::cmp::Eq for APPLICATIONLAUNCH_SETTING_VALUE {}
impl ::core::fmt::Debug for APPLICATIONLAUNCH_SETTING_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPLICATIONLAUNCH_SETTING_VALUE").field("ActivationTime", &self.ActivationTime).field("Flags", &self.Flags).field("ButtonInstanceID", &self.ButtonInstanceID).finish()
    }
}
impl ::core::default::Default for ARM64_FNPDATA_CR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ARM64_FNPDATA_CR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARM64_FNPDATA_CR").field(&self.0).finish()
    }
}
impl ::core::default::Default for ARM64_FNPDATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ARM64_FNPDATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARM64_FNPDATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ATF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ATF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ATF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ATF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ATF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ATF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ATF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CFE_UNDERLINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CFE_UNDERLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CFE_UNDERLINE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CFE_UNDERLINE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CFE_UNDERLINE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CFE_UNDERLINE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CFE_UNDERLINE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CFE_UNDERLINE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for COMPONENT_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COMPONENT_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.ComponentFlags == other.ComponentFlags
    }
}
impl ::core::cmp::Eq for COMPONENT_FILTER {}
impl ::core::fmt::Debug for COMPONENT_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPONENT_FILTER").field("ComponentFlags", &self.ComponentFlags).finish()
    }
}
impl ::core::default::Default for DEVICE_EVENT_BECOMING_READY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_EVENT_BECOMING_READY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Reason == other.Reason && self.Estimated100msToReady == other.Estimated100msToReady
    }
}
impl ::core::cmp::Eq for DEVICE_EVENT_BECOMING_READY {}
impl ::core::fmt::Debug for DEVICE_EVENT_BECOMING_READY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_EVENT_BECOMING_READY").field("Version", &self.Version).field("Reason", &self.Reason).field("Estimated100msToReady", &self.Estimated100msToReady).finish()
    }
}
impl ::core::default::Default for DEVICE_EVENT_EXTERNAL_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_EVENT_EXTERNAL_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.DeviceClass == other.DeviceClass && self.ButtonStatus == other.ButtonStatus && self.Request == other.Request && self.SystemTime == other.SystemTime
    }
}
impl ::core::cmp::Eq for DEVICE_EVENT_EXTERNAL_REQUEST {}
impl ::core::fmt::Debug for DEVICE_EVENT_EXTERNAL_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_EVENT_EXTERNAL_REQUEST").field("Version", &self.Version).field("DeviceClass", &self.DeviceClass).field("ButtonStatus", &self.ButtonStatus).field("Request", &self.Request).field("SystemTime", &self.SystemTime).finish()
    }
}
impl ::core::default::Default for DEVICE_EVENT_GENERIC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_EVENT_GENERIC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.EventNumber == other.EventNumber
    }
}
impl ::core::cmp::Eq for DEVICE_EVENT_GENERIC_DATA {}
impl ::core::fmt::Debug for DEVICE_EVENT_GENERIC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_EVENT_GENERIC_DATA").field("EventNumber", &self.EventNumber).finish()
    }
}
impl ::core::default::Default for DEVICE_EVENT_MOUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_EVENT_MOUNT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.FileSystemNameLength == other.FileSystemNameLength && self.FileSystemNameOffset == other.FileSystemNameOffset
    }
}
impl ::core::cmp::Eq for DEVICE_EVENT_MOUNT {}
impl ::core::fmt::Debug for DEVICE_EVENT_MOUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_EVENT_MOUNT").field("Version", &self.Version).field("Flags", &self.Flags).field("FileSystemNameLength", &self.FileSystemNameLength).field("FileSystemNameOffset", &self.FileSystemNameOffset).finish()
    }
}
impl ::core::default::Default for DEVICE_EVENT_RBC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_EVENT_RBC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.EventNumber == other.EventNumber && self.SenseQualifier == other.SenseQualifier && self.SenseCode == other.SenseCode && self.SenseKey == other.SenseKey && self.Reserved == other.Reserved && self.Information == other.Information
    }
}
impl ::core::cmp::Eq for DEVICE_EVENT_RBC_DATA {}
impl ::core::fmt::Debug for DEVICE_EVENT_RBC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_EVENT_RBC_DATA").field("EventNumber", &self.EventNumber).field("SenseQualifier", &self.SenseQualifier).field("SenseCode", &self.SenseCode).field("SenseKey", &self.SenseKey).field("Reserved", &self.Reserved).field("Information", &self.Information).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEV_BROADCAST_DEVICEINTERFACE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEV_BROADCAST_DEVICEINTERFACE_A {
    fn eq(&self, other: &Self) -> bool {
        self.dbcc_size == other.dbcc_size && self.dbcc_devicetype == other.dbcc_devicetype && self.dbcc_reserved == other.dbcc_reserved && self.dbcc_classguid == other.dbcc_classguid && self.dbcc_name == other.dbcc_name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEV_BROADCAST_DEVICEINTERFACE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEV_BROADCAST_DEVICEINTERFACE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_DEVICEINTERFACE_A").field("dbcc_size", &self.dbcc_size).field("dbcc_devicetype", &self.dbcc_devicetype).field("dbcc_reserved", &self.dbcc_reserved).field("dbcc_classguid", &self.dbcc_classguid).field("dbcc_name", &self.dbcc_name).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_DEVICEINTERFACE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_DEVICEINTERFACE_W {
    fn eq(&self, other: &Self) -> bool {
        self.dbcc_size == other.dbcc_size && self.dbcc_devicetype == other.dbcc_devicetype && self.dbcc_reserved == other.dbcc_reserved && self.dbcc_classguid == other.dbcc_classguid && self.dbcc_name == other.dbcc_name
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_DEVICEINTERFACE_W {}
impl ::core::fmt::Debug for DEV_BROADCAST_DEVICEINTERFACE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_DEVICEINTERFACE_W").field("dbcc_size", &self.dbcc_size).field("dbcc_devicetype", &self.dbcc_devicetype).field("dbcc_reserved", &self.dbcc_reserved).field("dbcc_classguid", &self.dbcc_classguid).field("dbcc_name", &self.dbcc_name).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_DEVNODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_DEVNODE {
    fn eq(&self, other: &Self) -> bool {
        self.dbcd_size == other.dbcd_size && self.dbcd_devicetype == other.dbcd_devicetype && self.dbcd_reserved == other.dbcd_reserved && self.dbcd_devnode == other.dbcd_devnode
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_DEVNODE {}
impl ::core::fmt::Debug for DEV_BROADCAST_DEVNODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_DEVNODE").field("dbcd_size", &self.dbcd_size).field("dbcd_devicetype", &self.dbcd_devicetype).field("dbcd_reserved", &self.dbcd_reserved).field("dbcd_devnode", &self.dbcd_devnode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEV_BROADCAST_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEV_BROADCAST_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.dbch_size == other.dbch_size && self.dbch_devicetype == other.dbch_devicetype && self.dbch_reserved == other.dbch_reserved && self.dbch_handle == other.dbch_handle && self.dbch_hdevnotify == other.dbch_hdevnotify && self.dbch_eventguid == other.dbch_eventguid && self.dbch_nameoffset == other.dbch_nameoffset && self.dbch_data == other.dbch_data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEV_BROADCAST_HANDLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEV_BROADCAST_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_HANDLE").field("dbch_size", &self.dbch_size).field("dbch_devicetype", &self.dbch_devicetype).field("dbch_reserved", &self.dbch_reserved).field("dbch_handle", &self.dbch_handle).field("dbch_hdevnotify", &self.dbch_hdevnotify).field("dbch_eventguid", &self.dbch_eventguid).field("dbch_nameoffset", &self.dbch_nameoffset).field("dbch_data", &self.dbch_data).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_HANDLE32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_HANDLE32 {
    fn eq(&self, other: &Self) -> bool {
        self.dbch_size == other.dbch_size && self.dbch_devicetype == other.dbch_devicetype && self.dbch_reserved == other.dbch_reserved && self.dbch_handle == other.dbch_handle && self.dbch_hdevnotify == other.dbch_hdevnotify && self.dbch_eventguid == other.dbch_eventguid && self.dbch_nameoffset == other.dbch_nameoffset && self.dbch_data == other.dbch_data
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_HANDLE32 {}
impl ::core::fmt::Debug for DEV_BROADCAST_HANDLE32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_HANDLE32").field("dbch_size", &self.dbch_size).field("dbch_devicetype", &self.dbch_devicetype).field("dbch_reserved", &self.dbch_reserved).field("dbch_handle", &self.dbch_handle).field("dbch_hdevnotify", &self.dbch_hdevnotify).field("dbch_eventguid", &self.dbch_eventguid).field("dbch_nameoffset", &self.dbch_nameoffset).field("dbch_data", &self.dbch_data).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_HANDLE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_HANDLE64 {
    fn eq(&self, other: &Self) -> bool {
        self.dbch_size == other.dbch_size && self.dbch_devicetype == other.dbch_devicetype && self.dbch_reserved == other.dbch_reserved && self.dbch_handle == other.dbch_handle && self.dbch_hdevnotify == other.dbch_hdevnotify && self.dbch_eventguid == other.dbch_eventguid && self.dbch_nameoffset == other.dbch_nameoffset && self.dbch_data == other.dbch_data
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_HANDLE64 {}
impl ::core::fmt::Debug for DEV_BROADCAST_HANDLE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_HANDLE64").field("dbch_size", &self.dbch_size).field("dbch_devicetype", &self.dbch_devicetype).field("dbch_reserved", &self.dbch_reserved).field("dbch_handle", &self.dbch_handle).field("dbch_hdevnotify", &self.dbch_hdevnotify).field("dbch_eventguid", &self.dbch_eventguid).field("dbch_nameoffset", &self.dbch_nameoffset).field("dbch_data", &self.dbch_data).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_HDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_HDR {
    fn eq(&self, other: &Self) -> bool {
        self.dbch_size == other.dbch_size && self.dbch_devicetype == other.dbch_devicetype && self.dbch_reserved == other.dbch_reserved
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_HDR {}
impl ::core::fmt::Debug for DEV_BROADCAST_HDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_HDR").field("dbch_size", &self.dbch_size).field("dbch_devicetype", &self.dbch_devicetype).field("dbch_reserved", &self.dbch_reserved).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_HDR_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEV_BROADCAST_HDR_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEV_BROADCAST_HDR_DEVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_NET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_NET {
    fn eq(&self, other: &Self) -> bool {
        self.dbcn_size == other.dbcn_size && self.dbcn_devicetype == other.dbcn_devicetype && self.dbcn_reserved == other.dbcn_reserved && self.dbcn_resource == other.dbcn_resource && self.dbcn_flags == other.dbcn_flags
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_NET {}
impl ::core::fmt::Debug for DEV_BROADCAST_NET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_NET").field("dbcn_size", &self.dbcn_size).field("dbcn_devicetype", &self.dbcn_devicetype).field("dbcn_reserved", &self.dbcn_reserved).field("dbcn_resource", &self.dbcn_resource).field("dbcn_flags", &self.dbcn_flags).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_OEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_OEM {
    fn eq(&self, other: &Self) -> bool {
        self.dbco_size == other.dbco_size && self.dbco_devicetype == other.dbco_devicetype && self.dbco_reserved == other.dbco_reserved && self.dbco_identifier == other.dbco_identifier && self.dbco_suppfunc == other.dbco_suppfunc
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_OEM {}
impl ::core::fmt::Debug for DEV_BROADCAST_OEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_OEM").field("dbco_size", &self.dbco_size).field("dbco_devicetype", &self.dbco_devicetype).field("dbco_reserved", &self.dbco_reserved).field("dbco_identifier", &self.dbco_identifier).field("dbco_suppfunc", &self.dbco_suppfunc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEV_BROADCAST_PORT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEV_BROADCAST_PORT_A {
    fn eq(&self, other: &Self) -> bool {
        self.dbcp_size == other.dbcp_size && self.dbcp_devicetype == other.dbcp_devicetype && self.dbcp_reserved == other.dbcp_reserved && self.dbcp_name == other.dbcp_name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEV_BROADCAST_PORT_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEV_BROADCAST_PORT_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_PORT_A").field("dbcp_size", &self.dbcp_size).field("dbcp_devicetype", &self.dbcp_devicetype).field("dbcp_reserved", &self.dbcp_reserved).field("dbcp_name", &self.dbcp_name).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_PORT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_PORT_W {
    fn eq(&self, other: &Self) -> bool {
        self.dbcp_size == other.dbcp_size && self.dbcp_devicetype == other.dbcp_devicetype && self.dbcp_reserved == other.dbcp_reserved && self.dbcp_name == other.dbcp_name
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_PORT_W {}
impl ::core::fmt::Debug for DEV_BROADCAST_PORT_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_PORT_W").field("dbcp_size", &self.dbcp_size).field("dbcp_devicetype", &self.dbcp_devicetype).field("dbcp_reserved", &self.dbcp_reserved).field("dbcp_name", &self.dbcp_name).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_VOLUME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEV_BROADCAST_VOLUME {
    fn eq(&self, other: &Self) -> bool {
        self.dbcv_size == other.dbcv_size && self.dbcv_devicetype == other.dbcv_devicetype && self.dbcv_reserved == other.dbcv_reserved && self.dbcv_unitmask == other.dbcv_unitmask && self.dbcv_flags == other.dbcv_flags
    }
}
impl ::core::cmp::Eq for DEV_BROADCAST_VOLUME {}
impl ::core::fmt::Debug for DEV_BROADCAST_VOLUME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_BROADCAST_VOLUME").field("dbcv_size", &self.dbcv_size).field("dbcv_devicetype", &self.dbcv_devicetype).field("dbcv_reserved", &self.dbcv_reserved).field("dbcv_unitmask", &self.dbcv_unitmask).field("dbcv_flags", &self.dbcv_flags).finish()
    }
}
impl ::core::default::Default for DEV_BROADCAST_VOLUME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEV_BROADCAST_VOLUME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEV_BROADCAST_VOLUME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISK_HEALTH_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_HEALTH_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceGuid == other.DeviceGuid
    }
}
impl ::core::cmp::Eq for DISK_HEALTH_NOTIFICATION_DATA {}
impl ::core::fmt::Debug for DISK_HEALTH_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_HEALTH_NOTIFICATION_DATA").field("DeviceGuid", &self.DeviceGuid).finish()
    }
}
impl ::core::default::Default for DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ENLISTMENT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENLISTMENT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.EnlistmentId == other.EnlistmentId && self.TransactionId == other.TransactionId && self.ResourceManagerId == other.ResourceManagerId
    }
}
impl ::core::cmp::Eq for ENLISTMENT_BASIC_INFORMATION {}
impl ::core::fmt::Debug for ENLISTMENT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENLISTMENT_BASIC_INFORMATION").field("EnlistmentId", &self.EnlistmentId).field("TransactionId", &self.TransactionId).field("ResourceManagerId", &self.ResourceManagerId).finish()
    }
}
impl ::core::default::Default for ENLISTMENT_CRM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENLISTMENT_CRM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CrmTransactionManagerId == other.CrmTransactionManagerId && self.CrmResourceManagerId == other.CrmResourceManagerId && self.CrmEnlistmentId == other.CrmEnlistmentId
    }
}
impl ::core::cmp::Eq for ENLISTMENT_CRM_INFORMATION {}
impl ::core::fmt::Debug for ENLISTMENT_CRM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENLISTMENT_CRM_INFORMATION").field("CrmTransactionManagerId", &self.CrmTransactionManagerId).field("CrmResourceManagerId", &self.CrmResourceManagerId).field("CrmEnlistmentId", &self.CrmEnlistmentId).finish()
    }
}
impl ::core::default::Default for ENLISTMENT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENLISTMENT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENLISTMENT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for GDI_NONREMOTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GESTURECONFIG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GESTURECONFIG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GESTURECONFIG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GESTURECONFIG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GESTURECONFIG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GESTURECONFIG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GESTURECONFIG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GESTURECONFIG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.DiskNumber == other.DiskNumber
    }
}
impl ::core::cmp::Eq for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {}
impl ::core::fmt::Debug for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION").field("DiskNumber", &self.DiskNumber).finish()
    }
}
impl ::core::default::Default for HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for HEAP_OPTIMIZE_RESOURCES_INFORMATION {}
impl ::core::fmt::Debug for HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAP_OPTIMIZE_RESOURCES_INFORMATION").field("Version", &self.Version).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for HIBERFILE_BUCKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HIBERFILE_BUCKET {
    fn eq(&self, other: &Self) -> bool {
        self.MaxPhysicalMemory == other.MaxPhysicalMemory && self.PhysicalMemoryPercent == other.PhysicalMemoryPercent
    }
}
impl ::core::cmp::Eq for HIBERFILE_BUCKET {}
impl ::core::fmt::Debug for HIBERFILE_BUCKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIBERFILE_BUCKET").field("MaxPhysicalMemory", &self.MaxPhysicalMemory).field("PhysicalMemoryPercent", &self.PhysicalMemoryPercent).finish()
    }
}
impl ::core::default::Default for HIBERFILE_BUCKET_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HIBERFILE_BUCKET_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIBERFILE_BUCKET_SIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IGP_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IGP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGP_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.BeginAddress == other.BeginAddress && self.EndAddress == other.EndAddress && self.ExceptionHandler == other.ExceptionHandler && self.HandlerData == other.HandlerData && self.PrologEndAddress == other.PrologEndAddress
    }
}
impl ::core::cmp::Eq for IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {}
impl ::core::fmt::Debug for IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY").field("BeginAddress", &self.BeginAddress).field("EndAddress", &self.EndAddress).field("ExceptionHandler", &self.ExceptionHandler).field("HandlerData", &self.HandlerData).field("PrologEndAddress", &self.PrologEndAddress).finish()
    }
}
impl ::core::default::Default for IMAGE_ARCHITECTURE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_ARCHITECTURE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.FixupInstRVA == other.FixupInstRVA && self.NewInst == other.NewInst
    }
}
impl ::core::cmp::Eq for IMAGE_ARCHITECTURE_ENTRY {}
impl ::core::fmt::Debug for IMAGE_ARCHITECTURE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ARCHITECTURE_ENTRY").field("FixupInstRVA", &self.FixupInstRVA).field("NewInst", &self.NewInst).finish()
    }
}
impl ::core::default::Default for IMAGE_ARCHITECTURE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_ARCHITECTURE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.FirstEntryRVA == other.FirstEntryRVA
    }
}
impl ::core::cmp::Eq for IMAGE_ARCHITECTURE_HEADER {}
impl ::core::fmt::Debug for IMAGE_ARCHITECTURE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ARCHITECTURE_HEADER").field("_bitfield", &self._bitfield).field("FirstEntryRVA", &self.FirstEntryRVA).finish()
    }
}
impl ::core::default::Default for IMAGE_ARCHIVE_MEMBER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_ARCHIVE_MEMBER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Date == other.Date && self.UserID == other.UserID && self.GroupID == other.GroupID && self.Mode == other.Mode && self.Size == other.Size && self.EndHeader == other.EndHeader
    }
}
impl ::core::cmp::Eq for IMAGE_ARCHIVE_MEMBER_HEADER {}
impl ::core::fmt::Debug for IMAGE_ARCHIVE_MEMBER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ARCHIVE_MEMBER_HEADER").field("Name", &self.Name).field("Date", &self.Date).field("UserID", &self.UserID).field("GroupID", &self.GroupID).field("Mode", &self.Mode).field("Size", &self.Size).field("EndHeader", &self.EndHeader).finish()
    }
}
impl ::core::default::Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_AUX_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_AUX_SYMBOL_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_AUX_SYMBOL_TOKEN_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_AUX_SYMBOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_AUX_SYMBOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_AUX_SYMBOL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_BASE_RELOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_BASE_RELOCATION {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualAddress == other.VirtualAddress && self.SizeOfBlock == other.SizeOfBlock
    }
}
impl ::core::cmp::Eq for IMAGE_BASE_RELOCATION {}
impl ::core::fmt::Debug for IMAGE_BASE_RELOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_BASE_RELOCATION").field("VirtualAddress", &self.VirtualAddress).field("SizeOfBlock", &self.SizeOfBlock).finish()
    }
}
impl ::core::default::Default for IMAGE_BOUND_FORWARDER_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_BOUND_FORWARDER_REF {
    fn eq(&self, other: &Self) -> bool {
        self.TimeDateStamp == other.TimeDateStamp && self.OffsetModuleName == other.OffsetModuleName && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for IMAGE_BOUND_FORWARDER_REF {}
impl ::core::fmt::Debug for IMAGE_BOUND_FORWARDER_REF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_BOUND_FORWARDER_REF").field("TimeDateStamp", &self.TimeDateStamp).field("OffsetModuleName", &self.OffsetModuleName).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for IMAGE_BOUND_IMPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_BOUND_IMPORT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.TimeDateStamp == other.TimeDateStamp && self.OffsetModuleName == other.OffsetModuleName && self.NumberOfModuleForwarderRefs == other.NumberOfModuleForwarderRefs
    }
}
impl ::core::cmp::Eq for IMAGE_BOUND_IMPORT_DESCRIPTOR {}
impl ::core::fmt::Debug for IMAGE_BOUND_IMPORT_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_BOUND_IMPORT_DESCRIPTOR").field("TimeDateStamp", &self.TimeDateStamp).field("OffsetModuleName", &self.OffsetModuleName).field("NumberOfModuleForwarderRefs", &self.NumberOfModuleForwarderRefs).finish()
    }
}
impl ::core::default::Default for IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.FuncStart == other.FuncStart && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for IMAGE_CE_RUNTIME_FUNCTION_ENTRY {}
impl ::core::fmt::Debug for IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_CE_RUNTIME_FUNCTION_ENTRY").field("FuncStart", &self.FuncStart).field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGE_DEBUG_MISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGE_DEBUG_MISC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.Length == other.Length && self.Unicode == other.Unicode && self.Reserved == other.Reserved && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGE_DEBUG_MISC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGE_DEBUG_MISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DEBUG_MISC").field("DataType", &self.DataType).field("Length", &self.Length).field("Unicode", &self.Unicode).field("Reserved", &self.Reserved).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for IMAGE_DOS_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_DYNAMIC_RELOCATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_DYNAMIC_RELOCATION32_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_DYNAMIC_RELOCATION64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_DYNAMIC_RELOCATION64_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_DYNAMIC_RELOCATION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_DYNAMIC_RELOCATION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for IMAGE_DYNAMIC_RELOCATION_TABLE {}
impl ::core::fmt::Debug for IMAGE_DYNAMIC_RELOCATION_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DYNAMIC_RELOCATION_TABLE").field("Version", &self.Version).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_EXPORT_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_EXPORT_DIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.Characteristics == other.Characteristics && self.TimeDateStamp == other.TimeDateStamp && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.Name == other.Name && self.Base == other.Base && self.NumberOfFunctions == other.NumberOfFunctions && self.NumberOfNames == other.NumberOfNames && self.AddressOfFunctions == other.AddressOfFunctions && self.AddressOfNames == other.AddressOfNames && self.AddressOfNameOrdinals == other.AddressOfNameOrdinals
    }
}
impl ::core::cmp::Eq for IMAGE_EXPORT_DIRECTORY {}
impl ::core::fmt::Debug for IMAGE_EXPORT_DIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_EXPORT_DIRECTORY")
            .field("Characteristics", &self.Characteristics)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("Name", &self.Name)
            .field("Base", &self.Base)
            .field("NumberOfFunctions", &self.NumberOfFunctions)
            .field("NumberOfNames", &self.NumberOfNames)
            .field("AddressOfFunctions", &self.AddressOfFunctions)
            .field("AddressOfNames", &self.AddressOfNames)
            .field("AddressOfNameOrdinals", &self.AddressOfNameOrdinals)
            .finish()
    }
}
impl ::core::default::Default for IMAGE_HOT_PATCH_BASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_HOT_PATCH_BASE {
    fn eq(&self, other: &Self) -> bool {
        self.SequenceNumber == other.SequenceNumber && self.Flags == other.Flags && self.OriginalTimeDateStamp == other.OriginalTimeDateStamp && self.OriginalCheckSum == other.OriginalCheckSum && self.CodeIntegrityInfo == other.CodeIntegrityInfo && self.CodeIntegritySize == other.CodeIntegritySize && self.PatchTable == other.PatchTable && self.BufferOffset == other.BufferOffset
    }
}
impl ::core::cmp::Eq for IMAGE_HOT_PATCH_BASE {}
impl ::core::fmt::Debug for IMAGE_HOT_PATCH_BASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_HOT_PATCH_BASE").field("SequenceNumber", &self.SequenceNumber).field("Flags", &self.Flags).field("OriginalTimeDateStamp", &self.OriginalTimeDateStamp).field("OriginalCheckSum", &self.OriginalCheckSum).field("CodeIntegrityInfo", &self.CodeIntegrityInfo).field("CodeIntegritySize", &self.CodeIntegritySize).field("PatchTable", &self.PatchTable).field("BufferOffset", &self.BufferOffset).finish()
    }
}
impl ::core::default::Default for IMAGE_HOT_PATCH_HASHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_HOT_PATCH_HASHES {
    fn eq(&self, other: &Self) -> bool {
        self.SHA256 == other.SHA256 && self.SHA1 == other.SHA1
    }
}
impl ::core::cmp::Eq for IMAGE_HOT_PATCH_HASHES {}
impl ::core::fmt::Debug for IMAGE_HOT_PATCH_HASHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_HOT_PATCH_HASHES").field("SHA256", &self.SHA256).field("SHA1", &self.SHA1).finish()
    }
}
impl ::core::default::Default for IMAGE_HOT_PATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_HOT_PATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SequenceNumber == other.SequenceNumber && self.BaseImageList == other.BaseImageList && self.BaseImageCount == other.BaseImageCount && self.BufferOffset == other.BufferOffset && self.ExtraPatchSize == other.ExtraPatchSize
    }
}
impl ::core::cmp::Eq for IMAGE_HOT_PATCH_INFO {}
impl ::core::fmt::Debug for IMAGE_HOT_PATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_HOT_PATCH_INFO").field("Version", &self.Version).field("Size", &self.Size).field("SequenceNumber", &self.SequenceNumber).field("BaseImageList", &self.BaseImageList).field("BaseImageCount", &self.BaseImageCount).field("BufferOffset", &self.BufferOffset).field("ExtraPatchSize", &self.ExtraPatchSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGE_IMPORT_BY_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGE_IMPORT_BY_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Hint == other.Hint && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGE_IMPORT_BY_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGE_IMPORT_BY_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_IMPORT_BY_NAME").field("Hint", &self.Hint).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_IMPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_LINENUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGE_OS2_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGE_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_POLICY_ENTRY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_POLICY_ENTRY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_POLICY_ENTRY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_POLICY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_POLICY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_POLICY_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGE_POLICY_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.PrologueByteCount == other.PrologueByteCount
    }
}
impl ::core::cmp::Eq for IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {}
impl ::core::fmt::Debug for IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER").field("PrologueByteCount", &self.PrologueByteCount).finish()
    }
}
impl ::core::default::Default for IMAGE_RELOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_RESOURCE_DATA_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_RESOURCE_DATA_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetToData == other.OffsetToData && self.Size == other.Size && self.CodePage == other.CodePage && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for IMAGE_RESOURCE_DATA_ENTRY {}
impl ::core::fmt::Debug for IMAGE_RESOURCE_DATA_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_RESOURCE_DATA_ENTRY").field("OffsetToData", &self.OffsetToData).field("Size", &self.Size).field("CodePage", &self.CodePage).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for IMAGE_RESOURCE_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_RESOURCE_DIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.Characteristics == other.Characteristics && self.TimeDateStamp == other.TimeDateStamp && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.NumberOfNamedEntries == other.NumberOfNamedEntries && self.NumberOfIdEntries == other.NumberOfIdEntries
    }
}
impl ::core::cmp::Eq for IMAGE_RESOURCE_DIRECTORY {}
impl ::core::fmt::Debug for IMAGE_RESOURCE_DIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_RESOURCE_DIRECTORY").field("Characteristics", &self.Characteristics).field("TimeDateStamp", &self.TimeDateStamp).field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("NumberOfNamedEntries", &self.NumberOfNamedEntries).field("NumberOfIdEntries", &self.NumberOfIdEntries).finish()
    }
}
impl ::core::default::Default for IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGE_RESOURCE_DIRECTORY_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGE_RESOURCE_DIRECTORY_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.NameString == other.NameString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGE_RESOURCE_DIRECTORY_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGE_RESOURCE_DIRECTORY_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_RESOURCE_DIRECTORY_STRING").field("Length", &self.Length).field("NameString", &self.NameString).finish()
    }
}
impl ::core::default::Default for IMAGE_RESOURCE_DIR_STRING_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_RESOURCE_DIR_STRING_U {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.NameString == other.NameString
    }
}
impl ::core::cmp::Eq for IMAGE_RESOURCE_DIR_STRING_U {}
impl ::core::fmt::Debug for IMAGE_RESOURCE_DIR_STRING_U {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_RESOURCE_DIR_STRING_U").field("Length", &self.Length).field("NameString", &self.NameString).finish()
    }
}
impl ::core::default::Default for IMAGE_SEPARATE_DEBUG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_SEPARATE_DEBUG_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.Flags == other.Flags && self.Machine == other.Machine && self.Characteristics == other.Characteristics && self.TimeDateStamp == other.TimeDateStamp && self.CheckSum == other.CheckSum && self.ImageBase == other.ImageBase && self.SizeOfImage == other.SizeOfImage && self.NumberOfSections == other.NumberOfSections && self.ExportedNamesSize == other.ExportedNamesSize && self.DebugDirectorySize == other.DebugDirectorySize && self.SectionAlignment == other.SectionAlignment && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for IMAGE_SEPARATE_DEBUG_HEADER {}
impl ::core::fmt::Debug for IMAGE_SEPARATE_DEBUG_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_SEPARATE_DEBUG_HEADER")
            .field("Signature", &self.Signature)
            .field("Flags", &self.Flags)
            .field("Machine", &self.Machine)
            .field("Characteristics", &self.Characteristics)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("CheckSum", &self.CheckSum)
            .field("ImageBase", &self.ImageBase)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("NumberOfSections", &self.NumberOfSections)
            .field("ExportedNamesSize", &self.ExportedNamesSize)
            .field("DebugDirectorySize", &self.DebugDirectorySize)
            .field("SectionAlignment", &self.SectionAlignment)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_SYMBOL_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_TLS_DIRECTORY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_TLS_DIRECTORY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_VXD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMPORT_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMPORT_OBJECT_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMPORT_OBJECT_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMPORT_OBJECT_NAME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMPORT_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMPORT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMPORT_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_JobObjects")]
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_JobObjects")]
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops && self.MaxBandwidth == other.MaxBandwidth && self.ReservationIops == other.ReservationIops && self.VolumeName == other.VolumeName && self.BaseIoSize == other.BaseIoSize && self.ControlFlags == other.ControlFlags && self.VolumeNameLength == other.VolumeNameLength
    }
}
#[cfg(feature = "Win32_System_JobObjects")]
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {}
#[cfg(feature = "Win32_System_JobObjects")]
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1").field("MaxIops", &self.MaxIops).field("MaxBandwidth", &self.MaxBandwidth).field("ReservationIops", &self.ReservationIops).field("VolumeName", &self.VolumeName).field("BaseIoSize", &self.BaseIoSize).field("ControlFlags", &self.ControlFlags).field("VolumeNameLength", &self.VolumeNameLength).finish()
    }
}
impl ::core::default::Default for KERNEL_CET_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KTMOBJECT_CURSOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KTMOBJECT_CURSOR {
    fn eq(&self, other: &Self) -> bool {
        self.LastQuery == other.LastQuery && self.ObjectIdCount == other.ObjectIdCount && self.ObjectIds == other.ObjectIds
    }
}
impl ::core::cmp::Eq for KTMOBJECT_CURSOR {}
impl ::core::fmt::Debug for KTMOBJECT_CURSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KTMOBJECT_CURSOR").field("LastQuery", &self.LastQuery).field("ObjectIdCount", &self.ObjectIdCount).field("ObjectIds", &self.ObjectIds).finish()
    }
}
impl ::core::default::Default for KTMOBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KTMOBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KTMOBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MAXVERSIONTESTED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAXVERSIONTESTED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaxVersionTested == other.MaxVersionTested
    }
}
impl ::core::cmp::Eq for MAXVERSIONTESTED_INFO {}
impl ::core::fmt::Debug for MAXVERSIONTESTED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAXVERSIONTESTED_INFO").field("MaxVersionTested", &self.MaxVersionTested).finish()
    }
}
impl ::core::default::Default for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved == other.Reserved && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {}
impl ::core::fmt::Debug for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE").field("Type", &self.Type).field("Reserved", &self.Reserved).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.SizeOfInformation == other.SizeOfInformation && self.Flags == other.Flags && self.AttributesOffset == other.AttributesOffset && self.AttributeCount == other.AttributeCount && self.Reserved == other.Reserved && self.TypeId == other.TypeId
    }
}
impl ::core::cmp::Eq for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {}
impl ::core::fmt::Debug for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("SizeOfInformation", &self.SizeOfInformation).field("Flags", &self.Flags).field("AttributesOffset", &self.AttributesOffset).field("AttributeCount", &self.AttributeCount).field("Reserved", &self.Reserved).field("TypeId", &self.TypeId).finish()
    }
}
impl ::core::default::Default for MEM_DEDICATED_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEM_DEDICATED_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_DEDICATED_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_SECTION_EXTENDED_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MODIFIERKEYS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODIFIERKEYS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODIFIERKEYS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODIFIERKEYS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODIFIERKEYS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODIFIERKEYS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODIFIERKEYS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODIFIERKEYS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MONITOR_DISPLAY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MONITOR_DISPLAY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_DISPLAY_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NETWORK_APP_INSTANCE_EA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETWORK_APP_INSTANCE_EA {
    fn eq(&self, other: &Self) -> bool {
        self.AppInstanceID == other.AppInstanceID && self.CsvFlags == other.CsvFlags
    }
}
impl ::core::cmp::Eq for NETWORK_APP_INSTANCE_EA {}
impl ::core::fmt::Debug for NETWORK_APP_INSTANCE_EA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_APP_INSTANCE_EA").field("AppInstanceID", &self.AppInstanceID).field("CsvFlags", &self.CsvFlags).finish()
    }
}
impl ::core::default::Default for NON_PAGED_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NOTIFY_USER_POWER_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NOTIFY_USER_POWER_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid
    }
}
impl ::core::cmp::Eq for NOTIFY_USER_POWER_SETTING {}
impl ::core::fmt::Debug for NOTIFY_USER_POWER_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFY_USER_POWER_SETTING").field("Guid", &self.Guid).finish()
    }
}
impl ::core::default::Default for NT_TIB32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NT_TIB64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PACKEDEVENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PACKEDEVENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.ulNumEventsForLogFile == other.ulNumEventsForLogFile && self.ulOffsets == other.ulOffsets
    }
}
impl ::core::cmp::Eq for PACKEDEVENTINFO {}
impl ::core::fmt::Debug for PACKEDEVENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKEDEVENTINFO").field("ulSize", &self.ulSize).field("ulNumEventsForLogFile", &self.ulNumEventsForLogFile).field("ulOffsets", &self.ulOffsets).finish()
    }
}
impl ::core::default::Default for POWER_IDLE_RESILIENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POWER_IDLE_RESILIENCY {
    fn eq(&self, other: &Self) -> bool {
        self.CoalescingTimeout == other.CoalescingTimeout && self.IdleResiliencyPeriod == other.IdleResiliencyPeriod
    }
}
impl ::core::cmp::Eq for POWER_IDLE_RESILIENCY {}
impl ::core::fmt::Debug for POWER_IDLE_RESILIENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_IDLE_RESILIENCY").field("CoalescingTimeout", &self.CoalescingTimeout).field("IdleResiliencyPeriod", &self.IdleResiliencyPeriod).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_MONITOR_INVOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POWER_MONITOR_INVOCATION {
    fn eq(&self, other: &Self) -> bool {
        self.Console == other.Console && self.RequestReason == other.RequestReason
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POWER_MONITOR_INVOCATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POWER_MONITOR_INVOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_MONITOR_INVOCATION").field("Console", &self.Console).field("RequestReason", &self.RequestReason).finish()
    }
}
impl ::core::default::Default for POWER_MONITOR_REQUEST_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_MONITOR_REQUEST_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_MONITOR_REQUEST_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_MONITOR_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_MONITOR_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_MONITOR_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_PLATFORM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POWER_PLATFORM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AoAc == other.AoAc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POWER_PLATFORM_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POWER_PLATFORM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_PLATFORM_INFORMATION").field("AoAc", &self.AoAc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    fn eq(&self, other: &Self) -> bool {
        self.IsAllowed == other.IsAllowed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES").field("IsAllowed", &self.IsAllowed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_SESSION_CONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POWER_SESSION_CONNECT {
    fn eq(&self, other: &Self) -> bool {
        self.Connected == other.Connected && self.Console == other.Console
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POWER_SESSION_CONNECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POWER_SESSION_CONNECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_SESSION_CONNECT").field("Connected", &self.Connected).field("Console", &self.Console).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_SESSION_RIT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POWER_SESSION_RIT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Active == other.Active && self.LastInputTime == other.LastInputTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POWER_SESSION_RIT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POWER_SESSION_RIT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_SESSION_RIT_STATE").field("Active", &self.Active).field("LastInputTime", &self.LastInputTime).finish()
    }
}
impl ::core::default::Default for POWER_SESSION_TIMEOUTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POWER_SESSION_TIMEOUTS {
    fn eq(&self, other: &Self) -> bool {
        self.InputTimeout == other.InputTimeout && self.DisplayTimeout == other.DisplayTimeout
    }
}
impl ::core::cmp::Eq for POWER_SESSION_TIMEOUTS {}
impl ::core::fmt::Debug for POWER_SESSION_TIMEOUTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_SESSION_TIMEOUTS").field("InputTimeout", &self.InputTimeout).field("DisplayTimeout", &self.DisplayTimeout).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POWER_SESSION_WINLOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POWER_SESSION_WINLOGON {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.Console == other.Console && self.Locked == other.Locked
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POWER_SESSION_WINLOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POWER_SESSION_WINLOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_SESSION_WINLOGON").field("SessionId", &self.SessionId).field("Console", &self.Console).field("Locked", &self.Locked).finish()
    }
}
impl ::core::default::Default for POWER_SETTING_ALTITUDE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_SETTING_ALTITUDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_SETTING_ALTITUDE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POWER_USER_PRESENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POWER_USER_PRESENCE {
    fn eq(&self, other: &Self) -> bool {
        self.UserPresence == other.UserPresence
    }
}
impl ::core::cmp::Eq for POWER_USER_PRESENCE {}
impl ::core::fmt::Debug for POWER_USER_PRESENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POWER_USER_PRESENCE").field("UserPresence", &self.UserPresence).finish()
    }
}
impl ::core::default::Default for POWER_USER_PRESENCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_USER_PRESENCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_USER_PRESENCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PPM_IDLESTATE_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_IDLESTATE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.NewState == other.NewState && self.OldState == other.OldState && self.Processors == other.Processors
    }
}
impl ::core::cmp::Eq for PPM_IDLESTATE_EVENT {}
impl ::core::fmt::Debug for PPM_IDLESTATE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_IDLESTATE_EVENT").field("NewState", &self.NewState).field("OldState", &self.OldState).field("Processors", &self.Processors).finish()
    }
}
impl ::core::default::Default for PPM_IDLE_ACCOUNTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_IDLE_ACCOUNTING {
    fn eq(&self, other: &Self) -> bool {
        self.StateCount == other.StateCount && self.TotalTransitions == other.TotalTransitions && self.ResetCount == other.ResetCount && self.StartTime == other.StartTime && self.State == other.State
    }
}
impl ::core::cmp::Eq for PPM_IDLE_ACCOUNTING {}
impl ::core::fmt::Debug for PPM_IDLE_ACCOUNTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_IDLE_ACCOUNTING").field("StateCount", &self.StateCount).field("TotalTransitions", &self.TotalTransitions).field("ResetCount", &self.ResetCount).field("StartTime", &self.StartTime).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for PPM_IDLE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_IDLE_ACCOUNTING_EX {
    fn eq(&self, other: &Self) -> bool {
        self.StateCount == other.StateCount && self.TotalTransitions == other.TotalTransitions && self.ResetCount == other.ResetCount && self.AbortCount == other.AbortCount && self.StartTime == other.StartTime && self.State == other.State
    }
}
impl ::core::cmp::Eq for PPM_IDLE_ACCOUNTING_EX {}
impl ::core::fmt::Debug for PPM_IDLE_ACCOUNTING_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_IDLE_ACCOUNTING_EX").field("StateCount", &self.StateCount).field("TotalTransitions", &self.TotalTransitions).field("ResetCount", &self.ResetCount).field("AbortCount", &self.AbortCount).field("StartTime", &self.StartTime).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for PPM_IDLE_STATE_ACCOUNTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_IDLE_STATE_ACCOUNTING {
    fn eq(&self, other: &Self) -> bool {
        self.IdleTransitions == other.IdleTransitions && self.FailedTransitions == other.FailedTransitions && self.InvalidBucketIndex == other.InvalidBucketIndex && self.TotalTime == other.TotalTime && self.IdleTimeBuckets == other.IdleTimeBuckets
    }
}
impl ::core::cmp::Eq for PPM_IDLE_STATE_ACCOUNTING {}
impl ::core::fmt::Debug for PPM_IDLE_STATE_ACCOUNTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_IDLE_STATE_ACCOUNTING").field("IdleTransitions", &self.IdleTransitions).field("FailedTransitions", &self.FailedTransitions).field("InvalidBucketIndex", &self.InvalidBucketIndex).field("TotalTime", &self.TotalTime).field("IdleTimeBuckets", &self.IdleTimeBuckets).finish()
    }
}
impl ::core::default::Default for PPM_IDLE_STATE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_IDLE_STATE_ACCOUNTING_EX {
    fn eq(&self, other: &Self) -> bool {
        self.TotalTime == other.TotalTime && self.IdleTransitions == other.IdleTransitions && self.FailedTransitions == other.FailedTransitions && self.InvalidBucketIndex == other.InvalidBucketIndex && self.MinTimeUs == other.MinTimeUs && self.MaxTimeUs == other.MaxTimeUs && self.CancelledTransitions == other.CancelledTransitions && self.IdleTimeBuckets == other.IdleTimeBuckets
    }
}
impl ::core::cmp::Eq for PPM_IDLE_STATE_ACCOUNTING_EX {}
impl ::core::fmt::Debug for PPM_IDLE_STATE_ACCOUNTING_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_IDLE_STATE_ACCOUNTING_EX").field("TotalTime", &self.TotalTime).field("IdleTransitions", &self.IdleTransitions).field("FailedTransitions", &self.FailedTransitions).field("InvalidBucketIndex", &self.InvalidBucketIndex).field("MinTimeUs", &self.MinTimeUs).field("MaxTimeUs", &self.MaxTimeUs).field("CancelledTransitions", &self.CancelledTransitions).field("IdleTimeBuckets", &self.IdleTimeBuckets).finish()
    }
}
impl ::core::default::Default for PPM_IDLE_STATE_BUCKET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_IDLE_STATE_BUCKET_EX {
    fn eq(&self, other: &Self) -> bool {
        self.TotalTimeUs == other.TotalTimeUs && self.MinTimeUs == other.MinTimeUs && self.MaxTimeUs == other.MaxTimeUs && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for PPM_IDLE_STATE_BUCKET_EX {}
impl ::core::fmt::Debug for PPM_IDLE_STATE_BUCKET_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_IDLE_STATE_BUCKET_EX").field("TotalTimeUs", &self.TotalTimeUs).field("MinTimeUs", &self.MinTimeUs).field("MaxTimeUs", &self.MaxTimeUs).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for PPM_PERFSTATE_DOMAIN_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_PERFSTATE_DOMAIN_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.Latency == other.Latency && self.Speed == other.Speed && self.Processors == other.Processors
    }
}
impl ::core::cmp::Eq for PPM_PERFSTATE_DOMAIN_EVENT {}
impl ::core::fmt::Debug for PPM_PERFSTATE_DOMAIN_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_PERFSTATE_DOMAIN_EVENT").field("State", &self.State).field("Latency", &self.Latency).field("Speed", &self.Speed).field("Processors", &self.Processors).finish()
    }
}
impl ::core::default::Default for PPM_PERFSTATE_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_PERFSTATE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.Status == other.Status && self.Latency == other.Latency && self.Speed == other.Speed && self.Processor == other.Processor
    }
}
impl ::core::cmp::Eq for PPM_PERFSTATE_EVENT {}
impl ::core::fmt::Debug for PPM_PERFSTATE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_PERFSTATE_EVENT").field("State", &self.State).field("Status", &self.Status).field("Latency", &self.Latency).field("Speed", &self.Speed).field("Processor", &self.Processor).finish()
    }
}
impl ::core::default::Default for PPM_THERMALCHANGE_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_THERMALCHANGE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.ThermalConstraint == other.ThermalConstraint && self.Processors == other.Processors
    }
}
impl ::core::cmp::Eq for PPM_THERMALCHANGE_EVENT {}
impl ::core::fmt::Debug for PPM_THERMALCHANGE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_THERMALCHANGE_EVENT").field("ThermalConstraint", &self.ThermalConstraint).field("Processors", &self.Processors).finish()
    }
}
impl ::core::default::Default for PPM_THERMAL_POLICY_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_THERMAL_POLICY_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Processors == other.Processors
    }
}
impl ::core::cmp::Eq for PPM_THERMAL_POLICY_EVENT {}
impl ::core::fmt::Debug for PPM_THERMAL_POLICY_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_THERMAL_POLICY_EVENT").field("Mode", &self.Mode).field("Processors", &self.Processors).finish()
    }
}
impl ::core::default::Default for PPM_WMI_IDLE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_WMI_IDLE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Latency == other.Latency && self.Power == other.Power && self.TimeCheck == other.TimeCheck && self.PromotePercent == other.PromotePercent && self.DemotePercent == other.DemotePercent && self.StateType == other.StateType && self.Reserved == other.Reserved && self.StateFlags == other.StateFlags && self.Context == other.Context && self.IdleHandler == other.IdleHandler && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for PPM_WMI_IDLE_STATE {}
impl ::core::fmt::Debug for PPM_WMI_IDLE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_WMI_IDLE_STATE").field("Latency", &self.Latency).field("Power", &self.Power).field("TimeCheck", &self.TimeCheck).field("PromotePercent", &self.PromotePercent).field("DemotePercent", &self.DemotePercent).field("StateType", &self.StateType).field("Reserved", &self.Reserved).field("StateFlags", &self.StateFlags).field("Context", &self.Context).field("IdleHandler", &self.IdleHandler).field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for PPM_WMI_IDLE_STATES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_WMI_IDLE_STATES {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.TargetState == other.TargetState && self.OldState == other.OldState && self.TargetProcessors == other.TargetProcessors && self.State == other.State
    }
}
impl ::core::cmp::Eq for PPM_WMI_IDLE_STATES {}
impl ::core::fmt::Debug for PPM_WMI_IDLE_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_WMI_IDLE_STATES").field("Type", &self.Type).field("Count", &self.Count).field("TargetState", &self.TargetState).field("OldState", &self.OldState).field("TargetProcessors", &self.TargetProcessors).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for PPM_WMI_IDLE_STATES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_WMI_IDLE_STATES_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.TargetState == other.TargetState && self.OldState == other.OldState && self.TargetProcessors == other.TargetProcessors && self.State == other.State
    }
}
impl ::core::cmp::Eq for PPM_WMI_IDLE_STATES_EX {}
impl ::core::fmt::Debug for PPM_WMI_IDLE_STATES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_WMI_IDLE_STATES_EX").field("Type", &self.Type).field("Count", &self.Count).field("TargetState", &self.TargetState).field("OldState", &self.OldState).field("TargetProcessors", &self.TargetProcessors).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for PPM_WMI_LEGACY_PERFSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_WMI_LEGACY_PERFSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.Frequency == other.Frequency && self.Flags == other.Flags && self.PercentFrequency == other.PercentFrequency
    }
}
impl ::core::cmp::Eq for PPM_WMI_LEGACY_PERFSTATE {}
impl ::core::fmt::Debug for PPM_WMI_LEGACY_PERFSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_WMI_LEGACY_PERFSTATE").field("Frequency", &self.Frequency).field("Flags", &self.Flags).field("PercentFrequency", &self.PercentFrequency).finish()
    }
}
impl ::core::default::Default for PPM_WMI_PERF_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_WMI_PERF_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Frequency == other.Frequency && self.Power == other.Power && self.PercentFrequency == other.PercentFrequency && self.IncreaseLevel == other.IncreaseLevel && self.DecreaseLevel == other.DecreaseLevel && self.Type == other.Type && self.IncreaseTime == other.IncreaseTime && self.DecreaseTime == other.DecreaseTime && self.Control == other.Control && self.Status == other.Status && self.HitCount == other.HitCount && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for PPM_WMI_PERF_STATE {}
impl ::core::fmt::Debug for PPM_WMI_PERF_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_WMI_PERF_STATE")
            .field("Frequency", &self.Frequency)
            .field("Power", &self.Power)
            .field("PercentFrequency", &self.PercentFrequency)
            .field("IncreaseLevel", &self.IncreaseLevel)
            .field("DecreaseLevel", &self.DecreaseLevel)
            .field("Type", &self.Type)
            .field("IncreaseTime", &self.IncreaseTime)
            .field("DecreaseTime", &self.DecreaseTime)
            .field("Control", &self.Control)
            .field("Status", &self.Status)
            .field("HitCount", &self.HitCount)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .finish()
    }
}
impl ::core::default::Default for PPM_WMI_PERF_STATES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_WMI_PERF_STATES {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count
            && self.MaxFrequency == other.MaxFrequency
            && self.CurrentState == other.CurrentState
            && self.MaxPerfState == other.MaxPerfState
            && self.MinPerfState == other.MinPerfState
            && self.LowestPerfState == other.LowestPerfState
            && self.ThermalConstraint == other.ThermalConstraint
            && self.BusyAdjThreshold == other.BusyAdjThreshold
            && self.PolicyType == other.PolicyType
            && self.Type == other.Type
            && self.Reserved == other.Reserved
            && self.TimerInterval == other.TimerInterval
            && self.TargetProcessors == other.TargetProcessors
            && self.PStateHandler == other.PStateHandler
            && self.PStateContext == other.PStateContext
            && self.TStateHandler == other.TStateHandler
            && self.TStateContext == other.TStateContext
            && self.FeedbackHandler == other.FeedbackHandler
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.State == other.State
    }
}
impl ::core::cmp::Eq for PPM_WMI_PERF_STATES {}
impl ::core::fmt::Debug for PPM_WMI_PERF_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_WMI_PERF_STATES")
            .field("Count", &self.Count)
            .field("MaxFrequency", &self.MaxFrequency)
            .field("CurrentState", &self.CurrentState)
            .field("MaxPerfState", &self.MaxPerfState)
            .field("MinPerfState", &self.MinPerfState)
            .field("LowestPerfState", &self.LowestPerfState)
            .field("ThermalConstraint", &self.ThermalConstraint)
            .field("BusyAdjThreshold", &self.BusyAdjThreshold)
            .field("PolicyType", &self.PolicyType)
            .field("Type", &self.Type)
            .field("Reserved", &self.Reserved)
            .field("TimerInterval", &self.TimerInterval)
            .field("TargetProcessors", &self.TargetProcessors)
            .field("PStateHandler", &self.PStateHandler)
            .field("PStateContext", &self.PStateContext)
            .field("TStateHandler", &self.TStateHandler)
            .field("TStateContext", &self.TStateContext)
            .field("FeedbackHandler", &self.FeedbackHandler)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("State", &self.State)
            .finish()
    }
}
impl ::core::default::Default for PPM_WMI_PERF_STATES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPM_WMI_PERF_STATES_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count
            && self.MaxFrequency == other.MaxFrequency
            && self.CurrentState == other.CurrentState
            && self.MaxPerfState == other.MaxPerfState
            && self.MinPerfState == other.MinPerfState
            && self.LowestPerfState == other.LowestPerfState
            && self.ThermalConstraint == other.ThermalConstraint
            && self.BusyAdjThreshold == other.BusyAdjThreshold
            && self.PolicyType == other.PolicyType
            && self.Type == other.Type
            && self.Reserved == other.Reserved
            && self.TimerInterval == other.TimerInterval
            && self.TargetProcessors == other.TargetProcessors
            && self.PStateHandler == other.PStateHandler
            && self.PStateContext == other.PStateContext
            && self.TStateHandler == other.TStateHandler
            && self.TStateContext == other.TStateContext
            && self.FeedbackHandler == other.FeedbackHandler
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.State == other.State
    }
}
impl ::core::cmp::Eq for PPM_WMI_PERF_STATES_EX {}
impl ::core::fmt::Debug for PPM_WMI_PERF_STATES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPM_WMI_PERF_STATES_EX")
            .field("Count", &self.Count)
            .field("MaxFrequency", &self.MaxFrequency)
            .field("CurrentState", &self.CurrentState)
            .field("MaxPerfState", &self.MaxPerfState)
            .field("MinPerfState", &self.MinPerfState)
            .field("LowestPerfState", &self.LowestPerfState)
            .field("ThermalConstraint", &self.ThermalConstraint)
            .field("BusyAdjThreshold", &self.BusyAdjThreshold)
            .field("PolicyType", &self.PolicyType)
            .field("Type", &self.Type)
            .field("Reserved", &self.Reserved)
            .field("TimerInterval", &self.TimerInterval)
            .field("TargetProcessors", &self.TargetProcessors)
            .field("PStateHandler", &self.PStateHandler)
            .field("PStateContext", &self.PStateContext)
            .field("TStateHandler", &self.TStateHandler)
            .field("TStateContext", &self.TStateContext)
            .field("FeedbackHandler", &self.FeedbackHandler)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("State", &self.State)
            .finish()
    }
}
impl ::core::default::Default for PROCESSOR_IDLESTATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_IDLESTATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TimeCheck == other.TimeCheck && self.DemotePercent == other.DemotePercent && self.PromotePercent == other.PromotePercent && self.Spare == other.Spare
    }
}
impl ::core::cmp::Eq for PROCESSOR_IDLESTATE_INFO {}
impl ::core::fmt::Debug for PROCESSOR_IDLESTATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_IDLESTATE_INFO").field("TimeCheck", &self.TimeCheck).field("DemotePercent", &self.DemotePercent).field("PromotePercent", &self.PromotePercent).field("Spare", &self.Spare).finish()
    }
}
impl ::core::default::Default for PROCESSOR_IDLESTATE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESSOR_PERFSTATE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_ASLR_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_MITIGATION_DEP_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for QUOTA_LIMITS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RATE_QUOTA_LIMIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REARRANGE_FILE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REARRANGE_FILE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.SourceStartingOffset == other.SourceStartingOffset && self.TargetOffset == other.TargetOffset && self.SourceFileHandle == other.SourceFileHandle && self.Length == other.Length && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REARRANGE_FILE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REARRANGE_FILE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REARRANGE_FILE_DATA").field("SourceStartingOffset", &self.SourceStartingOffset).field("TargetOffset", &self.TargetOffset).field("SourceFileHandle", &self.SourceFileHandle).field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for REARRANGE_FILE_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for REARRANGE_FILE_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        self.SourceStartingOffset == other.SourceStartingOffset && self.TargetOffset == other.TargetOffset && self.SourceFileHandle == other.SourceFileHandle && self.Length == other.Length && self.Flags == other.Flags
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for REARRANGE_FILE_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for REARRANGE_FILE_DATA32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REARRANGE_FILE_DATA32").field("SourceStartingOffset", &self.SourceStartingOffset).field("TargetOffset", &self.TargetOffset).field("SourceFileHandle", &self.SourceFileHandle).field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for RECO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RECO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RECO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RECO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RECO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RECO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RECO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RECO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Accurate == other.Accurate && self.Supported == other.Supported && self.AccurateMask0 == other.AccurateMask0
    }
}
impl ::core::cmp::Eq for REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {}
impl ::core::fmt::Debug for REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO").field("Version", &self.Version).field("Accurate", &self.Accurate).field("Supported", &self.Supported).field("AccurateMask0", &self.AccurateMask0).finish()
    }
}
impl ::core::default::Default for RESOURCEMANAGER_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESOURCEMANAGER_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ResourceManagerId == other.ResourceManagerId && self.DescriptionLength == other.DescriptionLength && self.Description == other.Description
    }
}
impl ::core::cmp::Eq for RESOURCEMANAGER_BASIC_INFORMATION {}
impl ::core::fmt::Debug for RESOURCEMANAGER_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESOURCEMANAGER_BASIC_INFORMATION").field("ResourceManagerId", &self.ResourceManagerId).field("DescriptionLength", &self.DescriptionLength).field("Description", &self.Description).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESOURCEMANAGER_COMPLETION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESOURCEMANAGER_COMPLETION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IoCompletionPortHandle == other.IoCompletionPortHandle && self.CompletionKey == other.CompletionKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESOURCEMANAGER_COMPLETION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESOURCEMANAGER_COMPLETION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESOURCEMANAGER_COMPLETION_INFORMATION").field("IoCompletionPortHandle", &self.IoCompletionPortHandle).field("CompletionKey", &self.CompletionKey).finish()
    }
}
impl ::core::default::Default for RESOURCEMANAGER_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESOURCEMANAGER_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESOURCEMANAGER_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RESUME_PERFORMANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESUME_PERFORMANCE {
    fn eq(&self, other: &Self) -> bool {
        self.PostTimeMs == other.PostTimeMs && self.TotalResumeTimeMs == other.TotalResumeTimeMs && self.ResumeCompleteTimestamp == other.ResumeCompleteTimestamp
    }
}
impl ::core::cmp::Eq for RESUME_PERFORMANCE {}
impl ::core::fmt::Debug for RESUME_PERFORMANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESUME_PERFORMANCE").field("PostTimeMs", &self.PostTimeMs).field("TotalResumeTimeMs", &self.TotalResumeTimeMs).field("ResumeCompleteTimestamp", &self.ResumeCompleteTimestamp).finish()
    }
}
impl ::core::default::Default for RTL_UMS_SCHEDULER_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTL_UMS_SCHEDULER_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_UMS_SCHEDULER_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for RemHBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RemHBITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemHBITMAP {}
impl ::core::fmt::Debug for RemHBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemHBITMAP").field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for RemHBRUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RemHBRUSH {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemHBRUSH {}
impl ::core::fmt::Debug for RemHBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemHBRUSH").field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for RemHENHMETAFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RemHENHMETAFILE {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemHENHMETAFILE {}
impl ::core::fmt::Debug for RemHENHMETAFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemHENHMETAFILE").field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for RemHGLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RemHGLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.fNullHGlobal == other.fNullHGlobal && self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemHGLOBAL {}
impl ::core::fmt::Debug for RemHGLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemHGLOBAL").field("fNullHGlobal", &self.fNullHGlobal).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for RemHMETAFILEPICT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RemHMETAFILEPICT {
    fn eq(&self, other: &Self) -> bool {
        self.mm == other.mm && self.xExt == other.xExt && self.yExt == other.yExt && self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemHMETAFILEPICT {}
impl ::core::fmt::Debug for RemHMETAFILEPICT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemHMETAFILEPICT").field("mm", &self.mm).field("xExt", &self.xExt).field("yExt", &self.yExt).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for RemHPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RemHPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemHPALETTE {}
impl ::core::fmt::Debug for RemHPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemHPALETTE").field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for RemotableHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ReplacesCorHdrNumericDefines {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ReplacesCorHdrNumericDefines {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReplacesCorHdrNumericDefines").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCOPE_TABLE_AMD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_TABLE_AMD64 {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.ScopeRecord == other.ScopeRecord
    }
}
impl ::core::cmp::Eq for SCOPE_TABLE_AMD64 {}
impl ::core::fmt::Debug for SCOPE_TABLE_AMD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_TABLE_AMD64").field("Count", &self.Count).field("ScopeRecord", &self.ScopeRecord).finish()
    }
}
impl ::core::default::Default for SCOPE_TABLE_AMD64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_TABLE_AMD64_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BeginAddress == other.BeginAddress && self.EndAddress == other.EndAddress && self.HandlerAddress == other.HandlerAddress && self.JumpTarget == other.JumpTarget
    }
}
impl ::core::cmp::Eq for SCOPE_TABLE_AMD64_0 {}
impl ::core::fmt::Debug for SCOPE_TABLE_AMD64_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_TABLE_AMD64_0").field("BeginAddress", &self.BeginAddress).field("EndAddress", &self.EndAddress).field("HandlerAddress", &self.HandlerAddress).field("JumpTarget", &self.JumpTarget).finish()
    }
}
impl ::core::default::Default for SCOPE_TABLE_ARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_TABLE_ARM {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.ScopeRecord == other.ScopeRecord
    }
}
impl ::core::cmp::Eq for SCOPE_TABLE_ARM {}
impl ::core::fmt::Debug for SCOPE_TABLE_ARM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_TABLE_ARM").field("Count", &self.Count).field("ScopeRecord", &self.ScopeRecord).finish()
    }
}
impl ::core::default::Default for SCOPE_TABLE_ARM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_TABLE_ARM_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BeginAddress == other.BeginAddress && self.EndAddress == other.EndAddress && self.HandlerAddress == other.HandlerAddress && self.JumpTarget == other.JumpTarget
    }
}
impl ::core::cmp::Eq for SCOPE_TABLE_ARM_0 {}
impl ::core::fmt::Debug for SCOPE_TABLE_ARM_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_TABLE_ARM_0").field("BeginAddress", &self.BeginAddress).field("EndAddress", &self.EndAddress).field("HandlerAddress", &self.HandlerAddress).field("JumpTarget", &self.JumpTarget).finish()
    }
}
impl ::core::default::Default for SCOPE_TABLE_ARM64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_TABLE_ARM64 {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.ScopeRecord == other.ScopeRecord
    }
}
impl ::core::cmp::Eq for SCOPE_TABLE_ARM64 {}
impl ::core::fmt::Debug for SCOPE_TABLE_ARM64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_TABLE_ARM64").field("Count", &self.Count).field("ScopeRecord", &self.ScopeRecord).finish()
    }
}
impl ::core::default::Default for SCOPE_TABLE_ARM64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_TABLE_ARM64_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BeginAddress == other.BeginAddress && self.EndAddress == other.EndAddress && self.HandlerAddress == other.HandlerAddress && self.JumpTarget == other.JumpTarget
    }
}
impl ::core::cmp::Eq for SCOPE_TABLE_ARM64_0 {}
impl ::core::fmt::Debug for SCOPE_TABLE_ARM64_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_TABLE_ARM64_0").field("BeginAddress", &self.BeginAddress).field("EndAddress", &self.EndAddress).field("HandlerAddress", &self.HandlerAddress).field("JumpTarget", &self.JumpTarget).finish()
    }
}
impl ::core::default::Default for SCRUB_DATA_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRUB_DATA_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.MaximumIos == other.MaximumIos && self.ObjectId == other.ObjectId && self.Reserved == other.Reserved && self.ResumeContext == other.ResumeContext
    }
}
impl ::core::cmp::Eq for SCRUB_DATA_INPUT {}
impl ::core::fmt::Debug for SCRUB_DATA_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRUB_DATA_INPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("MaximumIos", &self.MaximumIos).field("ObjectId", &self.ObjectId).field("Reserved", &self.Reserved).field("ResumeContext", &self.ResumeContext).finish()
    }
}
impl ::core::default::Default for SCRUB_DATA_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRUB_DATA_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Flags == other.Flags
            && self.Status == other.Status
            && self.ErrorFileOffset == other.ErrorFileOffset
            && self.ErrorLength == other.ErrorLength
            && self.NumberOfBytesRepaired == other.NumberOfBytesRepaired
            && self.NumberOfBytesFailed == other.NumberOfBytesFailed
            && self.InternalFileReference == other.InternalFileReference
            && self.ResumeContextLength == other.ResumeContextLength
            && self.ParityExtentDataOffset == other.ParityExtentDataOffset
            && self.Reserved == other.Reserved
            && self.NumberOfMetadataBytesProcessed == other.NumberOfMetadataBytesProcessed
            && self.NumberOfDataBytesProcessed == other.NumberOfDataBytesProcessed
            && self.TotalNumberOfMetadataBytesInUse == other.TotalNumberOfMetadataBytesInUse
            && self.TotalNumberOfDataBytesInUse == other.TotalNumberOfDataBytesInUse
            && self.DataBytesSkippedDueToNoAllocation == other.DataBytesSkippedDueToNoAllocation
            && self.DataBytesSkippedDueToInvalidRun == other.DataBytesSkippedDueToInvalidRun
            && self.DataBytesSkippedDueToIntegrityStream == other.DataBytesSkippedDueToIntegrityStream
            && self.DataBytesSkippedDueToRegionBeingClean == other.DataBytesSkippedDueToRegionBeingClean
            && self.DataBytesSkippedDueToLockConflict == other.DataBytesSkippedDueToLockConflict
            && self.DataBytesSkippedDueToNoScrubDataFlag == other.DataBytesSkippedDueToNoScrubDataFlag
            && self.DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag == other.DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag
            && self.DataBytesScrubbed == other.DataBytesScrubbed
            && self.ResumeContext == other.ResumeContext
    }
}
impl ::core::cmp::Eq for SCRUB_DATA_OUTPUT {}
impl ::core::fmt::Debug for SCRUB_DATA_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRUB_DATA_OUTPUT")
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field("Status", &self.Status)
            .field("ErrorFileOffset", &self.ErrorFileOffset)
            .field("ErrorLength", &self.ErrorLength)
            .field("NumberOfBytesRepaired", &self.NumberOfBytesRepaired)
            .field("NumberOfBytesFailed", &self.NumberOfBytesFailed)
            .field("InternalFileReference", &self.InternalFileReference)
            .field("ResumeContextLength", &self.ResumeContextLength)
            .field("ParityExtentDataOffset", &self.ParityExtentDataOffset)
            .field("Reserved", &self.Reserved)
            .field("NumberOfMetadataBytesProcessed", &self.NumberOfMetadataBytesProcessed)
            .field("NumberOfDataBytesProcessed", &self.NumberOfDataBytesProcessed)
            .field("TotalNumberOfMetadataBytesInUse", &self.TotalNumberOfMetadataBytesInUse)
            .field("TotalNumberOfDataBytesInUse", &self.TotalNumberOfDataBytesInUse)
            .field("DataBytesSkippedDueToNoAllocation", &self.DataBytesSkippedDueToNoAllocation)
            .field("DataBytesSkippedDueToInvalidRun", &self.DataBytesSkippedDueToInvalidRun)
            .field("DataBytesSkippedDueToIntegrityStream", &self.DataBytesSkippedDueToIntegrityStream)
            .field("DataBytesSkippedDueToRegionBeingClean", &self.DataBytesSkippedDueToRegionBeingClean)
            .field("DataBytesSkippedDueToLockConflict", &self.DataBytesSkippedDueToLockConflict)
            .field("DataBytesSkippedDueToNoScrubDataFlag", &self.DataBytesSkippedDueToNoScrubDataFlag)
            .field("DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag", &self.DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag)
            .field("DataBytesScrubbed", &self.DataBytesScrubbed)
            .field("ResumeContext", &self.ResumeContext)
            .finish()
    }
}
impl ::core::default::Default for SCRUB_PARITY_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRUB_PARITY_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for SCRUB_PARITY_EXTENT {}
impl ::core::fmt::Debug for SCRUB_PARITY_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRUB_PARITY_EXTENT").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for SCRUB_PARITY_EXTENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRUB_PARITY_EXTENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.NumberOfParityExtents == other.NumberOfParityExtents && self.MaximumNumberOfParityExtents == other.MaximumNumberOfParityExtents && self.ParityExtents == other.ParityExtents
    }
}
impl ::core::cmp::Eq for SCRUB_PARITY_EXTENT_DATA {}
impl ::core::fmt::Debug for SCRUB_PARITY_EXTENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRUB_PARITY_EXTENT_DATA").field("Size", &self.Size).field("Flags", &self.Flags).field("NumberOfParityExtents", &self.NumberOfParityExtents).field("MaximumNumberOfParityExtents", &self.MaximumNumberOfParityExtents).field("ParityExtents", &self.ParityExtents).finish()
    }
}
impl ::core::default::Default for SECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SECURITY_OBJECT_AI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECURITY_OBJECT_AI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ConstraintMask == other.ConstraintMask
    }
}
impl ::core::cmp::Eq for SECURITY_OBJECT_AI_PARAMS {}
impl ::core::fmt::Debug for SECURITY_OBJECT_AI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_OBJECT_AI_PARAMS").field("Size", &self.Size).field("ConstraintMask", &self.ConstraintMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVERSILO_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVERSILO_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceSessionId == other.ServiceSessionId && self.State == other.State && self.ExitStatus == other.ExitStatus && self.IsDownlevelContainer == other.IsDownlevelContainer && self.ApiSetSchema == other.ApiSetSchema && self.HostApiSetSchema == other.HostApiSetSchema
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVERSILO_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVERSILO_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVERSILO_BASIC_INFORMATION").field("ServiceSessionId", &self.ServiceSessionId).field("State", &self.State).field("ExitStatus", &self.ExitStatus).field("IsDownlevelContainer", &self.IsDownlevelContainer).field("ApiSetSchema", &self.ApiSetSchema).field("HostApiSetSchema", &self.HostApiSetSchema).finish()
    }
}
impl ::core::default::Default for SERVERSILO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVERSILO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVERSILO_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SERVICE_ERROR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVICE_ERROR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_ERROR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SERVICE_LOAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVICE_LOAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_LOAD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SERVICE_NODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVICE_NODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_NODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SE_IMAGE_SIGNATURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SE_IMAGE_SIGNATURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SE_IMAGE_SIGNATURE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SE_LEARNING_MODE_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SE_LEARNING_MODE_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SE_LEARNING_MODE_DATA_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for SE_TOKEN_USER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SFGAO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SFGAO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SFGAO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SFGAO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SFGAO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SFGAO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SFGAO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SFGAO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHARED_VIRTUAL_DISK_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARED_VIRTUAL_DISK_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.SharedVirtualDiskSupport == other.SharedVirtualDiskSupport && self.HandleState == other.HandleState
    }
}
impl ::core::cmp::Eq for SHARED_VIRTUAL_DISK_SUPPORT {}
impl ::core::fmt::Debug for SHARED_VIRTUAL_DISK_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARED_VIRTUAL_DISK_SUPPORT").field("SharedVirtualDiskSupport", &self.SharedVirtualDiskSupport).field("HandleState", &self.HandleState).finish()
    }
}
impl ::core::default::Default for SHUFFLE_FILE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHUFFLE_FILE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.Length == other.Length && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SHUFFLE_FILE_DATA {}
impl ::core::fmt::Debug for SHUFFLE_FILE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHUFFLE_FILE_DATA").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SILOOBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SILOOBJECT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SiloId == other.SiloId && self.SiloParentId == other.SiloParentId && self.NumberOfProcesses == other.NumberOfProcesses && self.IsInServerSilo == other.IsInServerSilo && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SILOOBJECT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SILOOBJECT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SILOOBJECT_BASIC_INFORMATION").field("SiloId", &self.SiloId).field("SiloParentId", &self.SiloParentId).field("NumberOfProcesses", &self.NumberOfProcesses).field("IsInServerSilo", &self.IsInServerSilo).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for STATIC_STYLES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STATIC_STYLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATIC_STYLES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SUPPORTED_OS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SUPPORTED_OS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for SUPPORTED_OS_INFO {}
impl ::core::fmt::Debug for SUPPORTED_OS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SUPPORTED_OS_INFO").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::core::default::Default for SharedVirtualDiskHandleState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SharedVirtualDiskHandleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedVirtualDiskHandleState").field(&self.0).finish()
    }
}
impl ::core::default::Default for SharedVirtualDiskSupportType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SharedVirtualDiskSupportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedVirtualDiskSupportType").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAPE_CREATE_PARTITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPE_CREATE_PARTITION {
    fn eq(&self, other: &Self) -> bool {
        self.Method == other.Method && self.Count == other.Count && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for TAPE_CREATE_PARTITION {}
impl ::core::fmt::Debug for TAPE_CREATE_PARTITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_CREATE_PARTITION").field("Method", &self.Method).field("Count", &self.Count).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for TAPE_DRIVE_PROBLEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_DRIVE_PROBLEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_DRIVE_PROBLEM_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_GET_DRIVE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_GET_DRIVE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ECC == other.ECC && self.Compression == other.Compression && self.DataPadding == other.DataPadding && self.ReportSetmarks == other.ReportSetmarks && self.DefaultBlockSize == other.DefaultBlockSize && self.MaximumBlockSize == other.MaximumBlockSize && self.MinimumBlockSize == other.MinimumBlockSize && self.MaximumPartitionCount == other.MaximumPartitionCount && self.FeaturesLow == other.FeaturesLow && self.FeaturesHigh == other.FeaturesHigh && self.EOTWarningZoneSize == other.EOTWarningZoneSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_GET_DRIVE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_GET_DRIVE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_GET_DRIVE_PARAMETERS")
            .field("ECC", &self.ECC)
            .field("Compression", &self.Compression)
            .field("DataPadding", &self.DataPadding)
            .field("ReportSetmarks", &self.ReportSetmarks)
            .field("DefaultBlockSize", &self.DefaultBlockSize)
            .field("MaximumBlockSize", &self.MaximumBlockSize)
            .field("MinimumBlockSize", &self.MinimumBlockSize)
            .field("MaximumPartitionCount", &self.MaximumPartitionCount)
            .field("FeaturesLow", &self.FeaturesLow)
            .field("FeaturesHigh", &self.FeaturesHigh)
            .field("EOTWarningZoneSize", &self.EOTWarningZoneSize)
            .finish()
    }
}
impl ::core::default::Default for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_GET_MEDIA_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_GET_MEDIA_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Capacity == other.Capacity && self.Remaining == other.Remaining && self.BlockSize == other.BlockSize && self.PartitionCount == other.PartitionCount && self.WriteProtected == other.WriteProtected
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_GET_MEDIA_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_GET_MEDIA_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_GET_MEDIA_PARAMETERS").field("Capacity", &self.Capacity).field("Remaining", &self.Remaining).field("BlockSize", &self.BlockSize).field("PartitionCount", &self.PartitionCount).field("WriteProtected", &self.WriteProtected).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_SET_DRIVE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_SET_DRIVE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ECC == other.ECC && self.Compression == other.Compression && self.DataPadding == other.DataPadding && self.ReportSetmarks == other.ReportSetmarks && self.EOTWarningZoneSize == other.EOTWarningZoneSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_SET_DRIVE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_SET_DRIVE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_SET_DRIVE_PARAMETERS").field("ECC", &self.ECC).field("Compression", &self.Compression).field("DataPadding", &self.DataPadding).field("ReportSetmarks", &self.ReportSetmarks).field("EOTWarningZoneSize", &self.EOTWarningZoneSize).finish()
    }
}
impl ::core::default::Default for TAPE_SET_MEDIA_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPE_SET_MEDIA_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.BlockSize == other.BlockSize
    }
}
impl ::core::cmp::Eq for TAPE_SET_MEDIA_PARAMETERS {}
impl ::core::fmt::Debug for TAPE_SET_MEDIA_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_SET_MEDIA_PARAMETERS").field("BlockSize", &self.BlockSize).finish()
    }
}
impl ::core::default::Default for TAPE_WMI_OPERATIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPE_WMI_OPERATIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Method == other.Method && self.DataBufferSize == other.DataBufferSize && self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for TAPE_WMI_OPERATIONS {}
impl ::core::fmt::Debug for TAPE_WMI_OPERATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_WMI_OPERATIONS").field("Method", &self.Method).field("DataBufferSize", &self.DataBufferSize).field("DataBuffer", &self.DataBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_BNO_ISOLATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_BNO_ISOLATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IsolationPrefix == other.IsolationPrefix && self.IsolationEnabled == other.IsolationEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_BNO_ISOLATION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_BNO_ISOLATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_BNO_ISOLATION_INFORMATION").field("IsolationPrefix", &self.IsolationPrefix).field("IsolationEnabled", &self.IsolationEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_SID_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_SID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Sid == other.Sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_SID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_SID_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_SID_INFORMATION").field("Sid", &self.Sid).finish()
    }
}
impl ::core::default::Default for TRANSACTIONMANAGER_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTIONMANAGER_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TmIdentity == other.TmIdentity && self.VirtualClock == other.VirtualClock
    }
}
impl ::core::cmp::Eq for TRANSACTIONMANAGER_BASIC_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTIONMANAGER_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTIONMANAGER_BASIC_INFORMATION").field("TmIdentity", &self.TmIdentity).field("VirtualClock", &self.VirtualClock).finish()
    }
}
impl ::core::default::Default for TRANSACTIONMANAGER_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSACTIONMANAGER_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSACTIONMANAGER_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LogPathLength == other.LogPathLength && self.LogPath == other.LogPath
    }
}
impl ::core::cmp::Eq for TRANSACTIONMANAGER_LOGPATH_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTIONMANAGER_LOGPATH_INFORMATION").field("LogPathLength", &self.LogPathLength).field("LogPath", &self.LogPath).finish()
    }
}
impl ::core::default::Default for TRANSACTIONMANAGER_LOG_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTIONMANAGER_LOG_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LogIdentity == other.LogIdentity
    }
}
impl ::core::cmp::Eq for TRANSACTIONMANAGER_LOG_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTIONMANAGER_LOG_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTIONMANAGER_LOG_INFORMATION").field("LogIdentity", &self.LogIdentity).finish()
    }
}
impl ::core::default::Default for TRANSACTIONMANAGER_OLDEST_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTIONMANAGER_OLDEST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.OldestTransactionGuid == other.OldestTransactionGuid
    }
}
impl ::core::cmp::Eq for TRANSACTIONMANAGER_OLDEST_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTIONMANAGER_OLDEST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTIONMANAGER_OLDEST_INFORMATION").field("OldestTransactionGuid", &self.OldestTransactionGuid).finish()
    }
}
impl ::core::default::Default for TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LastRecoveredLsn == other.LastRecoveredLsn
    }
}
impl ::core::cmp::Eq for TRANSACTIONMANAGER_RECOVERY_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTIONMANAGER_RECOVERY_INFORMATION").field("LastRecoveredLsn", &self.LastRecoveredLsn).finish()
    }
}
impl ::core::default::Default for TRANSACTION_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TransactionId == other.TransactionId && self.State == other.State && self.Outcome == other.Outcome
    }
}
impl ::core::cmp::Eq for TRANSACTION_BASIC_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTION_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_BASIC_INFORMATION").field("TransactionId", &self.TransactionId).field("State", &self.State).field("Outcome", &self.Outcome).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSACTION_BIND_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSACTION_BIND_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TmHandle == other.TmHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSACTION_BIND_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSACTION_BIND_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_BIND_INFORMATION").field("TmHandle", &self.TmHandle).finish()
    }
}
impl ::core::default::Default for TRANSACTION_ENLISTMENTS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_ENLISTMENTS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfEnlistments == other.NumberOfEnlistments && self.EnlistmentPair == other.EnlistmentPair
    }
}
impl ::core::cmp::Eq for TRANSACTION_ENLISTMENTS_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTION_ENLISTMENTS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_ENLISTMENTS_INFORMATION").field("NumberOfEnlistments", &self.NumberOfEnlistments).field("EnlistmentPair", &self.EnlistmentPair).finish()
    }
}
impl ::core::default::Default for TRANSACTION_ENLISTMENT_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_ENLISTMENT_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.EnlistmentId == other.EnlistmentId && self.ResourceManagerId == other.ResourceManagerId
    }
}
impl ::core::cmp::Eq for TRANSACTION_ENLISTMENT_PAIR {}
impl ::core::fmt::Debug for TRANSACTION_ENLISTMENT_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_ENLISTMENT_PAIR").field("EnlistmentId", &self.EnlistmentId).field("ResourceManagerId", &self.ResourceManagerId).finish()
    }
}
impl ::core::default::Default for TRANSACTION_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSACTION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSACTION_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSACTION_LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.UOW == other.UOW
    }
}
impl ::core::cmp::Eq for TRANSACTION_LIST_ENTRY {}
impl ::core::fmt::Debug for TRANSACTION_LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_LIST_ENTRY").field("UOW", &self.UOW).finish()
    }
}
impl ::core::default::Default for TRANSACTION_LIST_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_LIST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfTransactions == other.NumberOfTransactions && self.TransactionInformation == other.TransactionInformation
    }
}
impl ::core::cmp::Eq for TRANSACTION_LIST_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTION_LIST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_LIST_INFORMATION").field("NumberOfTransactions", &self.NumberOfTransactions).field("TransactionInformation", &self.TransactionInformation).finish()
    }
}
impl ::core::default::Default for TRANSACTION_PROPERTIES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_PROPERTIES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IsolationLevel == other.IsolationLevel && self.IsolationFlags == other.IsolationFlags && self.Timeout == other.Timeout && self.Outcome == other.Outcome && self.DescriptionLength == other.DescriptionLength && self.Description == other.Description
    }
}
impl ::core::cmp::Eq for TRANSACTION_PROPERTIES_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTION_PROPERTIES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_PROPERTIES_INFORMATION").field("IsolationLevel", &self.IsolationLevel).field("IsolationFlags", &self.IsolationFlags).field("Timeout", &self.Timeout).field("Outcome", &self.Outcome).field("DescriptionLength", &self.DescriptionLength).field("Description", &self.Description).finish()
    }
}
impl ::core::default::Default for TRANSACTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSACTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSACTION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SuperiorEnlistmentPair == other.SuperiorEnlistmentPair
    }
}
impl ::core::cmp::Eq for TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {}
impl ::core::fmt::Debug for TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION").field("SuperiorEnlistmentPair", &self.SuperiorEnlistmentPair).finish()
    }
}
impl ::core::default::Default for UMS_CREATE_THREAD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UMS_CREATE_THREAD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.UmsVersion == other.UmsVersion && self.UmsContext == other.UmsContext && self.UmsCompletionList == other.UmsCompletionList
    }
}
impl ::core::cmp::Eq for UMS_CREATE_THREAD_ATTRIBUTES {}
impl ::core::fmt::Debug for UMS_CREATE_THREAD_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UMS_CREATE_THREAD_ATTRIBUTES").field("UmsVersion", &self.UmsVersion).field("UmsContext", &self.UmsContext).field("UmsCompletionList", &self.UmsCompletionList).finish()
    }
}
impl ::core::default::Default for USER_ACTIVITY_PRESENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USER_ACTIVITY_PRESENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_ACTIVITY_PRESENCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VolLockBroadcast {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VolLockBroadcast {
    fn eq(&self, other: &Self) -> bool {
        self.vlb_dbh == other.vlb_dbh && self.vlb_owner == other.vlb_owner && self.vlb_perms == other.vlb_perms && self.vlb_lockType == other.vlb_lockType && self.vlb_drive == other.vlb_drive && self.vlb_flags == other.vlb_flags
    }
}
impl ::core::cmp::Eq for VolLockBroadcast {}
impl ::core::fmt::Debug for VolLockBroadcast {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VolLockBroadcast").field("vlb_dbh", &self.vlb_dbh).field("vlb_owner", &self.vlb_owner).field("vlb_perms", &self.vlb_perms).field("vlb_lockType", &self.vlb_lockType).field("vlb_drive", &self.vlb_drive).field("vlb_flags", &self.vlb_flags).finish()
    }
}
impl ::core::default::Default for XSAVE_CET_U_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XSAVE_CET_U_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Ia32CetUMsr == other.Ia32CetUMsr && self.Ia32Pl3SspMsr == other.Ia32Pl3SspMsr
    }
}
impl ::core::cmp::Eq for XSAVE_CET_U_FORMAT {}
impl ::core::fmt::Debug for XSAVE_CET_U_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_CET_U_FORMAT").field("Ia32CetUMsr", &self.Ia32CetUMsr).field("Ia32Pl3SspMsr", &self.Ia32Pl3SspMsr).finish()
    }
}
impl ::core::default::Default for _DEV_BROADCAST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _DEV_BROADCAST_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dbcd_size == other.dbcd_size && self.dbcd_devicetype == other.dbcd_devicetype && self.dbcd_reserved == other.dbcd_reserved
    }
}
impl ::core::cmp::Eq for _DEV_BROADCAST_HEADER {}
impl ::core::fmt::Debug for _DEV_BROADCAST_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_DEV_BROADCAST_HEADER").field("dbcd_size", &self.dbcd_size).field("dbcd_devicetype", &self.dbcd_devicetype).field("dbcd_reserved", &self.dbcd_reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _DEV_BROADCAST_USERDEFINED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _DEV_BROADCAST_USERDEFINED {
    fn eq(&self, other: &Self) -> bool {
        self.dbud_dbh == other.dbud_dbh && self.dbud_szName == other.dbud_szName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _DEV_BROADCAST_USERDEFINED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _DEV_BROADCAST_USERDEFINED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_DEV_BROADCAST_USERDEFINED").field("dbud_dbh", &self.dbud_dbh).field("dbud_szName", &self.dbud_szName).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for remoteMETAFILEPICT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for remoteMETAFILEPICT {
    fn eq(&self, other: &Self) -> bool {
        self.mm == other.mm && self.xExt == other.xExt && self.yExt == other.yExt && self.hMF == other.hMF
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for remoteMETAFILEPICT {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for remoteMETAFILEPICT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("remoteMETAFILEPICT").field("mm", &self.mm).field("xExt", &self.xExt).field("yExt", &self.yExt).field("hMF", &self.hMF).finish()
    }
}
impl ::core::default::Default for userBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for userBITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.bmType == other.bmType && self.bmWidth == other.bmWidth && self.bmHeight == other.bmHeight && self.bmWidthBytes == other.bmWidthBytes && self.bmPlanes == other.bmPlanes && self.bmBitsPixel == other.bmBitsPixel && self.cbSize == other.cbSize && self.pBuffer == other.pBuffer
    }
}
impl ::core::cmp::Eq for userBITMAP {}
impl ::core::fmt::Debug for userBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userBITMAP").field("bmType", &self.bmType).field("bmWidth", &self.bmWidth).field("bmHeight", &self.bmHeight).field("bmWidthBytes", &self.bmWidthBytes).field("bmPlanes", &self.bmPlanes).field("bmBitsPixel", &self.bmBitsPixel).field("cbSize", &self.cbSize).field("pBuffer", &self.pBuffer).finish()
    }
}
impl ::core::default::Default for userCLIPFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for userHBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for userHENHMETAFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for userHGLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for userHMETAFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for userHMETAFILEPICT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for userHPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
