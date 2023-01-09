impl ::core::default::Default for DEVPROPCOMPKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVPROPCOMPKEY {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Store == other.Store && self.LocaleName == other.LocaleName
    }
}
impl ::core::cmp::Eq for DEVPROPCOMPKEY {}
impl ::core::fmt::Debug for DEVPROPCOMPKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVPROPCOMPKEY").field("Key", &self.Key).field("Store", &self.Store).field("LocaleName", &self.LocaleName).finish()
    }
}
impl ::core::default::Default for DEVPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVPROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.CompKey == other.CompKey && self.Type == other.Type && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for DEVPROPERTY {}
impl ::core::fmt::Debug for DEVPROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVPROPERTY").field("CompKey", &self.CompKey).field("Type", &self.Type).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for DEVPROPKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVPROPKEY {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.pid == other.pid
    }
}
impl ::core::cmp::Eq for DEVPROPKEY {}
impl ::core::fmt::Debug for DEVPROPKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVPROPKEY").field("fmtid", &self.fmtid).field("pid", &self.pid).finish()
    }
}
impl ::core::default::Default for DEVPROPSTORE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVPROPSTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVPROPSTORE").field(&self.0).finish()
    }
}
