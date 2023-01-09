impl ::core::cmp::PartialEq for IFunctionDiscovery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscovery {}
impl ::core::fmt::Debug for IFunctionDiscovery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscovery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryNotification {}
impl ::core::fmt::Debug for IFunctionDiscoveryNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProvider {}
impl ::core::fmt::Debug for IFunctionDiscoveryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProviderFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProviderFactory {}
impl ::core::fmt::Debug for IFunctionDiscoveryProviderFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProviderFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProviderQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProviderQuery {}
impl ::core::fmt::Debug for IFunctionDiscoveryProviderQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProviderQuery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryServiceProvider {}
impl ::core::fmt::Debug for IFunctionDiscoveryServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryServiceProvider").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFunctionInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFunctionInstance {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFunctionInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstance").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstance {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryService)(::windows::core::Vtable::as_raw(self), guidservice, riid, ppvobject).ok()
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceCollection {}
impl ::core::fmt::Debug for IFunctionInstanceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceCollectionQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceCollectionQuery {}
impl ::core::fmt::Debug for IFunctionInstanceCollectionQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceCollectionQuery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceQuery {}
impl ::core::fmt::Debug for IFunctionInstanceQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceQuery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPNPXAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPNPXAssociation {}
impl ::core::fmt::Debug for IPNPXAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPNPXAssociation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPNPXDeviceAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPNPXDeviceAssociation {}
impl ::core::fmt::Debug for IPNPXDeviceAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPNPXDeviceAssociation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertyStoreCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCollection {}
impl ::core::fmt::Debug for IPropertyStoreCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProviderProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderProperties {}
impl ::core::fmt::Debug for IProviderProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProviderPropertyConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderPropertyConstraintCollection {}
impl ::core::fmt::Debug for IProviderPropertyConstraintCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderPropertyConstraintCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProviderPublishing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderPublishing {}
impl ::core::fmt::Debug for IProviderPublishing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderPublishing").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProviderQueryConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderQueryConstraintCollection {}
impl ::core::fmt::Debug for IProviderQueryConstraintCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderQueryConstraintCollection").field(&self.0).finish()
    }
}
impl ::core::default::Default for PropertyConstraint {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PropertyConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyConstraint").field(&self.0).finish()
    }
}
impl ::core::default::Default for QueryCategoryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QueryCategoryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryCategoryType").field(&self.0).finish()
    }
}
impl ::core::default::Default for QueryUpdateAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QueryUpdateAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryUpdateAction").field(&self.0).finish()
    }
}
impl ::core::default::Default for SystemVisibilityFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SystemVisibilityFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemVisibilityFlags").field(&self.0).finish()
    }
}
