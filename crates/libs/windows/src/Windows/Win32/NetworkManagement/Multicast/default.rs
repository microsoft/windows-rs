impl ::core::default::Default for IPNG_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCAST_CLIENT_UID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MCAST_CLIENT_UID {
    fn eq(&self, other: &Self) -> bool {
        self.ClientUID == other.ClientUID && self.ClientUIDLength == other.ClientUIDLength
    }
}
impl ::core::cmp::Eq for MCAST_CLIENT_UID {}
impl ::core::fmt::Debug for MCAST_CLIENT_UID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCAST_CLIENT_UID").field("ClientUID", &self.ClientUID).field("ClientUIDLength", &self.ClientUIDLength).finish()
    }
}
impl ::core::default::Default for MCAST_LEASE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCAST_LEASE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCAST_SCOPE_CTX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCAST_SCOPE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
