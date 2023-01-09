impl ::core::cmp::PartialEq for ISceSvcAttachmentData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISceSvcAttachmentData {}
impl ::core::fmt::Debug for ISceSvcAttachmentData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISceSvcAttachmentData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISceSvcAttachmentPersistInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISceSvcAttachmentPersistInfo {}
impl ::core::fmt::Debug for ISceSvcAttachmentPersistInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISceSvcAttachmentPersistInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCESVC_ANALYSIS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCESVC_ANALYSIS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::core::cmp::Eq for SCESVC_ANALYSIS_INFO {}
impl ::core::fmt::Debug for SCESVC_ANALYSIS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_ANALYSIS_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
impl ::core::default::Default for SCESVC_ANALYSIS_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCESVC_ANALYSIS_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::core::cmp::Eq for SCESVC_ANALYSIS_LINE {}
impl ::core::fmt::Debug for SCESVC_ANALYSIS_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_ANALYSIS_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCESVC_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SCESVC_CONFIGURATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCESVC_CONFIGURATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::core::cmp::Eq for SCESVC_CONFIGURATION_INFO {}
impl ::core::fmt::Debug for SCESVC_CONFIGURATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_CONFIGURATION_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
impl ::core::default::Default for SCESVC_CONFIGURATION_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCESVC_CONFIGURATION_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::core::cmp::Eq for SCESVC_CONFIGURATION_LINE {}
impl ::core::fmt::Debug for SCESVC_CONFIGURATION_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_CONFIGURATION_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
impl ::core::default::Default for SCESVC_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCESVC_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCESVC_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCE_LOG_ERR_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCE_LOG_ERR_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCE_LOG_ERR_LEVEL").field(&self.0).finish()
    }
}
