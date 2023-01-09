#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONNECTDLGSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONNECTDLGSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure && self.hwndOwner == other.hwndOwner && self.lpConnRes == other.lpConnRes && self.dwFlags == other.dwFlags && self.dwDevNum == other.dwDevNum
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONNECTDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONNECTDLGSTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDLGSTRUCTA").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpConnRes", &self.lpConnRes).field("dwFlags", &self.dwFlags).field("dwDevNum", &self.dwDevNum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONNECTDLGSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONNECTDLGSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure && self.hwndOwner == other.hwndOwner && self.lpConnRes == other.lpConnRes && self.dwFlags == other.dwFlags && self.dwDevNum == other.dwDevNum
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONNECTDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONNECTDLGSTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDLGSTRUCTW").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpConnRes", &self.lpConnRes).field("dwFlags", &self.dwFlags).field("dwDevNum", &self.dwDevNum).finish()
    }
}
impl ::core::default::Default for CONNECTDLGSTRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONNECTDLGSTRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONNECTDLGSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISCDLGSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISCDLGSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure && self.hwndOwner == other.hwndOwner && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISCDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISCDLGSTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISCDLGSTRUCTA").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISCDLGSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISCDLGSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure && self.hwndOwner == other.hwndOwner && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISCDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISCDLGSTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISCDLGSTRUCTW").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DISCDLGSTRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISCDLGSTRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISCDLGSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DISCDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DISCDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NETCONNECTINFOSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETCONNECTINFOSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure && self.dwFlags == other.dwFlags && self.dwSpeed == other.dwSpeed && self.dwDelay == other.dwDelay && self.dwOptDataSize == other.dwOptDataSize
    }
}
impl ::core::cmp::Eq for NETCONNECTINFOSTRUCT {}
impl ::core::fmt::Debug for NETCONNECTINFOSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETCONNECTINFOSTRUCT").field("cbStructure", &self.cbStructure).field("dwFlags", &self.dwFlags).field("dwSpeed", &self.dwSpeed).field("dwDelay", &self.dwDelay).field("dwOptDataSize", &self.dwOptDataSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETINFOSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETINFOSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure && self.dwProviderVersion == other.dwProviderVersion && self.dwStatus == other.dwStatus && self.dwCharacteristics == other.dwCharacteristics && self.dwHandle == other.dwHandle && self.wNetType == other.wNetType && self.dwPrinters == other.dwPrinters && self.dwDrives == other.dwDrives
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETINFOSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NETINFOSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETINFOSTRUCT").field("cbStructure", &self.cbStructure).field("dwProviderVersion", &self.dwProviderVersion).field("dwStatus", &self.dwStatus).field("dwCharacteristics", &self.dwCharacteristics).field("dwHandle", &self.dwHandle).field("wNetType", &self.wNetType).field("dwPrinters", &self.dwPrinters).field("dwDrives", &self.dwDrives).finish()
    }
}
impl ::core::default::Default for NETINFOSTRUCT_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETINFOSTRUCT_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETINFOSTRUCT_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NETRESOURCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETRESOURCEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope && self.dwType == other.dwType && self.dwDisplayType == other.dwDisplayType && self.dwUsage == other.dwUsage && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.lpComment == other.lpComment && self.lpProvider == other.lpProvider
    }
}
impl ::core::cmp::Eq for NETRESOURCEA {}
impl ::core::fmt::Debug for NETRESOURCEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCEA").field("dwScope", &self.dwScope).field("dwType", &self.dwType).field("dwDisplayType", &self.dwDisplayType).field("dwUsage", &self.dwUsage).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("lpComment", &self.lpComment).field("lpProvider", &self.lpProvider).finish()
    }
}
impl ::core::default::Default for NETRESOURCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETRESOURCEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope && self.dwType == other.dwType && self.dwDisplayType == other.dwDisplayType && self.dwUsage == other.dwUsage && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.lpComment == other.lpComment && self.lpProvider == other.lpProvider
    }
}
impl ::core::cmp::Eq for NETRESOURCEW {}
impl ::core::fmt::Debug for NETRESOURCEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCEW").field("dwScope", &self.dwScope).field("dwType", &self.dwType).field("dwDisplayType", &self.dwDisplayType).field("dwUsage", &self.dwUsage).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("lpComment", &self.lpComment).field("lpProvider", &self.lpProvider).finish()
    }
}
impl ::core::default::Default for NETWORK_NAME_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETWORK_NAME_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETWORK_NAME_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_RESOURCE_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_RESOURCE_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_RESOURCE_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_RESOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_RESOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_RESOURCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_RESOURCE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_RESOURCE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_RESOURCE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NET_USE_CONNECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_USE_CONNECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_USE_CONNECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_USE_CONNECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_USE_CONNECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFYADD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NOTIFYADD {
    fn eq(&self, other: &Self) -> bool {
        self.hwndOwner == other.hwndOwner && self.NetResource == other.NetResource && self.dwAddFlags == other.dwAddFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NOTIFYADD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NOTIFYADD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFYADD").field("hwndOwner", &self.hwndOwner).field("NetResource", &self.NetResource).field("dwAddFlags", &self.dwAddFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFYCANCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NOTIFYCANCEL {
    fn eq(&self, other: &Self) -> bool {
        self.lpName == other.lpName && self.lpProvider == other.lpProvider && self.dwFlags == other.dwFlags && self.fForce == other.fForce
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NOTIFYCANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NOTIFYCANCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFYCANCEL").field("lpName", &self.lpName).field("lpProvider", &self.lpProvider).field("dwFlags", &self.dwFlags).field("fForce", &self.fForce).finish()
    }
}
impl ::core::default::Default for NOTIFYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NOTIFYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNotifyStatus == other.dwNotifyStatus && self.dwOperationStatus == other.dwOperationStatus && self.lpContext == other.lpContext
    }
}
impl ::core::cmp::Eq for NOTIFYINFO {}
impl ::core::fmt::Debug for NOTIFYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFYINFO").field("dwNotifyStatus", &self.dwNotifyStatus).field("dwOperationStatus", &self.dwOperationStatus).field("lpContext", &self.lpContext).finish()
    }
}
impl ::core::default::Default for NPDIRECTORY_NOTIFY_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NPDIRECTORY_NOTIFY_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NPDIRECTORY_NOTIFY_OPERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for NP_PROPERTY_DIALOG_SELECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NP_PROPERTY_DIALOG_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NP_PROPERTY_DIALOG_SELECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for REMOTE_NAME_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REMOTE_NAME_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName && self.lpConnectionName == other.lpConnectionName && self.lpRemainingPath == other.lpRemainingPath
    }
}
impl ::core::cmp::Eq for REMOTE_NAME_INFOA {}
impl ::core::fmt::Debug for REMOTE_NAME_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMOTE_NAME_INFOA").field("lpUniversalName", &self.lpUniversalName).field("lpConnectionName", &self.lpConnectionName).field("lpRemainingPath", &self.lpRemainingPath).finish()
    }
}
impl ::core::default::Default for REMOTE_NAME_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REMOTE_NAME_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName && self.lpConnectionName == other.lpConnectionName && self.lpRemainingPath == other.lpRemainingPath
    }
}
impl ::core::cmp::Eq for REMOTE_NAME_INFOW {}
impl ::core::fmt::Debug for REMOTE_NAME_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMOTE_NAME_INFOW").field("lpUniversalName", &self.lpUniversalName).field("lpConnectionName", &self.lpConnectionName).field("lpRemainingPath", &self.lpRemainingPath).finish()
    }
}
impl ::core::default::Default for UNC_INFO_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNC_INFO_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNC_INFO_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNIVERSAL_NAME_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNIVERSAL_NAME_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName
    }
}
impl ::core::cmp::Eq for UNIVERSAL_NAME_INFOA {}
impl ::core::fmt::Debug for UNIVERSAL_NAME_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNIVERSAL_NAME_INFOA").field("lpUniversalName", &self.lpUniversalName).finish()
    }
}
impl ::core::default::Default for UNIVERSAL_NAME_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNIVERSAL_NAME_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName
    }
}
impl ::core::cmp::Eq for UNIVERSAL_NAME_INFOW {}
impl ::core::fmt::Debug for UNIVERSAL_NAME_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNIVERSAL_NAME_INFOW").field("lpUniversalName", &self.lpUniversalName).finish()
    }
}
impl ::core::default::Default for WNET_OPEN_ENUM_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WNET_OPEN_ENUM_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNET_OPEN_ENUM_USAGE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WNET_OPEN_ENUM_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WNET_OPEN_ENUM_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WNPERM_DLG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WNPERM_DLG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNPERM_DLG").field(&self.0).finish()
    }
}
