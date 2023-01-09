#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESTOREPOINTINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESTOREPOINTINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RESTOREPOINTINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RESTOREPOINTINFO_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESTOREPOINTINFO_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTOREPOINTINFO_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RESTOREPOINTINFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESTOREPOINTINFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTOREPOINTINFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STATEMGRSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
