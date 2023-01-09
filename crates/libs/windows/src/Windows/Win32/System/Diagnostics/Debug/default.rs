#[cfg(target_arch = "x86")]
impl ::core::default::Default for ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Segment == other.Segment && self.Mode == other.Mode
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for ADDRESS {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRESS").field("Offset", &self.Offset).field("Segment", &self.Segment).field("Mode", &self.Mode).finish()
    }
}
impl ::core::default::Default for ADDRESS64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADDRESS64 {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Segment == other.Segment && self.Mode == other.Mode
    }
}
impl ::core::cmp::Eq for ADDRESS64 {}
impl ::core::fmt::Debug for ADDRESS64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRESS64").field("Offset", &self.Offset).field("Segment", &self.Segment).field("Mode", &self.Mode).finish()
    }
}
impl ::core::default::Default for ADDRESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADDRESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AER_BRIDGE_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AER_ENDPOINT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AER_ROOTPORT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for API_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for API_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.Revision == other.Revision && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for API_VERSION {}
impl ::core::fmt::Debug for API_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("API_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("Revision", &self.Revision).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for APPLICATION_NODE_EVENT_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPLICATION_NODE_EVENT_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATION_NODE_EVENT_FILTER").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::default::Default for ARM64_NT_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ARM64_NT_NEON128 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ArrayDimension {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ArrayDimension {
    fn eq(&self, other: &Self) -> bool {
        self.LowerBound == other.LowerBound && self.Length == other.Length && self.Stride == other.Stride
    }
}
impl ::core::cmp::Eq for ArrayDimension {}
impl ::core::fmt::Debug for ArrayDimension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ArrayDimension").field("LowerBound", &self.LowerBound).field("Length", &self.Length).field("Stride", &self.Stride).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIDebugApplicationNodeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIDebugApplicationNodeEvents {}
impl ::core::fmt::Debug for AsyncIDebugApplicationNodeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIDebugApplicationNodeEvents").field(&self.0).finish()
    }
}
impl ::core::default::Default for BREAKPOINT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BREAKPOINT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BREAKPOINT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BREAKREASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BREAKREASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BREAKREASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for BREAKRESUMEACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BREAKRESUMEACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BREAKRESUMEACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for BUGCHECK_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BUGCHECK_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BUGCHECK_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for BUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.BusDataType == other.BusDataType && self.BusNumber == other.BusNumber && self.SlotNumber == other.SlotNumber && self.Buffer == other.Buffer && self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for BUSDATA {}
impl ::core::fmt::Debug for BUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BUSDATA").field("BusDataType", &self.BusDataType).field("BusNumber", &self.BusNumber).field("SlotNumber", &self.SlotNumber).field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CPU_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::default::Default for CREATE_PROCESS_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::default::Default for CREATE_THREAD_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CallingConventionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CallingConventionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallingConventionKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBGHELP_DATA_REPORT_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBGHELP_DATA_REPORT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pBinPathNonExist == other.pBinPathNonExist && self.pSymbolPathNonExist == other.pSymbolPathNonExist
    }
}
impl ::core::cmp::Eq for DBGHELP_DATA_REPORT_STRUCT {}
impl ::core::fmt::Debug for DBGHELP_DATA_REPORT_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBGHELP_DATA_REPORT_STRUCT").field("pBinPathNonExist", &self.pBinPathNonExist).field("pSymbolPathNonExist", &self.pSymbolPathNonExist).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for DBGKD_DEBUG_DATA_HEADER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for DBGKD_DEBUG_DATA_HEADER32 {
    fn eq(&self, other: &Self) -> bool {
        self.List == other.List && self.OwnerTag == other.OwnerTag && self.Size == other.Size
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for DBGKD_DEBUG_DATA_HEADER32 {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for DBGKD_DEBUG_DATA_HEADER32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBGKD_DEBUG_DATA_HEADER32").field("List", &self.List).field("OwnerTag", &self.OwnerTag).field("Size", &self.Size).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for DBGKD_DEBUG_DATA_HEADER64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for DBGKD_DEBUG_DATA_HEADER64 {
    fn eq(&self, other: &Self) -> bool {
        self.List == other.List && self.OwnerTag == other.OwnerTag && self.Size == other.Size
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for DBGKD_DEBUG_DATA_HEADER64 {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for DBGKD_DEBUG_DATA_HEADER64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBGKD_DEBUG_DATA_HEADER64").field("List", &self.List).field("OwnerTag", &self.OwnerTag).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for DBGKD_GET_VERSION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBGKD_GET_VERSION32 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.ProtocolVersion == other.ProtocolVersion && self.Flags == other.Flags && self.KernBase == other.KernBase && self.PsLoadedModuleList == other.PsLoadedModuleList && self.MachineType == other.MachineType && self.ThCallbackStack == other.ThCallbackStack && self.NextCallback == other.NextCallback && self.FramePointer == other.FramePointer && self.KiCallUserMode == other.KiCallUserMode && self.KeUserCallbackDispatcher == other.KeUserCallbackDispatcher && self.BreakpointWithStatus == other.BreakpointWithStatus && self.DebuggerDataList == other.DebuggerDataList
    }
}
impl ::core::cmp::Eq for DBGKD_GET_VERSION32 {}
impl ::core::fmt::Debug for DBGKD_GET_VERSION32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBGKD_GET_VERSION32")
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("ProtocolVersion", &self.ProtocolVersion)
            .field("Flags", &self.Flags)
            .field("KernBase", &self.KernBase)
            .field("PsLoadedModuleList", &self.PsLoadedModuleList)
            .field("MachineType", &self.MachineType)
            .field("ThCallbackStack", &self.ThCallbackStack)
            .field("NextCallback", &self.NextCallback)
            .field("FramePointer", &self.FramePointer)
            .field("KiCallUserMode", &self.KiCallUserMode)
            .field("KeUserCallbackDispatcher", &self.KeUserCallbackDispatcher)
            .field("BreakpointWithStatus", &self.BreakpointWithStatus)
            .field("DebuggerDataList", &self.DebuggerDataList)
            .finish()
    }
}
impl ::core::default::Default for DBGKD_GET_VERSION64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBGKD_GET_VERSION64 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.ProtocolVersion == other.ProtocolVersion && self.KdSecondaryVersion == other.KdSecondaryVersion && self.Flags == other.Flags && self.MachineType == other.MachineType && self.MaxPacketType == other.MaxPacketType && self.MaxStateChange == other.MaxStateChange && self.MaxManipulate == other.MaxManipulate && self.Simulation == other.Simulation && self.Unused == other.Unused && self.KernBase == other.KernBase && self.PsLoadedModuleList == other.PsLoadedModuleList && self.DebuggerDataList == other.DebuggerDataList
    }
}
impl ::core::cmp::Eq for DBGKD_GET_VERSION64 {}
impl ::core::fmt::Debug for DBGKD_GET_VERSION64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBGKD_GET_VERSION64")
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("ProtocolVersion", &self.ProtocolVersion)
            .field("KdSecondaryVersion", &self.KdSecondaryVersion)
            .field("Flags", &self.Flags)
            .field("MachineType", &self.MachineType)
            .field("MaxPacketType", &self.MaxPacketType)
            .field("MaxStateChange", &self.MaxStateChange)
            .field("MaxManipulate", &self.MaxManipulate)
            .field("Simulation", &self.Simulation)
            .field("Unused", &self.Unused)
            .field("KernBase", &self.KernBase)
            .field("PsLoadedModuleList", &self.PsLoadedModuleList)
            .field("DebuggerDataList", &self.DebuggerDataList)
            .finish()
    }
}
impl ::core::default::Default for DBGKD_MAJOR_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBGKD_MAJOR_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBGKD_MAJOR_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBGPROP_ATTRIB_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBGPROP_ATTRIB_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBGPROP_ATTRIB_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DBGPROP_ATTRIB_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DBGPROP_ATTRIB_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DBGPROP_ATTRIB_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DBGPROP_ATTRIB_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DBGPROP_ATTRIB_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DBGPROP_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBGPROP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBGPROP_INFO").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DBGPROP_INFO {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DBGPROP_INFO {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DBGPROP_INFO {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DBGPROP_INFO {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DBGPROP_INFO {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DEBUG_BREAKPOINT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_BREAKPOINT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Id == other.Id && self.BreakType == other.BreakType && self.ProcType == other.ProcType && self.Flags == other.Flags && self.DataSize == other.DataSize && self.DataAccessType == other.DataAccessType && self.PassCount == other.PassCount && self.CurrentPassCount == other.CurrentPassCount && self.MatchThread == other.MatchThread && self.CommandSize == other.CommandSize && self.OffsetExpressionSize == other.OffsetExpressionSize
    }
}
impl ::core::cmp::Eq for DEBUG_BREAKPOINT_PARAMETERS {}
impl ::core::fmt::Debug for DEBUG_BREAKPOINT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_BREAKPOINT_PARAMETERS")
            .field("Offset", &self.Offset)
            .field("Id", &self.Id)
            .field("BreakType", &self.BreakType)
            .field("ProcType", &self.ProcType)
            .field("Flags", &self.Flags)
            .field("DataSize", &self.DataSize)
            .field("DataAccessType", &self.DataAccessType)
            .field("PassCount", &self.PassCount)
            .field("CurrentPassCount", &self.CurrentPassCount)
            .field("MatchThread", &self.MatchThread)
            .field("CommandSize", &self.CommandSize)
            .field("OffsetExpressionSize", &self.OffsetExpressionSize)
            .finish()
    }
}
impl ::core::default::Default for DEBUG_CACHED_SYMBOL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_CACHED_SYMBOL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ModBase == other.ModBase && self.Arg1 == other.Arg1 && self.Arg2 == other.Arg2 && self.Id == other.Id && self.Arg3 == other.Arg3
    }
}
impl ::core::cmp::Eq for DEBUG_CACHED_SYMBOL_INFO {}
impl ::core::fmt::Debug for DEBUG_CACHED_SYMBOL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_CACHED_SYMBOL_INFO").field("ModBase", &self.ModBase).field("Arg1", &self.Arg1).field("Arg2", &self.Arg2).field("Id", &self.Id).field("Arg3", &self.Arg3).finish()
    }
}
impl ::core::default::Default for DEBUG_CLIENT_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_CLIENT_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.eClient == other.eClient
    }
}
impl ::core::cmp::Eq for DEBUG_CLIENT_CONTEXT {}
impl ::core::fmt::Debug for DEBUG_CLIENT_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_CLIENT_CONTEXT").field("cbSize", &self.cbSize).field("eClient", &self.eClient).finish()
    }
}
impl ::core::default::Default for DEBUG_CREATE_PROCESS_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_CREATE_PROCESS_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.CreateFlags == other.CreateFlags && self.EngCreateFlags == other.EngCreateFlags && self.VerifierFlags == other.VerifierFlags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEBUG_CREATE_PROCESS_OPTIONS {}
impl ::core::fmt::Debug for DEBUG_CREATE_PROCESS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_CREATE_PROCESS_OPTIONS").field("CreateFlags", &self.CreateFlags).field("EngCreateFlags", &self.EngCreateFlags).field("VerifierFlags", &self.VerifierFlags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::default::Default for DEBUG_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEBUG_EVENT_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEBUG_EVENT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEBUG_EVENT_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEBUG_EVENT_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_EVENT_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ProcessEngineId == other.ProcessEngineId && self.ThreadEngineId == other.ThreadEngineId && self.FrameEngineId == other.FrameEngineId
    }
}
impl ::core::cmp::Eq for DEBUG_EVENT_CONTEXT {}
impl ::core::fmt::Debug for DEBUG_EVENT_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_EVENT_CONTEXT").field("Size", &self.Size).field("ProcessEngineId", &self.ProcessEngineId).field("ThreadEngineId", &self.ThreadEngineId).field("FrameEngineId", &self.FrameEngineId).finish()
    }
}
impl ::core::default::Default for DEBUG_EVENT_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEBUG_EVENT_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEBUG_EVENT_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEBUG_EXCEPTION_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_EXCEPTION_FILTER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ExecutionOption == other.ExecutionOption && self.ContinueOption == other.ContinueOption && self.TextSize == other.TextSize && self.CommandSize == other.CommandSize && self.SecondCommandSize == other.SecondCommandSize && self.ExceptionCode == other.ExceptionCode
    }
}
impl ::core::cmp::Eq for DEBUG_EXCEPTION_FILTER_PARAMETERS {}
impl ::core::fmt::Debug for DEBUG_EXCEPTION_FILTER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_EXCEPTION_FILTER_PARAMETERS").field("ExecutionOption", &self.ExecutionOption).field("ContinueOption", &self.ContinueOption).field("TextSize", &self.TextSize).field("CommandSize", &self.CommandSize).field("SecondCommandSize", &self.SecondCommandSize).field("ExceptionCode", &self.ExceptionCode).finish()
    }
}
impl ::core::default::Default for DEBUG_GET_TEXT_COMPLETIONS_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_GET_TEXT_COMPLETIONS_IN {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MatchCountLimit == other.MatchCountLimit && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEBUG_GET_TEXT_COMPLETIONS_IN {}
impl ::core::fmt::Debug for DEBUG_GET_TEXT_COMPLETIONS_IN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_GET_TEXT_COMPLETIONS_IN").field("Flags", &self.Flags).field("MatchCountLimit", &self.MatchCountLimit).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DEBUG_GET_TEXT_COMPLETIONS_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_GET_TEXT_COMPLETIONS_OUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.ReplaceIndex == other.ReplaceIndex && self.MatchCount == other.MatchCount && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for DEBUG_GET_TEXT_COMPLETIONS_OUT {}
impl ::core::fmt::Debug for DEBUG_GET_TEXT_COMPLETIONS_OUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_GET_TEXT_COMPLETIONS_OUT").field("Flags", &self.Flags).field("ReplaceIndex", &self.ReplaceIndex).field("MatchCount", &self.MatchCount).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for DEBUG_HANDLE_DATA_BASIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_HANDLE_DATA_BASIC {
    fn eq(&self, other: &Self) -> bool {
        self.TypeNameSize == other.TypeNameSize && self.ObjectNameSize == other.ObjectNameSize && self.Attributes == other.Attributes && self.GrantedAccess == other.GrantedAccess && self.HandleCount == other.HandleCount && self.PointerCount == other.PointerCount
    }
}
impl ::core::cmp::Eq for DEBUG_HANDLE_DATA_BASIC {}
impl ::core::fmt::Debug for DEBUG_HANDLE_DATA_BASIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_HANDLE_DATA_BASIC").field("TypeNameSize", &self.TypeNameSize).field("ObjectNameSize", &self.ObjectNameSize).field("Attributes", &self.Attributes).field("GrantedAccess", &self.GrantedAccess).field("HandleCount", &self.HandleCount).field("PointerCount", &self.PointerCount).finish()
    }
}
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_BREAKPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_BREAKPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
    }
}
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_BREAKPOINT {}
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_BREAKPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_BREAKPOINT").field("Id", &self.Id).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_EXCEPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_EXCEPTION {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionRecord == other.ExceptionRecord && self.FirstChance == other.FirstChance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_EXCEPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_EXCEPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_EXCEPTION").field("ExceptionRecord", &self.ExceptionRecord).field("FirstChance", &self.FirstChance).finish()
    }
}
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_EXIT_PROCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_EXIT_PROCESS {
    fn eq(&self, other: &Self) -> bool {
        self.ExitCode == other.ExitCode
    }
}
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_EXIT_PROCESS {}
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_EXIT_PROCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_EXIT_PROCESS").field("ExitCode", &self.ExitCode).finish()
    }
}
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_EXIT_THREAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_EXIT_THREAD {
    fn eq(&self, other: &Self) -> bool {
        self.ExitCode == other.ExitCode
    }
}
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_EXIT_THREAD {}
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_EXIT_THREAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_EXIT_THREAD").field("ExitCode", &self.ExitCode).finish()
    }
}
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_LOAD_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_LOAD_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base
    }
}
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_LOAD_MODULE {}
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_LOAD_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_LOAD_MODULE").field("Base", &self.Base).finish()
    }
}
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.DataSize == other.DataSize && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION {}
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION").field("Kind", &self.Kind).field("DataSize", &self.DataSize).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.Error == other.Error && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR {}
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR").field("Error", &self.Error).field("Level", &self.Level).finish()
    }
}
impl ::core::default::Default for DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base
    }
}
impl ::core::cmp::Eq for DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE {}
impl ::core::fmt::Debug for DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE").field("Base", &self.Base).finish()
    }
}
impl ::core::default::Default for DEBUG_MODULE_AND_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_MODULE_AND_ID {
    fn eq(&self, other: &Self) -> bool {
        self.ModuleBase == other.ModuleBase && self.Id == other.Id
    }
}
impl ::core::cmp::Eq for DEBUG_MODULE_AND_ID {}
impl ::core::fmt::Debug for DEBUG_MODULE_AND_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_MODULE_AND_ID").field("ModuleBase", &self.ModuleBase).field("Id", &self.Id).finish()
    }
}
impl ::core::default::Default for DEBUG_MODULE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_MODULE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.Size == other.Size && self.TimeDateStamp == other.TimeDateStamp && self.Checksum == other.Checksum && self.Flags == other.Flags && self.SymbolType == other.SymbolType && self.ImageNameSize == other.ImageNameSize && self.ModuleNameSize == other.ModuleNameSize && self.LoadedImageNameSize == other.LoadedImageNameSize && self.SymbolFileNameSize == other.SymbolFileNameSize && self.MappedImageNameSize == other.MappedImageNameSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEBUG_MODULE_PARAMETERS {}
impl ::core::fmt::Debug for DEBUG_MODULE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_MODULE_PARAMETERS")
            .field("Base", &self.Base)
            .field("Size", &self.Size)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("Checksum", &self.Checksum)
            .field("Flags", &self.Flags)
            .field("SymbolType", &self.SymbolType)
            .field("ImageNameSize", &self.ImageNameSize)
            .field("ModuleNameSize", &self.ModuleNameSize)
            .field("LoadedImageNameSize", &self.LoadedImageNameSize)
            .field("SymbolFileNameSize", &self.SymbolFileNameSize)
            .field("MappedImageNameSize", &self.MappedImageNameSize)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for DEBUG_OFFSET_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_OFFSET_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for DEBUG_OFFSET_REGION {}
impl ::core::fmt::Debug for DEBUG_OFFSET_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_OFFSET_REGION").field("Base", &self.Base).field("Size", &self.Size).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_PROCESSOR_IDENTIFICATION_ALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEBUG_PROCESSOR_IDENTIFICATION_ALPHA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_PROCESSOR_IDENTIFICATION_ALPHA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for DEBUG_PROCESSOR_IDENTIFICATION_ALPHA {}
impl ::core::fmt::Debug for DEBUG_PROCESSOR_IDENTIFICATION_ALPHA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_PROCESSOR_IDENTIFICATION_ALPHA").field("Type", &self.Type).field("Revision", &self.Revision).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_PROCESSOR_IDENTIFICATION_AMD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_PROCESSOR_IDENTIFICATION_AMD64 {
    fn eq(&self, other: &Self) -> bool {
        self.Family == other.Family && self.Model == other.Model && self.Stepping == other.Stepping && self.VendorString == other.VendorString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_PROCESSOR_IDENTIFICATION_AMD64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_PROCESSOR_IDENTIFICATION_AMD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_PROCESSOR_IDENTIFICATION_AMD64").field("Family", &self.Family).field("Model", &self.Model).field("Stepping", &self.Stepping).field("VendorString", &self.VendorString).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_PROCESSOR_IDENTIFICATION_ARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_PROCESSOR_IDENTIFICATION_ARM {
    fn eq(&self, other: &Self) -> bool {
        self.Model == other.Model && self.Revision == other.Revision && self.VendorString == other.VendorString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_PROCESSOR_IDENTIFICATION_ARM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_PROCESSOR_IDENTIFICATION_ARM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_PROCESSOR_IDENTIFICATION_ARM").field("Model", &self.Model).field("Revision", &self.Revision).field("VendorString", &self.VendorString).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_PROCESSOR_IDENTIFICATION_ARM64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_PROCESSOR_IDENTIFICATION_ARM64 {
    fn eq(&self, other: &Self) -> bool {
        self.Model == other.Model && self.Revision == other.Revision && self.VendorString == other.VendorString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_PROCESSOR_IDENTIFICATION_ARM64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_PROCESSOR_IDENTIFICATION_ARM64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_PROCESSOR_IDENTIFICATION_ARM64").field("Model", &self.Model).field("Revision", &self.Revision).field("VendorString", &self.VendorString).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_PROCESSOR_IDENTIFICATION_IA64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_PROCESSOR_IDENTIFICATION_IA64 {
    fn eq(&self, other: &Self) -> bool {
        self.Model == other.Model && self.Revision == other.Revision && self.Family == other.Family && self.ArchRev == other.ArchRev && self.VendorString == other.VendorString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_PROCESSOR_IDENTIFICATION_IA64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_PROCESSOR_IDENTIFICATION_IA64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_PROCESSOR_IDENTIFICATION_IA64").field("Model", &self.Model).field("Revision", &self.Revision).field("Family", &self.Family).field("ArchRev", &self.ArchRev).field("VendorString", &self.VendorString).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_PROCESSOR_IDENTIFICATION_X86 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_PROCESSOR_IDENTIFICATION_X86 {
    fn eq(&self, other: &Self) -> bool {
        self.Family == other.Family && self.Model == other.Model && self.Stepping == other.Stepping && self.VendorString == other.VendorString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_PROCESSOR_IDENTIFICATION_X86 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_PROCESSOR_IDENTIFICATION_X86 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_PROCESSOR_IDENTIFICATION_X86").field("Family", &self.Family).field("Model", &self.Model).field("Stepping", &self.Stepping).field("VendorString", &self.VendorString).finish()
    }
}
impl ::core::default::Default for DEBUG_READ_USER_MINIDUMP_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_READ_USER_MINIDUMP_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.StreamType == other.StreamType && self.Flags == other.Flags && self.Offset == other.Offset && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.BufferUsed == other.BufferUsed
    }
}
impl ::core::cmp::Eq for DEBUG_READ_USER_MINIDUMP_STREAM {}
impl ::core::fmt::Debug for DEBUG_READ_USER_MINIDUMP_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_READ_USER_MINIDUMP_STREAM").field("StreamType", &self.StreamType).field("Flags", &self.Flags).field("Offset", &self.Offset).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("BufferUsed", &self.BufferUsed).finish()
    }
}
impl ::core::default::Default for DEBUG_REGISTER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_REGISTER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Flags == other.Flags && self.SubregMaster == other.SubregMaster && self.SubregLength == other.SubregLength && self.SubregMask == other.SubregMask && self.SubregShift == other.SubregShift && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for DEBUG_REGISTER_DESCRIPTION {}
impl ::core::fmt::Debug for DEBUG_REGISTER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_REGISTER_DESCRIPTION").field("Type", &self.Type).field("Flags", &self.Flags).field("SubregMaster", &self.SubregMaster).field("SubregLength", &self.SubregLength).field("SubregMask", &self.SubregMask).field("SubregShift", &self.SubregShift).field("Reserved0", &self.Reserved0).finish()
    }
}
impl ::core::default::Default for DEBUG_SPECIFIC_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_SPECIFIC_FILTER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ExecutionOption == other.ExecutionOption && self.ContinueOption == other.ContinueOption && self.TextSize == other.TextSize && self.CommandSize == other.CommandSize && self.ArgumentSize == other.ArgumentSize
    }
}
impl ::core::cmp::Eq for DEBUG_SPECIFIC_FILTER_PARAMETERS {}
impl ::core::fmt::Debug for DEBUG_SPECIFIC_FILTER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_SPECIFIC_FILTER_PARAMETERS").field("ExecutionOption", &self.ExecutionOption).field("ContinueOption", &self.ContinueOption).field("TextSize", &self.TextSize).field("CommandSize", &self.CommandSize).field("ArgumentSize", &self.ArgumentSize).finish()
    }
}
impl ::core::default::Default for DEBUG_STACKFRAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEBUG_STACKFRAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEBUG_STACKFRAME_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_STACK_FRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_STACK_FRAME {
    fn eq(&self, other: &Self) -> bool {
        self.InstructionOffset == other.InstructionOffset && self.ReturnOffset == other.ReturnOffset && self.FrameOffset == other.FrameOffset && self.StackOffset == other.StackOffset && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Reserved == other.Reserved && self.Virtual == other.Virtual && self.FrameNumber == other.FrameNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_STACK_FRAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_STACK_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_STACK_FRAME").field("InstructionOffset", &self.InstructionOffset).field("ReturnOffset", &self.ReturnOffset).field("FrameOffset", &self.FrameOffset).field("StackOffset", &self.StackOffset).field("FuncTableEntry", &self.FuncTableEntry).field("Params", &self.Params).field("Reserved", &self.Reserved).field("Virtual", &self.Virtual).field("FrameNumber", &self.FrameNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_STACK_FRAME_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUG_STACK_FRAME_EX {
    fn eq(&self, other: &Self) -> bool {
        self.InstructionOffset == other.InstructionOffset && self.ReturnOffset == other.ReturnOffset && self.FrameOffset == other.FrameOffset && self.StackOffset == other.StackOffset && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Reserved == other.Reserved && self.Virtual == other.Virtual && self.FrameNumber == other.FrameNumber && self.InlineFrameContext == other.InlineFrameContext && self.Reserved1 == other.Reserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUG_STACK_FRAME_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUG_STACK_FRAME_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_STACK_FRAME_EX")
            .field("InstructionOffset", &self.InstructionOffset)
            .field("ReturnOffset", &self.ReturnOffset)
            .field("FrameOffset", &self.FrameOffset)
            .field("StackOffset", &self.StackOffset)
            .field("FuncTableEntry", &self.FuncTableEntry)
            .field("Params", &self.Params)
            .field("Reserved", &self.Reserved)
            .field("Virtual", &self.Virtual)
            .field("FrameNumber", &self.FrameNumber)
            .field("InlineFrameContext", &self.InlineFrameContext)
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::default::Default for DEBUG_SYMBOL_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_SYMBOL_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ModuleBase == other.ModuleBase && self.Offset == other.Offset && self.Id == other.Id && self.Arg64 == other.Arg64 && self.Size == other.Size && self.Flags == other.Flags && self.TypeId == other.TypeId && self.NameSize == other.NameSize && self.Token == other.Token && self.Tag == other.Tag && self.Arg32 == other.Arg32 && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEBUG_SYMBOL_ENTRY {}
impl ::core::fmt::Debug for DEBUG_SYMBOL_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_SYMBOL_ENTRY").field("ModuleBase", &self.ModuleBase).field("Offset", &self.Offset).field("Id", &self.Id).field("Arg64", &self.Arg64).field("Size", &self.Size).field("Flags", &self.Flags).field("TypeId", &self.TypeId).field("NameSize", &self.NameSize).field("Token", &self.Token).field("Tag", &self.Tag).field("Arg32", &self.Arg32).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DEBUG_SYMBOL_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_SYMBOL_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Module == other.Module && self.TypeId == other.TypeId && self.ParentSymbol == other.ParentSymbol && self.SubElements == other.SubElements && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEBUG_SYMBOL_PARAMETERS {}
impl ::core::fmt::Debug for DEBUG_SYMBOL_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_SYMBOL_PARAMETERS").field("Module", &self.Module).field("TypeId", &self.TypeId).field("ParentSymbol", &self.ParentSymbol).field("SubElements", &self.SubElements).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DEBUG_SYMBOL_SOURCE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_SYMBOL_SOURCE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ModuleBase == other.ModuleBase && self.Offset == other.Offset && self.FileNameId == other.FileNameId && self.EngineInternal == other.EngineInternal && self.Size == other.Size && self.Flags == other.Flags && self.FileNameSize == other.FileNameSize && self.StartLine == other.StartLine && self.EndLine == other.EndLine && self.StartColumn == other.StartColumn && self.EndColumn == other.EndColumn && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEBUG_SYMBOL_SOURCE_ENTRY {}
impl ::core::fmt::Debug for DEBUG_SYMBOL_SOURCE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_SYMBOL_SOURCE_ENTRY").field("ModuleBase", &self.ModuleBase).field("Offset", &self.Offset).field("FileNameId", &self.FileNameId).field("EngineInternal", &self.EngineInternal).field("Size", &self.Size).field("Flags", &self.Flags).field("FileNameSize", &self.FileNameSize).field("StartLine", &self.StartLine).field("EndLine", &self.EndLine).field("StartColumn", &self.StartColumn).field("EndColumn", &self.EndColumn).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DEBUG_THREAD_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_THREAD_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Valid == other.Valid && self.ExitStatus == other.ExitStatus && self.PriorityClass == other.PriorityClass && self.Priority == other.Priority && self.CreateTime == other.CreateTime && self.ExitTime == other.ExitTime && self.KernelTime == other.KernelTime && self.UserTime == other.UserTime && self.StartOffset == other.StartOffset && self.Affinity == other.Affinity
    }
}
impl ::core::cmp::Eq for DEBUG_THREAD_BASIC_INFORMATION {}
impl ::core::fmt::Debug for DEBUG_THREAD_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_THREAD_BASIC_INFORMATION").field("Valid", &self.Valid).field("ExitStatus", &self.ExitStatus).field("PriorityClass", &self.PriorityClass).field("Priority", &self.Priority).field("CreateTime", &self.CreateTime).field("ExitTime", &self.ExitTime).field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).field("StartOffset", &self.StartOffset).field("Affinity", &self.Affinity).finish()
    }
}
impl ::core::default::Default for DEBUG_TYPED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEBUG_TYPED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ModBase == other.ModBase && self.Offset == other.Offset && self.EngineHandle == other.EngineHandle && self.Data == other.Data && self.Size == other.Size && self.Flags == other.Flags && self.TypeId == other.TypeId && self.BaseTypeId == other.BaseTypeId && self.Tag == other.Tag && self.Register == other.Register && self.Internal == other.Internal
    }
}
impl ::core::cmp::Eq for DEBUG_TYPED_DATA {}
impl ::core::fmt::Debug for DEBUG_TYPED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUG_TYPED_DATA").field("ModBase", &self.ModBase).field("Offset", &self.Offset).field("EngineHandle", &self.EngineHandle).field("Data", &self.Data).field("Size", &self.Size).field("Flags", &self.Flags).field("TypeId", &self.TypeId).field("BaseTypeId", &self.BaseTypeId).field("Tag", &self.Tag).field("Register", &self.Register).field("Internal", &self.Internal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUG_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for DISPATCHER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for DISPATCHER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DOCUMENTNAMETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOCUMENTNAMETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOCUMENTNAMETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DUMP_FILE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_HEADER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_HEADER64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DUMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DUMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DUMP_TYPE").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DebugBaseEventCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DebugBaseEventCallbacks {}
impl ::core::fmt::Debug for DebugBaseEventCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DebugBaseEventCallbacks").field(&self.0).finish()
    }
}
impl DebugBaseEventCallbacks {
    pub unsafe fn GetInterestMask(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetInterestMask)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Breakpoint<P0>(&self, bp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugBreakpoint>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Breakpoint)(::windows::core::Vtable::as_raw(self), bp.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Exception(&self, exception: *const EXCEPTION_RECORD64, firstchance: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Exception)(::windows::core::Vtable::as_raw(self), exception, firstchance).ok()
    }
    pub unsafe fn CreateThread(&self, handle: u64, dataoffset: u64, startoffset: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateThread)(::windows::core::Vtable::as_raw(self), handle, dataoffset, startoffset).ok()
    }
    pub unsafe fn ExitThread(&self, exitcode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ExitThread)(::windows::core::Vtable::as_raw(self), exitcode).ok()
    }
    pub unsafe fn CreateProcessA<P0, P1>(&self, imagefilehandle: u64, handle: u64, baseoffset: u64, modulesize: u32, modulename: P0, imagename: P1, checksum: u32, timedatestamp: u32, initialthreadhandle: u64, threaddataoffset: u64, startoffset: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateProcessA)(::windows::core::Vtable::as_raw(self), imagefilehandle, handle, baseoffset, modulesize, modulename.into().abi(), imagename.into().abi(), checksum, timedatestamp, initialthreadhandle, threaddataoffset, startoffset).ok()
    }
    pub unsafe fn ExitProcess(&self, exitcode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ExitProcess)(::windows::core::Vtable::as_raw(self), exitcode).ok()
    }
    pub unsafe fn LoadModule<P0, P1>(&self, imagefilehandle: u64, baseoffset: u64, modulesize: u32, modulename: P0, imagename: P1, checksum: u32, timedatestamp: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LoadModule)(::windows::core::Vtable::as_raw(self), imagefilehandle, baseoffset, modulesize, modulename.into().abi(), imagename.into().abi(), checksum, timedatestamp).ok()
    }
    pub unsafe fn UnloadModule<P0>(&self, imagebasename: P0, baseoffset: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnloadModule)(::windows::core::Vtable::as_raw(self), imagebasename.into().abi(), baseoffset).ok()
    }
    pub unsafe fn SystemError(&self, error: u32, level: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SystemError)(::windows::core::Vtable::as_raw(self), error, level).ok()
    }
    pub unsafe fn SessionStatus(&self, status: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SessionStatus)(::windows::core::Vtable::as_raw(self), status).ok()
    }
    pub unsafe fn ChangeDebuggeeState(&self, flags: u32, argument: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeDebuggeeState)(::windows::core::Vtable::as_raw(self), flags, argument).ok()
    }
    pub unsafe fn ChangeEngineState(&self, flags: u32, argument: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeEngineState)(::windows::core::Vtable::as_raw(self), flags, argument).ok()
    }
    pub unsafe fn ChangeSymbolState(&self, flags: u32, argument: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeSymbolState)(::windows::core::Vtable::as_raw(self), flags, argument).ok()
    }
}
impl ::core::cmp::PartialEq for DebugBaseEventCallbacksWide {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DebugBaseEventCallbacksWide {}
impl ::core::fmt::Debug for DebugBaseEventCallbacksWide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DebugBaseEventCallbacksWide").field(&self.0).finish()
    }
}
impl DebugBaseEventCallbacksWide {
    pub unsafe fn GetInterestMask(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetInterestMask)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Breakpoint<P0>(&self, bp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugBreakpoint2>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Breakpoint)(::windows::core::Vtable::as_raw(self), bp.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Exception(&self, exception: *const EXCEPTION_RECORD64, firstchance: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Exception)(::windows::core::Vtable::as_raw(self), exception, firstchance).ok()
    }
    pub unsafe fn CreateThread(&self, handle: u64, dataoffset: u64, startoffset: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateThread)(::windows::core::Vtable::as_raw(self), handle, dataoffset, startoffset).ok()
    }
    pub unsafe fn ExitThread(&self, exitcode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ExitThread)(::windows::core::Vtable::as_raw(self), exitcode).ok()
    }
    pub unsafe fn CreateProcessA<P0, P1>(&self, imagefilehandle: u64, handle: u64, baseoffset: u64, modulesize: u32, modulename: P0, imagename: P1, checksum: u32, timedatestamp: u32, initialthreadhandle: u64, threaddataoffset: u64, startoffset: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateProcessA)(::windows::core::Vtable::as_raw(self), imagefilehandle, handle, baseoffset, modulesize, modulename.into().abi(), imagename.into().abi(), checksum, timedatestamp, initialthreadhandle, threaddataoffset, startoffset).ok()
    }
    pub unsafe fn ExitProcess(&self, exitcode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ExitProcess)(::windows::core::Vtable::as_raw(self), exitcode).ok()
    }
    pub unsafe fn LoadModule<P0, P1>(&self, imagefilehandle: u64, baseoffset: u64, modulesize: u32, modulename: P0, imagename: P1, checksum: u32, timedatestamp: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LoadModule)(::windows::core::Vtable::as_raw(self), imagefilehandle, baseoffset, modulesize, modulename.into().abi(), imagename.into().abi(), checksum, timedatestamp).ok()
    }
    pub unsafe fn UnloadModule<P0>(&self, imagebasename: P0, baseoffset: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnloadModule)(::windows::core::Vtable::as_raw(self), imagebasename.into().abi(), baseoffset).ok()
    }
    pub unsafe fn SystemError(&self, error: u32, level: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SystemError)(::windows::core::Vtable::as_raw(self), error, level).ok()
    }
    pub unsafe fn SessionStatus(&self, status: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SessionStatus)(::windows::core::Vtable::as_raw(self), status).ok()
    }
    pub unsafe fn ChangeDebuggeeState(&self, flags: u32, argument: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeDebuggeeState)(::windows::core::Vtable::as_raw(self), flags, argument).ok()
    }
    pub unsafe fn ChangeEngineState(&self, flags: u32, argument: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeEngineState)(::windows::core::Vtable::as_raw(self), flags, argument).ok()
    }
    pub unsafe fn ChangeSymbolState(&self, flags: u32, argument: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeSymbolState)(::windows::core::Vtable::as_raw(self), flags, argument).ok()
    }
}
impl ::core::default::Default for DebugPropertyInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DebugPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwValidFields == other.m_dwValidFields && self.m_bstrName == other.m_bstrName && self.m_bstrType == other.m_bstrType && self.m_bstrValue == other.m_bstrValue && self.m_bstrFullName == other.m_bstrFullName && self.m_dwAttrib == other.m_dwAttrib && self.m_pDebugProp == other.m_pDebugProp
    }
}
impl ::core::cmp::Eq for DebugPropertyInfo {}
impl ::core::fmt::Debug for DebugPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DebugPropertyInfo").field("m_dwValidFields", &self.m_dwValidFields).field("m_bstrName", &self.m_bstrName).field("m_bstrType", &self.m_bstrType).field("m_bstrValue", &self.m_bstrValue).field("m_bstrFullName", &self.m_bstrFullName).field("m_dwAttrib", &self.m_dwAttrib).field("m_pDebugProp", &self.m_pDebugProp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DebugStackFrameDescriptor {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DebugStackFrameDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.pdsf == other.pdsf && self.dwMin == other.dwMin && self.dwLim == other.dwLim && self.fFinal == other.fFinal && self.punkFinal == other.punkFinal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DebugStackFrameDescriptor {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DebugStackFrameDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DebugStackFrameDescriptor").field("pdsf", &self.pdsf).field("dwMin", &self.dwMin).field("dwLim", &self.dwLim).field("fFinal", &self.fFinal).field("punkFinal", &self.punkFinal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DebugStackFrameDescriptor64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DebugStackFrameDescriptor64 {
    fn eq(&self, other: &Self) -> bool {
        self.pdsf == other.pdsf && self.dwMin == other.dwMin && self.dwLim == other.dwLim && self.fFinal == other.fFinal && self.punkFinal == other.punkFinal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DebugStackFrameDescriptor64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DebugStackFrameDescriptor64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DebugStackFrameDescriptor64").field("pdsf", &self.pdsf).field("dwMin", &self.dwMin).field("dwLim", &self.dwLim).field("fFinal", &self.fFinal).field("punkFinal", &self.punkFinal).finish()
    }
}
impl ::core::default::Default for ERRORRESUMEACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ERRORRESUMEACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERRORRESUMEACTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionRecord == other.ExceptionRecord && self.dwFirstChance == other.dwFirstChance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_DEBUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_DEBUG_INFO").field("ExceptionRecord", &self.ExceptionRecord).field("dwFirstChance", &self.dwFirstChance).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for EXCEPTION_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for EXCEPTION_POINTERS {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionRecord == other.ExceptionRecord && self.ContextRecord == other.ContextRecord
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for EXCEPTION_POINTERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for EXCEPTION_POINTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_POINTERS").field("ExceptionRecord", &self.ExceptionRecord).field("ContextRecord", &self.ContextRecord).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.ExceptionFlags == other.ExceptionFlags && self.ExceptionRecord == other.ExceptionRecord && self.ExceptionAddress == other.ExceptionAddress && self.NumberParameters == other.NumberParameters && self.ExceptionInformation == other.ExceptionInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_RECORD").field("ExceptionCode", &self.ExceptionCode).field("ExceptionFlags", &self.ExceptionFlags).field("ExceptionRecord", &self.ExceptionRecord).field("ExceptionAddress", &self.ExceptionAddress).field("NumberParameters", &self.NumberParameters).field("ExceptionInformation", &self.ExceptionInformation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_RECORD32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_RECORD32 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.ExceptionFlags == other.ExceptionFlags && self.ExceptionRecord == other.ExceptionRecord && self.ExceptionAddress == other.ExceptionAddress && self.NumberParameters == other.NumberParameters && self.ExceptionInformation == other.ExceptionInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_RECORD32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_RECORD32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_RECORD32").field("ExceptionCode", &self.ExceptionCode).field("ExceptionFlags", &self.ExceptionFlags).field("ExceptionRecord", &self.ExceptionRecord).field("ExceptionAddress", &self.ExceptionAddress).field("NumberParameters", &self.NumberParameters).field("ExceptionInformation", &self.ExceptionInformation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_RECORD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_RECORD64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.ExceptionFlags == other.ExceptionFlags && self.ExceptionRecord == other.ExceptionRecord && self.ExceptionAddress == other.ExceptionAddress && self.NumberParameters == other.NumberParameters && self.__unusedAlignment == other.__unusedAlignment && self.ExceptionInformation == other.ExceptionInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_RECORD64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_RECORD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_RECORD64").field("ExceptionCode", &self.ExceptionCode).field("ExceptionFlags", &self.ExceptionFlags).field("ExceptionRecord", &self.ExceptionRecord).field("ExceptionAddress", &self.ExceptionAddress).field("NumberParameters", &self.NumberParameters).field("__unusedAlignment", &self.__unusedAlignment).field("ExceptionInformation", &self.ExceptionInformation).finish()
    }
}
impl ::core::default::Default for EXIT_PROCESS_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXIT_PROCESS_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwExitCode == other.dwExitCode
    }
}
impl ::core::cmp::Eq for EXIT_PROCESS_DEBUG_INFO {}
impl ::core::fmt::Debug for EXIT_PROCESS_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXIT_PROCESS_DEBUG_INFO").field("dwExitCode", &self.dwExitCode).finish()
    }
}
impl ::core::default::Default for EXIT_THREAD_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXIT_THREAD_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwExitCode == other.dwExitCode
    }
}
impl ::core::cmp::Eq for EXIT_THREAD_DEBUG_INFO {}
impl ::core::fmt::Debug for EXIT_THREAD_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXIT_THREAD_DEBUG_INFO").field("dwExitCode", &self.dwExitCode).finish()
    }
}
impl ::core::default::Default for EXTSTACKTRACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTSTACKTRACE {
    fn eq(&self, other: &Self) -> bool {
        self.FramePointer == other.FramePointer && self.ProgramCounter == other.ProgramCounter && self.ReturnAddress == other.ReturnAddress && self.Args == other.Args
    }
}
impl ::core::cmp::Eq for EXTSTACKTRACE {}
impl ::core::fmt::Debug for EXTSTACKTRACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTSTACKTRACE").field("FramePointer", &self.FramePointer).field("ProgramCounter", &self.ProgramCounter).field("ReturnAddress", &self.ReturnAddress).field("Args", &self.Args).finish()
    }
}
impl ::core::default::Default for EXTSTACKTRACE32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTSTACKTRACE32 {
    fn eq(&self, other: &Self) -> bool {
        self.FramePointer == other.FramePointer && self.ProgramCounter == other.ProgramCounter && self.ReturnAddress == other.ReturnAddress && self.Args == other.Args
    }
}
impl ::core::cmp::Eq for EXTSTACKTRACE32 {}
impl ::core::fmt::Debug for EXTSTACKTRACE32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTSTACKTRACE32").field("FramePointer", &self.FramePointer).field("ProgramCounter", &self.ProgramCounter).field("ReturnAddress", &self.ReturnAddress).field("Args", &self.Args).finish()
    }
}
impl ::core::default::Default for EXTSTACKTRACE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTSTACKTRACE64 {
    fn eq(&self, other: &Self) -> bool {
        self.FramePointer == other.FramePointer && self.ProgramCounter == other.ProgramCounter && self.ReturnAddress == other.ReturnAddress && self.Args == other.Args
    }
}
impl ::core::cmp::Eq for EXTSTACKTRACE64 {}
impl ::core::fmt::Debug for EXTSTACKTRACE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTSTACKTRACE64").field("FramePointer", &self.FramePointer).field("ProgramCounter", &self.ProgramCounter).field("ReturnAddress", &self.ReturnAddress).field("Args", &self.Args).finish()
    }
}
impl ::core::default::Default for EXT_API_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXT_API_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.Revision == other.Revision && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for EXT_API_VERSION {}
impl ::core::fmt::Debug for EXT_API_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXT_API_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("Revision", &self.Revision).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXT_FIND_FILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXT_FIND_FILE {
    fn eq(&self, other: &Self) -> bool {
        self.FileName == other.FileName && self.IndexedSize == other.IndexedSize && self.ImageTimeDateStamp == other.ImageTimeDateStamp && self.ImageCheckSum == other.ImageCheckSum && self.ExtraInfo == other.ExtraInfo && self.ExtraInfoSize == other.ExtraInfoSize && self.Flags == other.Flags && self.FileMapping == other.FileMapping && self.FileMappingSize == other.FileMappingSize && self.FileHandle == other.FileHandle && self.FoundFileName == other.FoundFileName && self.FoundFileNameChars == other.FoundFileNameChars
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXT_FIND_FILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXT_FIND_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXT_FIND_FILE")
            .field("FileName", &self.FileName)
            .field("IndexedSize", &self.IndexedSize)
            .field("ImageTimeDateStamp", &self.ImageTimeDateStamp)
            .field("ImageCheckSum", &self.ImageCheckSum)
            .field("ExtraInfo", &self.ExtraInfo)
            .field("ExtraInfoSize", &self.ExtraInfoSize)
            .field("Flags", &self.Flags)
            .field("FileMapping", &self.FileMapping)
            .field("FileMappingSize", &self.FileMappingSize)
            .field("FileHandle", &self.FileHandle)
            .field("FoundFileName", &self.FoundFileName)
            .field("FoundFileNameChars", &self.FoundFileNameChars)
            .finish()
    }
}
impl ::core::default::Default for EXT_MATCH_PATTERN_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXT_MATCH_PATTERN_A {
    fn eq(&self, other: &Self) -> bool {
        self.Str == other.Str && self.Pattern == other.Pattern && self.CaseSensitive == other.CaseSensitive
    }
}
impl ::core::cmp::Eq for EXT_MATCH_PATTERN_A {}
impl ::core::fmt::Debug for EXT_MATCH_PATTERN_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXT_MATCH_PATTERN_A").field("Str", &self.Str).field("Pattern", &self.Pattern).field("CaseSensitive", &self.CaseSensitive).finish()
    }
}
impl ::core::default::Default for EXT_TDOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXT_TDOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXT_TDOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXT_TYPED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXT_TYPED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.Flags == other.Flags && self.InData == other.InData && self.OutData == other.OutData && self.InStrIndex == other.InStrIndex && self.In32 == other.In32 && self.Out32 == other.Out32 && self.In64 == other.In64 && self.Out64 == other.Out64 && self.StrBufferIndex == other.StrBufferIndex && self.StrBufferChars == other.StrBufferChars && self.StrCharsNeeded == other.StrCharsNeeded && self.DataBufferIndex == other.DataBufferIndex && self.DataBufferBytes == other.DataBufferBytes && self.DataBytesNeeded == other.DataBytesNeeded && self.Status == other.Status && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for EXT_TYPED_DATA {}
impl ::core::fmt::Debug for EXT_TYPED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXT_TYPED_DATA")
            .field("Operation", &self.Operation)
            .field("Flags", &self.Flags)
            .field("InData", &self.InData)
            .field("OutData", &self.OutData)
            .field("InStrIndex", &self.InStrIndex)
            .field("In32", &self.In32)
            .field("Out32", &self.Out32)
            .field("In64", &self.In64)
            .field("Out64", &self.Out64)
            .field("StrBufferIndex", &self.StrBufferIndex)
            .field("StrBufferChars", &self.StrBufferChars)
            .field("StrCharsNeeded", &self.StrCharsNeeded)
            .field("DataBufferIndex", &self.DataBufferIndex)
            .field("DataBufferBytes", &self.DataBufferBytes)
            .field("DataBytesNeeded", &self.DataBytesNeeded)
            .field("Status", &self.Status)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for EX_PROP_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EX_PROP_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EX_PROP_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ErrorClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ErrorClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorClass").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ExtendedDebugPropertyInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FACILITY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FACILITY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FACILITY_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FIELD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FORMAT_MESSAGE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FORMAT_MESSAGE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORMAT_MESSAGE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FORMAT_MESSAGE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FORMAT_MESSAGE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FPO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FPO_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffStart == other.ulOffStart && self.cbProcSize == other.cbProcSize && self.cdwLocals == other.cdwLocals && self.cdwParams == other.cdwParams && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for FPO_DATA {}
impl ::core::fmt::Debug for FPO_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FPO_DATA").field("ulOffStart", &self.ulOffStart).field("cbProcSize", &self.cbProcSize).field("cdwLocals", &self.cdwLocals).field("cdwParams", &self.cdwParams).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for GET_CONTEXT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_CONTEXT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.ContextSize == other.ContextSize && self.pContext == other.pContext
    }
}
impl ::core::cmp::Eq for GET_CONTEXT_EX {}
impl ::core::fmt::Debug for GET_CONTEXT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_CONTEXT_EX").field("Status", &self.Status).field("ContextSize", &self.ContextSize).field("pContext", &self.pContext).finish()
    }
}
impl ::core::default::Default for GET_CURRENT_PROCESS_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_CURRENT_PROCESS_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.CurrentThread == other.CurrentThread && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for GET_CURRENT_PROCESS_ADDRESS {}
impl ::core::fmt::Debug for GET_CURRENT_PROCESS_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_CURRENT_PROCESS_ADDRESS").field("Processor", &self.Processor).field("CurrentThread", &self.CurrentThread).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for GET_CURRENT_THREAD_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_CURRENT_THREAD_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for GET_CURRENT_THREAD_ADDRESS {}
impl ::core::fmt::Debug for GET_CURRENT_THREAD_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_CURRENT_THREAD_ADDRESS").field("Processor", &self.Processor).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for GET_EXPRESSION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_EXPRESSION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Expression == other.Expression && self.Remainder == other.Remainder && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for GET_EXPRESSION_EX {}
impl ::core::fmt::Debug for GET_EXPRESSION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_EXPRESSION_EX").field("Expression", &self.Expression).field("Remainder", &self.Remainder).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for GET_INPUT_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_INPUT_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Prompt == other.Prompt && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.InputSize == other.InputSize
    }
}
impl ::core::cmp::Eq for GET_INPUT_LINE {}
impl ::core::fmt::Debug for GET_INPUT_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_INPUT_LINE").field("Prompt", &self.Prompt).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("InputSize", &self.InputSize).finish()
    }
}
impl ::core::default::Default for GET_PEB_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_PEB_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentThread == other.CurrentThread && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for GET_PEB_ADDRESS {}
impl ::core::fmt::Debug for GET_PEB_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_PEB_ADDRESS").field("CurrentThread", &self.CurrentThread).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for GET_SET_SYMPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_SET_SYMPATH {
    fn eq(&self, other: &Self) -> bool {
        self.Args == other.Args && self.Result == other.Result && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for GET_SET_SYMPATH {}
impl ::core::fmt::Debug for GET_SET_SYMPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_SET_SYMPATH").field("Args", &self.Args).field("Result", &self.Result).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for GET_TEB_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_TEB_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address
    }
}
impl ::core::cmp::Eq for GET_TEB_ADDRESS {}
impl ::core::fmt::Debug for GET_TEB_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_TEB_ADDRESS").field("Address", &self.Address).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScript {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScript {}
impl ::core::fmt::Debug for IActiveScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScript").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptAuthor {}
impl ::core::fmt::Debug for IActiveScriptAuthor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptAuthor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptAuthorProcedure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptAuthorProcedure {}
impl ::core::fmt::Debug for IActiveScriptAuthorProcedure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptAuthorProcedure").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptDebug32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptDebug32 {}
impl ::core::fmt::Debug for IActiveScriptDebug32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptDebug32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptDebug64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptDebug64 {}
impl ::core::fmt::Debug for IActiveScriptDebug64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptDebug64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptEncode {}
impl ::core::fmt::Debug for IActiveScriptEncode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptEncode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptError {}
impl ::core::fmt::Debug for IActiveScriptError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptError").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptError64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptError64 {}
impl ::core::fmt::Debug for IActiveScriptError64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptError64").field(&self.0).finish()
    }
}
impl IActiveScriptError64 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetExceptionInfo)(::windows::core::Vtable::as_raw(self), pexcepinfo).ok()
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSourcePosition)(::windows::core::Vtable::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
    pub unsafe fn GetSourceLineText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSourceLineText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IActiveScriptErrorDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptErrorDebug {}
impl ::core::fmt::Debug for IActiveScriptErrorDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptErrorDebug").field(&self.0).finish()
    }
}
impl IActiveScriptErrorDebug {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetExceptionInfo)(::windows::core::Vtable::as_raw(self), pexcepinfo).ok()
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSourcePosition)(::windows::core::Vtable::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
    pub unsafe fn GetSourceLineText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSourceLineText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IActiveScriptErrorDebug110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptErrorDebug110 {}
impl ::core::fmt::Debug for IActiveScriptErrorDebug110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptErrorDebug110").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptGarbageCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptGarbageCollector {}
impl ::core::fmt::Debug for IActiveScriptGarbageCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptGarbageCollector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptHostEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptHostEncode {}
impl ::core::fmt::Debug for IActiveScriptHostEncode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptHostEncode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParse32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParse32 {}
impl ::core::fmt::Debug for IActiveScriptParse32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParse32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParse64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParse64 {}
impl ::core::fmt::Debug for IActiveScriptParse64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParse64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure2_32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure2_32 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure2_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure2_32").field(&self.0).finish()
    }
}
impl IActiveScriptParseProcedure2_32 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParseProcedureText)(::windows::core::Vtable::as_raw(self), pstrcode.into().abi(), pstrformalparams.into().abi(), pstrprocedurename.into().abi(), pstritemname.into().abi(), punkcontext.into().abi(), pstrdelimiter.into().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure2_64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure2_64 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure2_64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure2_64").field(&self.0).finish()
    }
}
impl IActiveScriptParseProcedure2_64 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParseProcedureText)(::windows::core::Vtable::as_raw(self), pstrcode.into().abi(), pstrformalparams.into().abi(), pstrprocedurename.into().abi(), pstritemname.into().abi(), punkcontext.into().abi(), pstrdelimiter.into().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure32 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure64 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParseProcedureOld32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedureOld32 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedureOld32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedureOld32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptParseProcedureOld64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedureOld64 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedureOld64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedureOld64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerCallback {}
impl ::core::fmt::Debug for IActiveScriptProfilerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerCallback2 {}
impl ::core::fmt::Debug for IActiveScriptProfilerCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerCallback2").field(&self.0).finish()
    }
}
impl IActiveScriptProfilerCallback2 {
    pub unsafe fn Initialize(&self, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), dwcontext).ok()
    }
    pub unsafe fn Shutdown(&self, hrreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self), hrreason).ok()
    }
    pub unsafe fn ScriptCompiled<P0>(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ScriptCompiled)(::windows::core::Vtable::as_raw(self), scriptid, r#type, pidebugdocumentcontext.into().abi()).ok()
    }
    pub unsafe fn FunctionCompiled<P0, P1, P2>(&self, functionid: i32, scriptid: i32, pwszfunctionname: P0, pwszfunctionnamehint: P1, pidebugdocumentcontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FunctionCompiled)(::windows::core::Vtable::as_raw(self), functionid, scriptid, pwszfunctionname.into().abi(), pwszfunctionnamehint.into().abi(), pidebugdocumentcontext.into().abi()).ok()
    }
    pub unsafe fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnFunctionEnter)(::windows::core::Vtable::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnFunctionExit)(::windows::core::Vtable::as_raw(self), scriptid, functionid).ok()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerCallback3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerCallback3 {}
impl ::core::fmt::Debug for IActiveScriptProfilerCallback3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerCallback3").field(&self.0).finish()
    }
}
impl IActiveScriptProfilerCallback3 {
    pub unsafe fn Initialize(&self, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), dwcontext).ok()
    }
    pub unsafe fn Shutdown(&self, hrreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Shutdown)(::windows::core::Vtable::as_raw(self), hrreason).ok()
    }
    pub unsafe fn ScriptCompiled<P0>(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ScriptCompiled)(::windows::core::Vtable::as_raw(self), scriptid, r#type, pidebugdocumentcontext.into().abi()).ok()
    }
    pub unsafe fn FunctionCompiled<P0, P1, P2>(&self, functionid: i32, scriptid: i32, pwszfunctionname: P0, pwszfunctionnamehint: P1, pidebugdocumentcontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FunctionCompiled)(::windows::core::Vtable::as_raw(self), functionid, scriptid, pwszfunctionname.into().abi(), pwszfunctionnamehint.into().abi(), pidebugdocumentcontext.into().abi()).ok()
    }
    pub unsafe fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnFunctionEnter)(::windows::core::Vtable::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnFunctionExit)(::windows::core::Vtable::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionEnterByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnFunctionEnterByName)(::windows::core::Vtable::as_raw(self), pwszfunctionname.into().abi(), r#type).ok()
    }
    pub unsafe fn OnFunctionExitByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnFunctionExitByName)(::windows::core::Vtable::as_raw(self), pwszfunctionname.into().abi(), r#type).ok()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl2 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl2").field(&self.0).finish()
    }
}
impl IActiveScriptProfilerControl2 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StartProfiling)(::windows::core::Vtable::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProfilerEventMask)(::windows::core::Vtable::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StopProfiling)(::windows::core::Vtable::as_raw(self), hrshutdownreason).ok()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl3 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl3").field(&self.0).finish()
    }
}
impl IActiveScriptProfilerControl3 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StartProfiling)(::windows::core::Vtable::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProfilerEventMask)(::windows::core::Vtable::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StopProfiling)(::windows::core::Vtable::as_raw(self), hrshutdownreason).ok()
    }
    pub unsafe fn CompleteProfilerStart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CompleteProfilerStart)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PrepareProfilerStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PrepareProfilerStop)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl4 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl4").field(&self.0).finish()
    }
}
impl IActiveScriptProfilerControl4 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StartProfiling)(::windows::core::Vtable::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetProfilerEventMask)(::windows::core::Vtable::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StopProfiling)(::windows::core::Vtable::as_raw(self), hrshutdownreason).ok()
    }
    pub unsafe fn CompleteProfilerStart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CompleteProfilerStart)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PrepareProfilerStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PrepareProfilerStop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EnumHeap(&self) -> ::windows::core::Result<IActiveScriptProfilerHeapEnum> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumHeap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl5 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl5").field(&self.0).finish()
    }
}
impl IActiveScriptProfilerControl5 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.StartProfiling)(::windows::core::Vtable::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetProfilerEventMask)(::windows::core::Vtable::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.StopProfiling)(::windows::core::Vtable::as_raw(self), hrshutdownreason).ok()
    }
    pub unsafe fn CompleteProfilerStart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CompleteProfilerStart)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PrepareProfilerStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PrepareProfilerStop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EnumHeap(&self) -> ::windows::core::Result<IActiveScriptProfilerHeapEnum> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumHeap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SummarizeHeap(&self, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SummarizeHeap)(::windows::core::Vtable::as_raw(self), heapsummary).ok()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProfilerHeapEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerHeapEnum {}
impl ::core::fmt::Debug for IActiveScriptProfilerHeapEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerHeapEnum").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProperty {}
impl ::core::fmt::Debug for IActiveScriptProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProperty").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSIPInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSIPInfo {}
impl ::core::fmt::Debug for IActiveScriptSIPInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSIPInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSite {}
impl ::core::fmt::Debug for IActiveScriptSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSiteDebug32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteDebug32 {}
impl ::core::fmt::Debug for IActiveScriptSiteDebug32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteDebug32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSiteDebug64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteDebug64 {}
impl ::core::fmt::Debug for IActiveScriptSiteDebug64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteDebug64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSiteDebugEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteDebugEx {}
impl ::core::fmt::Debug for IActiveScriptSiteDebugEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteDebugEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSiteInterruptPoll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteInterruptPoll {}
impl ::core::fmt::Debug for IActiveScriptSiteInterruptPoll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteInterruptPoll").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSiteTraceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteTraceInfo {}
impl ::core::fmt::Debug for IActiveScriptSiteTraceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteTraceInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSiteUIControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteUIControl {}
impl ::core::fmt::Debug for IActiveScriptSiteUIControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteUIControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptSiteWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteWindow {}
impl ::core::fmt::Debug for IActiveScriptSiteWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteWindow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptStats {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptStats {}
impl ::core::fmt::Debug for IActiveScriptStats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptStats").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptStringCompare {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptStringCompare {}
impl ::core::fmt::Debug for IActiveScriptStringCompare {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptStringCompare").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptTraceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptTraceInfo {}
impl ::core::fmt::Debug for IActiveScriptTraceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptTraceInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveScriptWinRTErrorDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptWinRTErrorDebug {}
impl ::core::fmt::Debug for IActiveScriptWinRTErrorDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptWinRTErrorDebug").field(&self.0).finish()
    }
}
impl IActiveScriptWinRTErrorDebug {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetExceptionInfo)(::windows::core::Vtable::as_raw(self), pexcepinfo).ok()
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSourcePosition)(::windows::core::Vtable::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
    pub unsafe fn GetSourceLineText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSourceLineText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IApplicationDebugger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDebugger {}
impl ::core::fmt::Debug for IApplicationDebugger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDebugger").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IApplicationDebuggerUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDebuggerUI {}
impl ::core::fmt::Debug for IApplicationDebuggerUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDebuggerUI").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBindEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindEventHandler {}
impl ::core::fmt::Debug for IBindEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICodeAddressConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICodeAddressConcept {}
impl ::core::fmt::Debug for ICodeAddressConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICodeAddressConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IComparableConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComparableConcept {}
impl ::core::fmt::Debug for IComparableConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComparableConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelConcept {}
impl ::core::fmt::Debug for IDataModelConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelManager {}
impl ::core::fmt::Debug for IDataModelManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelManager2 {}
impl ::core::fmt::Debug for IDataModelManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelManager2").field(&self.0).finish()
    }
}
impl IDataModelManager2 {
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CreateNoValue(&self) -> ::windows::core::Result<IModelObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateNoValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateErrorObject<P0>(&self, hrerror: ::windows::core::HRESULT, pwszmessage: P0) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateErrorObject)(::windows::core::Vtable::as_raw(self), hrerror, pwszmessage.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypedObject<P0, P1>(&self, context: P0, objectlocation: Location, objecttype: P1) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDebugHostType>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTypedObject)(::windows::core::Vtable::as_raw(self), context.into().abi(), ::core::mem::transmute(objectlocation), objecttype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypedObjectReference<P0, P1>(&self, context: P0, objectlocation: Location, objecttype: P1) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDebugHostType>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTypedObjectReference)(::windows::core::Vtable::as_raw(self), context.into().abi(), ::core::mem::transmute(objectlocation), objecttype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSyntheticObject<P0>(&self, context: P0) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSyntheticObject)(::windows::core::Vtable::as_raw(self), context.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDataModelObject<P0>(&self, datamodel: P0) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDataModelConcept>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDataModelObject)(::windows::core::Vtable::as_raw(self), datamodel.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateIntrinsicObject(&self, objectkind: ModelObjectKind, intrinsicdata: *const super::super::Com::VARIANT) -> ::windows::core::Result<IModelObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateIntrinsicObject)(::windows::core::Vtable::as_raw(self), objectkind, intrinsicdata, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTypedIntrinsicObject<P0>(&self, intrinsicdata: *const super::super::Com::VARIANT, r#type: P0) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostType>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTypedIntrinsicObject)(::windows::core::Vtable::as_raw(self), intrinsicdata, r#type.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetModelForTypeSignature<P0>(&self, typesignature: P0) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostTypeSignature>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetModelForTypeSignature)(::windows::core::Vtable::as_raw(self), typesignature.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetModelForType<P0>(&self, r#type: P0, datamodel: *mut ::core::option::Option<IModelObject>, typesignature: ::core::option::Option<*mut ::core::option::Option<IDebugHostTypeSignature>>, wildcardmatches: ::core::option::Option<*mut ::core::option::Option<IDebugHostSymbolEnumerator>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostType>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetModelForType)(::windows::core::Vtable::as_raw(self), r#type.into().abi(), ::core::mem::transmute(datamodel), ::core::mem::transmute(typesignature.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(wildcardmatches.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn RegisterModelForTypeSignature<P0, P1>(&self, typesignature: P0, datamodel: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostTypeSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterModelForTypeSignature)(::windows::core::Vtable::as_raw(self), typesignature.into().abi(), datamodel.into().abi()).ok()
    }
    pub unsafe fn UnregisterModelForTypeSignature<P0, P1>(&self, datamodel: P0, typesignature: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDebugHostTypeSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnregisterModelForTypeSignature)(::windows::core::Vtable::as_raw(self), datamodel.into().abi(), typesignature.into().abi()).ok()
    }
    pub unsafe fn RegisterExtensionForTypeSignature<P0, P1>(&self, typesignature: P0, datamodel: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostTypeSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterExtensionForTypeSignature)(::windows::core::Vtable::as_raw(self), typesignature.into().abi(), datamodel.into().abi()).ok()
    }
    pub unsafe fn UnregisterExtensionForTypeSignature<P0, P1>(&self, datamodel: P0, typesignature: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDebugHostTypeSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnregisterExtensionForTypeSignature)(::windows::core::Vtable::as_raw(self), datamodel.into().abi(), typesignature.into().abi()).ok()
    }
    pub unsafe fn CreateMetadataStore<P0>(&self, parentstore: P0) -> ::windows::core::Result<IKeyStore>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IKeyStore>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMetadataStore)(::windows::core::Vtable::as_raw(self), parentstore.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootNamespace(&self) -> ::windows::core::Result<IModelObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRootNamespace)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterNamedModel<P0, P1>(&self, modelname: P0, modeobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterNamedModel)(::windows::core::Vtable::as_raw(self), modelname.into().abi(), modeobject.into().abi()).ok()
    }
    pub unsafe fn UnregisterNamedModel<P0>(&self, modelname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnregisterNamedModel)(::windows::core::Vtable::as_raw(self), modelname.into().abi()).ok()
    }
    pub unsafe fn AcquireNamedModel<P0>(&self, modelname: P0) -> ::windows::core::Result<IModelObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AcquireNamedModel)(::windows::core::Vtable::as_raw(self), modelname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDataModelNameBinder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelNameBinder {}
impl ::core::fmt::Debug for IDataModelNameBinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelNameBinder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScript {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScript {}
impl ::core::fmt::Debug for IDataModelScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScript").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptClient {}
impl ::core::fmt::Debug for IDataModelScriptClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebug {}
impl ::core::fmt::Debug for IDataModelScriptDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebug").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebug2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebug2 {}
impl ::core::fmt::Debug for IDataModelScriptDebug2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebug2").field(&self.0).finish()
    }
}
impl IDataModelScriptDebug2 {
    pub unsafe fn GetDebugState(&self) -> ScriptDebugState {
        (::windows::core::Vtable::vtable(self).base__.GetDebugState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentPosition(&self, currentposition: *mut ScriptDebugPosition, positionspanend: ::core::option::Option<*mut ScriptDebugPosition>, linetext: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPosition)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(positionspanend.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(linetext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStack(&self) -> ::windows::core::Result<IDataModelScriptDebugStack> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStack)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBreakpoint(&self, lineposition: u32, columnposition: u32) -> ::windows::core::Result<IDataModelScriptDebugBreakpoint> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SetBreakpoint)(::windows::core::Vtable::as_raw(self), lineposition, columnposition, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindBreakpointById(&self, breakpointid: u64) -> ::windows::core::Result<IDataModelScriptDebugBreakpoint> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindBreakpointById)(::windows::core::Vtable::as_raw(self), breakpointid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateBreakpoints(&self) -> ::windows::core::Result<IDataModelScriptDebugBreakpointEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateBreakpoints)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEventFilter(&self, eventfilter: ScriptDebugEventFilter) -> ::windows::core::Result<bool> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEventFilter)(::windows::core::Vtable::as_raw(self), eventfilter, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventFilter(&self, eventfilter: ScriptDebugEventFilter, isbreakenabled: u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEventFilter)(::windows::core::Vtable::as_raw(self), eventfilter, isbreakenabled).ok()
    }
    pub unsafe fn StartDebugging<P0>(&self, debugclient: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDataModelScriptDebugClient>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StartDebugging)(::windows::core::Vtable::as_raw(self), debugclient.into().abi()).ok()
    }
    pub unsafe fn StopDebugging<P0>(&self, debugclient: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDataModelScriptDebugClient>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StopDebugging)(::windows::core::Vtable::as_raw(self), debugclient.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebugBreakpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebugBreakpoint {}
impl ::core::fmt::Debug for IDataModelScriptDebugBreakpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebugBreakpoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebugBreakpointEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebugBreakpointEnumerator {}
impl ::core::fmt::Debug for IDataModelScriptDebugBreakpointEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebugBreakpointEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebugClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebugClient {}
impl ::core::fmt::Debug for IDataModelScriptDebugClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebugClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebugStack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebugStack {}
impl ::core::fmt::Debug for IDataModelScriptDebugStack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebugStack").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebugStackFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebugStackFrame {}
impl ::core::fmt::Debug for IDataModelScriptDebugStackFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebugStackFrame").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptDebugVariableSetEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptDebugVariableSetEnumerator {}
impl ::core::fmt::Debug for IDataModelScriptDebugVariableSetEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptDebugVariableSetEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptHostContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptHostContext {}
impl ::core::fmt::Debug for IDataModelScriptHostContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptHostContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptManager {}
impl ::core::fmt::Debug for IDataModelScriptManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptProvider {}
impl ::core::fmt::Debug for IDataModelScriptProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptProviderEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptProviderEnumerator {}
impl ::core::fmt::Debug for IDataModelScriptProviderEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptProviderEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptTemplate {}
impl ::core::fmt::Debug for IDataModelScriptTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptTemplate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataModelScriptTemplateEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataModelScriptTemplateEnumerator {}
impl ::core::fmt::Debug for IDataModelScriptTemplateEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataModelScriptTemplateEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAdvanced {}
impl ::core::fmt::Debug for IDebugAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAdvanced").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAdvanced2 {}
impl ::core::fmt::Debug for IDebugAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAdvanced2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAdvanced3 {}
impl ::core::fmt::Debug for IDebugAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAdvanced3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugAdvanced4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAdvanced4 {}
impl ::core::fmt::Debug for IDebugAdvanced4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAdvanced4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugApplication11032 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication11032 {}
impl ::core::fmt::Debug for IDebugApplication11032 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication11032").field(&self.0).finish()
    }
}
impl IDebugApplication11032 {
    pub unsafe fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDebuggerOptions)(::windows::core::Vtable::as_raw(self), mask, value).ok()
    }
    pub unsafe fn GetCurrentDebuggerOptions(&self) -> ::windows::core::Result<SCRIPT_DEBUGGER_OPTIONS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentDebuggerOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMainThread(&self) -> ::windows::core::Result<IRemoteDebugApplicationThread> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMainThread)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugApplication11064 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication11064 {}
impl ::core::fmt::Debug for IDebugApplication11064 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication11064").field(&self.0).finish()
    }
}
impl IDebugApplication11064 {
    pub unsafe fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDebuggerOptions)(::windows::core::Vtable::as_raw(self), mask, value).ok()
    }
    pub unsafe fn GetCurrentDebuggerOptions(&self) -> ::windows::core::Result<SCRIPT_DEBUGGER_OPTIONS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentDebuggerOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMainThread(&self) -> ::windows::core::Result<IRemoteDebugApplicationThread> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMainThread)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugApplication32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication32 {}
impl ::core::fmt::Debug for IDebugApplication32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication32").field(&self.0).finish()
    }
}
impl IDebugApplication32 {
    pub unsafe fn ResumeFromBreakPoint<P0>(&self, prptfocus: P0, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRemoteDebugApplicationThread>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResumeFromBreakPoint)(::windows::core::Vtable::as_raw(self), prptfocus.into().abi(), bra, era).ok()
    }
    pub unsafe fn CauseBreak(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CauseBreak)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ConnectDebugger<P0>(&self, pad: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IApplicationDebugger>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConnectDebugger)(::windows::core::Vtable::as_raw(self), pad.into().abi()).ok()
    }
    pub unsafe fn DisconnectDebugger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisconnectDebugger)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDebugger(&self) -> ::windows::core::Result<IApplicationDebugger> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDebugger)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateInstanceAtApplication<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInstanceAtApplication)(::windows::core::Vtable::as_raw(self), rclsid, punkouter.into().abi(), dwclscontext, riid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryAlive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryAlive)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<IEnumRemoteDebugApplicationThreads> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumThreads)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRootNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumGlobalExpressionContexts(&self) -> ::windows::core::Result<IEnumDebugExpressionContexts> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumGlobalExpressionContexts)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugApplication64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication64 {}
impl ::core::fmt::Debug for IDebugApplication64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication64").field(&self.0).finish()
    }
}
impl IDebugApplication64 {
    pub unsafe fn ResumeFromBreakPoint<P0>(&self, prptfocus: P0, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRemoteDebugApplicationThread>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResumeFromBreakPoint)(::windows::core::Vtable::as_raw(self), prptfocus.into().abi(), bra, era).ok()
    }
    pub unsafe fn CauseBreak(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CauseBreak)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ConnectDebugger<P0>(&self, pad: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IApplicationDebugger>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConnectDebugger)(::windows::core::Vtable::as_raw(self), pad.into().abi()).ok()
    }
    pub unsafe fn DisconnectDebugger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisconnectDebugger)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDebugger(&self) -> ::windows::core::Result<IApplicationDebugger> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDebugger)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateInstanceAtApplication<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInstanceAtApplication)(::windows::core::Vtable::as_raw(self), rclsid, punkouter.into().abi(), dwclscontext, riid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryAlive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryAlive)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<IEnumRemoteDebugApplicationThreads> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumThreads)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRootNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumGlobalExpressionContexts(&self) -> ::windows::core::Result<IEnumDebugExpressionContexts> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumGlobalExpressionContexts)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationNode {}
impl ::core::fmt::Debug for IDebugApplicationNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationNode").field(&self.0).finish()
    }
}
impl IDebugApplicationNode {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), dnt, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDocumentClassId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<IDebugDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDocument)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationNode100 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationNode100 {}
impl ::core::fmt::Debug for IDebugApplicationNode100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationNode100").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationNodeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationNodeEvents {}
impl ::core::fmt::Debug for IDebugApplicationNodeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationNodeEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationThread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread {}
impl ::core::fmt::Debug for IDebugApplicationThread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread").field(&self.0).finish()
    }
}
impl IDebugApplicationThread {
    pub unsafe fn GetSystemThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemThreadId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<IRemoteDebugApplication> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetApplication)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumStackFrames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self, pbstrdescription: *mut ::windows::core::BSTR, pbstrstate: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdescription), ::core::mem::transmute(pbstrstate)).ok()
    }
    pub unsafe fn SetNextStatement<P0, P1>(&self, pstackframe: P0, pcodecontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugStackFrame>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDebugCodeContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNextStatement)(::windows::core::Vtable::as_raw(self), pstackframe.into().abi(), pcodecontext.into().abi()).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Suspend)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Resume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSuspendCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSuspendCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationThread11032 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread11032 {}
impl ::core::fmt::Debug for IDebugApplicationThread11032 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread11032").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationThread11064 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread11064 {}
impl ::core::fmt::Debug for IDebugApplicationThread11064 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread11064").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationThread64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread64 {}
impl ::core::fmt::Debug for IDebugApplicationThread64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread64").field(&self.0).finish()
    }
}
impl IDebugApplicationThread64 {
    pub unsafe fn GetSystemThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSystemThreadId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<IRemoteDebugApplication> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetApplication)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumStackFrames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self, pbstrdescription: *mut ::windows::core::BSTR, pbstrstate: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdescription), ::core::mem::transmute(pbstrstate)).ok()
    }
    pub unsafe fn SetNextStatement<P0, P1>(&self, pstackframe: P0, pcodecontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugStackFrame>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDebugCodeContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetNextStatement)(::windows::core::Vtable::as_raw(self), pstackframe.into().abi(), pcodecontext.into().abi()).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Suspend)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Resume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSuspendCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSuspendCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SynchronousCallIntoThread32<P0>(&self, pstcb: P0, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugThreadCall32>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SynchronousCallIntoThread32)(::windows::core::Vtable::as_raw(self), pstcb.into().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    pub unsafe fn QueryIsCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryIsCurrentThread)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn QueryIsDebuggerThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryIsDebuggerThread)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), pstrdescription.into().abi()).ok()
    }
    pub unsafe fn SetStateString<P0>(&self, pstrstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStateString)(::windows::core::Vtable::as_raw(self), pstrstate.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDebugApplicationThreadEvents110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThreadEvents110 {}
impl ::core::fmt::Debug for IDebugApplicationThreadEvents110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThreadEvents110").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAsyncOperation {}
impl ::core::fmt::Debug for IDebugAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAsyncOperation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugAsyncOperationCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAsyncOperationCallBack {}
impl ::core::fmt::Debug for IDebugAsyncOperationCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAsyncOperationCallBack").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugBreakpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugBreakpoint {}
impl ::core::fmt::Debug for IDebugBreakpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugBreakpoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugBreakpoint2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugBreakpoint2 {}
impl ::core::fmt::Debug for IDebugBreakpoint2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugBreakpoint2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugBreakpoint3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugBreakpoint3 {}
impl ::core::fmt::Debug for IDebugBreakpoint3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugBreakpoint3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient {}
impl ::core::fmt::Debug for IDebugClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient2 {}
impl ::core::fmt::Debug for IDebugClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient3 {}
impl ::core::fmt::Debug for IDebugClient3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient4 {}
impl ::core::fmt::Debug for IDebugClient4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient5 {}
impl ::core::fmt::Debug for IDebugClient5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient5").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient6 {}
impl ::core::fmt::Debug for IDebugClient6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient6").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient7 {}
impl ::core::fmt::Debug for IDebugClient7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient7").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugClient8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugClient8 {}
impl ::core::fmt::Debug for IDebugClient8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugClient8").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugCodeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugCodeContext {}
impl ::core::fmt::Debug for IDebugCodeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugCodeContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugControl {}
impl ::core::fmt::Debug for IDebugControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugControl2 {}
impl ::core::fmt::Debug for IDebugControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugControl2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugControl3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugControl3 {}
impl ::core::fmt::Debug for IDebugControl3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugControl3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugControl4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugControl4 {}
impl ::core::fmt::Debug for IDebugControl4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugControl4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugControl5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugControl5 {}
impl ::core::fmt::Debug for IDebugControl5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugControl5").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugControl6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugControl6 {}
impl ::core::fmt::Debug for IDebugControl6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugControl6").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugControl7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugControl7 {}
impl ::core::fmt::Debug for IDebugControl7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugControl7").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugCookie {}
impl ::core::fmt::Debug for IDebugCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugCookie").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDataSpaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDataSpaces {}
impl ::core::fmt::Debug for IDebugDataSpaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDataSpaces").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDataSpaces2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDataSpaces2 {}
impl ::core::fmt::Debug for IDebugDataSpaces2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDataSpaces2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDataSpaces3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDataSpaces3 {}
impl ::core::fmt::Debug for IDebugDataSpaces3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDataSpaces3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDataSpaces4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDataSpaces4 {}
impl ::core::fmt::Debug for IDebugDataSpaces4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDataSpaces4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocument {}
impl ::core::fmt::Debug for IDebugDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocument").field(&self.0).finish()
    }
}
impl IDebugDocument {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), dnt, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDocumentClassId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentContext {}
impl ::core::fmt::Debug for IDebugDocumentContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentHelper32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentHelper32 {}
impl ::core::fmt::Debug for IDebugDocumentHelper32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentHelper32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentHelper64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentHelper64 {}
impl ::core::fmt::Debug for IDebugDocumentHelper64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentHelper64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentHost {}
impl ::core::fmt::Debug for IDebugDocumentHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentInfo {}
impl ::core::fmt::Debug for IDebugDocumentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentProvider {}
impl ::core::fmt::Debug for IDebugDocumentProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentProvider").field(&self.0).finish()
    }
}
impl IDebugDocumentProvider {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), dnt, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDocumentClassId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentText {}
impl ::core::fmt::Debug for IDebugDocumentText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentText").field(&self.0).finish()
    }
}
impl IDebugDocumentText {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), dnt, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDocumentClassId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentTextAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentTextAuthor {}
impl ::core::fmt::Debug for IDebugDocumentTextAuthor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentTextAuthor").field(&self.0).finish()
    }
}
impl IDebugDocumentTextAuthor {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetName)(::windows::core::Vtable::as_raw(self), dnt, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDocumentClassId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentAttributes(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDocumentAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), pcnumlines, pcnumchars).ok()
    }
    pub unsafe fn GetPositionOfLine(&self, clinenumber: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPositionOfLine)(::windows::core::Vtable::as_raw(self), clinenumber, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLineOfPosition(&self, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLineOfPosition)(::windows::core::Vtable::as_raw(self), ccharacterposition, pclinenumber, pccharacteroffsetinline).ok()
    }
    pub unsafe fn GetText(&self, ccharacterposition: u32, pchartext: &mut [u16], pstatextattr: ::core::option::Option<*mut u16>, pcnumchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetText)(::windows::core::Vtable::as_raw(self), ccharacterposition, ::core::mem::transmute(pchartext.as_ptr()), ::core::mem::transmute(pstatextattr.unwrap_or(::std::ptr::null_mut())), pcnumchars, pchartext.len() as _).ok()
    }
    pub unsafe fn GetPositionOfContext<P0>(&self, psc: P0, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugDocumentContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetPositionOfContext)(::windows::core::Vtable::as_raw(self), psc.into().abi(), pccharacterposition, cnumchars).ok()
    }
    pub unsafe fn GetContextOfPosition(&self, ccharacterposition: u32, cnumchars: u32) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContextOfPosition)(::windows::core::Vtable::as_raw(self), ccharacterposition, cnumchars, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentTextEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentTextEvents {}
impl ::core::fmt::Debug for IDebugDocumentTextEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentTextEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugDocumentTextExternalAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentTextExternalAuthor {}
impl ::core::fmt::Debug for IDebugDocumentTextExternalAuthor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentTextExternalAuthor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugEventCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugEventCallbacks {}
impl ::core::fmt::Debug for IDebugEventCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugEventCallbacks").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugEventCallbacksWide {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugEventCallbacksWide {}
impl ::core::fmt::Debug for IDebugEventCallbacksWide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugEventCallbacksWide").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugEventContextCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugEventContextCallbacks {}
impl ::core::fmt::Debug for IDebugEventContextCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugEventContextCallbacks").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugExpression {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExpression {}
impl ::core::fmt::Debug for IDebugExpression {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExpression").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugExpressionCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExpressionCallBack {}
impl ::core::fmt::Debug for IDebugExpressionCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExpressionCallBack").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugExpressionContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExpressionContext {}
impl ::core::fmt::Debug for IDebugExpressionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExpressionContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugExtendedProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExtendedProperty {}
impl ::core::fmt::Debug for IDebugExtendedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExtendedProperty").field(&self.0).finish()
    }
}
impl IDebugExtendedProperty {
    pub unsafe fn GetPropertyInfo(&self, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropertyInfo)(::windows::core::Vtable::as_raw(self), dwfieldspec, nradix, ppropertyinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetExtendedInfo(&self, cinfos: u32, rgguidextendedinfo: *const ::windows::core::GUID, rgvar: *mut super::super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetExtendedInfo)(::windows::core::Vtable::as_raw(self), cinfos, rgguidextendedinfo, rgvar).ok()
    }
    pub unsafe fn SetValueAsString<P0>(&self, pszvalue: P0, nradix: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueAsString)(::windows::core::Vtable::as_raw(self), pszvalue.into().abi(), nradix).ok()
    }
    pub unsafe fn EnumMembers(&self, dwfieldspec: u32, nradix: u32, refiid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDebugPropertyInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumMembers)(::windows::core::Vtable::as_raw(self), dwfieldspec, nradix, refiid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IDebugProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugFormatter {}
impl ::core::fmt::Debug for IDebugFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugFormatter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHelper {}
impl ::core::fmt::Debug for IDebugHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHost {}
impl ::core::fmt::Debug for IDebugHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostBaseClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostBaseClass {}
impl ::core::fmt::Debug for IDebugHostBaseClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostBaseClass").field(&self.0).finish()
    }
}
impl IDebugHostBaseClass {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostConstant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostConstant {}
impl ::core::fmt::Debug for IDebugHostConstant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostConstant").field(&self.0).finish()
    }
}
impl IDebugHostConstant {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostContext {}
impl ::core::fmt::Debug for IDebugHostContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostData {}
impl ::core::fmt::Debug for IDebugHostData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostData").field(&self.0).finish()
    }
}
impl IDebugHostData {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostErrorSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostErrorSink {}
impl ::core::fmt::Debug for IDebugHostErrorSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostErrorSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostEvaluator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostEvaluator {}
impl ::core::fmt::Debug for IDebugHostEvaluator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostEvaluator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostEvaluator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostEvaluator2 {}
impl ::core::fmt::Debug for IDebugHostEvaluator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostEvaluator2").field(&self.0).finish()
    }
}
impl IDebugHostEvaluator2 {
    pub unsafe fn EvaluateExpression<P0, P1, P2>(&self, context: P0, expression: P1, bindingcontext: P2, result: *mut ::core::option::Option<IModelObject>, metadata: ::core::option::Option<*mut ::core::option::Option<IKeyStore>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EvaluateExpression)(::windows::core::Vtable::as_raw(self), context.into().abi(), expression.into().abi(), bindingcontext.into().abi(), ::core::mem::transmute(result), ::core::mem::transmute(metadata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EvaluateExtendedExpression<P0, P1, P2>(&self, context: P0, expression: P1, bindingcontext: P2, result: *mut ::core::option::Option<IModelObject>, metadata: ::core::option::Option<*mut ::core::option::Option<IKeyStore>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EvaluateExtendedExpression)(::windows::core::Vtable::as_raw(self), context.into().abi(), expression.into().abi(), bindingcontext.into().abi(), ::core::mem::transmute(result), ::core::mem::transmute(metadata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IDebugHostExtensibility {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostExtensibility {}
impl ::core::fmt::Debug for IDebugHostExtensibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostExtensibility").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostField {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostField {}
impl ::core::fmt::Debug for IDebugHostField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostField").field(&self.0).finish()
    }
}
impl IDebugHostField {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostMemory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostMemory {}
impl ::core::fmt::Debug for IDebugHostMemory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostMemory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostMemory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostMemory2 {}
impl ::core::fmt::Debug for IDebugHostMemory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostMemory2").field(&self.0).finish()
    }
}
impl IDebugHostMemory2 {
    pub unsafe fn ReadBytes<P0>(&self, context: P0, location: Location, buffer: *mut ::core::ffi::c_void, buffersize: u64, bytesread: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReadBytes)(::windows::core::Vtable::as_raw(self), context.into().abi(), ::core::mem::transmute(location), buffer, buffersize, ::core::mem::transmute(bytesread.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn WriteBytes<P0>(&self, context: P0, location: Location, buffer: *const ::core::ffi::c_void, buffersize: u64, byteswritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.WriteBytes)(::windows::core::Vtable::as_raw(self), context.into().abi(), ::core::mem::transmute(location), buffer, buffersize, ::core::mem::transmute(byteswritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReadPointers<P0>(&self, context: P0, location: Location, pointers: &mut [u64]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReadPointers)(::windows::core::Vtable::as_raw(self), context.into().abi(), ::core::mem::transmute(location), pointers.len() as _, ::core::mem::transmute(pointers.as_ptr())).ok()
    }
    pub unsafe fn WritePointers<P0>(&self, context: P0, location: Location, pointers: &[u64]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.WritePointers)(::windows::core::Vtable::as_raw(self), context.into().abi(), ::core::mem::transmute(location), pointers.len() as _, ::core::mem::transmute(pointers.as_ptr())).ok()
    }
    pub unsafe fn GetDisplayStringForLocation<P0>(&self, context: P0, location: Location, verbose: u8) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayStringForLocation)(::windows::core::Vtable::as_raw(self), context.into().abi(), ::core::mem::transmute(location), verbose, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostModule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostModule {}
impl ::core::fmt::Debug for IDebugHostModule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostModule").field(&self.0).finish()
    }
}
impl IDebugHostModule {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostModule2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostModule2 {}
impl ::core::fmt::Debug for IDebugHostModule2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostModule2").field(&self.0).finish()
    }
}
impl IDebugHostModule2 {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetImageName(&self, allowpath: u8) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImageName)(::windows::core::Vtable::as_raw(self), allowpath, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBaseLocation(&self) -> ::windows::core::Result<Location> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBaseLocation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, fileversion: ::core::option::Option<*mut u64>, productversion: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fileversion.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(productversion.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FindTypeByName<P0>(&self, typename: P0) -> ::windows::core::Result<IDebugHostType>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindTypeByName)(::windows::core::Vtable::as_raw(self), typename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindSymbolByRVA(&self, rva: u64) -> ::windows::core::Result<IDebugHostSymbol> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindSymbolByRVA)(::windows::core::Vtable::as_raw(self), rva, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindSymbolByName<P0>(&self, symbolname: P0) -> ::windows::core::Result<IDebugHostSymbol>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindSymbolByName)(::windows::core::Vtable::as_raw(self), symbolname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostModuleSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostModuleSignature {}
impl ::core::fmt::Debug for IDebugHostModuleSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostModuleSignature").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostPublic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostPublic {}
impl ::core::fmt::Debug for IDebugHostPublic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostPublic").field(&self.0).finish()
    }
}
impl IDebugHostPublic {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostScriptHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostScriptHost {}
impl ::core::fmt::Debug for IDebugHostScriptHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostScriptHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostStatus {}
impl ::core::fmt::Debug for IDebugHostStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostSymbol {}
impl ::core::fmt::Debug for IDebugHostSymbol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostSymbol").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostSymbol2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostSymbol2 {}
impl ::core::fmt::Debug for IDebugHostSymbol2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostSymbol2").field(&self.0).finish()
    }
}
impl IDebugHostSymbol2 {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostSymbolEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostSymbolEnumerator {}
impl ::core::fmt::Debug for IDebugHostSymbolEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostSymbolEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostSymbols {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostSymbols {}
impl ::core::fmt::Debug for IDebugHostSymbols {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostSymbols").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugHostType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostType {}
impl ::core::fmt::Debug for IDebugHostType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostType").field(&self.0).finish()
    }
}
impl IDebugHostType {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostType2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostType2 {}
impl ::core::fmt::Debug for IDebugHostType2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostType2").field(&self.0).finish()
    }
}
impl IDebugHostType2 {
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<IDebugHostContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateChildren<P0>(&self, kind: SymbolKind, name: P0) -> ::windows::core::Result<IDebugHostSymbolEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerateChildren)(::windows::core::Vtable::as_raw(self), kind, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSymbolKind(&self) -> ::windows::core::Result<SymbolKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSymbolKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainingModule(&self) -> ::windows::core::Result<IDebugHostModule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetContainingModule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareAgainst<P0>(&self, pcomparisonsymbol: P0, comparisonflags: u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDebugHostSymbol>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareAgainst)(::windows::core::Vtable::as_raw(self), pcomparisonsymbol.into().abi(), comparisonflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypeKind(&self) -> ::windows::core::Result<TypeKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTypeKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBaseType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBaseType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHashCode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHashCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIntrinsicType(&self, intrinsickind: ::core::option::Option<*mut IntrinsicKind>, carriertype: ::core::option::Option<*mut u16>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIntrinsicType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(intrinsickind.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(carriertype.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetBitField(&self, lsboffield: *mut u32, lengthoffield: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBitField)(::windows::core::Vtable::as_raw(self), lsboffield, lengthoffield).ok()
    }
    pub unsafe fn GetPointerKind(&self) -> ::windows::core::Result<PointerKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPointerKind)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMemberType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMemberType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePointerTo(&self, kind: PointerKind) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePointerTo)(::windows::core::Vtable::as_raw(self), kind, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetArrayDimensionality(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetArrayDimensionality)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetArrayDimensions(&self, pdimensions: &mut [ArrayDimension]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetArrayDimensions)(::windows::core::Vtable::as_raw(self), pdimensions.len() as _, ::core::mem::transmute(pdimensions.as_ptr())).ok()
    }
    pub unsafe fn CreateArrayOf(&self, pdimensions: &[ArrayDimension]) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateArrayOf)(::windows::core::Vtable::as_raw(self), pdimensions.len() as _, ::core::mem::transmute(pdimensions.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFunctionCallingConvention(&self) -> ::windows::core::Result<CallingConventionKind> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFunctionCallingConvention)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFunctionReturnType(&self) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFunctionReturnType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFunctionParameterTypeCount(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFunctionParameterTypeCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFunctionParameterTypeAt(&self, i: u64) -> ::windows::core::Result<IDebugHostType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFunctionParameterTypeAt)(::windows::core::Vtable::as_raw(self), i, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsGeneric(&self) -> ::windows::core::Result<bool> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsGeneric)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGenericArgumentCount(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGenericArgumentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGenericArgumentAt(&self, i: u64) -> ::windows::core::Result<IDebugHostSymbol> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGenericArgumentAt)(::windows::core::Vtable::as_raw(self), i, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugHostTypeSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHostTypeSignature {}
impl ::core::fmt::Debug for IDebugHostTypeSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHostTypeSignature").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugInputCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugInputCallbacks {}
impl ::core::fmt::Debug for IDebugInputCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugInputCallbacks").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugOutputCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugOutputCallbacks {}
impl ::core::fmt::Debug for IDebugOutputCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugOutputCallbacks").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugOutputCallbacks2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugOutputCallbacks2 {}
impl ::core::fmt::Debug for IDebugOutputCallbacks2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugOutputCallbacks2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugOutputCallbacksWide {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugOutputCallbacksWide {}
impl ::core::fmt::Debug for IDebugOutputCallbacksWide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugOutputCallbacksWide").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugOutputStream {}
impl ::core::fmt::Debug for IDebugOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugOutputStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugPlmClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPlmClient {}
impl ::core::fmt::Debug for IDebugPlmClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPlmClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugPlmClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPlmClient2 {}
impl ::core::fmt::Debug for IDebugPlmClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPlmClient2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugPlmClient3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPlmClient3 {}
impl ::core::fmt::Debug for IDebugPlmClient3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPlmClient3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugProperty {}
impl ::core::fmt::Debug for IDebugProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugProperty").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_All {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_All {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_All {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_All").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_Arguments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_Arguments {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_Arguments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_Arguments").field(&self.0).finish()
    }
}
impl IDebugPropertyEnumType_Arguments {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_Locals {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_Locals {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_Locals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_Locals").field(&self.0).finish()
    }
}
impl IDebugPropertyEnumType_Locals {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_LocalsPlusArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_LocalsPlusArgs {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_LocalsPlusArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_LocalsPlusArgs").field(&self.0).finish()
    }
}
impl IDebugPropertyEnumType_LocalsPlusArgs {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_Registers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_Registers {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_Registers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_Registers").field(&self.0).finish()
    }
}
impl IDebugPropertyEnumType_Registers {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugRegisters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugRegisters {}
impl ::core::fmt::Debug for IDebugRegisters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugRegisters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugRegisters2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugRegisters2 {}
impl ::core::fmt::Debug for IDebugRegisters2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugRegisters2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSessionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSessionProvider {}
impl ::core::fmt::Debug for IDebugSessionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSessionProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugStackFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrame {}
impl ::core::fmt::Debug for IDebugStackFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrame").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugStackFrame110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrame110 {}
impl ::core::fmt::Debug for IDebugStackFrame110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrame110").field(&self.0).finish()
    }
}
impl IDebugStackFrame110 {
    pub unsafe fn GetCodeContext(&self) -> ::windows::core::Result<IDebugCodeContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCodeContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescriptionString<P0>(&self, flong: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescriptionString)(::windows::core::Vtable::as_raw(self), flong.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguageString<P0>(&self, flong: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguageString)(::windows::core::Vtable::as_raw(self), flong.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetThread(&self) -> ::windows::core::Result<IDebugApplicationThread> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetThread)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDebugProperty(&self) -> ::windows::core::Result<IDebugProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDebugProperty)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugStackFrameSniffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrameSniffer {}
impl ::core::fmt::Debug for IDebugStackFrameSniffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrameSniffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugStackFrameSnifferEx32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrameSnifferEx32 {}
impl ::core::fmt::Debug for IDebugStackFrameSnifferEx32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrameSnifferEx32").field(&self.0).finish()
    }
}
impl IDebugStackFrameSnifferEx32 {
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumStackFrames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugStackFrameSnifferEx64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrameSnifferEx64 {}
impl ::core::fmt::Debug for IDebugStackFrameSnifferEx64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrameSnifferEx64").field(&self.0).finish()
    }
}
impl IDebugStackFrameSnifferEx64 {
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumStackFrames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDebugSymbolGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSymbolGroup {}
impl ::core::fmt::Debug for IDebugSymbolGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSymbolGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSymbolGroup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSymbolGroup2 {}
impl ::core::fmt::Debug for IDebugSymbolGroup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSymbolGroup2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSymbols {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSymbols {}
impl ::core::fmt::Debug for IDebugSymbols {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSymbols").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSymbols2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSymbols2 {}
impl ::core::fmt::Debug for IDebugSymbols2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSymbols2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSymbols3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSymbols3 {}
impl ::core::fmt::Debug for IDebugSymbols3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSymbols3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSymbols4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSymbols4 {}
impl ::core::fmt::Debug for IDebugSymbols4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSymbols4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSymbols5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSymbols5 {}
impl ::core::fmt::Debug for IDebugSymbols5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSymbols5").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSyncOperation {}
impl ::core::fmt::Debug for IDebugSyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSyncOperation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSystemObjects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSystemObjects {}
impl ::core::fmt::Debug for IDebugSystemObjects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSystemObjects").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSystemObjects2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSystemObjects2 {}
impl ::core::fmt::Debug for IDebugSystemObjects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSystemObjects2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSystemObjects3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSystemObjects3 {}
impl ::core::fmt::Debug for IDebugSystemObjects3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSystemObjects3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugSystemObjects4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSystemObjects4 {}
impl ::core::fmt::Debug for IDebugSystemObjects4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSystemObjects4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugThreadCall32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugThreadCall32 {}
impl ::core::fmt::Debug for IDebugThreadCall32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugThreadCall32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDebugThreadCall64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugThreadCall64 {}
impl ::core::fmt::Debug for IDebugThreadCall64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugThreadCall64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDynamicConceptProviderConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDynamicConceptProviderConcept {}
impl ::core::fmt::Debug for IDynamicConceptProviderConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicConceptProviderConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDynamicKeyProviderConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDynamicKeyProviderConcept {}
impl ::core::fmt::Debug for IDynamicKeyProviderConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicKeyProviderConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDebugApplicationNodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugApplicationNodes {}
impl ::core::fmt::Debug for IEnumDebugApplicationNodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugApplicationNodes").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDebugCodeContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugCodeContexts {}
impl ::core::fmt::Debug for IEnumDebugCodeContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugCodeContexts").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDebugExpressionContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugExpressionContexts {}
impl ::core::fmt::Debug for IEnumDebugExpressionContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugExpressionContexts").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDebugExtendedPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugExtendedPropertyInfo {}
impl ::core::fmt::Debug for IEnumDebugExtendedPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugExtendedPropertyInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDebugPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugPropertyInfo {}
impl ::core::fmt::Debug for IEnumDebugPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugPropertyInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDebugStackFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugStackFrames {}
impl ::core::fmt::Debug for IEnumDebugStackFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugStackFrames").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDebugStackFrames64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugStackFrames64 {}
impl ::core::fmt::Debug for IEnumDebugStackFrames64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugStackFrames64").field(&self.0).finish()
    }
}
impl IEnumDebugStackFrames64 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Next)(::windows::core::Vtable::as_raw(self), celt, prgdsfd, pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IEnumJsStackFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumJsStackFrames {}
impl ::core::fmt::Debug for IEnumJsStackFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumJsStackFrames").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumRemoteDebugApplicationThreads {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRemoteDebugApplicationThreads {}
impl ::core::fmt::Debug for IEnumRemoteDebugApplicationThreads {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRemoteDebugApplicationThreads").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumRemoteDebugApplications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRemoteDebugApplications {}
impl ::core::fmt::Debug for IEnumRemoteDebugApplications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRemoteDebugApplications").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEquatableConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEquatableConcept {}
impl ::core::fmt::Debug for IEquatableConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEquatableConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHostDataModelAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHostDataModelAccess {}
impl ::core::fmt::Debug for IHostDataModelAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHostDataModelAccess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IIndexableConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIndexableConcept {}
impl ::core::fmt::Debug for IIndexableConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIndexableConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IIterableConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIterableConcept {}
impl ::core::fmt::Debug for IIterableConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIterableConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebug {}
impl ::core::fmt::Debug for IJsDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebug").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsDebugBreakPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugBreakPoint {}
impl ::core::fmt::Debug for IJsDebugBreakPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugBreakPoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsDebugDataTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugDataTarget {}
impl ::core::fmt::Debug for IJsDebugDataTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugDataTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsDebugFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugFrame {}
impl ::core::fmt::Debug for IJsDebugFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugFrame").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsDebugProcess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugProcess {}
impl ::core::fmt::Debug for IJsDebugProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugProcess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsDebugProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugProperty {}
impl ::core::fmt::Debug for IJsDebugProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugProperty").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsDebugStackWalker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugStackWalker {}
impl ::core::fmt::Debug for IJsDebugStackWalker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugStackWalker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IJsEnumDebugProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsEnumDebugProperty {}
impl ::core::fmt::Debug for IJsEnumDebugProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsEnumDebugProperty").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKeyEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKeyEnumerator {}
impl ::core::fmt::Debug for IKeyEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKeyEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKeyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKeyStore {}
impl ::core::fmt::Debug for IKeyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKeyStore").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_CBA_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_CBA_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity && self.code == other.code && self.desc == other.desc && self.object == other.object
    }
}
impl ::core::cmp::Eq for IMAGEHLP_CBA_EVENT {}
impl ::core::fmt::Debug for IMAGEHLP_CBA_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_CBA_EVENT").field("severity", &self.severity).field("code", &self.code).field("desc", &self.desc).field("object", &self.object).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_CBA_EVENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_CBA_EVENTW {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity && self.code == other.code && self.desc == other.desc && self.object == other.object
    }
}
impl ::core::cmp::Eq for IMAGEHLP_CBA_EVENTW {}
impl ::core::fmt::Debug for IMAGEHLP_CBA_EVENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_CBA_EVENTW").field("severity", &self.severity).field("code", &self.code).field("desc", &self.desc).field("object", &self.object).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_CBA_EVENT_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGEHLP_CBA_EVENT_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_CBA_EVENT_SEVERITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_CBA_READ_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_CBA_READ_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.buf == other.buf && self.bytes == other.bytes && self.bytesread == other.bytesread
    }
}
impl ::core::cmp::Eq for IMAGEHLP_CBA_READ_MEMORY {}
impl ::core::fmt::Debug for IMAGEHLP_CBA_READ_MEMORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_CBA_READ_MEMORY").field("addr", &self.addr).field("buf", &self.buf).field("bytes", &self.bytes).field("bytesread", &self.bytesread).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.CheckSum == other.CheckSum && self.TimeDateStamp == other.TimeDateStamp && self.FileName == other.FileName && self.Reparse == other.Reparse && self.hFile == other.hFile
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DEFERRED_SYMBOL_LOAD {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DEFERRED_SYMBOL_LOAD").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("CheckSum", &self.CheckSum).field("TimeDateStamp", &self.TimeDateStamp).field("FileName", &self.FileName).field("Reparse", &self.Reparse).field("hFile", &self.hFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.CheckSum == other.CheckSum && self.TimeDateStamp == other.TimeDateStamp && self.FileName == other.FileName && self.Reparse == other.Reparse && self.hFile == other.hFile && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DEFERRED_SYMBOL_LOAD64").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("CheckSum", &self.CheckSum).field("TimeDateStamp", &self.TimeDateStamp).field("FileName", &self.FileName).field("Reparse", &self.Reparse).field("hFile", &self.hFile).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.CheckSum == other.CheckSum && self.TimeDateStamp == other.TimeDateStamp && self.FileName == other.FileName && self.Reparse == other.Reparse && self.hFile == other.hFile && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DEFERRED_SYMBOL_LOADW64").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("CheckSum", &self.CheckSum).field("TimeDateStamp", &self.TimeDateStamp).field("FileName", &self.FileName).field("Reparse", &self.Reparse).field("hFile", &self.hFile).field("Flags", &self.Flags).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DUPLICATE_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DUPLICATE_SYMBOL {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.NumberOfDups == other.NumberOfDups && self.Symbol == other.Symbol && self.SelectedSymbol == other.SelectedSymbol
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DUPLICATE_SYMBOL {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DUPLICATE_SYMBOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DUPLICATE_SYMBOL").field("SizeOfStruct", &self.SizeOfStruct).field("NumberOfDups", &self.NumberOfDups).field("Symbol", &self.Symbol).field("SelectedSymbol", &self.SelectedSymbol).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.NumberOfDups == other.NumberOfDups && self.Symbol == other.Symbol && self.SelectedSymbol == other.SelectedSymbol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DUPLICATE_SYMBOL64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DUPLICATE_SYMBOL64").field("SizeOfStruct", &self.SizeOfStruct).field("NumberOfDups", &self.NumberOfDups).field("Symbol", &self.Symbol).field("SelectedSymbol", &self.SelectedSymbol).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_EXTENDED_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGEHLP_EXTENDED_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_EXTENDED_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_GET_TYPE_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGEHLP_GET_TYPE_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_GET_TYPE_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Flags == other.Flags && self.NumIds == other.NumIds && self.TypeIds == other.TypeIds && self.TagFilter == other.TagFilter && self.NumReqs == other.NumReqs && self.ReqKinds == other.ReqKinds && self.ReqOffsets == other.ReqOffsets && self.ReqSizes == other.ReqSizes && self.ReqStride == other.ReqStride && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer && self.EntriesMatched == other.EntriesMatched && self.EntriesFilled == other.EntriesFilled && self.TagsFound == other.TagsFound && self.AllReqsValid == other.AllReqsValid && self.NumReqsValid == other.NumReqsValid && self.ReqsValid == other.ReqsValid
    }
}
impl ::core::cmp::Eq for IMAGEHLP_GET_TYPE_INFO_PARAMS {}
impl ::core::fmt::Debug for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_GET_TYPE_INFO_PARAMS")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Flags", &self.Flags)
            .field("NumIds", &self.NumIds)
            .field("TypeIds", &self.TypeIds)
            .field("TagFilter", &self.TagFilter)
            .field("NumReqs", &self.NumReqs)
            .field("ReqKinds", &self.ReqKinds)
            .field("ReqOffsets", &self.ReqOffsets)
            .field("ReqSizes", &self.ReqSizes)
            .field("ReqStride", &self.ReqStride)
            .field("BufferSize", &self.BufferSize)
            .field("Buffer", &self.Buffer)
            .field("EntriesMatched", &self.EntriesMatched)
            .field("EntriesFilled", &self.EntriesFilled)
            .field("TagsFound", &self.TagsFound)
            .field("AllReqsValid", &self.AllReqsValid)
            .field("NumReqsValid", &self.NumReqsValid)
            .field("ReqsValid", &self.ReqsValid)
            .finish()
    }
}
impl ::core::default::Default for IMAGEHLP_HD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGEHLP_HD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_HD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_JIT_SYMBOLMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_JIT_SYMBOLMAP {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.BaseOfImage == other.BaseOfImage
    }
}
impl ::core::cmp::Eq for IMAGEHLP_JIT_SYMBOLMAP {}
impl ::core::fmt::Debug for IMAGEHLP_JIT_SYMBOLMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_JIT_SYMBOLMAP").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("BaseOfImage", &self.BaseOfImage).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_LINE {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINE").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_LINE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_LINE64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for IMAGEHLP_LINE64 {}
impl ::core::fmt::Debug for IMAGEHLP_LINE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINE64").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_LINEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_LINEW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_LINEW {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_LINEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINEW").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_LINEW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_LINEW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for IMAGEHLP_LINEW64 {}
impl ::core::fmt::Debug for IMAGEHLP_LINEW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINEW64").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.ImageSize == other.ImageSize && self.TimeDateStamp == other.TimeDateStamp && self.CheckSum == other.CheckSum && self.NumSyms == other.NumSyms && self.SymType == other.SymType && self.ModuleName == other.ModuleName && self.ImageName == other.ImageName && self.LoadedImageName == other.LoadedImageName
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULE").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("ImageSize", &self.ImageSize).field("TimeDateStamp", &self.TimeDateStamp).field("CheckSum", &self.CheckSum).field("NumSyms", &self.NumSyms).field("SymType", &self.SymType).field("ModuleName", &self.ModuleName).field("ImageName", &self.ImageName).field("LoadedImageName", &self.LoadedImageName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULE64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.BaseOfImage == other.BaseOfImage
            && self.ImageSize == other.ImageSize
            && self.TimeDateStamp == other.TimeDateStamp
            && self.CheckSum == other.CheckSum
            && self.NumSyms == other.NumSyms
            && self.SymType == other.SymType
            && self.ModuleName == other.ModuleName
            && self.ImageName == other.ImageName
            && self.LoadedImageName == other.LoadedImageName
            && self.LoadedPdbName == other.LoadedPdbName
            && self.CVSig == other.CVSig
            && self.CVData == other.CVData
            && self.PdbSig == other.PdbSig
            && self.PdbSig70 == other.PdbSig70
            && self.PdbAge == other.PdbAge
            && self.PdbUnmatched == other.PdbUnmatched
            && self.DbgUnmatched == other.DbgUnmatched
            && self.LineNumbers == other.LineNumbers
            && self.GlobalSymbols == other.GlobalSymbols
            && self.TypeInfo == other.TypeInfo
            && self.SourceIndexed == other.SourceIndexed
            && self.Publics == other.Publics
            && self.MachineType == other.MachineType
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULE64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULE64")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("BaseOfImage", &self.BaseOfImage)
            .field("ImageSize", &self.ImageSize)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("CheckSum", &self.CheckSum)
            .field("NumSyms", &self.NumSyms)
            .field("SymType", &self.SymType)
            .field("ModuleName", &self.ModuleName)
            .field("ImageName", &self.ImageName)
            .field("LoadedImageName", &self.LoadedImageName)
            .field("LoadedPdbName", &self.LoadedPdbName)
            .field("CVSig", &self.CVSig)
            .field("CVData", &self.CVData)
            .field("PdbSig", &self.PdbSig)
            .field("PdbSig70", &self.PdbSig70)
            .field("PdbAge", &self.PdbAge)
            .field("PdbUnmatched", &self.PdbUnmatched)
            .field("DbgUnmatched", &self.DbgUnmatched)
            .field("LineNumbers", &self.LineNumbers)
            .field("GlobalSymbols", &self.GlobalSymbols)
            .field("TypeInfo", &self.TypeInfo)
            .field("SourceIndexed", &self.SourceIndexed)
            .field("Publics", &self.Publics)
            .field("MachineType", &self.MachineType)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULE64_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULE64_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Module == other.Module && self.RegionFlags == other.RegionFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULE64_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULE64_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULE64_EX").field("Module", &self.Module).field("RegionFlags", &self.RegionFlags).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_MODULEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULEW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.ImageSize == other.ImageSize && self.TimeDateStamp == other.TimeDateStamp && self.CheckSum == other.CheckSum && self.NumSyms == other.NumSyms && self.SymType == other.SymType && self.ModuleName == other.ModuleName && self.ImageName == other.ImageName && self.LoadedImageName == other.LoadedImageName
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_MODULEW {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_MODULEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULEW").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("ImageSize", &self.ImageSize).field("TimeDateStamp", &self.TimeDateStamp).field("CheckSum", &self.CheckSum).field("NumSyms", &self.NumSyms).field("SymType", &self.SymType).field("ModuleName", &self.ModuleName).field("ImageName", &self.ImageName).field("LoadedImageName", &self.LoadedImageName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULEW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULEW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.BaseOfImage == other.BaseOfImage
            && self.ImageSize == other.ImageSize
            && self.TimeDateStamp == other.TimeDateStamp
            && self.CheckSum == other.CheckSum
            && self.NumSyms == other.NumSyms
            && self.SymType == other.SymType
            && self.ModuleName == other.ModuleName
            && self.ImageName == other.ImageName
            && self.LoadedImageName == other.LoadedImageName
            && self.LoadedPdbName == other.LoadedPdbName
            && self.CVSig == other.CVSig
            && self.CVData == other.CVData
            && self.PdbSig == other.PdbSig
            && self.PdbSig70 == other.PdbSig70
            && self.PdbAge == other.PdbAge
            && self.PdbUnmatched == other.PdbUnmatched
            && self.DbgUnmatched == other.DbgUnmatched
            && self.LineNumbers == other.LineNumbers
            && self.GlobalSymbols == other.GlobalSymbols
            && self.TypeInfo == other.TypeInfo
            && self.SourceIndexed == other.SourceIndexed
            && self.Publics == other.Publics
            && self.MachineType == other.MachineType
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULEW64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULEW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULEW64")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("BaseOfImage", &self.BaseOfImage)
            .field("ImageSize", &self.ImageSize)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("CheckSum", &self.CheckSum)
            .field("NumSyms", &self.NumSyms)
            .field("SymType", &self.SymType)
            .field("ModuleName", &self.ModuleName)
            .field("ImageName", &self.ImageName)
            .field("LoadedImageName", &self.LoadedImageName)
            .field("LoadedPdbName", &self.LoadedPdbName)
            .field("CVSig", &self.CVSig)
            .field("CVData", &self.CVData)
            .field("PdbSig", &self.PdbSig)
            .field("PdbSig70", &self.PdbSig70)
            .field("PdbAge", &self.PdbAge)
            .field("PdbUnmatched", &self.PdbUnmatched)
            .field("DbgUnmatched", &self.DbgUnmatched)
            .field("LineNumbers", &self.LineNumbers)
            .field("GlobalSymbols", &self.GlobalSymbols)
            .field("TypeInfo", &self.TypeInfo)
            .field("SourceIndexed", &self.SourceIndexed)
            .field("Publics", &self.Publics)
            .field("MachineType", &self.MachineType)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULEW64_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULEW64_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Module == other.Module && self.RegionFlags == other.RegionFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULEW64_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULEW64_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULEW64_EX").field("Module", &self.Module).field("RegionFlags", &self.RegionFlags).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGEHLP_SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_SF_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_STACK_FRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_STACK_FRAME {
    fn eq(&self, other: &Self) -> bool {
        self.InstructionOffset == other.InstructionOffset && self.ReturnOffset == other.ReturnOffset && self.FrameOffset == other.FrameOffset && self.StackOffset == other.StackOffset && self.BackingStoreOffset == other.BackingStoreOffset && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Reserved == other.Reserved && self.Virtual == other.Virtual && self.Reserved2 == other.Reserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_STACK_FRAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_STACK_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_STACK_FRAME").field("InstructionOffset", &self.InstructionOffset).field("ReturnOffset", &self.ReturnOffset).field("FrameOffset", &self.FrameOffset).field("StackOffset", &self.StackOffset).field("BackingStoreOffset", &self.BackingStoreOffset).field("FuncTableEntry", &self.FuncTableEntry).field("Params", &self.Params).field("Reserved", &self.Reserved).field("Virtual", &self.Virtual).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_STATUS_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGEHLP_STATUS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_STATUS_REASON").field(&self.0).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_SYMBOL64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL64").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_SYMBOL64_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL64_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL64_PACKAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL64_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL64_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_SYMBOLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_SYMBOLW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW64 {}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW64").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW64_PACKAGE {}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW64_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_SYMBOLW_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW_PACKAGE {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_SYMBOL_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL_PACKAGE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_SYMBOL_SRC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL_SRC {
    fn eq(&self, other: &Self) -> bool {
        self.sizeofstruct == other.sizeofstruct && self.r#type == other.r#type && self.file == other.file
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL_SRC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL_SRC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL_SRC").field("sizeofstruct", &self.sizeofstruct).field("type", &self.r#type).field("file", &self.file).finish()
    }
}
impl ::core::default::Default for IMAGEHLP_SYMBOL_TYPE_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_SYMBOL_TYPE_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_COFF_SYMBOLS_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_COFF_SYMBOLS_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfSymbols == other.NumberOfSymbols && self.LvaToFirstSymbol == other.LvaToFirstSymbol && self.NumberOfLinenumbers == other.NumberOfLinenumbers && self.LvaToFirstLinenumber == other.LvaToFirstLinenumber && self.RvaToFirstByteOfCode == other.RvaToFirstByteOfCode && self.RvaToLastByteOfCode == other.RvaToLastByteOfCode && self.RvaToFirstByteOfData == other.RvaToFirstByteOfData && self.RvaToLastByteOfData == other.RvaToLastByteOfData
    }
}
impl ::core::cmp::Eq for IMAGE_COFF_SYMBOLS_HEADER {}
impl ::core::fmt::Debug for IMAGE_COFF_SYMBOLS_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_COFF_SYMBOLS_HEADER")
            .field("NumberOfSymbols", &self.NumberOfSymbols)
            .field("LvaToFirstSymbol", &self.LvaToFirstSymbol)
            .field("NumberOfLinenumbers", &self.NumberOfLinenumbers)
            .field("LvaToFirstLinenumber", &self.LvaToFirstLinenumber)
            .field("RvaToFirstByteOfCode", &self.RvaToFirstByteOfCode)
            .field("RvaToLastByteOfCode", &self.RvaToLastByteOfCode)
            .field("RvaToFirstByteOfData", &self.RvaToFirstByteOfData)
            .field("RvaToLastByteOfData", &self.RvaToLastByteOfData)
            .finish()
    }
}
impl ::core::default::Default for IMAGE_COR20_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_DATA_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_DATA_DIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualAddress == other.VirtualAddress && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for IMAGE_DATA_DIRECTORY {}
impl ::core::fmt::Debug for IMAGE_DATA_DIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DATA_DIRECTORY").field("VirtualAddress", &self.VirtualAddress).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for IMAGE_DEBUG_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_DEBUG_DIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.Characteristics == other.Characteristics && self.TimeDateStamp == other.TimeDateStamp && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.Type == other.Type && self.SizeOfData == other.SizeOfData && self.AddressOfRawData == other.AddressOfRawData && self.PointerToRawData == other.PointerToRawData
    }
}
impl ::core::cmp::Eq for IMAGE_DEBUG_DIRECTORY {}
impl ::core::fmt::Debug for IMAGE_DEBUG_DIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DEBUG_DIRECTORY").field("Characteristics", &self.Characteristics).field("TimeDateStamp", &self.TimeDateStamp).field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("Type", &self.Type).field("SizeOfData", &self.SizeOfData).field("AddressOfRawData", &self.AddressOfRawData).field("PointerToRawData", &self.PointerToRawData).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for IMAGE_DEBUG_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for IMAGE_DEBUG_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.List == other.List
            && self.ReservedSize == other.ReservedSize
            && self.ReservedMappedBase == other.ReservedMappedBase
            && self.ReservedMachine == other.ReservedMachine
            && self.ReservedCharacteristics == other.ReservedCharacteristics
            && self.ReservedCheckSum == other.ReservedCheckSum
            && self.ImageBase == other.ImageBase
            && self.SizeOfImage == other.SizeOfImage
            && self.ReservedNumberOfSections == other.ReservedNumberOfSections
            && self.ReservedSections == other.ReservedSections
            && self.ReservedExportedNamesSize == other.ReservedExportedNamesSize
            && self.ReservedExportedNames == other.ReservedExportedNames
            && self.ReservedNumberOfFunctionTableEntries == other.ReservedNumberOfFunctionTableEntries
            && self.ReservedFunctionTableEntries == other.ReservedFunctionTableEntries
            && self.ReservedLowestFunctionStartingAddress == other.ReservedLowestFunctionStartingAddress
            && self.ReservedHighestFunctionEndingAddress == other.ReservedHighestFunctionEndingAddress
            && self.ReservedNumberOfFpoTableEntries == other.ReservedNumberOfFpoTableEntries
            && self.ReservedFpoTableEntries == other.ReservedFpoTableEntries
            && self.SizeOfCoffSymbols == other.SizeOfCoffSymbols
            && self.CoffSymbols == other.CoffSymbols
            && self.ReservedSizeOfCodeViewSymbols == other.ReservedSizeOfCodeViewSymbols
            && self.ReservedCodeViewSymbols == other.ReservedCodeViewSymbols
            && self.ImageFilePath == other.ImageFilePath
            && self.ImageFileName == other.ImageFileName
            && self.ReservedDebugFilePath == other.ReservedDebugFilePath
            && self.ReservedTimeDateStamp == other.ReservedTimeDateStamp
            && self.ReservedRomImage == other.ReservedRomImage
            && self.ReservedDebugDirectory == other.ReservedDebugDirectory
            && self.ReservedNumberOfDebugDirectories == other.ReservedNumberOfDebugDirectories
            && self.ReservedOriginalFunctionTableBaseAddress == other.ReservedOriginalFunctionTableBaseAddress
            && self.Reserved == other.Reserved
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for IMAGE_DEBUG_INFORMATION {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for IMAGE_DEBUG_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DEBUG_INFORMATION")
            .field("List", &self.List)
            .field("ReservedSize", &self.ReservedSize)
            .field("ReservedMappedBase", &self.ReservedMappedBase)
            .field("ReservedMachine", &self.ReservedMachine)
            .field("ReservedCharacteristics", &self.ReservedCharacteristics)
            .field("ReservedCheckSum", &self.ReservedCheckSum)
            .field("ImageBase", &self.ImageBase)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("ReservedNumberOfSections", &self.ReservedNumberOfSections)
            .field("ReservedSections", &self.ReservedSections)
            .field("ReservedExportedNamesSize", &self.ReservedExportedNamesSize)
            .field("ReservedExportedNames", &self.ReservedExportedNames)
            .field("ReservedNumberOfFunctionTableEntries", &self.ReservedNumberOfFunctionTableEntries)
            .field("ReservedFunctionTableEntries", &self.ReservedFunctionTableEntries)
            .field("ReservedLowestFunctionStartingAddress", &self.ReservedLowestFunctionStartingAddress)
            .field("ReservedHighestFunctionEndingAddress", &self.ReservedHighestFunctionEndingAddress)
            .field("ReservedNumberOfFpoTableEntries", &self.ReservedNumberOfFpoTableEntries)
            .field("ReservedFpoTableEntries", &self.ReservedFpoTableEntries)
            .field("SizeOfCoffSymbols", &self.SizeOfCoffSymbols)
            .field("CoffSymbols", &self.CoffSymbols)
            .field("ReservedSizeOfCodeViewSymbols", &self.ReservedSizeOfCodeViewSymbols)
            .field("ReservedCodeViewSymbols", &self.ReservedCodeViewSymbols)
            .field("ImageFilePath", &self.ImageFilePath)
            .field("ImageFileName", &self.ImageFileName)
            .field("ReservedDebugFilePath", &self.ReservedDebugFilePath)
            .field("ReservedTimeDateStamp", &self.ReservedTimeDateStamp)
            .field("ReservedRomImage", &self.ReservedRomImage)
            .field("ReservedDebugDirectory", &self.ReservedDebugDirectory)
            .field("ReservedNumberOfDebugDirectories", &self.ReservedNumberOfDebugDirectories)
            .field("ReservedOriginalFunctionTableBaseAddress", &self.ReservedOriginalFunctionTableBaseAddress)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for IMAGE_DEBUG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_DEBUG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_DEBUG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_DIRECTORY_ENTRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_DIRECTORY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_DIRECTORY_ENTRY").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_DLL_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_DLL_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_DLL_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_DLL_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_DLL_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_DLL_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_DLL_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_DLL_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IMAGE_FILE_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_FILE_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FILE_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_FILE_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_FILE_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_FILE_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_FILE_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_FILE_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IMAGE_FILE_CHARACTERISTICS2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_FILE_CHARACTERISTICS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FILE_CHARACTERISTICS2").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_FILE_CHARACTERISTICS2 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_FILE_CHARACTERISTICS2 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_FILE_CHARACTERISTICS2 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_FILE_CHARACTERISTICS2 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_FILE_CHARACTERISTICS2 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_FILE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for IMAGE_FILE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Machine == other.Machine && self.NumberOfSections == other.NumberOfSections && self.TimeDateStamp == other.TimeDateStamp && self.PointerToSymbolTable == other.PointerToSymbolTable && self.NumberOfSymbols == other.NumberOfSymbols && self.SizeOfOptionalHeader == other.SizeOfOptionalHeader && self.Characteristics == other.Characteristics
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for IMAGE_FILE_HEADER {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for IMAGE_FILE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_FILE_HEADER").field("Machine", &self.Machine).field("NumberOfSections", &self.NumberOfSections).field("TimeDateStamp", &self.TimeDateStamp).field("PointerToSymbolTable", &self.PointerToSymbolTable).field("NumberOfSymbols", &self.NumberOfSymbols).field("SizeOfOptionalHeader", &self.SizeOfOptionalHeader).field("Characteristics", &self.Characteristics).finish()
    }
}
impl ::core::default::Default for IMAGE_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_FUNCTION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.StartingAddress == other.StartingAddress && self.EndingAddress == other.EndingAddress && self.EndOfPrologue == other.EndOfPrologue
    }
}
impl ::core::cmp::Eq for IMAGE_FUNCTION_ENTRY {}
impl ::core::fmt::Debug for IMAGE_FUNCTION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_FUNCTION_ENTRY").field("StartingAddress", &self.StartingAddress).field("EndingAddress", &self.EndingAddress).field("EndOfPrologue", &self.EndOfPrologue).finish()
    }
}
impl ::core::default::Default for IMAGE_FUNCTION_ENTRY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Catalog == other.Catalog && self.CatalogOffset == other.CatalogOffset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {}
impl ::core::fmt::Debug for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_LOAD_CONFIG_CODE_INTEGRITY").field("Flags", &self.Flags).field("Catalog", &self.Catalog).field("CatalogOffset", &self.CatalogOffset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.TimeDateStamp == other.TimeDateStamp
            && self.MajorVersion == other.MajorVersion
            && self.MinorVersion == other.MinorVersion
            && self.GlobalFlagsClear == other.GlobalFlagsClear
            && self.GlobalFlagsSet == other.GlobalFlagsSet
            && self.CriticalSectionDefaultTimeout == other.CriticalSectionDefaultTimeout
            && self.DeCommitFreeBlockThreshold == other.DeCommitFreeBlockThreshold
            && self.DeCommitTotalFreeThreshold == other.DeCommitTotalFreeThreshold
            && self.LockPrefixTable == other.LockPrefixTable
            && self.MaximumAllocationSize == other.MaximumAllocationSize
            && self.VirtualMemoryThreshold == other.VirtualMemoryThreshold
            && self.ProcessHeapFlags == other.ProcessHeapFlags
            && self.ProcessAffinityMask == other.ProcessAffinityMask
            && self.CSDVersion == other.CSDVersion
            && self.DependentLoadFlags == other.DependentLoadFlags
            && self.EditList == other.EditList
            && self.SecurityCookie == other.SecurityCookie
            && self.SEHandlerTable == other.SEHandlerTable
            && self.SEHandlerCount == other.SEHandlerCount
            && self.GuardCFCheckFunctionPointer == other.GuardCFCheckFunctionPointer
            && self.GuardCFDispatchFunctionPointer == other.GuardCFDispatchFunctionPointer
            && self.GuardCFFunctionTable == other.GuardCFFunctionTable
            && self.GuardCFFunctionCount == other.GuardCFFunctionCount
            && self.GuardFlags == other.GuardFlags
            && self.CodeIntegrity == other.CodeIntegrity
            && self.GuardAddressTakenIatEntryTable == other.GuardAddressTakenIatEntryTable
            && self.GuardAddressTakenIatEntryCount == other.GuardAddressTakenIatEntryCount
            && self.GuardLongJumpTargetTable == other.GuardLongJumpTargetTable
            && self.GuardLongJumpTargetCount == other.GuardLongJumpTargetCount
            && self.DynamicValueRelocTable == other.DynamicValueRelocTable
            && self.CHPEMetadataPointer == other.CHPEMetadataPointer
            && self.GuardRFFailureRoutine == other.GuardRFFailureRoutine
            && self.GuardRFFailureRoutineFunctionPointer == other.GuardRFFailureRoutineFunctionPointer
            && self.DynamicValueRelocTableOffset == other.DynamicValueRelocTableOffset
            && self.DynamicValueRelocTableSection == other.DynamicValueRelocTableSection
            && self.Reserved2 == other.Reserved2
            && self.GuardRFVerifyStackPointerFunctionPointer == other.GuardRFVerifyStackPointerFunctionPointer
            && self.HotPatchTableOffset == other.HotPatchTableOffset
            && self.Reserved3 == other.Reserved3
            && self.EnclaveConfigurationPointer == other.EnclaveConfigurationPointer
            && self.VolatileMetadataPointer == other.VolatileMetadataPointer
            && self.GuardEHContinuationTable == other.GuardEHContinuationTable
            && self.GuardEHContinuationCount == other.GuardEHContinuationCount
            && self.GuardXFGCheckFunctionPointer == other.GuardXFGCheckFunctionPointer
            && self.GuardXFGDispatchFunctionPointer == other.GuardXFGDispatchFunctionPointer
            && self.GuardXFGTableDispatchFunctionPointer == other.GuardXFGTableDispatchFunctionPointer
            && self.CastGuardOsDeterminedFailureMode == other.CastGuardOsDeterminedFailureMode
    }
}
impl ::core::cmp::Eq for IMAGE_LOAD_CONFIG_DIRECTORY32 {}
impl ::core::fmt::Debug for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_LOAD_CONFIG_DIRECTORY32")
            .field("Size", &self.Size)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("GlobalFlagsClear", &self.GlobalFlagsClear)
            .field("GlobalFlagsSet", &self.GlobalFlagsSet)
            .field("CriticalSectionDefaultTimeout", &self.CriticalSectionDefaultTimeout)
            .field("DeCommitFreeBlockThreshold", &self.DeCommitFreeBlockThreshold)
            .field("DeCommitTotalFreeThreshold", &self.DeCommitTotalFreeThreshold)
            .field("LockPrefixTable", &self.LockPrefixTable)
            .field("MaximumAllocationSize", &self.MaximumAllocationSize)
            .field("VirtualMemoryThreshold", &self.VirtualMemoryThreshold)
            .field("ProcessHeapFlags", &self.ProcessHeapFlags)
            .field("ProcessAffinityMask", &self.ProcessAffinityMask)
            .field("CSDVersion", &self.CSDVersion)
            .field("DependentLoadFlags", &self.DependentLoadFlags)
            .field("EditList", &self.EditList)
            .field("SecurityCookie", &self.SecurityCookie)
            .field("SEHandlerTable", &self.SEHandlerTable)
            .field("SEHandlerCount", &self.SEHandlerCount)
            .field("GuardCFCheckFunctionPointer", &self.GuardCFCheckFunctionPointer)
            .field("GuardCFDispatchFunctionPointer", &self.GuardCFDispatchFunctionPointer)
            .field("GuardCFFunctionTable", &self.GuardCFFunctionTable)
            .field("GuardCFFunctionCount", &self.GuardCFFunctionCount)
            .field("GuardFlags", &self.GuardFlags)
            .field("CodeIntegrity", &self.CodeIntegrity)
            .field("GuardAddressTakenIatEntryTable", &self.GuardAddressTakenIatEntryTable)
            .field("GuardAddressTakenIatEntryCount", &self.GuardAddressTakenIatEntryCount)
            .field("GuardLongJumpTargetTable", &self.GuardLongJumpTargetTable)
            .field("GuardLongJumpTargetCount", &self.GuardLongJumpTargetCount)
            .field("DynamicValueRelocTable", &self.DynamicValueRelocTable)
            .field("CHPEMetadataPointer", &self.CHPEMetadataPointer)
            .field("GuardRFFailureRoutine", &self.GuardRFFailureRoutine)
            .field("GuardRFFailureRoutineFunctionPointer", &self.GuardRFFailureRoutineFunctionPointer)
            .field("DynamicValueRelocTableOffset", &self.DynamicValueRelocTableOffset)
            .field("DynamicValueRelocTableSection", &self.DynamicValueRelocTableSection)
            .field("Reserved2", &self.Reserved2)
            .field("GuardRFVerifyStackPointerFunctionPointer", &self.GuardRFVerifyStackPointerFunctionPointer)
            .field("HotPatchTableOffset", &self.HotPatchTableOffset)
            .field("Reserved3", &self.Reserved3)
            .field("EnclaveConfigurationPointer", &self.EnclaveConfigurationPointer)
            .field("VolatileMetadataPointer", &self.VolatileMetadataPointer)
            .field("GuardEHContinuationTable", &self.GuardEHContinuationTable)
            .field("GuardEHContinuationCount", &self.GuardEHContinuationCount)
            .field("GuardXFGCheckFunctionPointer", &self.GuardXFGCheckFunctionPointer)
            .field("GuardXFGDispatchFunctionPointer", &self.GuardXFGDispatchFunctionPointer)
            .field("GuardXFGTableDispatchFunctionPointer", &self.GuardXFGTableDispatchFunctionPointer)
            .field("CastGuardOsDeterminedFailureMode", &self.CastGuardOsDeterminedFailureMode)
            .finish()
    }
}
impl ::core::default::Default for IMAGE_LOAD_CONFIG_DIRECTORY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_NT_HEADERS32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for IMAGE_NT_HEADERS32 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.FileHeader == other.FileHeader && self.OptionalHeader == other.OptionalHeader
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for IMAGE_NT_HEADERS32 {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for IMAGE_NT_HEADERS32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_NT_HEADERS32").field("Signature", &self.Signature).field("FileHeader", &self.FileHeader).field("OptionalHeader", &self.OptionalHeader).finish()
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_NT_HEADERS64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_OPTIONAL_HEADER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_OPTIONAL_HEADER32 {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic
            && self.MajorLinkerVersion == other.MajorLinkerVersion
            && self.MinorLinkerVersion == other.MinorLinkerVersion
            && self.SizeOfCode == other.SizeOfCode
            && self.SizeOfInitializedData == other.SizeOfInitializedData
            && self.SizeOfUninitializedData == other.SizeOfUninitializedData
            && self.AddressOfEntryPoint == other.AddressOfEntryPoint
            && self.BaseOfCode == other.BaseOfCode
            && self.BaseOfData == other.BaseOfData
            && self.ImageBase == other.ImageBase
            && self.SectionAlignment == other.SectionAlignment
            && self.FileAlignment == other.FileAlignment
            && self.MajorOperatingSystemVersion == other.MajorOperatingSystemVersion
            && self.MinorOperatingSystemVersion == other.MinorOperatingSystemVersion
            && self.MajorImageVersion == other.MajorImageVersion
            && self.MinorImageVersion == other.MinorImageVersion
            && self.MajorSubsystemVersion == other.MajorSubsystemVersion
            && self.MinorSubsystemVersion == other.MinorSubsystemVersion
            && self.Win32VersionValue == other.Win32VersionValue
            && self.SizeOfImage == other.SizeOfImage
            && self.SizeOfHeaders == other.SizeOfHeaders
            && self.CheckSum == other.CheckSum
            && self.Subsystem == other.Subsystem
            && self.DllCharacteristics == other.DllCharacteristics
            && self.SizeOfStackReserve == other.SizeOfStackReserve
            && self.SizeOfStackCommit == other.SizeOfStackCommit
            && self.SizeOfHeapReserve == other.SizeOfHeapReserve
            && self.SizeOfHeapCommit == other.SizeOfHeapCommit
            && self.LoaderFlags == other.LoaderFlags
            && self.NumberOfRvaAndSizes == other.NumberOfRvaAndSizes
            && self.DataDirectory == other.DataDirectory
    }
}
impl ::core::cmp::Eq for IMAGE_OPTIONAL_HEADER32 {}
impl ::core::fmt::Debug for IMAGE_OPTIONAL_HEADER32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_OPTIONAL_HEADER32")
            .field("Magic", &self.Magic)
            .field("MajorLinkerVersion", &self.MajorLinkerVersion)
            .field("MinorLinkerVersion", &self.MinorLinkerVersion)
            .field("SizeOfCode", &self.SizeOfCode)
            .field("SizeOfInitializedData", &self.SizeOfInitializedData)
            .field("SizeOfUninitializedData", &self.SizeOfUninitializedData)
            .field("AddressOfEntryPoint", &self.AddressOfEntryPoint)
            .field("BaseOfCode", &self.BaseOfCode)
            .field("BaseOfData", &self.BaseOfData)
            .field("ImageBase", &self.ImageBase)
            .field("SectionAlignment", &self.SectionAlignment)
            .field("FileAlignment", &self.FileAlignment)
            .field("MajorOperatingSystemVersion", &self.MajorOperatingSystemVersion)
            .field("MinorOperatingSystemVersion", &self.MinorOperatingSystemVersion)
            .field("MajorImageVersion", &self.MajorImageVersion)
            .field("MinorImageVersion", &self.MinorImageVersion)
            .field("MajorSubsystemVersion", &self.MajorSubsystemVersion)
            .field("MinorSubsystemVersion", &self.MinorSubsystemVersion)
            .field("Win32VersionValue", &self.Win32VersionValue)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("SizeOfHeaders", &self.SizeOfHeaders)
            .field("CheckSum", &self.CheckSum)
            .field("Subsystem", &self.Subsystem)
            .field("DllCharacteristics", &self.DllCharacteristics)
            .field("SizeOfStackReserve", &self.SizeOfStackReserve)
            .field("SizeOfStackCommit", &self.SizeOfStackCommit)
            .field("SizeOfHeapReserve", &self.SizeOfHeapReserve)
            .field("SizeOfHeapCommit", &self.SizeOfHeapCommit)
            .field("LoaderFlags", &self.LoaderFlags)
            .field("NumberOfRvaAndSizes", &self.NumberOfRvaAndSizes)
            .field("DataDirectory", &self.DataDirectory)
            .finish()
    }
}
impl ::core::default::Default for IMAGE_OPTIONAL_HEADER64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_OPTIONAL_HEADER_MAGIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_OPTIONAL_HEADER_MAGIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_OPTIONAL_HEADER_MAGIC").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_ROM_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for IMAGE_ROM_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.FileHeader == other.FileHeader && self.OptionalHeader == other.OptionalHeader
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for IMAGE_ROM_HEADERS {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for IMAGE_ROM_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ROM_HEADERS").field("FileHeader", &self.FileHeader).field("OptionalHeader", &self.OptionalHeader).finish()
    }
}
impl ::core::default::Default for IMAGE_ROM_OPTIONAL_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGE_ROM_OPTIONAL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic && self.MajorLinkerVersion == other.MajorLinkerVersion && self.MinorLinkerVersion == other.MinorLinkerVersion && self.SizeOfCode == other.SizeOfCode && self.SizeOfInitializedData == other.SizeOfInitializedData && self.SizeOfUninitializedData == other.SizeOfUninitializedData && self.AddressOfEntryPoint == other.AddressOfEntryPoint && self.BaseOfCode == other.BaseOfCode && self.BaseOfData == other.BaseOfData && self.BaseOfBss == other.BaseOfBss && self.GprMask == other.GprMask && self.CprMask == other.CprMask && self.GpValue == other.GpValue
    }
}
impl ::core::cmp::Eq for IMAGE_ROM_OPTIONAL_HEADER {}
impl ::core::fmt::Debug for IMAGE_ROM_OPTIONAL_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ROM_OPTIONAL_HEADER")
            .field("Magic", &self.Magic)
            .field("MajorLinkerVersion", &self.MajorLinkerVersion)
            .field("MinorLinkerVersion", &self.MinorLinkerVersion)
            .field("SizeOfCode", &self.SizeOfCode)
            .field("SizeOfInitializedData", &self.SizeOfInitializedData)
            .field("SizeOfUninitializedData", &self.SizeOfUninitializedData)
            .field("AddressOfEntryPoint", &self.AddressOfEntryPoint)
            .field("BaseOfCode", &self.BaseOfCode)
            .field("BaseOfData", &self.BaseOfData)
            .field("BaseOfBss", &self.BaseOfBss)
            .field("GprMask", &self.GprMask)
            .field("CprMask", &self.CprMask)
            .field("GpValue", &self.GpValue)
            .finish()
    }
}
impl ::core::default::Default for IMAGE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_SECTION_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_SECTION_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_SECTION_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_SECTION_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_SECTION_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_SECTION_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_SECTION_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_SECTION_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IMAGE_SECTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IMAGE_SUBSYSTEM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_SUBSYSTEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_SUBSYSTEM").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMachineDebugManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineDebugManager {}
impl ::core::fmt::Debug for IMachineDebugManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineDebugManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMachineDebugManagerCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineDebugManagerCookie {}
impl ::core::fmt::Debug for IMachineDebugManagerCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineDebugManagerCookie").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMachineDebugManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineDebugManagerEvents {}
impl ::core::fmt::Debug for IMachineDebugManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineDebugManagerEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IModelIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IModelIterator {}
impl ::core::fmt::Debug for IModelIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IModelIterator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IModelKeyReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IModelKeyReference {}
impl ::core::fmt::Debug for IModelKeyReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IModelKeyReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IModelKeyReference2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IModelKeyReference2 {}
impl ::core::fmt::Debug for IModelKeyReference2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IModelKeyReference2").field(&self.0).finish()
    }
}
impl IModelKeyReference2 {
    pub unsafe fn GetKeyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetKeyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOriginalObject(&self) -> ::windows::core::Result<IModelObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOriginalObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContextObject(&self) -> ::windows::core::Result<IModelObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContextObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetKey(&self, object: ::core::option::Option<*mut ::core::option::Option<IModelObject>>, metadata: ::core::option::Option<*mut ::core::option::Option<IKeyStore>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetKey)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(metadata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetKeyValue(&self, object: ::core::option::Option<*mut ::core::option::Option<IModelObject>>, metadata: ::core::option::Option<*mut ::core::option::Option<IKeyStore>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetKeyValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(metadata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetKey<P0, P1>(&self, object: P0, metadata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<IKeyStore>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetKey)(::windows::core::Vtable::as_raw(self), object.into().abi(), metadata.into().abi()).ok()
    }
    pub unsafe fn SetKeyValue<P0>(&self, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IModelObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetKeyValue)(::windows::core::Vtable::as_raw(self), object.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IModelMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IModelMethod {}
impl ::core::fmt::Debug for IModelMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IModelMethod").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IModelObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IModelObject {}
impl ::core::fmt::Debug for IModelObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IModelObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IModelPropertyAccessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IModelPropertyAccessor {}
impl ::core::fmt::Debug for IModelPropertyAccessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IModelPropertyAccessor").field(&self.0).finish()
    }
}
impl ::core::default::Default for INLINE_FRAME_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IOSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IOSPACE {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for IOSPACE {}
impl ::core::fmt::Debug for IOSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IOSPACE").field("Address", &self.Address).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for IOSPACE32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IOSPACE32 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for IOSPACE32 {}
impl ::core::fmt::Debug for IOSPACE32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IOSPACE32").field("Address", &self.Address).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for IOSPACE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IOSPACE64 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for IOSPACE64 {}
impl ::core::fmt::Debug for IOSPACE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IOSPACE64").field("Address", &self.Address).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for IOSPACE_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IOSPACE_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length && self.Data == other.Data && self.InterfaceType == other.InterfaceType && self.BusNumber == other.BusNumber && self.AddressSpace == other.AddressSpace
    }
}
impl ::core::cmp::Eq for IOSPACE_EX {}
impl ::core::fmt::Debug for IOSPACE_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IOSPACE_EX").field("Address", &self.Address).field("Length", &self.Length).field("Data", &self.Data).field("InterfaceType", &self.InterfaceType).field("BusNumber", &self.BusNumber).field("AddressSpace", &self.AddressSpace).finish()
    }
}
impl ::core::default::Default for IOSPACE_EX32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IOSPACE_EX32 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length && self.Data == other.Data && self.InterfaceType == other.InterfaceType && self.BusNumber == other.BusNumber && self.AddressSpace == other.AddressSpace
    }
}
impl ::core::cmp::Eq for IOSPACE_EX32 {}
impl ::core::fmt::Debug for IOSPACE_EX32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IOSPACE_EX32").field("Address", &self.Address).field("Length", &self.Length).field("Data", &self.Data).field("InterfaceType", &self.InterfaceType).field("BusNumber", &self.BusNumber).field("AddressSpace", &self.AddressSpace).finish()
    }
}
impl ::core::default::Default for IOSPACE_EX64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IOSPACE_EX64 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length && self.Data == other.Data && self.InterfaceType == other.InterfaceType && self.BusNumber == other.BusNumber && self.AddressSpace == other.AddressSpace
    }
}
impl ::core::cmp::Eq for IOSPACE_EX64 {}
impl ::core::fmt::Debug for IOSPACE_EX64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IOSPACE_EX64").field("Address", &self.Address).field("Length", &self.Length).field("Data", &self.Data).field("InterfaceType", &self.InterfaceType).field("BusNumber", &self.BusNumber).field("AddressSpace", &self.AddressSpace).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectSafety {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectSafety {}
impl ::core::fmt::Debug for IObjectSafety {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectSafety").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPMI_OS_SEL_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPMI_OS_SEL_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPMI_OS_SEL_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMI_OS_SEL_RECORD_TYPE").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPerPropertyBrowsing2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerPropertyBrowsing2 {}
impl ::core::fmt::Debug for IPerPropertyBrowsing2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerPropertyBrowsing2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPreferredRuntimeTypeConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPreferredRuntimeTypeConcept {}
impl ::core::fmt::Debug for IPreferredRuntimeTypeConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPreferredRuntimeTypeConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProcessDebugManager32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessDebugManager32 {}
impl ::core::fmt::Debug for IProcessDebugManager32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessDebugManager32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProcessDebugManager64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessDebugManager64 {}
impl ::core::fmt::Debug for IProcessDebugManager64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessDebugManager64").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProvideExpressionContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideExpressionContexts {}
impl ::core::fmt::Debug for IProvideExpressionContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideExpressionContexts").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawEnumerator {}
impl ::core::fmt::Debug for IRawEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteDebugApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplication {}
impl ::core::fmt::Debug for IRemoteDebugApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplication").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteDebugApplication110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplication110 {}
impl ::core::fmt::Debug for IRemoteDebugApplication110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplication110").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteDebugApplicationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplicationEvents {}
impl ::core::fmt::Debug for IRemoteDebugApplicationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplicationEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteDebugApplicationThread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplicationThread {}
impl ::core::fmt::Debug for IRemoteDebugApplicationThread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplicationThread").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteDebugCriticalErrorEvent110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugCriticalErrorEvent110 {}
impl ::core::fmt::Debug for IRemoteDebugCriticalErrorEvent110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugCriticalErrorEvent110").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteDebugInfoEvent110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugInfoEvent110 {}
impl ::core::fmt::Debug for IRemoteDebugInfoEvent110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugInfoEvent110").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IScriptEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptEntry {}
impl ::core::fmt::Debug for IScriptEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptEntry").field(&self.0).finish()
    }
}
impl IScriptEntry {
    pub unsafe fn Alive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Alive)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIndexInParent(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIndexInParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCookie(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCookie)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNumberOfChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNumberOfChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChild(&self, isn: u32) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChild)(::windows::core::Vtable::as_raw(self), isn, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateChildEntry<P0>(&self, isn: u32, dwcookie: u32, pszdelimiter: P0) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateChildEntry)(::windows::core::Vtable::as_raw(self), isn, dwcookie, pszdelimiter.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateChildHandler<P0, P1, P2, P3>(&self, pszdefaultname: P0, prgpsznames: &[::windows::core::PCWSTR], pszevent: P1, pszdelimiter: P2, ptisignature: P3, imethodsignature: u32, isn: u32, dwcookie: u32) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::super::Com::ITypeInfo>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateChildHandler)(::windows::core::Vtable::as_raw(self), pszdefaultname.into().abi(), ::core::mem::transmute(prgpsznames.as_ptr()), prgpsznames.len() as _, pszevent.into().abi(), pszdelimiter.into().abi(), ptisignature.into().abi(), imethodsignature, isn, dwcookie, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IScriptInvocationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptInvocationContext {}
impl ::core::fmt::Debug for IScriptInvocationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptInvocationContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IScriptNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptNode {}
impl ::core::fmt::Debug for IScriptNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptNode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IScriptScriptlet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptScriptlet {}
impl ::core::fmt::Debug for IScriptScriptlet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptScriptlet").field(&self.0).finish()
    }
}
impl IScriptScriptlet {
    pub unsafe fn Alive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Alive)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIndexInParent(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetIndexInParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCookie(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCookie)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNumberOfChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNumberOfChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChild(&self, isn: u32) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetChild)(::windows::core::Vtable::as_raw(self), isn, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateChildEntry<P0>(&self, isn: u32, dwcookie: u32, pszdelimiter: P0) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateChildEntry)(::windows::core::Vtable::as_raw(self), isn, dwcookie, pszdelimiter.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateChildHandler<P0, P1, P2, P3>(&self, pszdefaultname: P0, prgpsznames: &[::windows::core::PCWSTR], pszevent: P1, pszdelimiter: P2, ptisignature: P3, imethodsignature: u32, isn: u32, dwcookie: u32) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::super::Com::ITypeInfo>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateChildHandler)(::windows::core::Vtable::as_raw(self), pszdefaultname.into().abi(), ::core::mem::transmute(prgpsznames.as_ptr()), prgpsznames.len() as _, pszevent.into().abi(), pszdelimiter.into().abi(), ptisignature.into().abi(), imethodsignature, isn, dwcookie, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetText<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetText)(::windows::core::Vtable::as_raw(self), psz.into().abi()).ok()
    }
    pub unsafe fn GetBody(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBody)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBody<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBody)(::windows::core::Vtable::as_raw(self), psz.into().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), psz.into().abi()).ok()
    }
    pub unsafe fn GetItemName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetItemName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetItemName)(::windows::core::Vtable::as_raw(self), psz.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignature(&self, ppti: *mut ::core::option::Option<super::super::Com::ITypeInfo>, pimethod: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSignature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppti), pimethod).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignature<P0>(&self, pti: P0, imethod: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Com::ITypeInfo>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSignature)(::windows::core::Vtable::as_raw(self), pti.into().abi(), imethod).ok()
    }
    pub unsafe fn GetRange(&self, pichmin: *mut u32, pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRange)(::windows::core::Vtable::as_raw(self), pichmin, pcch).ok()
    }
}
impl ::core::cmp::PartialEq for ISimpleConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleConnectionPoint {}
impl ::core::fmt::Debug for ISimpleConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleConnectionPoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStringDisplayableConcept {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStringDisplayableConcept {}
impl ::core::fmt::Debug for IStringDisplayableConcept {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStringDisplayableConcept").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITridentEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITridentEventSink {}
impl ::core::fmt::Debug for ITridentEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITridentEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWebAppDiagnosticsObjectInitialization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAppDiagnosticsObjectInitialization {}
impl ::core::fmt::Debug for IWebAppDiagnosticsObjectInitialization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAppDiagnosticsObjectInitialization").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWebAppDiagnosticsSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAppDiagnosticsSetup {}
impl ::core::fmt::Debug for IWebAppDiagnosticsSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAppDiagnosticsSetup").field(&self.0).finish()
    }
}
impl ::core::default::Default for IntrinsicKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IntrinsicKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IntrinsicKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for JS_NATIVE_FRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JS_NATIVE_FRAME {
    fn eq(&self, other: &Self) -> bool {
        self.InstructionOffset == other.InstructionOffset && self.ReturnOffset == other.ReturnOffset && self.FrameOffset == other.FrameOffset && self.StackOffset == other.StackOffset
    }
}
impl ::core::cmp::Eq for JS_NATIVE_FRAME {}
impl ::core::fmt::Debug for JS_NATIVE_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JS_NATIVE_FRAME").field("InstructionOffset", &self.InstructionOffset).field("ReturnOffset", &self.ReturnOffset).field("FrameOffset", &self.FrameOffset).field("StackOffset", &self.StackOffset).finish()
    }
}
impl ::core::default::Default for JS_PROPERTY_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JS_PROPERTY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JS_PROPERTY_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::default::Default for JS_PROPERTY_MEMBERS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JS_PROPERTY_MEMBERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JS_PROPERTY_MEMBERS").field(&self.0).finish()
    }
}
impl ::core::default::Default for JsDebugPropertyInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JsDebugPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.value == other.value && self.fullName == other.fullName && self.attr == other.attr
    }
}
impl ::core::cmp::Eq for JsDebugPropertyInfo {}
impl ::core::fmt::Debug for JsDebugPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JsDebugPropertyInfo").field("name", &self.name).field("type", &self.r#type).field("value", &self.value).field("fullName", &self.fullName).field("attr", &self.attr).finish()
    }
}
impl ::core::default::Default for JsDebugReadMemoryFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JsDebugReadMemoryFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsDebugReadMemoryFlags").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for KDDEBUGGER_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for KDDEBUGGER_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.KernBase == other.KernBase
            && self.BreakpointWithStatus == other.BreakpointWithStatus
            && self.SavedContext == other.SavedContext
            && self.ThCallbackStack == other.ThCallbackStack
            && self.NextCallback == other.NextCallback
            && self.FramePointer == other.FramePointer
            && self._bitfield == other._bitfield
            && self.KiCallUserMode == other.KiCallUserMode
            && self.KeUserCallbackDispatcher == other.KeUserCallbackDispatcher
            && self.PsLoadedModuleList == other.PsLoadedModuleList
            && self.PsActiveProcessHead == other.PsActiveProcessHead
            && self.PspCidTable == other.PspCidTable
            && self.ExpSystemResourcesList == other.ExpSystemResourcesList
            && self.ExpPagedPoolDescriptor == other.ExpPagedPoolDescriptor
            && self.ExpNumberOfPagedPools == other.ExpNumberOfPagedPools
            && self.KeTimeIncrement == other.KeTimeIncrement
            && self.KeBugCheckCallbackListHead == other.KeBugCheckCallbackListHead
            && self.KiBugcheckData == other.KiBugcheckData
            && self.IopErrorLogListHead == other.IopErrorLogListHead
            && self.ObpRootDirectoryObject == other.ObpRootDirectoryObject
            && self.ObpTypeObjectType == other.ObpTypeObjectType
            && self.MmSystemCacheStart == other.MmSystemCacheStart
            && self.MmSystemCacheEnd == other.MmSystemCacheEnd
            && self.MmSystemCacheWs == other.MmSystemCacheWs
            && self.MmPfnDatabase == other.MmPfnDatabase
            && self.MmSystemPtesStart == other.MmSystemPtesStart
            && self.MmSystemPtesEnd == other.MmSystemPtesEnd
            && self.MmSubsectionBase == other.MmSubsectionBase
            && self.MmNumberOfPagingFiles == other.MmNumberOfPagingFiles
            && self.MmLowestPhysicalPage == other.MmLowestPhysicalPage
            && self.MmHighestPhysicalPage == other.MmHighestPhysicalPage
            && self.MmNumberOfPhysicalPages == other.MmNumberOfPhysicalPages
            && self.MmMaximumNonPagedPoolInBytes == other.MmMaximumNonPagedPoolInBytes
            && self.MmNonPagedSystemStart == other.MmNonPagedSystemStart
            && self.MmNonPagedPoolStart == other.MmNonPagedPoolStart
            && self.MmNonPagedPoolEnd == other.MmNonPagedPoolEnd
            && self.MmPagedPoolStart == other.MmPagedPoolStart
            && self.MmPagedPoolEnd == other.MmPagedPoolEnd
            && self.MmPagedPoolInformation == other.MmPagedPoolInformation
            && self.MmPageSize == other.MmPageSize
            && self.MmSizeOfPagedPoolInBytes == other.MmSizeOfPagedPoolInBytes
            && self.MmTotalCommitLimit == other.MmTotalCommitLimit
            && self.MmTotalCommittedPages == other.MmTotalCommittedPages
            && self.MmSharedCommit == other.MmSharedCommit
            && self.MmDriverCommit == other.MmDriverCommit
            && self.MmProcessCommit == other.MmProcessCommit
            && self.MmPagedPoolCommit == other.MmPagedPoolCommit
            && self.MmExtendedCommit == other.MmExtendedCommit
            && self.MmZeroedPageListHead == other.MmZeroedPageListHead
            && self.MmFreePageListHead == other.MmFreePageListHead
            && self.MmStandbyPageListHead == other.MmStandbyPageListHead
            && self.MmModifiedPageListHead == other.MmModifiedPageListHead
            && self.MmModifiedNoWritePageListHead == other.MmModifiedNoWritePageListHead
            && self.MmAvailablePages == other.MmAvailablePages
            && self.MmResidentAvailablePages == other.MmResidentAvailablePages
            && self.PoolTrackTable == other.PoolTrackTable
            && self.NonPagedPoolDescriptor == other.NonPagedPoolDescriptor
            && self.MmHighestUserAddress == other.MmHighestUserAddress
            && self.MmSystemRangeStart == other.MmSystemRangeStart
            && self.MmUserProbeAddress == other.MmUserProbeAddress
            && self.KdPrintCircularBuffer == other.KdPrintCircularBuffer
            && self.KdPrintCircularBufferEnd == other.KdPrintCircularBufferEnd
            && self.KdPrintWritePointer == other.KdPrintWritePointer
            && self.KdPrintRolloverCount == other.KdPrintRolloverCount
            && self.MmLoadedUserImageList == other.MmLoadedUserImageList
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for KDDEBUGGER_DATA32 {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for KDDEBUGGER_DATA32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KDDEBUGGER_DATA32")
            .field("Header", &self.Header)
            .field("KernBase", &self.KernBase)
            .field("BreakpointWithStatus", &self.BreakpointWithStatus)
            .field("SavedContext", &self.SavedContext)
            .field("ThCallbackStack", &self.ThCallbackStack)
            .field("NextCallback", &self.NextCallback)
            .field("FramePointer", &self.FramePointer)
            .field("_bitfield", &self._bitfield)
            .field("KiCallUserMode", &self.KiCallUserMode)
            .field("KeUserCallbackDispatcher", &self.KeUserCallbackDispatcher)
            .field("PsLoadedModuleList", &self.PsLoadedModuleList)
            .field("PsActiveProcessHead", &self.PsActiveProcessHead)
            .field("PspCidTable", &self.PspCidTable)
            .field("ExpSystemResourcesList", &self.ExpSystemResourcesList)
            .field("ExpPagedPoolDescriptor", &self.ExpPagedPoolDescriptor)
            .field("ExpNumberOfPagedPools", &self.ExpNumberOfPagedPools)
            .field("KeTimeIncrement", &self.KeTimeIncrement)
            .field("KeBugCheckCallbackListHead", &self.KeBugCheckCallbackListHead)
            .field("KiBugcheckData", &self.KiBugcheckData)
            .field("IopErrorLogListHead", &self.IopErrorLogListHead)
            .field("ObpRootDirectoryObject", &self.ObpRootDirectoryObject)
            .field("ObpTypeObjectType", &self.ObpTypeObjectType)
            .field("MmSystemCacheStart", &self.MmSystemCacheStart)
            .field("MmSystemCacheEnd", &self.MmSystemCacheEnd)
            .field("MmSystemCacheWs", &self.MmSystemCacheWs)
            .field("MmPfnDatabase", &self.MmPfnDatabase)
            .field("MmSystemPtesStart", &self.MmSystemPtesStart)
            .field("MmSystemPtesEnd", &self.MmSystemPtesEnd)
            .field("MmSubsectionBase", &self.MmSubsectionBase)
            .field("MmNumberOfPagingFiles", &self.MmNumberOfPagingFiles)
            .field("MmLowestPhysicalPage", &self.MmLowestPhysicalPage)
            .field("MmHighestPhysicalPage", &self.MmHighestPhysicalPage)
            .field("MmNumberOfPhysicalPages", &self.MmNumberOfPhysicalPages)
            .field("MmMaximumNonPagedPoolInBytes", &self.MmMaximumNonPagedPoolInBytes)
            .field("MmNonPagedSystemStart", &self.MmNonPagedSystemStart)
            .field("MmNonPagedPoolStart", &self.MmNonPagedPoolStart)
            .field("MmNonPagedPoolEnd", &self.MmNonPagedPoolEnd)
            .field("MmPagedPoolStart", &self.MmPagedPoolStart)
            .field("MmPagedPoolEnd", &self.MmPagedPoolEnd)
            .field("MmPagedPoolInformation", &self.MmPagedPoolInformation)
            .field("MmPageSize", &self.MmPageSize)
            .field("MmSizeOfPagedPoolInBytes", &self.MmSizeOfPagedPoolInBytes)
            .field("MmTotalCommitLimit", &self.MmTotalCommitLimit)
            .field("MmTotalCommittedPages", &self.MmTotalCommittedPages)
            .field("MmSharedCommit", &self.MmSharedCommit)
            .field("MmDriverCommit", &self.MmDriverCommit)
            .field("MmProcessCommit", &self.MmProcessCommit)
            .field("MmPagedPoolCommit", &self.MmPagedPoolCommit)
            .field("MmExtendedCommit", &self.MmExtendedCommit)
            .field("MmZeroedPageListHead", &self.MmZeroedPageListHead)
            .field("MmFreePageListHead", &self.MmFreePageListHead)
            .field("MmStandbyPageListHead", &self.MmStandbyPageListHead)
            .field("MmModifiedPageListHead", &self.MmModifiedPageListHead)
            .field("MmModifiedNoWritePageListHead", &self.MmModifiedNoWritePageListHead)
            .field("MmAvailablePages", &self.MmAvailablePages)
            .field("MmResidentAvailablePages", &self.MmResidentAvailablePages)
            .field("PoolTrackTable", &self.PoolTrackTable)
            .field("NonPagedPoolDescriptor", &self.NonPagedPoolDescriptor)
            .field("MmHighestUserAddress", &self.MmHighestUserAddress)
            .field("MmSystemRangeStart", &self.MmSystemRangeStart)
            .field("MmUserProbeAddress", &self.MmUserProbeAddress)
            .field("KdPrintCircularBuffer", &self.KdPrintCircularBuffer)
            .field("KdPrintCircularBufferEnd", &self.KdPrintCircularBufferEnd)
            .field("KdPrintWritePointer", &self.KdPrintWritePointer)
            .field("KdPrintRolloverCount", &self.KdPrintRolloverCount)
            .field("MmLoadedUserImageList", &self.MmLoadedUserImageList)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for KDDEBUGGER_DATA64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for KDDEBUGGER_DATA64 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.KernBase == other.KernBase
            && self.BreakpointWithStatus == other.BreakpointWithStatus
            && self.SavedContext == other.SavedContext
            && self.ThCallbackStack == other.ThCallbackStack
            && self.NextCallback == other.NextCallback
            && self.FramePointer == other.FramePointer
            && self._bitfield == other._bitfield
            && self.KiCallUserMode == other.KiCallUserMode
            && self.KeUserCallbackDispatcher == other.KeUserCallbackDispatcher
            && self.PsLoadedModuleList == other.PsLoadedModuleList
            && self.PsActiveProcessHead == other.PsActiveProcessHead
            && self.PspCidTable == other.PspCidTable
            && self.ExpSystemResourcesList == other.ExpSystemResourcesList
            && self.ExpPagedPoolDescriptor == other.ExpPagedPoolDescriptor
            && self.ExpNumberOfPagedPools == other.ExpNumberOfPagedPools
            && self.KeTimeIncrement == other.KeTimeIncrement
            && self.KeBugCheckCallbackListHead == other.KeBugCheckCallbackListHead
            && self.KiBugcheckData == other.KiBugcheckData
            && self.IopErrorLogListHead == other.IopErrorLogListHead
            && self.ObpRootDirectoryObject == other.ObpRootDirectoryObject
            && self.ObpTypeObjectType == other.ObpTypeObjectType
            && self.MmSystemCacheStart == other.MmSystemCacheStart
            && self.MmSystemCacheEnd == other.MmSystemCacheEnd
            && self.MmSystemCacheWs == other.MmSystemCacheWs
            && self.MmPfnDatabase == other.MmPfnDatabase
            && self.MmSystemPtesStart == other.MmSystemPtesStart
            && self.MmSystemPtesEnd == other.MmSystemPtesEnd
            && self.MmSubsectionBase == other.MmSubsectionBase
            && self.MmNumberOfPagingFiles == other.MmNumberOfPagingFiles
            && self.MmLowestPhysicalPage == other.MmLowestPhysicalPage
            && self.MmHighestPhysicalPage == other.MmHighestPhysicalPage
            && self.MmNumberOfPhysicalPages == other.MmNumberOfPhysicalPages
            && self.MmMaximumNonPagedPoolInBytes == other.MmMaximumNonPagedPoolInBytes
            && self.MmNonPagedSystemStart == other.MmNonPagedSystemStart
            && self.MmNonPagedPoolStart == other.MmNonPagedPoolStart
            && self.MmNonPagedPoolEnd == other.MmNonPagedPoolEnd
            && self.MmPagedPoolStart == other.MmPagedPoolStart
            && self.MmPagedPoolEnd == other.MmPagedPoolEnd
            && self.MmPagedPoolInformation == other.MmPagedPoolInformation
            && self.MmPageSize == other.MmPageSize
            && self.MmSizeOfPagedPoolInBytes == other.MmSizeOfPagedPoolInBytes
            && self.MmTotalCommitLimit == other.MmTotalCommitLimit
            && self.MmTotalCommittedPages == other.MmTotalCommittedPages
            && self.MmSharedCommit == other.MmSharedCommit
            && self.MmDriverCommit == other.MmDriverCommit
            && self.MmProcessCommit == other.MmProcessCommit
            && self.MmPagedPoolCommit == other.MmPagedPoolCommit
            && self.MmExtendedCommit == other.MmExtendedCommit
            && self.MmZeroedPageListHead == other.MmZeroedPageListHead
            && self.MmFreePageListHead == other.MmFreePageListHead
            && self.MmStandbyPageListHead == other.MmStandbyPageListHead
            && self.MmModifiedPageListHead == other.MmModifiedPageListHead
            && self.MmModifiedNoWritePageListHead == other.MmModifiedNoWritePageListHead
            && self.MmAvailablePages == other.MmAvailablePages
            && self.MmResidentAvailablePages == other.MmResidentAvailablePages
            && self.PoolTrackTable == other.PoolTrackTable
            && self.NonPagedPoolDescriptor == other.NonPagedPoolDescriptor
            && self.MmHighestUserAddress == other.MmHighestUserAddress
            && self.MmSystemRangeStart == other.MmSystemRangeStart
            && self.MmUserProbeAddress == other.MmUserProbeAddress
            && self.KdPrintCircularBuffer == other.KdPrintCircularBuffer
            && self.KdPrintCircularBufferEnd == other.KdPrintCircularBufferEnd
            && self.KdPrintWritePointer == other.KdPrintWritePointer
            && self.KdPrintRolloverCount == other.KdPrintRolloverCount
            && self.MmLoadedUserImageList == other.MmLoadedUserImageList
            && self.NtBuildLab == other.NtBuildLab
            && self.KiNormalSystemCall == other.KiNormalSystemCall
            && self.KiProcessorBlock == other.KiProcessorBlock
            && self.MmUnloadedDrivers == other.MmUnloadedDrivers
            && self.MmLastUnloadedDriver == other.MmLastUnloadedDriver
            && self.MmTriageActionTaken == other.MmTriageActionTaken
            && self.MmSpecialPoolTag == other.MmSpecialPoolTag
            && self.KernelVerifier == other.KernelVerifier
            && self.MmVerifierData == other.MmVerifierData
            && self.MmAllocatedNonPagedPool == other.MmAllocatedNonPagedPool
            && self.MmPeakCommitment == other.MmPeakCommitment
            && self.MmTotalCommitLimitMaximum == other.MmTotalCommitLimitMaximum
            && self.CmNtCSDVersion == other.CmNtCSDVersion
            && self.MmPhysicalMemoryBlock == other.MmPhysicalMemoryBlock
            && self.MmSessionBase == other.MmSessionBase
            && self.MmSessionSize == other.MmSessionSize
            && self.MmSystemParentTablePage == other.MmSystemParentTablePage
            && self.MmVirtualTranslationBase == other.MmVirtualTranslationBase
            && self.OffsetKThreadNextProcessor == other.OffsetKThreadNextProcessor
            && self.OffsetKThreadTeb == other.OffsetKThreadTeb
            && self.OffsetKThreadKernelStack == other.OffsetKThreadKernelStack
            && self.OffsetKThreadInitialStack == other.OffsetKThreadInitialStack
            && self.OffsetKThreadApcProcess == other.OffsetKThreadApcProcess
            && self.OffsetKThreadState == other.OffsetKThreadState
            && self.OffsetKThreadBStore == other.OffsetKThreadBStore
            && self.OffsetKThreadBStoreLimit == other.OffsetKThreadBStoreLimit
            && self.SizeEProcess == other.SizeEProcess
            && self.OffsetEprocessPeb == other.OffsetEprocessPeb
            && self.OffsetEprocessParentCID == other.OffsetEprocessParentCID
            && self.OffsetEprocessDirectoryTableBase == other.OffsetEprocessDirectoryTableBase
            && self.SizePrcb == other.SizePrcb
            && self.OffsetPrcbDpcRoutine == other.OffsetPrcbDpcRoutine
            && self.OffsetPrcbCurrentThread == other.OffsetPrcbCurrentThread
            && self.OffsetPrcbMhz == other.OffsetPrcbMhz
            && self.OffsetPrcbCpuType == other.OffsetPrcbCpuType
            && self.OffsetPrcbVendorString == other.OffsetPrcbVendorString
            && self.OffsetPrcbProcStateContext == other.OffsetPrcbProcStateContext
            && self.OffsetPrcbNumber == other.OffsetPrcbNumber
            && self.SizeEThread == other.SizeEThread
            && self.L1tfHighPhysicalBitIndex == other.L1tfHighPhysicalBitIndex
            && self.L1tfSwizzleBitIndex == other.L1tfSwizzleBitIndex
            && self.Padding0 == other.Padding0
            && self.KdPrintCircularBufferPtr == other.KdPrintCircularBufferPtr
            && self.KdPrintBufferSize == other.KdPrintBufferSize
            && self.KeLoaderBlock == other.KeLoaderBlock
            && self.SizePcr == other.SizePcr
            && self.OffsetPcrSelfPcr == other.OffsetPcrSelfPcr
            && self.OffsetPcrCurrentPrcb == other.OffsetPcrCurrentPrcb
            && self.OffsetPcrContainedPrcb == other.OffsetPcrContainedPrcb
            && self.OffsetPcrInitialBStore == other.OffsetPcrInitialBStore
            && self.OffsetPcrBStoreLimit == other.OffsetPcrBStoreLimit
            && self.OffsetPcrInitialStack == other.OffsetPcrInitialStack
            && self.OffsetPcrStackLimit == other.OffsetPcrStackLimit
            && self.OffsetPrcbPcrPage == other.OffsetPrcbPcrPage
            && self.OffsetPrcbProcStateSpecialReg == other.OffsetPrcbProcStateSpecialReg
            && self.GdtR0Code == other.GdtR0Code
            && self.GdtR0Data == other.GdtR0Data
            && self.GdtR0Pcr == other.GdtR0Pcr
            && self.GdtR3Code == other.GdtR3Code
            && self.GdtR3Data == other.GdtR3Data
            && self.GdtR3Teb == other.GdtR3Teb
            && self.GdtLdt == other.GdtLdt
            && self.GdtTss == other.GdtTss
            && self.Gdt64R3CmCode == other.Gdt64R3CmCode
            && self.Gdt64R3CmTeb == other.Gdt64R3CmTeb
            && self.IopNumTriageDumpDataBlocks == other.IopNumTriageDumpDataBlocks
            && self.IopTriageDumpDataBlocks == other.IopTriageDumpDataBlocks
            && self.VfCrashDataBlock == other.VfCrashDataBlock
            && self.MmBadPagesDetected == other.MmBadPagesDetected
            && self.MmZeroedPageSingleBitErrorsDetected == other.MmZeroedPageSingleBitErrorsDetected
            && self.EtwpDebuggerData == other.EtwpDebuggerData
            && self.OffsetPrcbContext == other.OffsetPrcbContext
            && self.OffsetPrcbMaxBreakpoints == other.OffsetPrcbMaxBreakpoints
            && self.OffsetPrcbMaxWatchpoints == other.OffsetPrcbMaxWatchpoints
            && self.OffsetKThreadStackLimit == other.OffsetKThreadStackLimit
            && self.OffsetKThreadStackBase == other.OffsetKThreadStackBase
            && self.OffsetKThreadQueueListEntry == other.OffsetKThreadQueueListEntry
            && self.OffsetEThreadIrpList == other.OffsetEThreadIrpList
            && self.OffsetPrcbIdleThread == other.OffsetPrcbIdleThread
            && self.OffsetPrcbNormalDpcState == other.OffsetPrcbNormalDpcState
            && self.OffsetPrcbDpcStack == other.OffsetPrcbDpcStack
            && self.OffsetPrcbIsrStack == other.OffsetPrcbIsrStack
            && self.SizeKDPC_STACK_FRAME == other.SizeKDPC_STACK_FRAME
            && self.OffsetKPriQueueThreadListHead == other.OffsetKPriQueueThreadListHead
            && self.OffsetKThreadWaitReason == other.OffsetKThreadWaitReason
            && self.Padding1 == other.Padding1
            && self.PteBase == other.PteBase
            && self.RetpolineStubFunctionTable == other.RetpolineStubFunctionTable
            && self.RetpolineStubFunctionTableSize == other.RetpolineStubFunctionTableSize
            && self.RetpolineStubOffset == other.RetpolineStubOffset
            && self.RetpolineStubSize == other.RetpolineStubSize
            && self.OffsetEProcessMmHotPatchContext == other.OffsetEProcessMmHotPatchContext
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for KDDEBUGGER_DATA64 {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for KDDEBUGGER_DATA64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KDDEBUGGER_DATA64")
            .field("Header", &self.Header)
            .field("KernBase", &self.KernBase)
            .field("BreakpointWithStatus", &self.BreakpointWithStatus)
            .field("SavedContext", &self.SavedContext)
            .field("ThCallbackStack", &self.ThCallbackStack)
            .field("NextCallback", &self.NextCallback)
            .field("FramePointer", &self.FramePointer)
            .field("_bitfield", &self._bitfield)
            .field("KiCallUserMode", &self.KiCallUserMode)
            .field("KeUserCallbackDispatcher", &self.KeUserCallbackDispatcher)
            .field("PsLoadedModuleList", &self.PsLoadedModuleList)
            .field("PsActiveProcessHead", &self.PsActiveProcessHead)
            .field("PspCidTable", &self.PspCidTable)
            .field("ExpSystemResourcesList", &self.ExpSystemResourcesList)
            .field("ExpPagedPoolDescriptor", &self.ExpPagedPoolDescriptor)
            .field("ExpNumberOfPagedPools", &self.ExpNumberOfPagedPools)
            .field("KeTimeIncrement", &self.KeTimeIncrement)
            .field("KeBugCheckCallbackListHead", &self.KeBugCheckCallbackListHead)
            .field("KiBugcheckData", &self.KiBugcheckData)
            .field("IopErrorLogListHead", &self.IopErrorLogListHead)
            .field("ObpRootDirectoryObject", &self.ObpRootDirectoryObject)
            .field("ObpTypeObjectType", &self.ObpTypeObjectType)
            .field("MmSystemCacheStart", &self.MmSystemCacheStart)
            .field("MmSystemCacheEnd", &self.MmSystemCacheEnd)
            .field("MmSystemCacheWs", &self.MmSystemCacheWs)
            .field("MmPfnDatabase", &self.MmPfnDatabase)
            .field("MmSystemPtesStart", &self.MmSystemPtesStart)
            .field("MmSystemPtesEnd", &self.MmSystemPtesEnd)
            .field("MmSubsectionBase", &self.MmSubsectionBase)
            .field("MmNumberOfPagingFiles", &self.MmNumberOfPagingFiles)
            .field("MmLowestPhysicalPage", &self.MmLowestPhysicalPage)
            .field("MmHighestPhysicalPage", &self.MmHighestPhysicalPage)
            .field("MmNumberOfPhysicalPages", &self.MmNumberOfPhysicalPages)
            .field("MmMaximumNonPagedPoolInBytes", &self.MmMaximumNonPagedPoolInBytes)
            .field("MmNonPagedSystemStart", &self.MmNonPagedSystemStart)
            .field("MmNonPagedPoolStart", &self.MmNonPagedPoolStart)
            .field("MmNonPagedPoolEnd", &self.MmNonPagedPoolEnd)
            .field("MmPagedPoolStart", &self.MmPagedPoolStart)
            .field("MmPagedPoolEnd", &self.MmPagedPoolEnd)
            .field("MmPagedPoolInformation", &self.MmPagedPoolInformation)
            .field("MmPageSize", &self.MmPageSize)
            .field("MmSizeOfPagedPoolInBytes", &self.MmSizeOfPagedPoolInBytes)
            .field("MmTotalCommitLimit", &self.MmTotalCommitLimit)
            .field("MmTotalCommittedPages", &self.MmTotalCommittedPages)
            .field("MmSharedCommit", &self.MmSharedCommit)
            .field("MmDriverCommit", &self.MmDriverCommit)
            .field("MmProcessCommit", &self.MmProcessCommit)
            .field("MmPagedPoolCommit", &self.MmPagedPoolCommit)
            .field("MmExtendedCommit", &self.MmExtendedCommit)
            .field("MmZeroedPageListHead", &self.MmZeroedPageListHead)
            .field("MmFreePageListHead", &self.MmFreePageListHead)
            .field("MmStandbyPageListHead", &self.MmStandbyPageListHead)
            .field("MmModifiedPageListHead", &self.MmModifiedPageListHead)
            .field("MmModifiedNoWritePageListHead", &self.MmModifiedNoWritePageListHead)
            .field("MmAvailablePages", &self.MmAvailablePages)
            .field("MmResidentAvailablePages", &self.MmResidentAvailablePages)
            .field("PoolTrackTable", &self.PoolTrackTable)
            .field("NonPagedPoolDescriptor", &self.NonPagedPoolDescriptor)
            .field("MmHighestUserAddress", &self.MmHighestUserAddress)
            .field("MmSystemRangeStart", &self.MmSystemRangeStart)
            .field("MmUserProbeAddress", &self.MmUserProbeAddress)
            .field("KdPrintCircularBuffer", &self.KdPrintCircularBuffer)
            .field("KdPrintCircularBufferEnd", &self.KdPrintCircularBufferEnd)
            .field("KdPrintWritePointer", &self.KdPrintWritePointer)
            .field("KdPrintRolloverCount", &self.KdPrintRolloverCount)
            .field("MmLoadedUserImageList", &self.MmLoadedUserImageList)
            .field("NtBuildLab", &self.NtBuildLab)
            .field("KiNormalSystemCall", &self.KiNormalSystemCall)
            .field("KiProcessorBlock", &self.KiProcessorBlock)
            .field("MmUnloadedDrivers", &self.MmUnloadedDrivers)
            .field("MmLastUnloadedDriver", &self.MmLastUnloadedDriver)
            .field("MmTriageActionTaken", &self.MmTriageActionTaken)
            .field("MmSpecialPoolTag", &self.MmSpecialPoolTag)
            .field("KernelVerifier", &self.KernelVerifier)
            .field("MmVerifierData", &self.MmVerifierData)
            .field("MmAllocatedNonPagedPool", &self.MmAllocatedNonPagedPool)
            .field("MmPeakCommitment", &self.MmPeakCommitment)
            .field("MmTotalCommitLimitMaximum", &self.MmTotalCommitLimitMaximum)
            .field("CmNtCSDVersion", &self.CmNtCSDVersion)
            .field("MmPhysicalMemoryBlock", &self.MmPhysicalMemoryBlock)
            .field("MmSessionBase", &self.MmSessionBase)
            .field("MmSessionSize", &self.MmSessionSize)
            .field("MmSystemParentTablePage", &self.MmSystemParentTablePage)
            .field("MmVirtualTranslationBase", &self.MmVirtualTranslationBase)
            .field("OffsetKThreadNextProcessor", &self.OffsetKThreadNextProcessor)
            .field("OffsetKThreadTeb", &self.OffsetKThreadTeb)
            .field("OffsetKThreadKernelStack", &self.OffsetKThreadKernelStack)
            .field("OffsetKThreadInitialStack", &self.OffsetKThreadInitialStack)
            .field("OffsetKThreadApcProcess", &self.OffsetKThreadApcProcess)
            .field("OffsetKThreadState", &self.OffsetKThreadState)
            .field("OffsetKThreadBStore", &self.OffsetKThreadBStore)
            .field("OffsetKThreadBStoreLimit", &self.OffsetKThreadBStoreLimit)
            .field("SizeEProcess", &self.SizeEProcess)
            .field("OffsetEprocessPeb", &self.OffsetEprocessPeb)
            .field("OffsetEprocessParentCID", &self.OffsetEprocessParentCID)
            .field("OffsetEprocessDirectoryTableBase", &self.OffsetEprocessDirectoryTableBase)
            .field("SizePrcb", &self.SizePrcb)
            .field("OffsetPrcbDpcRoutine", &self.OffsetPrcbDpcRoutine)
            .field("OffsetPrcbCurrentThread", &self.OffsetPrcbCurrentThread)
            .field("OffsetPrcbMhz", &self.OffsetPrcbMhz)
            .field("OffsetPrcbCpuType", &self.OffsetPrcbCpuType)
            .field("OffsetPrcbVendorString", &self.OffsetPrcbVendorString)
            .field("OffsetPrcbProcStateContext", &self.OffsetPrcbProcStateContext)
            .field("OffsetPrcbNumber", &self.OffsetPrcbNumber)
            .field("SizeEThread", &self.SizeEThread)
            .field("L1tfHighPhysicalBitIndex", &self.L1tfHighPhysicalBitIndex)
            .field("L1tfSwizzleBitIndex", &self.L1tfSwizzleBitIndex)
            .field("Padding0", &self.Padding0)
            .field("KdPrintCircularBufferPtr", &self.KdPrintCircularBufferPtr)
            .field("KdPrintBufferSize", &self.KdPrintBufferSize)
            .field("KeLoaderBlock", &self.KeLoaderBlock)
            .field("SizePcr", &self.SizePcr)
            .field("OffsetPcrSelfPcr", &self.OffsetPcrSelfPcr)
            .field("OffsetPcrCurrentPrcb", &self.OffsetPcrCurrentPrcb)
            .field("OffsetPcrContainedPrcb", &self.OffsetPcrContainedPrcb)
            .field("OffsetPcrInitialBStore", &self.OffsetPcrInitialBStore)
            .field("OffsetPcrBStoreLimit", &self.OffsetPcrBStoreLimit)
            .field("OffsetPcrInitialStack", &self.OffsetPcrInitialStack)
            .field("OffsetPcrStackLimit", &self.OffsetPcrStackLimit)
            .field("OffsetPrcbPcrPage", &self.OffsetPrcbPcrPage)
            .field("OffsetPrcbProcStateSpecialReg", &self.OffsetPrcbProcStateSpecialReg)
            .field("GdtR0Code", &self.GdtR0Code)
            .field("GdtR0Data", &self.GdtR0Data)
            .field("GdtR0Pcr", &self.GdtR0Pcr)
            .field("GdtR3Code", &self.GdtR3Code)
            .field("GdtR3Data", &self.GdtR3Data)
            .field("GdtR3Teb", &self.GdtR3Teb)
            .field("GdtLdt", &self.GdtLdt)
            .field("GdtTss", &self.GdtTss)
            .field("Gdt64R3CmCode", &self.Gdt64R3CmCode)
            .field("Gdt64R3CmTeb", &self.Gdt64R3CmTeb)
            .field("IopNumTriageDumpDataBlocks", &self.IopNumTriageDumpDataBlocks)
            .field("IopTriageDumpDataBlocks", &self.IopTriageDumpDataBlocks)
            .field("VfCrashDataBlock", &self.VfCrashDataBlock)
            .field("MmBadPagesDetected", &self.MmBadPagesDetected)
            .field("MmZeroedPageSingleBitErrorsDetected", &self.MmZeroedPageSingleBitErrorsDetected)
            .field("EtwpDebuggerData", &self.EtwpDebuggerData)
            .field("OffsetPrcbContext", &self.OffsetPrcbContext)
            .field("OffsetPrcbMaxBreakpoints", &self.OffsetPrcbMaxBreakpoints)
            .field("OffsetPrcbMaxWatchpoints", &self.OffsetPrcbMaxWatchpoints)
            .field("OffsetKThreadStackLimit", &self.OffsetKThreadStackLimit)
            .field("OffsetKThreadStackBase", &self.OffsetKThreadStackBase)
            .field("OffsetKThreadQueueListEntry", &self.OffsetKThreadQueueListEntry)
            .field("OffsetEThreadIrpList", &self.OffsetEThreadIrpList)
            .field("OffsetPrcbIdleThread", &self.OffsetPrcbIdleThread)
            .field("OffsetPrcbNormalDpcState", &self.OffsetPrcbNormalDpcState)
            .field("OffsetPrcbDpcStack", &self.OffsetPrcbDpcStack)
            .field("OffsetPrcbIsrStack", &self.OffsetPrcbIsrStack)
            .field("SizeKDPC_STACK_FRAME", &self.SizeKDPC_STACK_FRAME)
            .field("OffsetKPriQueueThreadListHead", &self.OffsetKPriQueueThreadListHead)
            .field("OffsetKThreadWaitReason", &self.OffsetKThreadWaitReason)
            .field("Padding1", &self.Padding1)
            .field("PteBase", &self.PteBase)
            .field("RetpolineStubFunctionTable", &self.RetpolineStubFunctionTable)
            .field("RetpolineStubFunctionTableSize", &self.RetpolineStubFunctionTableSize)
            .field("RetpolineStubOffset", &self.RetpolineStubOffset)
            .field("RetpolineStubSize", &self.RetpolineStubSize)
            .field("OffsetEProcessMmHotPatchContext", &self.OffsetEProcessMmHotPatchContext)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KDHELP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for KDHELP {
    fn eq(&self, other: &Self) -> bool {
        self.Thread == other.Thread && self.ThCallbackStack == other.ThCallbackStack && self.NextCallback == other.NextCallback && self.FramePointer == other.FramePointer && self.KiCallUserMode == other.KiCallUserMode && self.KeUserCallbackDispatcher == other.KeUserCallbackDispatcher && self.SystemRangeStart == other.SystemRangeStart && self.ThCallbackBStore == other.ThCallbackBStore && self.KiUserExceptionDispatcher == other.KiUserExceptionDispatcher && self.StackBase == other.StackBase && self.StackLimit == other.StackLimit && self.Reserved == other.Reserved
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for KDHELP {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for KDHELP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KDHELP")
            .field("Thread", &self.Thread)
            .field("ThCallbackStack", &self.ThCallbackStack)
            .field("NextCallback", &self.NextCallback)
            .field("FramePointer", &self.FramePointer)
            .field("KiCallUserMode", &self.KiCallUserMode)
            .field("KeUserCallbackDispatcher", &self.KeUserCallbackDispatcher)
            .field("SystemRangeStart", &self.SystemRangeStart)
            .field("ThCallbackBStore", &self.ThCallbackBStore)
            .field("KiUserExceptionDispatcher", &self.KiUserExceptionDispatcher)
            .field("StackBase", &self.StackBase)
            .field("StackLimit", &self.StackLimit)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for KDHELP64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KDHELP64 {
    fn eq(&self, other: &Self) -> bool {
        self.Thread == other.Thread
            && self.ThCallbackStack == other.ThCallbackStack
            && self.ThCallbackBStore == other.ThCallbackBStore
            && self.NextCallback == other.NextCallback
            && self.FramePointer == other.FramePointer
            && self.KiCallUserMode == other.KiCallUserMode
            && self.KeUserCallbackDispatcher == other.KeUserCallbackDispatcher
            && self.SystemRangeStart == other.SystemRangeStart
            && self.KiUserExceptionDispatcher == other.KiUserExceptionDispatcher
            && self.StackBase == other.StackBase
            && self.StackLimit == other.StackLimit
            && self.BuildVersion == other.BuildVersion
            && self.RetpolineStubFunctionTableSize == other.RetpolineStubFunctionTableSize
            && self.RetpolineStubFunctionTable == other.RetpolineStubFunctionTable
            && self.RetpolineStubOffset == other.RetpolineStubOffset
            && self.RetpolineStubSize == other.RetpolineStubSize
            && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for KDHELP64 {}
impl ::core::fmt::Debug for KDHELP64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KDHELP64")
            .field("Thread", &self.Thread)
            .field("ThCallbackStack", &self.ThCallbackStack)
            .field("ThCallbackBStore", &self.ThCallbackBStore)
            .field("NextCallback", &self.NextCallback)
            .field("FramePointer", &self.FramePointer)
            .field("KiCallUserMode", &self.KiCallUserMode)
            .field("KeUserCallbackDispatcher", &self.KeUserCallbackDispatcher)
            .field("SystemRangeStart", &self.SystemRangeStart)
            .field("KiUserExceptionDispatcher", &self.KiUserExceptionDispatcher)
            .field("StackBase", &self.StackBase)
            .field("StackLimit", &self.StackLimit)
            .field("BuildVersion", &self.BuildVersion)
            .field("RetpolineStubFunctionTableSize", &self.RetpolineStubFunctionTableSize)
            .field("RetpolineStubFunctionTable", &self.RetpolineStubFunctionTable)
            .field("RetpolineStubOffset", &self.RetpolineStubOffset)
            .field("RetpolineStubSize", &self.RetpolineStubSize)
            .field("Reserved0", &self.Reserved0)
            .finish()
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::PartialEq for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn eq(&self, other: &Self) -> bool {
        self.X19 == other.X19 && self.X20 == other.X20 && self.X21 == other.X21 && self.X22 == other.X22 && self.X23 == other.X23 && self.X24 == other.X24 && self.X25 == other.X25 && self.X26 == other.X26 && self.X27 == other.X27 && self.X28 == other.X28 && self.Fp == other.Fp && self.Lr == other.Lr && self.D8 == other.D8 && self.D9 == other.D9 && self.D10 == other.D10 && self.D11 == other.D11 && self.D12 == other.D12 && self.D13 == other.D13 && self.D14 == other.D14 && self.D15 == other.D15
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::Eq for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {}
#[cfg(target_arch = "aarch64")]
impl ::core::fmt::Debug for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KNONVOLATILE_CONTEXT_POINTERS_ARM64")
            .field("X19", &self.X19)
            .field("X20", &self.X20)
            .field("X21", &self.X21)
            .field("X22", &self.X22)
            .field("X23", &self.X23)
            .field("X24", &self.X24)
            .field("X25", &self.X25)
            .field("X26", &self.X26)
            .field("X27", &self.X27)
            .field("X28", &self.X28)
            .field("Fp", &self.Fp)
            .field("Lr", &self.Lr)
            .field("D8", &self.D8)
            .field("D9", &self.D9)
            .field("D10", &self.D10)
            .field("D11", &self.D11)
            .field("D12", &self.D12)
            .field("D13", &self.D13)
            .field("D14", &self.D14)
            .field("D15", &self.D15)
            .finish()
    }
}
impl ::core::default::Default for LDT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::default::Default for LOADED_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::PartialEq for LOADED_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ModuleName == other.ModuleName && self.hFile == other.hFile && self.MappedAddress == other.MappedAddress && self.FileHeader == other.FileHeader && self.LastRvaSection == other.LastRvaSection && self.NumberOfSections == other.NumberOfSections && self.Sections == other.Sections && self.Characteristics == other.Characteristics && self.fSystemImage == other.fSystemImage && self.fDOSImage == other.fDOSImage && self.fReadOnly == other.fReadOnly && self.Version == other.Version && self.Links == other.Links && self.SizeOfImage == other.SizeOfImage
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::Eq for LOADED_IMAGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::fmt::Debug for LOADED_IMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOADED_IMAGE")
            .field("ModuleName", &self.ModuleName)
            .field("hFile", &self.hFile)
            .field("MappedAddress", &self.MappedAddress)
            .field("FileHeader", &self.FileHeader)
            .field("LastRvaSection", &self.LastRvaSection)
            .field("NumberOfSections", &self.NumberOfSections)
            .field("Sections", &self.Sections)
            .field("Characteristics", &self.Characteristics)
            .field("fSystemImage", &self.fSystemImage)
            .field("fDOSImage", &self.fDOSImage)
            .field("fReadOnly", &self.fReadOnly)
            .field("Version", &self.Version)
            .field("Links", &self.Links)
            .field("SizeOfImage", &self.SizeOfImage)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::default::Default for LOADED_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::PartialEq for LOADED_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ModuleName == other.ModuleName && self.hFile == other.hFile && self.MappedAddress == other.MappedAddress && self.FileHeader == other.FileHeader && self.LastRvaSection == other.LastRvaSection && self.NumberOfSections == other.NumberOfSections && self.Sections == other.Sections && self.Characteristics == other.Characteristics && self.fSystemImage == other.fSystemImage && self.fDOSImage == other.fDOSImage && self.fReadOnly == other.fReadOnly && self.Version == other.Version && self.Links == other.Links && self.SizeOfImage == other.SizeOfImage
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::Eq for LOADED_IMAGE {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::fmt::Debug for LOADED_IMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOADED_IMAGE")
            .field("ModuleName", &self.ModuleName)
            .field("hFile", &self.hFile)
            .field("MappedAddress", &self.MappedAddress)
            .field("FileHeader", &self.FileHeader)
            .field("LastRvaSection", &self.LastRvaSection)
            .field("NumberOfSections", &self.NumberOfSections)
            .field("Sections", &self.Sections)
            .field("Characteristics", &self.Characteristics)
            .field("fSystemImage", &self.fSystemImage)
            .field("fDOSImage", &self.fDOSImage)
            .field("fReadOnly", &self.fReadOnly)
            .field("Version", &self.Version)
            .field("Links", &self.Links)
            .field("SizeOfImage", &self.SizeOfImage)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOAD_DLL_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOAD_DLL_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile && self.lpBaseOfDll == other.lpBaseOfDll && self.dwDebugInfoFileOffset == other.dwDebugInfoFileOffset && self.nDebugInfoSize == other.nDebugInfoSize && self.lpImageName == other.lpImageName && self.fUnicode == other.fUnicode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOAD_DLL_DEBUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOAD_DLL_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOAD_DLL_DEBUG_INFO").field("hFile", &self.hFile).field("lpBaseOfDll", &self.lpBaseOfDll).field("dwDebugInfoFileOffset", &self.dwDebugInfoFileOffset).field("nDebugInfoSize", &self.nDebugInfoSize).field("lpImageName", &self.lpImageName).field("fUnicode", &self.fUnicode).finish()
    }
}
impl ::core::default::Default for LanguageKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LanguageKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanguageKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for Location {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.HostDefined == other.HostDefined && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for Location {}
impl ::core::fmt::Debug for Location {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Location").field("HostDefined", &self.HostDefined).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for LocationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LocationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocationKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for M128A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for M128A {
    fn eq(&self, other: &Self) -> bool {
        self.Low == other.Low && self.High == other.High
    }
}
impl ::core::cmp::Eq for M128A {}
impl ::core::fmt::Debug for M128A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("M128A").field("Low", &self.Low).field("High", &self.High).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_CALLBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIDUMP_CALLBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_CALLBACK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MINIDUMP_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_EXCEPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINIDUMP_EXCEPTION_INFORMATION64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_EXCEPTION_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_FUNCTION_TABLE_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_HANDLE_DATA_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_HANDLE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_HANDLE_DESCRIPTOR_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_HANDLE_OBJECT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MINIDUMP_HANDLE_OPERATION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_INCLUDE_MODULE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_INCLUDE_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINIDUMP_IO_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_LOCATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_LOCATION_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MEMORY64_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MEMORY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MEMORY_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Memory")]
impl ::core::default::Default for MINIDUMP_MEMORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MEMORY_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MEMORY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MISC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MISC_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for MINIDUMP_MISC_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for MINIDUMP_MISC_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for MINIDUMP_MISC_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_MISC_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIDUMP_MISC_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_MISC_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MINIDUMP_MISC_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MINIDUMP_MISC_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MINIDUMP_MISC_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MINIDUMP_MISC_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MINIDUMP_MISC_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for MINIDUMP_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for MINIDUMP_MODULE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for MINIDUMP_MODULE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_PROCESS_VM_COUNTERS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_PROCESS_VM_COUNTERS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_SECONDARY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIDUMP_SECONDARY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_SECONDARY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MINIDUMP_STREAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIDUMP_STREAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_STREAM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MINIDUMP_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_SYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_EX_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_EX_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_EX_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_INFO_DUMP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIDUMP_THREAD_INFO_DUMP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_THREAD_INFO_DUMP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_TOKEN_INFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_TOKEN_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIDUMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MINIDUMP_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MINIDUMP_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MINIDUMP_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MINIDUMP_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MINIDUMP_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MINIDUMP_UNLOADED_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_UNLOADED_MODULE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_USER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_USER_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_USER_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_VM_POST_READ_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_VM_PRE_READ_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MINIDUMP_VM_QUERY_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MODLOAD_CVMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODLOAD_CVMISC {
    fn eq(&self, other: &Self) -> bool {
        self.oCV == other.oCV && self.cCV == other.cCV && self.oMisc == other.oMisc && self.cMisc == other.cMisc && self.dtImage == other.dtImage && self.cImage == other.cImage
    }
}
impl ::core::cmp::Eq for MODLOAD_CVMISC {}
impl ::core::fmt::Debug for MODLOAD_CVMISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODLOAD_CVMISC").field("oCV", &self.oCV).field("cCV", &self.cCV).field("oMisc", &self.oMisc).field("cMisc", &self.cMisc).field("dtImage", &self.dtImage).field("cImage", &self.cImage).finish()
    }
}
impl ::core::default::Default for MODLOAD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODLOAD_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ssize == other.ssize && self.ssig == other.ssig && self.data == other.data && self.size == other.size && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MODLOAD_DATA {}
impl ::core::fmt::Debug for MODLOAD_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODLOAD_DATA").field("ssize", &self.ssize).field("ssig", &self.ssig).field("data", &self.data).field("size", &self.size).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MODLOAD_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODLOAD_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODLOAD_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MODLOAD_PDBGUID_PDBAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODLOAD_PDBGUID_PDBAGE {
    fn eq(&self, other: &Self) -> bool {
        self.PdbGuid == other.PdbGuid && self.PdbAge == other.PdbAge
    }
}
impl ::core::cmp::Eq for MODLOAD_PDBGUID_PDBAGE {}
impl ::core::fmt::Debug for MODLOAD_PDBGUID_PDBAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODLOAD_PDBGUID_PDBAGE").field("PdbGuid", &self.PdbGuid).field("PdbAge", &self.PdbAge).finish()
    }
}
impl ::core::default::Default for MODULE_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODULE_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dataLength == other.dataLength && self.leaf == other.leaf && self.data == other.data
    }
}
impl ::core::cmp::Eq for MODULE_TYPE_INFO {}
impl ::core::fmt::Debug for MODULE_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODULE_TYPE_INFO").field("dataLength", &self.dataLength).field("leaf", &self.leaf).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for MODULE_WRITE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODULE_WRITE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODULE_WRITE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ModelObjectKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ModelObjectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ModelObjectKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for OBJECT_ATTRIB_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECT_ATTRIB_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_ATTRIB_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OMAP {
    fn eq(&self, other: &Self) -> bool {
        self.rva == other.rva && self.rvaTo == other.rvaTo
    }
}
impl ::core::cmp::Eq for OMAP {}
impl ::core::fmt::Debug for OMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OMAP").field("rva", &self.rva).field("rvaTo", &self.rvaTo).finish()
    }
}
impl ::core::default::Default for OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OUTPUT_DEBUG_STRING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OUTPUT_DEBUG_STRING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpDebugStringData == other.lpDebugStringData && self.fUnicode == other.fUnicode && self.nDebugStringLength == other.nDebugStringLength
    }
}
impl ::core::cmp::Eq for OUTPUT_DEBUG_STRING_INFO {}
impl ::core::fmt::Debug for OUTPUT_DEBUG_STRING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTPUT_DEBUG_STRING_INFO").field("lpDebugStringData", &self.lpDebugStringData).field("fUnicode", &self.fUnicode).field("nDebugStringLength", &self.nDebugStringLength).finish()
    }
}
impl ::core::default::Default for PHYSICAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.BufLen == other.BufLen && self.Buf == other.Buf
    }
}
impl ::core::cmp::Eq for PHYSICAL {}
impl ::core::fmt::Debug for PHYSICAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL").field("Address", &self.Address).field("BufLen", &self.BufLen).field("Buf", &self.Buf).finish()
    }
}
impl ::core::default::Default for PHYSICAL_MEMORY_DESCRIPTOR32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_DESCRIPTOR32 {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRuns == other.NumberOfRuns && self.NumberOfPages == other.NumberOfPages && self.Run == other.Run
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_DESCRIPTOR32 {}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_DESCRIPTOR32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_DESCRIPTOR32").field("NumberOfRuns", &self.NumberOfRuns).field("NumberOfPages", &self.NumberOfPages).field("Run", &self.Run).finish()
    }
}
impl ::core::default::Default for PHYSICAL_MEMORY_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_DESCRIPTOR64 {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRuns == other.NumberOfRuns && self.NumberOfPages == other.NumberOfPages && self.Run == other.Run
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_DESCRIPTOR64 {}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_DESCRIPTOR64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_DESCRIPTOR64").field("NumberOfRuns", &self.NumberOfRuns).field("NumberOfPages", &self.NumberOfPages).field("Run", &self.Run).finish()
    }
}
impl ::core::default::Default for PHYSICAL_MEMORY_RUN32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_RUN32 {
    fn eq(&self, other: &Self) -> bool {
        self.BasePage == other.BasePage && self.PageCount == other.PageCount
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_RUN32 {}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_RUN32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_RUN32").field("BasePage", &self.BasePage).field("PageCount", &self.PageCount).finish()
    }
}
impl ::core::default::Default for PHYSICAL_MEMORY_RUN64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_RUN64 {
    fn eq(&self, other: &Self) -> bool {
        self.BasePage == other.BasePage && self.PageCount == other.PageCount
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_RUN64 {}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_RUN64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_RUN64").field("BasePage", &self.BasePage).field("PageCount", &self.PageCount).finish()
    }
}
impl ::core::default::Default for PHYSICAL_TO_VIRTUAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_TO_VIRTUAL {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.Size == other.Size && self.PdeAddress == other.PdeAddress
    }
}
impl ::core::cmp::Eq for PHYSICAL_TO_VIRTUAL {}
impl ::core::fmt::Debug for PHYSICAL_TO_VIRTUAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_TO_VIRTUAL").field("Status", &self.Status).field("Size", &self.Size).field("PdeAddress", &self.PdeAddress).finish()
    }
}
impl ::core::default::Default for PHYSICAL_WITH_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_WITH_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.BufLen == other.BufLen && self.Flags == other.Flags && self.Buf == other.Buf
    }
}
impl ::core::cmp::Eq for PHYSICAL_WITH_FLAGS {}
impl ::core::fmt::Debug for PHYSICAL_WITH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_WITH_FLAGS").field("Address", &self.Address).field("BufLen", &self.BufLen).field("Flags", &self.Flags).field("Buf", &self.Buf).finish()
    }
}
impl ::core::default::Default for POINTER_SEARCH_PHYSICAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTER_SEARCH_PHYSICAL {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Length == other.Length && self.PointerMin == other.PointerMin && self.PointerMax == other.PointerMax && self.Flags == other.Flags && self.MatchOffsets == other.MatchOffsets && self.MatchOffsetsSize == other.MatchOffsetsSize && self.MatchOffsetsCount == other.MatchOffsetsCount
    }
}
impl ::core::cmp::Eq for POINTER_SEARCH_PHYSICAL {}
impl ::core::fmt::Debug for POINTER_SEARCH_PHYSICAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_SEARCH_PHYSICAL").field("Offset", &self.Offset).field("Length", &self.Length).field("PointerMin", &self.PointerMin).field("PointerMax", &self.PointerMax).field("Flags", &self.Flags).field("MatchOffsets", &self.MatchOffsets).field("MatchOffsetsSize", &self.MatchOffsetsSize).field("MatchOffsetsCount", &self.MatchOffsetsCount).finish()
    }
}
impl ::core::default::Default for PROCESSORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.NumberProcessors == other.NumberProcessors
    }
}
impl ::core::cmp::Eq for PROCESSORINFO {}
impl ::core::fmt::Debug for PROCESSORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSORINFO").field("Processor", &self.Processor).field("NumberProcessors", &self.NumberProcessors).finish()
    }
}
impl ::core::default::Default for PROCESSOR_ARCHITECTURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESSOR_ARCHITECTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_ARCHITECTURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESS_NAME_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_NAME_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessId == other.ProcessId && self.NameOffset == other.NameOffset && self.NameSize == other.NameSize && self.NextEntry == other.NextEntry
    }
}
impl ::core::cmp::Eq for PROCESS_NAME_ENTRY {}
impl ::core::fmt::Debug for PROCESS_NAME_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_NAME_ENTRY").field("ProcessId", &self.ProcessId).field("NameOffset", &self.NameOffset).field("NameSize", &self.NameSize).field("NextEntry", &self.NextEntry).finish()
    }
}
impl ::core::default::Default for PROFILER_EVENT_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_EVENT_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_EVENT_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROFILER_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROFILER_HEAP_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_HEAP_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_ENUM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_HEAP_ENUM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_HEAP_ENUM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_HEAP_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_HEAP_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.scopes == other.scopes
    }
}
impl ::core::cmp::Eq for PROFILER_HEAP_OBJECT_SCOPE_LIST {}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILER_HEAP_OBJECT_SCOPE_LIST").field("count", &self.count).field("scopes", &self.scopes).finish()
    }
}
impl ::core::default::Default for PROFILER_HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROFILER_HEAP_SUMMARY {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.totalHeapSize == other.totalHeapSize
    }
}
impl ::core::cmp::Eq for PROFILER_HEAP_SUMMARY {}
impl ::core::fmt::Debug for PROFILER_HEAP_SUMMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILER_HEAP_SUMMARY").field("version", &self.version).field("totalHeapSize", &self.totalHeapSize).finish()
    }
}
impl ::core::default::Default for PROFILER_HEAP_SUMMARY_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_HEAP_SUMMARY_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_SUMMARY_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
impl ::core::cmp::Eq for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {}
impl ::core::fmt::Debug for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILER_PROPERTY_TYPE_SUBSTRING_INFO").field("length", &self.length).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for PROFILER_RELATIONSHIP_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_RELATIONSHIP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_RELATIONSHIP_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROFILER_SCRIPT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROFILER_SCRIPT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_SCRIPT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROP_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROP_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROP_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PointerKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PointerKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for PreferredFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PreferredFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreferredFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for READCONTROLSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READCONTROLSPACE {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.Address == other.Address && self.BufLen == other.BufLen && self.Buf == other.Buf
    }
}
impl ::core::cmp::Eq for READCONTROLSPACE {}
impl ::core::fmt::Debug for READCONTROLSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READCONTROLSPACE").field("Processor", &self.Processor).field("Address", &self.Address).field("BufLen", &self.BufLen).field("Buf", &self.Buf).finish()
    }
}
impl ::core::default::Default for READCONTROLSPACE32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READCONTROLSPACE32 {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.Address == other.Address && self.BufLen == other.BufLen && self.Buf == other.Buf
    }
}
impl ::core::cmp::Eq for READCONTROLSPACE32 {}
impl ::core::fmt::Debug for READCONTROLSPACE32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READCONTROLSPACE32").field("Processor", &self.Processor).field("Address", &self.Address).field("BufLen", &self.BufLen).field("Buf", &self.Buf).finish()
    }
}
impl ::core::default::Default for READCONTROLSPACE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READCONTROLSPACE64 {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.Address == other.Address && self.BufLen == other.BufLen && self.Buf == other.Buf
    }
}
impl ::core::cmp::Eq for READCONTROLSPACE64 {}
impl ::core::fmt::Debug for READCONTROLSPACE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READCONTROLSPACE64").field("Processor", &self.Processor).field("Address", &self.Address).field("BufLen", &self.BufLen).field("Buf", &self.Buf).finish()
    }
}
impl ::core::default::Default for READ_WRITE_MSR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READ_WRITE_MSR {
    fn eq(&self, other: &Self) -> bool {
        self.Msr == other.Msr && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for READ_WRITE_MSR {}
impl ::core::fmt::Debug for READ_WRITE_MSR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_WRITE_MSR").field("Msr", &self.Msr).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for RIP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RIP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.dwType == other.dwType
    }
}
impl ::core::cmp::Eq for RIP_INFO {}
impl ::core::fmt::Debug for RIP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIP_INFO").field("dwError", &self.dwError).field("dwType", &self.dwType).finish()
    }
}
impl ::core::default::Default for RIP_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RIP_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RIP_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTL_VIRTUAL_UNWIND_HANDLER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTL_VIRTUAL_UNWIND_HANDLER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_VIRTUAL_UNWIND_HANDLER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RawSearchFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RawSearchFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RawSearchFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTGCTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTGCTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTGCTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTLANGUAGEVERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTLANGUAGEVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTLANGUAGEVERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTTHREADSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTTHREADSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTTHREADSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTTRACEINFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTTRACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTTRACEINFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTUICHANDLING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTUICHANDLING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTUICHANDLING").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTUICITEM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTUICITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTUICITEM").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPT_DEBUGGER_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPT_DEBUGGER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_DEBUGGER_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPT_INVOCATION_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPT_INVOCATION_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_INVOCATION_CONTEXT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEARCHMEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEARCHMEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.SearchAddress == other.SearchAddress && self.SearchLength == other.SearchLength && self.FoundAddress == other.FoundAddress && self.PatternLength == other.PatternLength && self.Pattern == other.Pattern
    }
}
impl ::core::cmp::Eq for SEARCHMEMORY {}
impl ::core::fmt::Debug for SEARCHMEMORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEARCHMEMORY").field("SearchAddress", &self.SearchAddress).field("SearchLength", &self.SearchLength).field("FoundAddress", &self.FoundAddress).field("PatternLength", &self.PatternLength).field("Pattern", &self.Pattern).finish()
    }
}
impl ::core::default::Default for SOURCEFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOURCEFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ModBase == other.ModBase && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for SOURCEFILE {}
impl ::core::fmt::Debug for SOURCEFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOURCEFILE").field("ModBase", &self.ModBase).field("FileName", &self.FileName).finish()
    }
}
impl ::core::default::Default for SOURCEFILEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOURCEFILEW {
    fn eq(&self, other: &Self) -> bool {
        self.ModBase == other.ModBase && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for SOURCEFILEW {}
impl ::core::fmt::Debug for SOURCEFILEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOURCEFILEW").field("ModBase", &self.ModBase).field("FileName", &self.FileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SRCCODEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SRCCODEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.ModBase == other.ModBase && self.Obj == other.Obj && self.FileName == other.FileName && self.LineNumber == other.LineNumber && self.Address == other.Address
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SRCCODEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SRCCODEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRCCODEINFO").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("ModBase", &self.ModBase).field("Obj", &self.Obj).field("FileName", &self.FileName).field("LineNumber", &self.LineNumber).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for SRCCODEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SRCCODEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.ModBase == other.ModBase && self.Obj == other.Obj && self.FileName == other.FileName && self.LineNumber == other.LineNumber && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for SRCCODEINFOW {}
impl ::core::fmt::Debug for SRCCODEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRCCODEINFOW").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("ModBase", &self.ModBase).field("Obj", &self.Obj).field("FileName", &self.FileName).field("LineNumber", &self.LineNumber).field("Address", &self.Address).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STACKFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STACKFRAME {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPC == other.AddrPC && self.AddrReturn == other.AddrReturn && self.AddrFrame == other.AddrFrame && self.AddrStack == other.AddrStack && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Far == other.Far && self.Virtual == other.Virtual && self.Reserved == other.Reserved && self.KdHelp == other.KdHelp && self.AddrBStore == other.AddrBStore
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STACKFRAME {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STACKFRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACKFRAME").field("AddrPC", &self.AddrPC).field("AddrReturn", &self.AddrReturn).field("AddrFrame", &self.AddrFrame).field("AddrStack", &self.AddrStack).field("FuncTableEntry", &self.FuncTableEntry).field("Params", &self.Params).field("Far", &self.Far).field("Virtual", &self.Virtual).field("Reserved", &self.Reserved).field("KdHelp", &self.KdHelp).field("AddrBStore", &self.AddrBStore).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STACKFRAME64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STACKFRAME64 {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPC == other.AddrPC && self.AddrReturn == other.AddrReturn && self.AddrFrame == other.AddrFrame && self.AddrStack == other.AddrStack && self.AddrBStore == other.AddrBStore && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Far == other.Far && self.Virtual == other.Virtual && self.Reserved == other.Reserved && self.KdHelp == other.KdHelp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STACKFRAME64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STACKFRAME64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACKFRAME64").field("AddrPC", &self.AddrPC).field("AddrReturn", &self.AddrReturn).field("AddrFrame", &self.AddrFrame).field("AddrStack", &self.AddrStack).field("AddrBStore", &self.AddrBStore).field("FuncTableEntry", &self.FuncTableEntry).field("Params", &self.Params).field("Far", &self.Far).field("Virtual", &self.Virtual).field("Reserved", &self.Reserved).field("KdHelp", &self.KdHelp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STACKFRAME_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STACKFRAME_EX {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPC == other.AddrPC && self.AddrReturn == other.AddrReturn && self.AddrFrame == other.AddrFrame && self.AddrStack == other.AddrStack && self.AddrBStore == other.AddrBStore && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Far == other.Far && self.Virtual == other.Virtual && self.Reserved == other.Reserved && self.KdHelp == other.KdHelp && self.StackFrameSize == other.StackFrameSize && self.InlineFrameContext == other.InlineFrameContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STACKFRAME_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STACKFRAME_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACKFRAME_EX")
            .field("AddrPC", &self.AddrPC)
            .field("AddrReturn", &self.AddrReturn)
            .field("AddrFrame", &self.AddrFrame)
            .field("AddrStack", &self.AddrStack)
            .field("AddrBStore", &self.AddrBStore)
            .field("FuncTableEntry", &self.FuncTableEntry)
            .field("Params", &self.Params)
            .field("Far", &self.Far)
            .field("Virtual", &self.Virtual)
            .field("Reserved", &self.Reserved)
            .field("KdHelp", &self.KdHelp)
            .field("StackFrameSize", &self.StackFrameSize)
            .field("InlineFrameContext", &self.InlineFrameContext)
            .finish()
    }
}
impl ::core::default::Default for STACK_SRC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STACK_SRC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ImagePath == other.ImagePath && self.ModuleName == other.ModuleName && self.Function == other.Function && self.Displacement == other.Displacement && self.Row == other.Row && self.Column == other.Column
    }
}
impl ::core::cmp::Eq for STACK_SRC_INFO {}
impl ::core::fmt::Debug for STACK_SRC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACK_SRC_INFO").field("ImagePath", &self.ImagePath).field("ModuleName", &self.ModuleName).field("Function", &self.Function).field("Displacement", &self.Displacement).field("Row", &self.Row).field("Column", &self.Column).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STACK_SYM_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STACK_SYM_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StackFrameEx == other.StackFrameEx && self.SrcInfo == other.SrcInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STACK_SYM_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STACK_SYM_FRAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACK_SYM_FRAME_INFO").field("StackFrameEx", &self.StackFrameEx).field("SrcInfo", &self.SrcInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYMBOL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYMBOL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TypeIndex == other.TypeIndex && self.Reserved == other.Reserved && self.Index == other.Index && self.Size == other.Size && self.ModBase == other.ModBase && self.Flags == other.Flags && self.Value == other.Value && self.Address == other.Address && self.Register == other.Register && self.Scope == other.Scope && self.Tag == other.Tag && self.NameLen == other.NameLen && self.MaxNameLen == other.MaxNameLen && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYMBOL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYMBOL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFO")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("TypeIndex", &self.TypeIndex)
            .field("Reserved", &self.Reserved)
            .field("Index", &self.Index)
            .field("Size", &self.Size)
            .field("ModBase", &self.ModBase)
            .field("Flags", &self.Flags)
            .field("Value", &self.Value)
            .field("Address", &self.Address)
            .field("Register", &self.Register)
            .field("Scope", &self.Scope)
            .field("Tag", &self.Tag)
            .field("NameLen", &self.NameLen)
            .field("MaxNameLen", &self.MaxNameLen)
            .field("Name", &self.Name)
            .finish()
    }
}
impl ::core::default::Default for SYMBOL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYMBOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TypeIndex == other.TypeIndex && self.Reserved == other.Reserved && self.Index == other.Index && self.Size == other.Size && self.ModBase == other.ModBase && self.Flags == other.Flags && self.Value == other.Value && self.Address == other.Address && self.Register == other.Register && self.Scope == other.Scope && self.Tag == other.Tag && self.NameLen == other.NameLen && self.MaxNameLen == other.MaxNameLen && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for SYMBOL_INFOW {}
impl ::core::fmt::Debug for SYMBOL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("TypeIndex", &self.TypeIndex)
            .field("Reserved", &self.Reserved)
            .field("Index", &self.Index)
            .field("Size", &self.Size)
            .field("ModBase", &self.ModBase)
            .field("Flags", &self.Flags)
            .field("Value", &self.Value)
            .field("Address", &self.Address)
            .field("Register", &self.Register)
            .field("Scope", &self.Scope)
            .field("Tag", &self.Tag)
            .field("NameLen", &self.NameLen)
            .field("MaxNameLen", &self.MaxNameLen)
            .field("Name", &self.Name)
            .finish()
    }
}
impl ::core::default::Default for SYMBOL_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYMBOL_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TypeOfInfo == other.TypeOfInfo && self.Offset == other.Offset && self.Line == other.Line && self.Displacement == other.Displacement && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SYMBOL_INFO_EX {}
impl ::core::fmt::Debug for SYMBOL_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFO_EX").field("SizeOfStruct", &self.SizeOfStruct).field("TypeOfInfo", &self.TypeOfInfo).field("Offset", &self.Offset).field("Line", &self.Line).field("Displacement", &self.Displacement).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for SYMBOL_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYMBOL_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYMBOL_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYMBOL_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYMBOL_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYMBOL_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYMBOL_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYMBOL_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYMBOL_INFO_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYMBOL_INFO_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.si == other.si && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYMBOL_INFO_PACKAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYMBOL_INFO_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFO_PACKAGE").field("si", &self.si).field("name", &self.name).finish()
    }
}
impl ::core::default::Default for SYMBOL_INFO_PACKAGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYMBOL_INFO_PACKAGEW {
    fn eq(&self, other: &Self) -> bool {
        self.si == other.si && self.name == other.name
    }
}
impl ::core::cmp::Eq for SYMBOL_INFO_PACKAGEW {}
impl ::core::fmt::Debug for SYMBOL_INFO_PACKAGEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFO_PACKAGEW").field("si", &self.si).field("name", &self.name).finish()
    }
}
impl ::core::default::Default for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.sizeOfStruct == other.sizeOfStruct && self.version == other.version && self.filePtrMsg == other.filePtrMsg
    }
}
impl ::core::cmp::Eq for SYMSRV_EXTENDED_OUTPUT_DATA {}
impl ::core::fmt::Debug for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMSRV_EXTENDED_OUTPUT_DATA").field("sizeOfStruct", &self.sizeOfStruct).field("version", &self.version).field("filePtrMsg", &self.filePtrMsg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYMSRV_INDEX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYMSRV_INDEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.sizeofstruct == other.sizeofstruct && self.file == other.file && self.stripped == other.stripped && self.timestamp == other.timestamp && self.size == other.size && self.dbgfile == other.dbgfile && self.pdbfile == other.pdbfile && self.guid == other.guid && self.sig == other.sig && self.age == other.age
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYMSRV_INDEX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYMSRV_INDEX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMSRV_INDEX_INFO").field("sizeofstruct", &self.sizeofstruct).field("file", &self.file).field("stripped", &self.stripped).field("timestamp", &self.timestamp).field("size", &self.size).field("dbgfile", &self.dbgfile).field("pdbfile", &self.pdbfile).field("guid", &self.guid).field("sig", &self.sig).field("age", &self.age).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYMSRV_INDEX_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYMSRV_INDEX_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.sizeofstruct == other.sizeofstruct && self.file == other.file && self.stripped == other.stripped && self.timestamp == other.timestamp && self.size == other.size && self.dbgfile == other.dbgfile && self.pdbfile == other.pdbfile && self.guid == other.guid && self.sig == other.sig && self.age == other.age
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYMSRV_INDEX_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYMSRV_INDEX_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMSRV_INDEX_INFOW").field("sizeofstruct", &self.sizeofstruct).field("file", &self.file).field("stripped", &self.stripped).field("timestamp", &self.timestamp).field("size", &self.size).field("dbgfile", &self.dbgfile).field("pdbfile", &self.pdbfile).field("guid", &self.guid).field("sig", &self.sig).field("age", &self.age).finish()
    }
}
impl ::core::default::Default for SYM_DUMP_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SYM_FIND_ID_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYM_FIND_ID_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_FIND_ID_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYM_LOAD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYM_LOAD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_LOAD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYM_LOAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYM_LOAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYM_LOAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYM_LOAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYM_LOAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SYM_SRV_STORE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYM_SRV_STORE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_SRV_STORE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ScriptChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScriptChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScriptChangeKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for ScriptDebugEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScriptDebugEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScriptDebugEvent").field(&self.0).finish()
    }
}
impl ::core::default::Default for ScriptDebugEventFilter {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScriptDebugEventFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScriptDebugEventFilter").field(&self.0).finish()
    }
}
impl ::core::default::Default for ScriptDebugEventInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ScriptDebugPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ScriptDebugPosition {
    fn eq(&self, other: &Self) -> bool {
        self.Line == other.Line && self.Column == other.Column
    }
}
impl ::core::cmp::Eq for ScriptDebugPosition {}
impl ::core::fmt::Debug for ScriptDebugPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ScriptDebugPosition").field("Line", &self.Line).field("Column", &self.Column).finish()
    }
}
impl ::core::default::Default for ScriptDebugState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScriptDebugState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScriptDebugState").field(&self.0).finish()
    }
}
impl ::core::default::Default for ScriptExecutionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScriptExecutionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScriptExecutionKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for SignatureComparison {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SignatureComparison {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignatureComparison").field(&self.0).finish()
    }
}
impl ::core::default::Default for SymbolKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SymbolKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SymbolKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for SymbolSearchOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SymbolSearchOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SymbolSearchOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXT_DOCUMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TEXT_DOCUMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.Members == other.Members
    }
}
impl ::core::cmp::Eq for TEXT_DOCUMENT_ARRAY {}
impl ::core::fmt::Debug for TEXT_DOCUMENT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXT_DOCUMENT_ARRAY").field("dwCount", &self.dwCount).field("Members", &self.Members).finish()
    }
}
impl ::core::default::Default for THREAD_ERROR_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_ERROR_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_ERROR_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for THREAD_ERROR_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_ERROR_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_ERROR_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_ERROR_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_ERROR_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for THREAD_WRITE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_WRITE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_WRITE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TI_FINDCHILDREN_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TI_FINDCHILDREN_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Start == other.Start && self.ChildId == other.ChildId
    }
}
impl ::core::cmp::Eq for TI_FINDCHILDREN_PARAMS {}
impl ::core::fmt::Debug for TI_FINDCHILDREN_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TI_FINDCHILDREN_PARAMS").field("Count", &self.Count).field("Start", &self.Start).field("ChildId", &self.ChildId).finish()
    }
}
impl ::core::default::Default for TRANSLATE_VIRTUAL_TO_PHYSICAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSLATE_VIRTUAL_TO_PHYSICAL {
    fn eq(&self, other: &Self) -> bool {
        self.Virtual == other.Virtual && self.Physical == other.Physical
    }
}
impl ::core::cmp::Eq for TRANSLATE_VIRTUAL_TO_PHYSICAL {}
impl ::core::fmt::Debug for TRANSLATE_VIRTUAL_TO_PHYSICAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSLATE_VIRTUAL_TO_PHYSICAL").field("Virtual", &self.Virtual).field("Physical", &self.Physical).finish()
    }
}
impl ::core::default::Default for TypeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TypeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TypeKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNLOAD_DLL_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNLOAD_DLL_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpBaseOfDll == other.lpBaseOfDll
    }
}
impl ::core::cmp::Eq for UNLOAD_DLL_DEBUG_INFO {}
impl ::core::fmt::Debug for UNLOAD_DLL_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNLOAD_DLL_DEBUG_INFO").field("lpBaseOfDll", &self.lpBaseOfDll).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for UNWIND_HISTORY_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for UNWIND_HISTORY_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.LocalHint == other.LocalHint && self.GlobalHint == other.GlobalHint && self.Search == other.Search && self.Once == other.Once && self.LowAddress == other.LowAddress && self.HighAddress == other.HighAddress && self.Entry == other.Entry
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for UNWIND_HISTORY_TABLE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for UNWIND_HISTORY_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNWIND_HISTORY_TABLE").field("Count", &self.Count).field("LocalHint", &self.LocalHint).field("GlobalHint", &self.GlobalHint).field("Search", &self.Search).field("Once", &self.Once).field("LowAddress", &self.LowAddress).field("HighAddress", &self.HighAddress).field("Entry", &self.Entry).finish()
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for UNWIND_HISTORY_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::PartialEq for UNWIND_HISTORY_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ImageBase == other.ImageBase && self.FunctionEntry == other.FunctionEntry
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::Eq for UNWIND_HISTORY_TABLE_ENTRY {}
#[cfg(target_arch = "aarch64")]
impl ::core::fmt::Debug for UNWIND_HISTORY_TABLE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNWIND_HISTORY_TABLE_ENTRY").field("ImageBase", &self.ImageBase).field("FunctionEntry", &self.FunctionEntry).finish()
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for UNWIND_HISTORY_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::PartialEq for UNWIND_HISTORY_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ImageBase == other.ImageBase && self.FunctionEntry == other.FunctionEntry
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::Eq for UNWIND_HISTORY_TABLE_ENTRY {}
#[cfg(target_arch = "x86_64")]
impl ::core::fmt::Debug for UNWIND_HISTORY_TABLE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNWIND_HISTORY_TABLE_ENTRY").field("ImageBase", &self.ImageBase).field("FunctionEntry", &self.FunctionEntry).finish()
    }
}
impl ::core::default::Default for VER_PLATFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_PLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_PLATFORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for VIRTUAL_TO_PHYSICAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIRTUAL_TO_PHYSICAL {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.Size == other.Size && self.PdeAddress == other.PdeAddress && self.Virtual == other.Virtual && self.Physical == other.Physical
    }
}
impl ::core::cmp::Eq for VIRTUAL_TO_PHYSICAL {}
impl ::core::fmt::Debug for VIRTUAL_TO_PHYSICAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUAL_TO_PHYSICAL").field("Status", &self.Status).field("Size", &self.Size).field("PdeAddress", &self.PdeAddress).field("Virtual", &self.Virtual).field("Physical", &self.Physical).finish()
    }
}
impl ::core::default::Default for VarArgsKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VarArgsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VarArgsKind").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAITCHAIN_NODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAIT_CHAIN_THREAD_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WAIT_CHAIN_THREAD_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WAIT_CHAIN_THREAD_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCT_OBJECT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCT_OBJECT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCT_OBJECT_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCT_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCT_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WDBGEXTS_CLR_DATA_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WDBGEXTS_CLR_DATA_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Iid == other.Iid && self.Iface == other.Iface
    }
}
impl ::core::cmp::Eq for WDBGEXTS_CLR_DATA_INTERFACE {}
impl ::core::fmt::Debug for WDBGEXTS_CLR_DATA_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDBGEXTS_CLR_DATA_INTERFACE").field("Iid", &self.Iid).field("Iface", &self.Iface).finish()
    }
}
impl ::core::default::Default for WDBGEXTS_DISASSEMBLE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WDBGEXTS_DISASSEMBLE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.InOffset == other.InOffset && self.OutOffset == other.OutOffset && self.AddrFlags == other.AddrFlags && self.FormatFlags == other.FormatFlags && self.DataBufferBytes == other.DataBufferBytes && self.DisasmBufferChars == other.DisasmBufferChars && self.DataBuffer == other.DataBuffer && self.DisasmBuffer == other.DisasmBuffer && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for WDBGEXTS_DISASSEMBLE_BUFFER {}
impl ::core::fmt::Debug for WDBGEXTS_DISASSEMBLE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDBGEXTS_DISASSEMBLE_BUFFER").field("InOffset", &self.InOffset).field("OutOffset", &self.OutOffset).field("AddrFlags", &self.AddrFlags).field("FormatFlags", &self.FormatFlags).field("DataBufferBytes", &self.DataBufferBytes).field("DisasmBufferChars", &self.DisasmBufferChars).field("DataBuffer", &self.DataBuffer).field("DisasmBuffer", &self.DisasmBuffer).field("Reserved0", &self.Reserved0).finish()
    }
}
impl ::core::default::Default for WDBGEXTS_MODULE_IN_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WDBGEXTS_MODULE_IN_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Start == other.Start && self.End == other.End && self.FoundModBase == other.FoundModBase && self.FoundModSize == other.FoundModSize
    }
}
impl ::core::cmp::Eq for WDBGEXTS_MODULE_IN_RANGE {}
impl ::core::fmt::Debug for WDBGEXTS_MODULE_IN_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDBGEXTS_MODULE_IN_RANGE").field("Start", &self.Start).field("End", &self.End).field("FoundModBase", &self.FoundModBase).field("FoundModSize", &self.FoundModSize).finish()
    }
}
impl ::core::default::Default for WDBGEXTS_QUERY_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WDBGEXTS_QUERY_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Iid == other.Iid && self.Iface == other.Iface
    }
}
impl ::core::cmp::Eq for WDBGEXTS_QUERY_INTERFACE {}
impl ::core::fmt::Debug for WDBGEXTS_QUERY_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDBGEXTS_QUERY_INTERFACE").field("Iid", &self.Iid).field("Iface", &self.Iface).finish()
    }
}
impl ::core::default::Default for WDBGEXTS_THREAD_OS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WDBGEXTS_THREAD_OS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadId == other.ThreadId && self.ExitStatus == other.ExitStatus && self.PriorityClass == other.PriorityClass && self.Priority == other.Priority && self.CreateTime == other.CreateTime && self.ExitTime == other.ExitTime && self.KernelTime == other.KernelTime && self.UserTime == other.UserTime && self.StartOffset == other.StartOffset && self.Affinity == other.Affinity
    }
}
impl ::core::cmp::Eq for WDBGEXTS_THREAD_OS_INFO {}
impl ::core::fmt::Debug for WDBGEXTS_THREAD_OS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDBGEXTS_THREAD_OS_INFO").field("ThreadId", &self.ThreadId).field("ExitStatus", &self.ExitStatus).field("PriorityClass", &self.PriorityClass).field("Priority", &self.Priority).field("CreateTime", &self.CreateTime).field("ExitTime", &self.ExitTime).field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).field("StartOffset", &self.StartOffset).field("Affinity", &self.Affinity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_AER_BRIDGE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_AER_ENDPOINT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_AER_ROOTPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_DEVICE_DRIVER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_DRIVER_BUFFER_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_ERROR_SOURCE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHEA_ERROR_SOURCE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHEA_ERROR_SOURCE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHEA_ERROR_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHEA_ERROR_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHEA_ERROR_SOURCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHEA_GENERIC_ERROR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_IPF_CMC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_IPF_CPE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_IPF_MCA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_NOTIFICATION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHEA_PCI_SLOT_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_CMC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_MCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_MC_BANK_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_NMI_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for WINDBG_EXTENSION_APIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for WINDBG_EXTENSION_APIS32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for WINDBG_EXTENSION_APIS64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINDBG_OLDKD_EXTENSION_APIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINDBG_OLD_EXTENSION_APIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WOW64_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WOW64_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.Dr0 == other.Dr0 && self.Dr1 == other.Dr1 && self.Dr2 == other.Dr2 && self.Dr3 == other.Dr3 && self.Dr6 == other.Dr6 && self.Dr7 == other.Dr7 && self.FloatSave == other.FloatSave && self.SegGs == other.SegGs && self.SegFs == other.SegFs && self.SegEs == other.SegEs && self.SegDs == other.SegDs && self.Edi == other.Edi && self.Esi == other.Esi && self.Ebx == other.Ebx && self.Edx == other.Edx && self.Ecx == other.Ecx && self.Eax == other.Eax && self.Ebp == other.Ebp && self.Eip == other.Eip && self.SegCs == other.SegCs && self.EFlags == other.EFlags && self.Esp == other.Esp && self.SegSs == other.SegSs && self.ExtendedRegisters == other.ExtendedRegisters
    }
}
impl ::core::cmp::Eq for WOW64_CONTEXT {}
impl ::core::fmt::Debug for WOW64_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOW64_CONTEXT")
            .field("ContextFlags", &self.ContextFlags)
            .field("Dr0", &self.Dr0)
            .field("Dr1", &self.Dr1)
            .field("Dr2", &self.Dr2)
            .field("Dr3", &self.Dr3)
            .field("Dr6", &self.Dr6)
            .field("Dr7", &self.Dr7)
            .field("FloatSave", &self.FloatSave)
            .field("SegGs", &self.SegGs)
            .field("SegFs", &self.SegFs)
            .field("SegEs", &self.SegEs)
            .field("SegDs", &self.SegDs)
            .field("Edi", &self.Edi)
            .field("Esi", &self.Esi)
            .field("Ebx", &self.Ebx)
            .field("Edx", &self.Edx)
            .field("Ecx", &self.Ecx)
            .field("Eax", &self.Eax)
            .field("Ebp", &self.Ebp)
            .field("Eip", &self.Eip)
            .field("SegCs", &self.SegCs)
            .field("EFlags", &self.EFlags)
            .field("Esp", &self.Esp)
            .field("SegSs", &self.SegSs)
            .field("ExtendedRegisters", &self.ExtendedRegisters)
            .finish()
    }
}
impl ::core::default::Default for WOW64_DESCRIPTOR_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WOW64_FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WOW64_FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.RegisterArea == other.RegisterArea && self.Cr0NpxState == other.Cr0NpxState
    }
}
impl ::core::cmp::Eq for WOW64_FLOATING_SAVE_AREA {}
impl ::core::fmt::Debug for WOW64_FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOW64_FLOATING_SAVE_AREA").field("ControlWord", &self.ControlWord).field("StatusWord", &self.StatusWord).field("TagWord", &self.TagWord).field("ErrorOffset", &self.ErrorOffset).field("ErrorSelector", &self.ErrorSelector).field("DataOffset", &self.DataOffset).field("DataSelector", &self.DataSelector).field("RegisterArea", &self.RegisterArea).field("Cr0NpxState", &self.Cr0NpxState).finish()
    }
}
impl ::core::default::Default for WOW64_LDT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XPF_MCE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XPF_MC_BANK_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XSAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XSAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.LegacyState == other.LegacyState && self.Header == other.Header
    }
}
impl ::core::cmp::Eq for XSAVE_AREA {}
impl ::core::fmt::Debug for XSAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_AREA").field("LegacyState", &self.LegacyState).field("Header", &self.Header).finish()
    }
}
impl ::core::default::Default for XSAVE_AREA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XSAVE_AREA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.CompactionMask == other.CompactionMask && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for XSAVE_AREA_HEADER {}
impl ::core::fmt::Debug for XSAVE_AREA_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_AREA_HEADER").field("Mask", &self.Mask).field("CompactionMask", &self.CompactionMask).field("Reserved2", &self.Reserved2).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for XSAVE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.Reserved1 == other.Reserved1 && self.ErrorOpcode == other.ErrorOpcode && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.Reserved2 == other.Reserved2 && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.Reserved3 == other.Reserved3 && self.MxCsr == other.MxCsr && self.MxCsr_Mask == other.MxCsr_Mask && self.FloatRegisters == other.FloatRegisters && self.XmmRegisters == other.XmmRegisters && self.Reserved4 == other.Reserved4
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for XSAVE_FORMAT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for XSAVE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_FORMAT")
            .field("ControlWord", &self.ControlWord)
            .field("StatusWord", &self.StatusWord)
            .field("TagWord", &self.TagWord)
            .field("Reserved1", &self.Reserved1)
            .field("ErrorOpcode", &self.ErrorOpcode)
            .field("ErrorOffset", &self.ErrorOffset)
            .field("ErrorSelector", &self.ErrorSelector)
            .field("Reserved2", &self.Reserved2)
            .field("DataOffset", &self.DataOffset)
            .field("DataSelector", &self.DataSelector)
            .field("Reserved3", &self.Reserved3)
            .field("MxCsr", &self.MxCsr)
            .field("MxCsr_Mask", &self.MxCsr_Mask)
            .field("FloatRegisters", &self.FloatRegisters)
            .field("XmmRegisters", &self.XmmRegisters)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for XSAVE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.Reserved1 == other.Reserved1 && self.ErrorOpcode == other.ErrorOpcode && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.Reserved2 == other.Reserved2 && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.Reserved3 == other.Reserved3 && self.MxCsr == other.MxCsr && self.MxCsr_Mask == other.MxCsr_Mask && self.FloatRegisters == other.FloatRegisters && self.XmmRegisters == other.XmmRegisters && self.Reserved4 == other.Reserved4
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for XSAVE_FORMAT {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for XSAVE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_FORMAT")
            .field("ControlWord", &self.ControlWord)
            .field("StatusWord", &self.StatusWord)
            .field("TagWord", &self.TagWord)
            .field("Reserved1", &self.Reserved1)
            .field("ErrorOpcode", &self.ErrorOpcode)
            .field("ErrorOffset", &self.ErrorOffset)
            .field("ErrorSelector", &self.ErrorSelector)
            .field("Reserved2", &self.Reserved2)
            .field("DataOffset", &self.DataOffset)
            .field("DataSelector", &self.DataSelector)
            .field("Reserved3", &self.Reserved3)
            .field("MxCsr", &self.MxCsr)
            .field("MxCsr_Mask", &self.MxCsr_Mask)
            .field("FloatRegisters", &self.FloatRegisters)
            .field("XmmRegisters", &self.XmmRegisters)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
impl ::core::default::Default for XSTATE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XSTATE_CONFIG_FEATURE_MSC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for XSTATE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for XSTATE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Length == other.Length && self.Reserved1 == other.Reserved1 && self.Area == other.Area && self.Buffer == other.Buffer
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for XSTATE_CONTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for XSTATE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSTATE_CONTEXT").field("Mask", &self.Mask).field("Length", &self.Length).field("Reserved1", &self.Reserved1).field("Area", &self.Area).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for XSTATE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for XSTATE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Length == other.Length && self.Reserved1 == other.Reserved1 && self.Area == other.Area && self.Reserved2 == other.Reserved2 && self.Buffer == other.Buffer && self.Reserved3 == other.Reserved3
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for XSTATE_CONTEXT {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for XSTATE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSTATE_CONTEXT").field("Mask", &self.Mask).field("Length", &self.Length).field("Reserved1", &self.Reserved1).field("Area", &self.Area).field("Reserved2", &self.Reserved2).field("Buffer", &self.Buffer).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::core::default::Default for XSTATE_FEATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XSTATE_FEATURE {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for XSTATE_FEATURE {}
impl ::core::fmt::Debug for XSTATE_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSTATE_FEATURE").field("Offset", &self.Offset).field("Size", &self.Size).finish()
    }
}
