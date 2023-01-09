impl ::core::cmp::PartialEq for IWsbApplicationAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWsbApplicationAsync {}
impl ::core::fmt::Debug for IWsbApplicationAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWsbApplicationAsync").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWsbApplicationBackupSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWsbApplicationBackupSupport {}
impl ::core::fmt::Debug for IWsbApplicationBackupSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWsbApplicationBackupSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWsbApplicationRestoreSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWsbApplicationRestoreSupport {}
impl ::core::fmt::Debug for IWsbApplicationRestoreSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWsbApplicationRestoreSupport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSB_OB_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSB_OB_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.m_wszResourceDLL == other.m_wszResourceDLL && self.m_guidSnapinId == other.m_guidSnapinId && self.m_dwProviderName == other.m_dwProviderName && self.m_dwProviderIcon == other.m_dwProviderIcon && self.m_bSupportsRemoting == other.m_bSupportsRemoting
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSB_OB_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSB_OB_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_REGISTRATION_INFO").field("m_wszResourceDLL", &self.m_wszResourceDLL).field("m_guidSnapinId", &self.m_guidSnapinId).field("m_dwProviderName", &self.m_dwProviderName).field("m_dwProviderIcon", &self.m_dwProviderIcon).field("m_bSupportsRemoting", &self.m_bSupportsRemoting).finish()
    }
}
impl ::core::default::Default for WSB_OB_STATUS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSB_OB_STATUS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwIcon == other.m_dwIcon && self.m_dwStatusEntryName == other.m_dwStatusEntryName && self.m_dwStatusEntryValue == other.m_dwStatusEntryValue && self.m_cValueTypePair == other.m_cValueTypePair && self.m_rgValueTypePair == other.m_rgValueTypePair
    }
}
impl ::core::cmp::Eq for WSB_OB_STATUS_ENTRY {}
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_STATUS_ENTRY").field("m_dwIcon", &self.m_dwIcon).field("m_dwStatusEntryName", &self.m_dwStatusEntryName).field("m_dwStatusEntryValue", &self.m_dwStatusEntryValue).field("m_cValueTypePair", &self.m_cValueTypePair).field("m_rgValueTypePair", &self.m_rgValueTypePair).finish()
    }
}
impl ::core::default::Default for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSB_OB_STATUS_ENTRY_PAIR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.m_wszObStatusEntryPairValue == other.m_wszObStatusEntryPairValue && self.m_ObStatusEntryPairType == other.m_ObStatusEntryPairType
    }
}
impl ::core::cmp::Eq for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {}
impl ::core::fmt::Debug for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR").field("m_wszObStatusEntryPairValue", &self.m_wszObStatusEntryPairValue).field("m_ObStatusEntryPairType", &self.m_ObStatusEntryPairType).finish()
    }
}
impl ::core::default::Default for WSB_OB_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSB_OB_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.m_guidSnapinId == other.m_guidSnapinId && self.m_cStatusEntry == other.m_cStatusEntry && self.m_rgStatusEntry == other.m_rgStatusEntry
    }
}
impl ::core::cmp::Eq for WSB_OB_STATUS_INFO {}
impl ::core::fmt::Debug for WSB_OB_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSB_OB_STATUS_INFO").field("m_guidSnapinId", &self.m_guidSnapinId).field("m_cStatusEntry", &self.m_cStatusEntry).field("m_rgStatusEntry", &self.m_rgStatusEntry).finish()
    }
}
