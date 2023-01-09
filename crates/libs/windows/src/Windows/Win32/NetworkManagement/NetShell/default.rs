#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMD_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMD_GROUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NS_CMD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NS_CMD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NS_CMD_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NS_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NS_EVENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NS_EVENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NS_EVENTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NS_HELPER_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NS_MODE_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NS_MODE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NS_MODE_CHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NS_REQS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NS_REQS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NS_REQS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAG_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAG_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszTag == other.pwszTag && self.dwRequired == other.dwRequired && self.bPresent == other.bPresent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAG_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAG_TYPE").field("pwszTag", &self.pwszTag).field("dwRequired", &self.dwRequired).field("bPresent", &self.bPresent).finish()
    }
}
impl ::core::default::Default for TOKEN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOKEN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszToken == other.pwszToken && self.dwValue == other.dwValue
    }
}
impl ::core::cmp::Eq for TOKEN_VALUE {}
impl ::core::fmt::Debug for TOKEN_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_VALUE").field("pwszToken", &self.pwszToken).field("dwValue", &self.dwValue).finish()
    }
}
