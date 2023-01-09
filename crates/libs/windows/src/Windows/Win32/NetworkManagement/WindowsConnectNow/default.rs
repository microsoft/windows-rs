impl ::core::cmp::PartialEq for IWCNConnectNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWCNConnectNotify {}
impl ::core::fmt::Debug for IWCNConnectNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWCNConnectNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWCNDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWCNDevice {}
impl ::core::fmt::Debug for IWCNDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWCNDevice").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_PASSWORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_PASSWORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_PASSWORD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_SESSION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_SESSION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_SESSION_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_ASSOCIATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_ASSOCIATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_ASSOCIATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_AUTHENTICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_AUTHENTICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_AUTHENTICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_BOOLEAN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_BOOLEAN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_BOOLEAN").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_CONFIGURATION_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_CONFIGURATION_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_CONFIGURATION_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_CONFIG_METHODS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_CONFIG_METHODS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_CONFIG_METHODS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_CONNECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_CONNECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_CONNECTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_DEVICE_PASSWORD_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_DEVICE_PASSWORD_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_DEVICE_PASSWORD_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_ENCRYPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_ENCRYPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_ENCRYPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_MESSAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_REQUEST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_RESPONSE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_RESPONSE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_RESPONSE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_RF_BANDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_RF_BANDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_RF_BANDS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCN_VENDOR_EXTENSION_SPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WCN_VENDOR_EXTENSION_SPEC {
    fn eq(&self, other: &Self) -> bool {
        self.VendorId == other.VendorId && self.SubType == other.SubType && self.Index == other.Index && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WCN_VENDOR_EXTENSION_SPEC {}
impl ::core::fmt::Debug for WCN_VENDOR_EXTENSION_SPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCN_VENDOR_EXTENSION_SPEC").field("VendorId", &self.VendorId).field("SubType", &self.SubType).field("Index", &self.Index).field("Flags", &self.Flags).finish()
    }
}
