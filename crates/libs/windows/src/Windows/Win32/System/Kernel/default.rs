impl ::core::default::Default for COMPARTMENT_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPARTMENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPARTMENT_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for CSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for CSTRING {}
impl ::core::fmt::Debug for CSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSTRING").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXCEPTION_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXCEPTION_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXCEPTION_DISPOSITION").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for EXCEPTION_REGISTRATION_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.RegisterArea == other.RegisterArea && self.Cr0NpxState == other.Cr0NpxState
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATING_SAVE_AREA").field("ControlWord", &self.ControlWord).field("StatusWord", &self.StatusWord).field("TagWord", &self.TagWord).field("ErrorOffset", &self.ErrorOffset).field("ErrorSelector", &self.ErrorSelector).field("DataOffset", &self.DataOffset).field("DataSelector", &self.DataSelector).field("RegisterArea", &self.RegisterArea).field("Cr0NpxState", &self.Cr0NpxState).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.RegisterArea == other.RegisterArea && self.Spare0 == other.Spare0
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATING_SAVE_AREA").field("ControlWord", &self.ControlWord).field("StatusWord", &self.StatusWord).field("TagWord", &self.TagWord).field("ErrorOffset", &self.ErrorOffset).field("ErrorSelector", &self.ErrorSelector).field("DataOffset", &self.DataOffset).field("DataSelector", &self.DataSelector).field("RegisterArea", &self.RegisterArea).field("Spare0", &self.Spare0).finish()
    }
}
impl ::core::default::Default for LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY {}
impl ::core::fmt::Debug for LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY").field("Flink", &self.Flink).field("Blink", &self.Blink).finish()
    }
}
impl ::core::default::Default for LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY32 {}
impl ::core::fmt::Debug for LIST_ENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY32").field("Flink", &self.Flink).field("Blink", &self.Blink).finish()
    }
}
impl ::core::default::Default for LIST_ENTRY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIST_ENTRY64 {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY64 {}
impl ::core::fmt::Debug for LIST_ENTRY64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY64").field("Flink", &self.Flink).field("Blink", &self.Blink).finish()
    }
}
impl ::core::default::Default for NT_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NT_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NT_PRODUCT_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for NT_TIB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OBJECTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OBJECTID {
    fn eq(&self, other: &Self) -> bool {
        self.Lineage == other.Lineage && self.Uniquifier == other.Uniquifier
    }
}
impl ::core::cmp::Eq for OBJECTID {}
impl ::core::fmt::Debug for OBJECTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTID").field("Lineage", &self.Lineage).field("Uniquifier", &self.Uniquifier).finish()
    }
}
impl ::core::default::Default for OBJECT_ATTRIBUTES32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES32 {}
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES32").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl ::core::default::Default for OBJECT_ATTRIBUTES64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES64 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES64 {}
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES64").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl ::core::default::Default for PROCESSOR_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.Group == other.Group && self.Number == other.Number && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PROCESSOR_NUMBER {}
impl ::core::fmt::Debug for PROCESSOR_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_NUMBER").field("Group", &self.Group).field("Number", &self.Number).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for QUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RTL_BALANCED_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY {}
impl ::core::fmt::Debug for SINGLE_LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SINGLE_LIST_ENTRY").field("Next", &self.Next).finish()
    }
}
impl ::core::default::Default for SINGLE_LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY32 {}
impl ::core::fmt::Debug for SINGLE_LIST_ENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SINGLE_LIST_ENTRY32").field("Next", &self.Next).finish()
    }
}
impl ::core::default::Default for SLIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SLIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SLIST_ENTRY {}
impl ::core::fmt::Debug for SLIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_ENTRY").field("Next", &self.Next).finish()
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING {}
impl ::core::fmt::Debug for STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for STRING32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRING32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING32 {}
impl ::core::fmt::Debug for STRING32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING32").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for STRING64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STRING64 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING64 {}
impl ::core::fmt::Debug for STRING64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING64").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for SUITE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SUITE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUITE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TIMER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TIMER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIMER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WAIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WAIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WAIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WNF_STATE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WNF_STATE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WNF_STATE_NAME {}
impl ::core::fmt::Debug for WNF_STATE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNF_STATE_NAME").field("Data", &self.Data).finish()
    }
}
