impl ::core::default::Default for WEB_SOCKET_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WEB_SOCKET_ACTION_QUEUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_ACTION_QUEUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_ACTION_QUEUE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WEB_SOCKET_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WEB_SOCKET_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_BUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WEB_SOCKET_CLOSE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WEB_SOCKET_HTTP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEB_SOCKET_HTTP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.pcName == other.pcName && self.ulNameLength == other.ulNameLength && self.pcValue == other.pcValue && self.ulValueLength == other.ulValueLength
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_HTTP_HEADER {}
impl ::core::fmt::Debug for WEB_SOCKET_HTTP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEB_SOCKET_HTTP_HEADER").field("pcName", &self.pcName).field("ulNameLength", &self.ulNameLength).field("pcValue", &self.pcValue).field("ulValueLength", &self.ulValueLength).finish()
    }
}
impl ::core::default::Default for WEB_SOCKET_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEB_SOCKET_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pvValue == other.pvValue && self.ulValueSize == other.ulValueSize
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_PROPERTY {}
impl ::core::fmt::Debug for WEB_SOCKET_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEB_SOCKET_PROPERTY").field("Type", &self.Type).field("pvValue", &self.pvValue).field("ulValueSize", &self.ulValueSize).finish()
    }
}
impl ::core::default::Default for WEB_SOCKET_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_PROPERTY_TYPE").field(&self.0).finish()
    }
}
