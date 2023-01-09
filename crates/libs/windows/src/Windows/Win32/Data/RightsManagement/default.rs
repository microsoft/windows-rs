impl ::core::default::Default for DRMATTESTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRMATTESTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMATTESTTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRMBOUNDLICENSEPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRMBOUNDLICENSEPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion && self.hEnablingPrincipal == other.hEnablingPrincipal && self.hSecureStore == other.hSecureStore && self.wszRightsRequested == other.wszRightsRequested && self.wszRightsGroup == other.wszRightsGroup && self.idResource == other.idResource && self.cAuthenticatorCount == other.cAuthenticatorCount && self.rghAuthenticators == other.rghAuthenticators && self.wszDefaultEnablingPrincipalCredentials == other.wszDefaultEnablingPrincipalCredentials && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DRMBOUNDLICENSEPARAMS {}
impl ::core::fmt::Debug for DRMBOUNDLICENSEPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRMBOUNDLICENSEPARAMS")
            .field("uVersion", &self.uVersion)
            .field("hEnablingPrincipal", &self.hEnablingPrincipal)
            .field("hSecureStore", &self.hSecureStore)
            .field("wszRightsRequested", &self.wszRightsRequested)
            .field("wszRightsGroup", &self.wszRightsGroup)
            .field("idResource", &self.idResource)
            .field("cAuthenticatorCount", &self.cAuthenticatorCount)
            .field("rghAuthenticators", &self.rghAuthenticators)
            .field("wszDefaultEnablingPrincipalCredentials", &self.wszDefaultEnablingPrincipalCredentials)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::default::Default for DRMENCODINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRMENCODINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMENCODINGTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRMGLOBALOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRMGLOBALOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMGLOBALOPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRMID {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion && self.wszIDType == other.wszIDType && self.wszID == other.wszID
    }
}
impl ::core::cmp::Eq for DRMID {}
impl ::core::fmt::Debug for DRMID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRMID").field("uVersion", &self.uVersion).field("wszIDType", &self.wszIDType).field("wszID", &self.wszID).finish()
    }
}
impl ::core::default::Default for DRMSECURITYPROVIDERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRMSECURITYPROVIDERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMSECURITYPROVIDERTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRMSPECTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRMSPECTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMSPECTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRMTIMETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRMTIMETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMTIMETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRM_ACTSERV_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_ACTSERV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion && self.wszPubKey == other.wszPubKey && self.wszURL == other.wszURL
    }
}
impl ::core::cmp::Eq for DRM_ACTSERV_INFO {}
impl ::core::fmt::Debug for DRM_ACTSERV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_ACTSERV_INFO").field("uVersion", &self.uVersion).field("wszPubKey", &self.wszPubKey).field("wszURL", &self.wszURL).finish()
    }
}
impl ::core::default::Default for DRM_CLIENT_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_CLIENT_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uStructVersion == other.uStructVersion && self.dwVersion == other.dwVersion && self.wszHierarchy == other.wszHierarchy && self.wszProductId == other.wszProductId && self.wszProductDescription == other.wszProductDescription
    }
}
impl ::core::cmp::Eq for DRM_CLIENT_VERSION_INFO {}
impl ::core::fmt::Debug for DRM_CLIENT_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_CLIENT_VERSION_INFO").field("uStructVersion", &self.uStructVersion).field("dwVersion", &self.dwVersion).field("wszHierarchy", &self.wszHierarchy).field("wszProductId", &self.wszProductId).field("wszProductDescription", &self.wszProductDescription).finish()
    }
}
impl ::core::default::Default for DRM_DISTRIBUTION_POINT_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRM_DISTRIBUTION_POINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_DISTRIBUTION_POINT_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRM_LICENSE_ACQ_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_LICENSE_ACQ_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion && self.wszURL == other.wszURL && self.wszLocalFilename == other.wszLocalFilename && self.pbPostData == other.pbPostData && self.dwPostDataSize == other.dwPostDataSize && self.wszFriendlyName == other.wszFriendlyName
    }
}
impl ::core::cmp::Eq for DRM_LICENSE_ACQ_DATA {}
impl ::core::fmt::Debug for DRM_LICENSE_ACQ_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_LICENSE_ACQ_DATA").field("uVersion", &self.uVersion).field("wszURL", &self.wszURL).field("wszLocalFilename", &self.wszLocalFilename).field("pbPostData", &self.pbPostData).field("dwPostDataSize", &self.dwPostDataSize).field("wszFriendlyName", &self.wszFriendlyName).finish()
    }
}
impl ::core::default::Default for DRM_STATUS_MSG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRM_STATUS_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_STATUS_MSG").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRM_USAGEPOLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRM_USAGEPOLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_USAGEPOLICY_TYPE").field(&self.0).finish()
    }
}
