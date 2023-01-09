impl ::core::default::Default for TBS_COMMAND_LOCALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TBS_COMMAND_LOCALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBS_COMMAND_LOCALITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for TBS_COMMAND_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TBS_COMMAND_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBS_COMMAND_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for TBS_CONTEXT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TBS_CONTEXT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}
impl ::core::cmp::Eq for TBS_CONTEXT_PARAMS {}
impl ::core::fmt::Debug for TBS_CONTEXT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBS_CONTEXT_PARAMS").field("version", &self.version).finish()
    }
}
impl ::core::default::Default for TBS_CONTEXT_PARAMS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TPM_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TPM_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.structVersion == other.structVersion && self.tpmVersion == other.tpmVersion && self.tpmInterfaceType == other.tpmInterfaceType && self.tpmImpRevision == other.tpmImpRevision
    }
}
impl ::core::cmp::Eq for TPM_DEVICE_INFO {}
impl ::core::fmt::Debug for TPM_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TPM_DEVICE_INFO").field("structVersion", &self.structVersion).field("tpmVersion", &self.tpmVersion).field("tpmInterfaceType", &self.tpmInterfaceType).field("tpmImpRevision", &self.tpmImpRevision).finish()
    }
}
impl ::core::default::Default for TPM_WNF_PROVISIONING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TPM_WNF_PROVISIONING {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.message == other.message
    }
}
impl ::core::cmp::Eq for TPM_WNF_PROVISIONING {}
impl ::core::fmt::Debug for TPM_WNF_PROVISIONING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TPM_WNF_PROVISIONING").field("status", &self.status).field("message", &self.message).finish()
    }
}
