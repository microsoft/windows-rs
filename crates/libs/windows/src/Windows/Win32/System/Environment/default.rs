impl ::core::default::Default for ENCLAVE_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ENCLAVE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENCLAVE_SEALING_IDENTITY_POLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize && self.Flags == other.Flags && self.EnclaveSVN == other.EnclaveSVN && self.SystemKeyID == other.SystemKeyID && self.CurrentSystemKeyID == other.CurrentSystemKeyID
    }
}
impl ::core::cmp::Eq for ENCLAVE_VBS_BASIC_KEY_REQUEST {}
impl ::core::fmt::Debug for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCLAVE_VBS_BASIC_KEY_REQUEST").field("RequestSize", &self.RequestSize).field("Flags", &self.Flags).field("EnclaveSVN", &self.EnclaveSVN).field("SystemKeyID", &self.SystemKeyID).field("CurrentSystemKeyID", &self.CurrentSystemKeyID).finish()
    }
}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.NumberParameters == other.NumberParameters && self.ExceptionInformation == other.ExceptionInformation && self.ExceptionRAX == other.ExceptionRAX && self.ExceptionRCX == other.ExceptionRCX && self.ExceptionRIP == other.ExceptionRIP && self.ExceptionRFLAGS == other.ExceptionRFLAGS && self.ExceptionRSP == other.ExceptionRSP
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_EXCEPTION_AMD64").field("ExceptionCode", &self.ExceptionCode).field("NumberParameters", &self.NumberParameters).field("ExceptionInformation", &self.ExceptionInformation).field("ExceptionRAX", &self.ExceptionRAX).field("ExceptionRCX", &self.ExceptionRCX).field("ExceptionRIP", &self.ExceptionRIP).field("ExceptionRFLAGS", &self.ExceptionRFLAGS).field("ExceptionRSP", &self.ExceptionRSP).finish()
    }
}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext && self.EntryPoint == other.EntryPoint && self.StackPointer == other.StackPointer && self.ExceptionEntryPoint == other.ExceptionEntryPoint && self.ExceptionStack == other.ExceptionStack && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32").field("ThreadContext", &self.ThreadContext).field("EntryPoint", &self.EntryPoint).field("StackPointer", &self.StackPointer).field("ExceptionEntryPoint", &self.ExceptionEntryPoint).field("ExceptionStack", &self.ExceptionStack).field("ExceptionActive", &self.ExceptionActive).finish()
    }
}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext && self.EntryPoint == other.EntryPoint && self.StackPointer == other.StackPointer && self.ExceptionEntryPoint == other.ExceptionEntryPoint && self.ExceptionStack == other.ExceptionStack && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64").field("ThreadContext", &self.ThreadContext).field("EntryPoint", &self.EntryPoint).field("StackPointer", &self.StackPointer).field("ExceptionEntryPoint", &self.ExceptionEntryPoint).field("ExceptionStack", &self.ExceptionStack).field("ExceptionActive", &self.ExceptionActive).finish()
    }
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
