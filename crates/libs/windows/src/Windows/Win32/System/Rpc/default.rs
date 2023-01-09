impl ::core::default::Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ARRAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Dimension == other.Dimension && self.BufferConformanceMark == other.BufferConformanceMark && self.BufferVarianceMark == other.BufferVarianceMark && self.MaxCountArray == other.MaxCountArray && self.OffsetArray == other.OffsetArray && self.ActualCountArray == other.ActualCountArray
    }
}
impl ::core::cmp::Eq for ARRAY_INFO {}
impl ::core::fmt::Debug for ARRAY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARRAY_INFO").field("Dimension", &self.Dimension).field("BufferConformanceMark", &self.BufferConformanceMark).field("BufferVarianceMark", &self.BufferVarianceMark).field("MaxCountArray", &self.MaxCountArray).field("OffsetArray", &self.OffsetArray).field("ActualCountArray", &self.ActualCountArray).finish()
    }
}
impl ::core::default::Default for BinaryParam {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BinaryParam {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for BinaryParam {}
impl ::core::fmt::Debug for BinaryParam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BinaryParam").field("Buffer", &self.Buffer).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COMM_FAULT_OFFSETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COMM_FAULT_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        self.CommOffset == other.CommOffset && self.FaultOffset == other.FaultOffset
    }
}
impl ::core::cmp::Eq for COMM_FAULT_OFFSETS {}
impl ::core::fmt::Debug for COMM_FAULT_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMM_FAULT_OFFSETS").field("CommOffset", &self.CommOffset).field("FaultOffset", &self.FaultOffset).finish()
    }
}
impl ::core::default::Default for EXPR_TOKEN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXPR_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPR_TOKEN").field(&self.0).finish()
    }
}
impl ::core::default::Default for ExtendedErrorParamTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ExtendedErrorParamTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedErrorParamTypes").field(&self.0).finish()
    }
}
impl ::core::default::Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FULL_PTR_XLAT_TABLES {
    fn eq(&self, other: &Self) -> bool {
        self.RefIdToPointer == other.RefIdToPointer && self.PointerToRefId == other.PointerToRefId && self.NextRefId == other.NextRefId && self.XlatSide == other.XlatSide
    }
}
impl ::core::cmp::Eq for FULL_PTR_XLAT_TABLES {}
impl ::core::fmt::Debug for FULL_PTR_XLAT_TABLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FULL_PTR_XLAT_TABLES").field("RefIdToPointer", &self.RefIdToPointer).field("PointerToRefId", &self.PointerToRefId).field("NextRefId", &self.NextRefId).field("XlatSide", &self.XlatSide).finish()
    }
}
impl ::core::default::Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GENERIC_BINDING_ROUTINE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GROUP_NAME_SYNTAX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GROUP_NAME_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_NAME_SYNTAX").field(&self.0).finish()
    }
}
impl ::core::default::Default for IDL_CS_CONVERT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IDL_CS_CONVERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDL_CS_CONVERT").field(&self.0).finish()
    }
}
impl ::core::default::Default for I_RpcProxyCallbackInterface {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MALLOC_FREE_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MALLOC_FREE_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pfnAllocate == other.pfnAllocate && self.pfnFree == other.pfnFree
    }
}
impl ::core::cmp::Eq for MALLOC_FREE_STRUCT {}
impl ::core::fmt::Debug for MALLOC_FREE_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MALLOC_FREE_STRUCT").field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).finish()
    }
}
impl ::core::default::Default for MIDL_ES_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIDL_ES_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDL_ES_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIDL_ES_HANDLE_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIDL_ES_HANDLE_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDL_ES_HANDLE_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDL_FORMAT_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Pad == other.Pad && self.Format == other.Format
    }
}
impl ::core::cmp::Eq for MIDL_FORMAT_STRING {}
impl ::core::fmt::Debug for MIDL_FORMAT_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_FORMAT_STRING").field("Pad", &self.Pad).field("Format", &self.Format).finish()
    }
}
impl ::core::default::Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDL_INTERCEPTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProcString == other.ProcString && self.ProcFormatOffsetTable == other.ProcFormatOffsetTable && self.ProcCount == other.ProcCount && self.TypeString == other.TypeString
    }
}
impl ::core::cmp::Eq for MIDL_INTERCEPTION_INFO {}
impl ::core::fmt::Debug for MIDL_INTERCEPTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_INTERCEPTION_INFO").field("Version", &self.Version).field("ProcString", &self.ProcString).field("ProcFormatOffsetTable", &self.ProcFormatOffsetTable).field("ProcCount", &self.ProcCount).field("TypeString", &self.TypeString).finish()
    }
}
impl ::core::default::Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.MethodCount == other.MethodCount && self.MethodProperties == other.MethodProperties
    }
}
impl ::core::cmp::Eq for MIDL_INTERFACE_METHOD_PROPERTIES {}
impl ::core::fmt::Debug for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_INTERFACE_METHOD_PROPERTIES").field("MethodCount", &self.MethodCount).field("MethodProperties", &self.MethodProperties).finish()
    }
}
impl ::core::default::Default for MIDL_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDL_METHOD_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for MIDL_METHOD_PROPERTY {}
impl ::core::fmt::Debug for MIDL_METHOD_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_METHOD_PROPERTY").field("Id", &self.Id).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDL_METHOD_PROPERTY_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Properties == other.Properties
    }
}
impl ::core::cmp::Eq for MIDL_METHOD_PROPERTY_MAP {}
impl ::core::fmt::Debug for MIDL_METHOD_PROPERTY_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_METHOD_PROPERTY_MAP").field("Count", &self.Count).field("Properties", &self.Properties).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_SERVER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pStubDesc == other.pStubDesc && self.DispatchTable == other.DispatchTable && self.ProcString == other.ProcString && self.FmtStringOffset == other.FmtStringOffset && self.ThunkTable == other.ThunkTable && self.pTransferSyntax == other.pTransferSyntax && self.nCount == other.nCount && self.pSyntaxInfo == other.pSyntaxInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_SERVER_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_SERVER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_SERVER_INFO").field("pStubDesc", &self.pStubDesc).field("DispatchTable", &self.DispatchTable).field("ProcString", &self.ProcString).field("FmtStringOffset", &self.FmtStringOffset).field("ThunkTable", &self.ThunkTable).field("pTransferSyntax", &self.pTransferSyntax).field("nCount", &self.nCount).field("pSyntaxInfo", &self.pSyntaxInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUBLESS_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pStubDesc == other.pStubDesc && self.ProcFormatString == other.ProcFormatString && self.FormatStringOffset == other.FormatStringOffset && self.pTransferSyntax == other.pTransferSyntax && self.nCount == other.nCount && self.pSyntaxInfo == other.pSyntaxInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUBLESS_PROXY_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_STUBLESS_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_STUBLESS_PROXY_INFO").field("pStubDesc", &self.pStubDesc).field("ProcFormatString", &self.ProcFormatString).field("FormatStringOffset", &self.FormatStringOffset).field("pTransferSyntax", &self.pTransferSyntax).field("nCount", &self.nCount).field("pSyntaxInfo", &self.pSyntaxInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUB_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.RpcMsg == other.RpcMsg
            && self.Buffer == other.Buffer
            && self.BufferStart == other.BufferStart
            && self.BufferEnd == other.BufferEnd
            && self.BufferMark == other.BufferMark
            && self.BufferLength == other.BufferLength
            && self.MemorySize == other.MemorySize
            && self.Memory == other.Memory
            && self.IsClient == other.IsClient
            && self.Pad == other.Pad
            && self.uFlags2 == other.uFlags2
            && self.ReuseBuffer == other.ReuseBuffer
            && self.pAllocAllNodesContext == other.pAllocAllNodesContext
            && self.pPointerQueueState == other.pPointerQueueState
            && self.IgnoreEmbeddedPointers == other.IgnoreEmbeddedPointers
            && self.PointerBufferMark == other.PointerBufferMark
            && self.CorrDespIncrement == other.CorrDespIncrement
            && self.uFlags == other.uFlags
            && self.UniquePtrCount == other.UniquePtrCount
            && self.MaxCount == other.MaxCount
            && self.Offset == other.Offset
            && self.ActualCount == other.ActualCount
            && self.pfnAllocate == other.pfnAllocate
            && self.pfnFree == other.pfnFree
            && self.StackTop == other.StackTop
            && self.pPresentedType == other.pPresentedType
            && self.pTransmitType == other.pTransmitType
            && self.SavedHandle == other.SavedHandle
            && self.StubDesc == other.StubDesc
            && self.FullPtrXlatTables == other.FullPtrXlatTables
            && self.FullPtrRefId == other.FullPtrRefId
            && self.PointerLength == other.PointerLength
            && self._bitfield == other._bitfield
            && self.dwDestContext == other.dwDestContext
            && self.pvDestContext == other.pvDestContext
            && self.SavedContextHandles == other.SavedContextHandles
            && self.ParamNumber == other.ParamNumber
            && self.pRpcChannelBuffer == other.pRpcChannelBuffer
            && self.pArrayInfo == other.pArrayInfo
            && self.SizePtrCountArray == other.SizePtrCountArray
            && self.SizePtrOffsetArray == other.SizePtrOffsetArray
            && self.SizePtrLengthArray == other.SizePtrLengthArray
            && self.pArgQueue == other.pArgQueue
            && self.dwStubPhase == other.dwStubPhase
            && self.LowStackMark == other.LowStackMark
            && self.pAsyncMsg == other.pAsyncMsg
            && self.pCorrInfo == other.pCorrInfo
            && self.pCorrMemory == other.pCorrMemory
            && self.pMemoryList == other.pMemoryList
            && self.pCSInfo == other.pCSInfo
            && self.ConformanceMark == other.ConformanceMark
            && self.VarianceMark == other.VarianceMark
            && self.Unused == other.Unused
            && self.pContext == other.pContext
            && self.ContextHandleHash == other.ContextHandleHash
            && self.pUserMarshalList == other.pUserMarshalList
            && self.Reserved51_3 == other.Reserved51_3
            && self.Reserved51_4 == other.Reserved51_4
            && self.Reserved51_5 == other.Reserved51_5
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUB_MESSAGE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_STUB_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_STUB_MESSAGE")
            .field("RpcMsg", &self.RpcMsg)
            .field("Buffer", &self.Buffer)
            .field("BufferStart", &self.BufferStart)
            .field("BufferEnd", &self.BufferEnd)
            .field("BufferMark", &self.BufferMark)
            .field("BufferLength", &self.BufferLength)
            .field("MemorySize", &self.MemorySize)
            .field("Memory", &self.Memory)
            .field("IsClient", &self.IsClient)
            .field("Pad", &self.Pad)
            .field("uFlags2", &self.uFlags2)
            .field("ReuseBuffer", &self.ReuseBuffer)
            .field("pAllocAllNodesContext", &self.pAllocAllNodesContext)
            .field("pPointerQueueState", &self.pPointerQueueState)
            .field("IgnoreEmbeddedPointers", &self.IgnoreEmbeddedPointers)
            .field("PointerBufferMark", &self.PointerBufferMark)
            .field("CorrDespIncrement", &self.CorrDespIncrement)
            .field("uFlags", &self.uFlags)
            .field("UniquePtrCount", &self.UniquePtrCount)
            .field("MaxCount", &self.MaxCount)
            .field("Offset", &self.Offset)
            .field("ActualCount", &self.ActualCount)
            .field("pfnAllocate", &self.pfnAllocate)
            .field("pfnFree", &self.pfnFree)
            .field("StackTop", &self.StackTop)
            .field("pPresentedType", &self.pPresentedType)
            .field("pTransmitType", &self.pTransmitType)
            .field("SavedHandle", &self.SavedHandle)
            .field("StubDesc", &self.StubDesc)
            .field("FullPtrXlatTables", &self.FullPtrXlatTables)
            .field("FullPtrRefId", &self.FullPtrRefId)
            .field("PointerLength", &self.PointerLength)
            .field("_bitfield", &self._bitfield)
            .field("dwDestContext", &self.dwDestContext)
            .field("pvDestContext", &self.pvDestContext)
            .field("SavedContextHandles", &self.SavedContextHandles)
            .field("ParamNumber", &self.ParamNumber)
            .field("pRpcChannelBuffer", &self.pRpcChannelBuffer)
            .field("pArrayInfo", &self.pArrayInfo)
            .field("SizePtrCountArray", &self.SizePtrCountArray)
            .field("SizePtrOffsetArray", &self.SizePtrOffsetArray)
            .field("SizePtrLengthArray", &self.SizePtrLengthArray)
            .field("pArgQueue", &self.pArgQueue)
            .field("dwStubPhase", &self.dwStubPhase)
            .field("LowStackMark", &self.LowStackMark)
            .field("pAsyncMsg", &self.pAsyncMsg)
            .field("pCorrInfo", &self.pCorrInfo)
            .field("pCorrMemory", &self.pCorrMemory)
            .field("pMemoryList", &self.pMemoryList)
            .field("pCSInfo", &self.pCSInfo)
            .field("ConformanceMark", &self.ConformanceMark)
            .field("VarianceMark", &self.VarianceMark)
            .field("Unused", &self.Unused)
            .field("pContext", &self.pContext)
            .field("ContextHandleHash", &self.ContextHandleHash)
            .field("pUserMarshalList", &self.pUserMarshalList)
            .field("Reserved51_3", &self.Reserved51_3)
            .field("Reserved51_4", &self.Reserved51_4)
            .field("Reserved51_5", &self.Reserved51_5)
            .finish()
    }
}
impl ::core::default::Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDL_SYNTAX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.ProcString == other.ProcString && self.FmtStringOffset == other.FmtStringOffset && self.TypeString == other.TypeString && self.aUserMarshalQuadruple == other.aUserMarshalQuadruple && self.pMethodProperties == other.pMethodProperties && self.pReserved2 == other.pReserved2
    }
}
impl ::core::cmp::Eq for MIDL_SYNTAX_INFO {}
impl ::core::fmt::Debug for MIDL_SYNTAX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_SYNTAX_INFO").field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("ProcString", &self.ProcString).field("FmtStringOffset", &self.FmtStringOffset).field("TypeString", &self.TypeString).field("aUserMarshalQuadruple", &self.aUserMarshalQuadruple).field("pMethodProperties", &self.pMethodProperties).field("pReserved2", &self.pReserved2).finish()
    }
}
impl ::core::default::Default for MIDL_TYPE_PICKLING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDL_TYPE_PICKLING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for MIDL_TYPE_PICKLING_INFO {}
impl ::core::fmt::Debug for MIDL_TYPE_PICKLING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_TYPE_PICKLING_INFO").field("Version", &self.Version).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.TypeFormatString == other.TypeFormatString && self.FormatStringSize == other.FormatStringSize && self.TypeOffset == other.TypeOffset && self.StubDesc == other.StubDesc
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_WINRT_TYPE_SERIALIZATION_INFO").field("Version", &self.Version).field("TypeFormatString", &self.TypeFormatString).field("FormatStringSize", &self.FormatStringSize).field("TypeOffset", &self.TypeOffset).field("StubDesc", &self.StubDesc).finish()
    }
}
impl ::core::default::Default for NDR64_ARRAY_ELEMENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_ARRAY_ELEMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ElementMemSize == other.ElementMemSize && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for NDR64_ARRAY_ELEMENT_INFO {}
impl ::core::fmt::Debug for NDR64_ARRAY_ELEMENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ARRAY_ELEMENT_INFO").field("ElementMemSize", &self.ElementMemSize).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for NDR64_ARRAY_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_ARRAY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_ARRAY_FLAGS {}
impl ::core::fmt::Debug for NDR64_ARRAY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ARRAY_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_BINDINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.Binding == other.Binding && self.NotifyIndex == other.NotifyIndex
    }
}
impl ::core::cmp::Eq for NDR64_BIND_AND_NOTIFY_EXTENSION {}
impl ::core::fmt::Debug for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_AND_NOTIFY_EXTENSION").field("Binding", &self.Binding).field("NotifyIndex", &self.NotifyIndex).finish()
    }
}
impl ::core::default::Default for NDR64_BIND_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_BIND_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.RoutineIndex == other.RoutineIndex && self.Ordinal == other.Ordinal
    }
}
impl ::core::cmp::Eq for NDR64_BIND_CONTEXT {}
impl ::core::fmt::Debug for NDR64_BIND_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_CONTEXT").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
impl ::core::default::Default for NDR64_BIND_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_BIND_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.RoutineIndex == other.RoutineIndex && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for NDR64_BIND_GENERIC {}
impl ::core::fmt::Debug for NDR64_BIND_GENERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_GENERIC").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for NDR64_BIND_PRIMITIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_BIND_PRIMITIVE {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_BIND_PRIMITIVE {}
impl ::core::fmt::Debug for NDR64_BIND_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_PRIMITIVE").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.NumberDims == other.NumberDims && self.NumberElements == other.NumberElements && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BOGUS_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("NumberDims", &self.NumberDims).field("NumberElements", &self.NumberElements).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize && self.OriginalMemberLayout == other.OriginalMemberLayout && self.OriginalPointerLayout == other.OriginalPointerLayout && self.PointerLayout == other.PointerLayout
    }
}
impl ::core::cmp::Eq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BOGUS_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).field("OriginalMemberLayout", &self.OriginalMemberLayout).field("OriginalPointerLayout", &self.OriginalPointerLayout).field("PointerLayout", &self.PointerLayout).finish()
    }
}
impl ::core::default::Default for NDR64_BUFFER_ALIGN_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_BUFFER_ALIGN_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for NDR64_BUFFER_ALIGN_FORMAT {}
impl ::core::fmt::Debug for NDR64_BUFFER_ALIGN_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BUFFER_ALIGN_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for NDR64_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
    }
}
impl ::core::cmp::Eq for NDR64_CONFORMANT_STRING_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).finish()
    }
}
impl ::core::default::Default for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ElementSize == other.ElementSize && self.ConfDescriptor == other.ConfDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_CONF_ARRAY_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ElementSize", &self.ElementSize).field("ConfDescriptor", &self.ConfDescriptor).finish()
    }
}
impl ::core::default::Default for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Dimensions == other.Dimensions && self.MemorySize == other.MemorySize && self.OriginalMemberLayout == other.OriginalMemberLayout && self.OriginalPointerLayout == other.OriginalPointerLayout && self.PointerLayout == other.PointerLayout && self.ConfArrayDescription == other.ConfArrayDescription
    }
}
impl ::core::cmp::Eq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Dimensions", &self.Dimensions).field("MemorySize", &self.MemorySize).field("OriginalMemberLayout", &self.OriginalMemberLayout).field("OriginalPointerLayout", &self.OriginalPointerLayout).field("PointerLayout", &self.PointerLayout).field("ConfArrayDescription", &self.ConfArrayDescription).finish()
    }
}
impl ::core::default::Default for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize && self.ArrayDescription == other.ArrayDescription
    }
}
impl ::core::cmp::Eq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).field("ArrayDescription", &self.ArrayDescription).finish()
    }
}
impl ::core::default::Default for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ElementSize == other.ElementSize && self.ConfDescriptor == other.ConfDescriptor && self.VarDescriptor == other.VarDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_VAR_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ElementSize", &self.ElementSize).field("ConfDescriptor", &self.ConfDescriptor).field("VarDescriptor", &self.VarDescriptor).finish()
    }
}
impl ::core::default::Default for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FixedArrayFormat == other.FixedArrayFormat && self.ConfDescription == other.ConfDescription && self.VarDescription == other.VarDescription && self.OffsetDescription == other.OffsetDescription
    }
}
impl ::core::cmp::Eq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT").field("FixedArrayFormat", &self.FixedArrayFormat).field("ConfDescription", &self.ConfDescription).field("VarDescription", &self.VarDescription).field("OffsetDescription", &self.OffsetDescription).finish()
    }
}
impl ::core::default::Default for NDR64_CONSTANT_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONSTANT_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Guid == other.Guid
    }
}
impl ::core::cmp::Eq for NDR64_CONSTANT_IID_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONSTANT_IID_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONSTANT_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Guid", &self.Guid).finish()
    }
}
impl ::core::default::Default for NDR64_CONTEXT_HANDLE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_CONTEXT_HANDLE_FLAGS {}
impl ::core::fmt::Debug for NDR64_CONTEXT_HANDLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONTEXT_HANDLE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_CONTEXT_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.ContextFlags == other.ContextFlags && self.RundownRoutineIndex == other.RundownRoutineIndex && self.Ordinal == other.Ordinal
    }
}
impl ::core::cmp::Eq for NDR64_CONTEXT_HANDLE_FORMAT {}
impl ::core::fmt::Debug for NDR64_CONTEXT_HANDLE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONTEXT_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("ContextFlags", &self.ContextFlags).field("RundownRoutineIndex", &self.RundownRoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
impl ::core::default::Default for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserve1 == other.Reserve1 && self.Reserve2 == other.Reserve2 && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for NDR64_EMBEDDED_COMPLEX_FORMAT {}
impl ::core::fmt::Debug for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EMBEDDED_COMPLEX_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("Reserve2", &self.Reserve2).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for NDR64_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.SwitchType == other.SwitchType && self.MemoryOffset == other.MemoryOffset && self.MemorySize == other.MemorySize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_ENCAPSULATED_UNION {}
impl ::core::fmt::Debug for NDR64_ENCAPSULATED_UNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ENCAPSULATED_UNION").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("SwitchType", &self.SwitchType).field("MemoryOffset", &self.MemoryOffset).field("MemorySize", &self.MemorySize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_EXPR_CONST32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_EXPR_CONST32 {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Reserved == other.Reserved && self.Reserved1 == other.Reserved1 && self.ConstValue == other.ConstValue
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_CONST32 {}
impl ::core::fmt::Debug for NDR64_EXPR_CONST32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_CONST32").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
impl ::core::default::Default for NDR64_EXPR_CONST64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_EXPR_CONST64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Reserved == other.Reserved && self.Reserved1 == other.Reserved1 && self.ConstValue == other.ConstValue
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_CONST64 {}
impl ::core::fmt::Debug for NDR64_EXPR_CONST64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_CONST64").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
impl ::core::default::Default for NDR64_EXPR_NOOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_EXPR_NOOP {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Size == other.Size && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_NOOP {}
impl ::core::fmt::Debug for NDR64_EXPR_NOOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_NOOP").field("ExprType", &self.ExprType).field("Size", &self.Size).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_EXPR_OPERATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_EXPR_OPERATOR {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Operator == other.Operator && self.CastType == other.CastType && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_OPERATOR {}
impl ::core::fmt::Debug for NDR64_EXPR_OPERATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_OPERATOR").field("ExprType", &self.ExprType).field("Operator", &self.Operator).field("CastType", &self.CastType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_EXPR_VAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_EXPR_VAR {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.VarType == other.VarType && self.Reserved == other.Reserved && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_VAR {}
impl ::core::fmt::Debug for NDR64_EXPR_VAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_VAR").field("ExprType", &self.ExprType).field("VarType", &self.VarType).field("Reserved", &self.Reserved).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for NDR64_FIXED_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_FIXED_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatFormat == other.RepeatFormat && self.Iterations == other.Iterations && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_FIXED_REPEAT_FORMAT {}
impl ::core::fmt::Debug for NDR64_FIXED_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_FIXED_REPEAT_FORMAT").field("RepeatFormat", &self.RepeatFormat).field("Iterations", &self.Iterations).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.TotalSize == other.TotalSize
    }
}
impl ::core::cmp::Eq for NDR64_FIX_ARRAY_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_FIX_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::core::default::Default for NDR64_IID_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_IID_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_IID_FLAGS {}
impl ::core::fmt::Debug for NDR64_IID_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_IID_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.IIDDescriptor == other.IIDDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_IID_FORMAT {}
impl ::core::fmt::Debug for NDR64_IID_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("IIDDescriptor", &self.IIDDescriptor).finish()
    }
}
impl ::core::default::Default for NDR64_MEMPAD_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_MEMPAD_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserve1 == other.Reserve1 && self.MemPad == other.MemPad && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for NDR64_MEMPAD_FORMAT {}
impl ::core::fmt::Debug for NDR64_MEMPAD_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_MEMPAD_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("MemPad", &self.MemPad).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TotalSize == other.TotalSize
    }
}
impl ::core::cmp::Eq for NDR64_NON_CONFORMANT_STRING_FORMAT {}
impl ::core::fmt::Debug for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NON_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::core::default::Default for NDR64_NON_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_NON_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.SwitchType == other.SwitchType && self.MemorySize == other.MemorySize && self.Switch == other.Switch && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_NON_ENCAPSULATED_UNION {}
impl ::core::fmt::Debug for NDR64_NON_ENCAPSULATED_UNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NON_ENCAPSULATED_UNION").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("SwitchType", &self.SwitchType).field("MemorySize", &self.MemorySize).field("Switch", &self.Switch).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_NO_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_NO_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for NDR64_NO_REPEAT_FORMAT {}
impl ::core::fmt::Debug for NDR64_NO_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NO_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for NDR64_PARAM_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_PARAM_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_PARAM_FLAGS {}
impl ::core::fmt::Debug for NDR64_PARAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PARAM_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_PARAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_PARAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Attributes == other.Attributes && self.Reserved == other.Reserved && self.StackOffset == other.StackOffset
    }
}
impl ::core::cmp::Eq for NDR64_PARAM_FORMAT {}
impl ::core::fmt::Debug for NDR64_PARAM_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PARAM_FORMAT").field("Type", &self.Type).field("Attributes", &self.Attributes).field("Reserved", &self.Reserved).field("StackOffset", &self.StackOffset).finish()
    }
}
impl ::core::default::Default for NDR64_PIPE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_PIPE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_PIPE_FLAGS {}
impl ::core::fmt::Debug for NDR64_PIPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PIPE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Type == other.Type && self.MemorySize == other.MemorySize && self.BufferSize == other.BufferSize
    }
}
impl ::core::cmp::Eq for NDR64_PIPE_FORMAT {}
impl ::core::fmt::Debug for NDR64_PIPE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PIPE_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Type", &self.Type).field("MemorySize", &self.MemorySize).field("BufferSize", &self.BufferSize).finish()
    }
}
impl ::core::default::Default for NDR64_POINTER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_POINTER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Pointee == other.Pointee
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_FORMAT {}
impl ::core::fmt::Debug for NDR64_POINTER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Pointee", &self.Pointee).finish()
    }
}
impl ::core::default::Default for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_INSTANCE_HEADER_FORMAT").field("Offset", &self.Offset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_POINTER_REPEAT_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_POINTER_REPEAT_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_REPEAT_FLAGS {}
impl ::core::fmt::Debug for NDR64_POINTER_REPEAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_REPEAT_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_PROC_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_PROC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_PROC_FLAGS {}
impl ::core::fmt::Debug for NDR64_PROC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PROC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_PROC_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_PROC_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.StackSize == other.StackSize && self.ConstantClientBufferSize == other.ConstantClientBufferSize && self.ConstantServerBufferSize == other.ConstantServerBufferSize && self.RpcFlags == other.RpcFlags && self.FloatDoubleMask == other.FloatDoubleMask && self.NumberOfParams == other.NumberOfParams && self.ExtensionSize == other.ExtensionSize
    }
}
impl ::core::cmp::Eq for NDR64_PROC_FORMAT {}
impl ::core::fmt::Debug for NDR64_PROC_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PROC_FORMAT").field("Flags", &self.Flags).field("StackSize", &self.StackSize).field("ConstantClientBufferSize", &self.ConstantClientBufferSize).field("ConstantServerBufferSize", &self.ConstantServerBufferSize).field("RpcFlags", &self.RpcFlags).field("FloatDoubleMask", &self.FloatDoubleMask).field("NumberOfParams", &self.NumberOfParams).field("ExtensionSize", &self.ExtensionSize).finish()
    }
}
impl ::core::default::Default for NDR64_RANGED_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_RANGED_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Reserved == other.Reserved && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for NDR64_RANGED_STRING_FORMAT {}
impl ::core::fmt::Debug for NDR64_RANGED_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGED_STRING_FORMAT").field("Header", &self.Header).field("Reserved", &self.Reserved).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::core::default::Default for NDR64_RANGE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_RANGE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.RangeType == other.RangeType && self.Reserved == other.Reserved && self.MinValue == other.MinValue && self.MaxValue == other.MaxValue
    }
}
impl ::core::cmp::Eq for NDR64_RANGE_FORMAT {}
impl ::core::fmt::Debug for NDR64_RANGE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGE_FORMAT").field("FormatCode", &self.FormatCode).field("RangeType", &self.RangeType).field("Reserved", &self.Reserved).field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).finish()
    }
}
impl ::core::default::Default for NDR64_RANGE_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_RANGE_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Type == other.Type && self.MemorySize == other.MemorySize && self.BufferSize == other.BufferSize && self.MinValue == other.MinValue && self.MaxValue == other.MaxValue
    }
}
impl ::core::cmp::Eq for NDR64_RANGE_PIPE_FORMAT {}
impl ::core::fmt::Debug for NDR64_RANGE_PIPE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGE_PIPE_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Type", &self.Type).field("MemorySize", &self.MemorySize).field("BufferSize", &self.BufferSize).field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).finish()
    }
}
impl ::core::default::Default for NDR64_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Increment == other.Increment && self.OffsetToArray == other.OffsetToArray && self.NumberOfPointers == other.NumberOfPointers
    }
}
impl ::core::cmp::Eq for NDR64_REPEAT_FORMAT {}
impl ::core::fmt::Debug for NDR64_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Increment", &self.Increment).field("OffsetToArray", &self.OffsetToArray).field("NumberOfPointers", &self.NumberOfPointers).finish()
    }
}
impl ::core::default::Default for NDR64_RPC_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_RPC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_RPC_FLAGS {}
impl ::core::fmt::Debug for NDR64_RPC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RPC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_SIMPLE_MEMBER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_SIMPLE_MEMBER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for NDR64_SIMPLE_MEMBER_FORMAT {}
impl ::core::fmt::Debug for NDR64_SIMPLE_MEMBER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIMPLE_MEMBER_FORMAT").field("FormatCode", &self.FormatCode).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::core::default::Default for NDR64_SIMPLE_REGION_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_SIMPLE_REGION_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.RegionSize == other.RegionSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_SIMPLE_REGION_FORMAT {}
impl ::core::fmt::Debug for NDR64_SIMPLE_REGION_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIMPLE_REGION_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("RegionSize", &self.RegionSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.SizeDescription == other.SizeDescription
    }
}
impl ::core::cmp::Eq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {}
impl ::core::fmt::Debug for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIZED_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("SizeDescription", &self.SizeDescription).finish()
    }
}
impl ::core::default::Default for NDR64_STRING_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_STRING_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_STRING_FLAGS {}
impl ::core::fmt::Debug for NDR64_STRING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRING_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_STRING_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_STRING_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.ElementSize == other.ElementSize
    }
}
impl ::core::cmp::Eq for NDR64_STRING_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_STRING_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRING_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("ElementSize", &self.ElementSize).finish()
    }
}
impl ::core::default::Default for NDR64_STRUCTURE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_STRUCTURE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_STRUCTURE_FLAGS {}
impl ::core::fmt::Debug for NDR64_STRUCTURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRUCTURE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize
    }
}
impl ::core::cmp::Eq for NDR64_STRUCTURE_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).finish()
    }
}
impl ::core::default::Default for NDR64_SYSTEM_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_SYSTEM_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.HandleType == other.HandleType && self.DesiredAccess == other.DesiredAccess
    }
}
impl ::core::cmp::Eq for NDR64_SYSTEM_HANDLE_FORMAT {}
impl ::core::fmt::Debug for NDR64_SYSTEM_HANDLE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SYSTEM_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("HandleType", &self.HandleType).field("DesiredAccess", &self.DesiredAccess).finish()
    }
}
impl ::core::default::Default for NDR64_TRANSMIT_AS_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_TRANSMIT_AS_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_TRANSMIT_AS_FLAGS {}
impl ::core::fmt::Debug for NDR64_TRANSMIT_AS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TRANSMIT_AS_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_TRANSMIT_AS_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_TRANSMIT_AS_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.RoutineIndex == other.RoutineIndex && self.TransmittedTypeWireAlignment == other.TransmittedTypeWireAlignment && self.MemoryAlignment == other.MemoryAlignment && self.PresentedTypeMemorySize == other.PresentedTypeMemorySize && self.TransmittedTypeBufferSize == other.TransmittedTypeBufferSize && self.TransmittedType == other.TransmittedType
    }
}
impl ::core::cmp::Eq for NDR64_TRANSMIT_AS_FORMAT {}
impl ::core::fmt::Debug for NDR64_TRANSMIT_AS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TRANSMIT_AS_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("RoutineIndex", &self.RoutineIndex).field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment).field("MemoryAlignment", &self.MemoryAlignment).field("PresentedTypeMemorySize", &self.PresentedTypeMemorySize).field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize).field("TransmittedType", &self.TransmittedType).finish()
    }
}
impl ::core::default::Default for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.RealFormatCode == other.RealFormatCode && self.Reserved == other.Reserved && self.Type == other.Type && self.CtxtFlags == other.CtxtFlags && self.CtxtID == other.CtxtID
    }
}
impl ::core::cmp::Eq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {}
impl ::core::fmt::Debug for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TYPE_STRICT_CONTEXT_HANDLE").field("FormatCode", &self.FormatCode).field("RealFormatCode", &self.RealFormatCode).field("Reserved", &self.Reserved).field("Type", &self.Type).field("CtxtFlags", &self.CtxtFlags).field("CtxtID", &self.CtxtID).finish()
    }
}
impl ::core::default::Default for NDR64_UNION_ARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_UNION_ARM {
    fn eq(&self, other: &Self) -> bool {
        self.CaseValue == other.CaseValue && self.Type == other.Type && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_UNION_ARM {}
impl ::core::fmt::Debug for NDR64_UNION_ARM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_UNION_ARM").field("CaseValue", &self.CaseValue).field("Type", &self.Type).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDR64_UNION_ARM_SELECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_UNION_ARM_SELECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Alignment == other.Alignment && self.Reserved2 == other.Reserved2 && self.Arms == other.Arms
    }
}
impl ::core::cmp::Eq for NDR64_UNION_ARM_SELECTOR {}
impl ::core::fmt::Debug for NDR64_UNION_ARM_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_UNION_ARM_SELECTOR").field("Reserved1", &self.Reserved1).field("Alignment", &self.Alignment).field("Reserved2", &self.Reserved2).field("Arms", &self.Arms).finish()
    }
}
impl ::core::default::Default for NDR64_USER_MARSHAL_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_USER_MARSHAL_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_USER_MARSHAL_FLAGS {}
impl ::core::fmt::Debug for NDR64_USER_MARSHAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_USER_MARSHAL_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDR64_USER_MARSHAL_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_USER_MARSHAL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.RoutineIndex == other.RoutineIndex && self.TransmittedTypeWireAlignment == other.TransmittedTypeWireAlignment && self.MemoryAlignment == other.MemoryAlignment && self.UserTypeMemorySize == other.UserTypeMemorySize && self.TransmittedTypeBufferSize == other.TransmittedTypeBufferSize && self.TransmittedType == other.TransmittedType
    }
}
impl ::core::cmp::Eq for NDR64_USER_MARSHAL_FORMAT {}
impl ::core::fmt::Debug for NDR64_USER_MARSHAL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_USER_MARSHAL_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("RoutineIndex", &self.RoutineIndex).field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment).field("MemoryAlignment", &self.MemoryAlignment).field("UserTypeMemorySize", &self.UserTypeMemorySize).field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize).field("TransmittedType", &self.TransmittedType).finish()
    }
}
impl ::core::default::Default for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.TotalSize == other.TotalSize && self.ElementSize == other.ElementSize && self.VarDescriptor == other.VarDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::fmt::Debug for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_VAR_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).field("ElementSize", &self.ElementSize).field("VarDescriptor", &self.VarDescriptor).finish()
    }
}
impl ::core::default::Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR_CS_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.pSizeConvertRoutines == other.pSizeConvertRoutines && self.pTagGettingRoutines == other.pTagGettingRoutines
    }
}
impl ::core::cmp::Eq for NDR_CS_ROUTINES {}
impl ::core::fmt::Debug for NDR_CS_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_CS_ROUTINES").field("pSizeConvertRoutines", &self.pSizeConvertRoutines).field("pTagGettingRoutines", &self.pTagGettingRoutines).finish()
    }
}
impl ::core::default::Default for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR_EXPR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pOffset == other.pOffset && self.pFormatExpr == other.pFormatExpr
    }
}
impl ::core::cmp::Eq for NDR_EXPR_DESC {}
impl ::core::fmt::Debug for NDR_EXPR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_EXPR_DESC").field("pOffset", &self.pOffset).field("pFormatExpr", &self.pFormatExpr).finish()
    }
}
impl ::core::default::Default for NDR_SCONTEXT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDR_SCONTEXT_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pad == other.pad && self.userContext == other.userContext
    }
}
impl ::core::cmp::Eq for NDR_SCONTEXT_1 {}
impl ::core::fmt::Debug for NDR_SCONTEXT_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_SCONTEXT_1").field("pad", &self.pad).field("userContext", &self.userContext).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.pfnAllocate == other.pfnAllocate && self.pfnFree == other.pfnFree && self.pRpcChannelBuffer == other.pRpcChannelBuffer && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for NDR_USER_MARSHAL_INFO_LEVEL1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_USER_MARSHAL_INFO_LEVEL1").field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).field("pRpcChannelBuffer", &self.pRpcChannelBuffer).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for PROXY_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROXY_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXY_PHASE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RDR_CALLOUT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RDR_CALLOUT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.LastError == other.LastError && self.LastEEInfo == other.LastEEInfo && self.LastCalledStage == other.LastCalledStage && self.ServerName == other.ServerName && self.ServerPort == other.ServerPort && self.RemoteUser == other.RemoteUser && self.AuthType == other.AuthType && self.ResourceTypePresent == other.ResourceTypePresent && self.SessionIdPresent == other.SessionIdPresent && self.InterfacePresent == other.InterfacePresent && self.ResourceType == other.ResourceType && self.SessionId == other.SessionId && self.Interface == other.Interface && self.CertContext == other.CertContext
    }
}
impl ::core::cmp::Eq for RDR_CALLOUT_STATE {}
impl ::core::fmt::Debug for RDR_CALLOUT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RDR_CALLOUT_STATE")
            .field("LastError", &self.LastError)
            .field("LastEEInfo", &self.LastEEInfo)
            .field("LastCalledStage", &self.LastCalledStage)
            .field("ServerName", &self.ServerName)
            .field("ServerPort", &self.ServerPort)
            .field("RemoteUser", &self.RemoteUser)
            .field("AuthType", &self.AuthType)
            .field("ResourceTypePresent", &self.ResourceTypePresent)
            .field("SessionIdPresent", &self.SessionIdPresent)
            .field("InterfacePresent", &self.InterfacePresent)
            .field("ResourceType", &self.ResourceType)
            .field("SessionId", &self.SessionId)
            .field("Interface", &self.Interface)
            .field("CertContext", &self.CertContext)
            .finish()
    }
}
impl ::core::default::Default for RPC_ADDRESS_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_ADDRESS_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_ADDRESS_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RPC_ASYNC_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_ASYNC_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_ASYNC_EVENT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_BINDING_HANDLE_OPTIONS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ComTimeout == other.ComTimeout && self.CallTimeout == other.CallTimeout
    }
}
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_OPTIONS_V1 {}
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_OPTIONS_V1").field("Version", &self.Version).field("Flags", &self.Flags).field("ComTimeout", &self.ComTimeout).field("CallTimeout", &self.CallTimeout).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerPrincName == other.ServerPrincName && self.AuthnLevel == other.AuthnLevel && self.AuthnSvc == other.AuthnSvc && self.AuthIdentity == other.AuthIdentity && self.SecurityQos == other.SecurityQos
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_A").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerPrincName == other.ServerPrincName && self.AuthnLevel == other.AuthnLevel && self.AuthnSvc == other.AuthnSvc && self.AuthIdentity == other.AuthIdentity && self.SecurityQos == other.SecurityQos
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_W").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_BINDING_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.BindingH == other.BindingH
    }
}
impl ::core::cmp::Eq for RPC_BINDING_VECTOR {}
impl ::core::fmt::Debug for RPC_BINDING_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_VECTOR").field("Count", &self.Count).field("BindingH", &self.BindingH).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength && self.ServerPrincipalName == other.ServerPrincipalName && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength && self.ClientPrincipalName == other.ClientPrincipalName && self.AuthenticationLevel == other.AuthenticationLevel && self.AuthenticationService == other.AuthenticationService && self.NullSession == other.NullSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V1_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength && self.ServerPrincipalName == other.ServerPrincipalName && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength && self.ClientPrincipalName == other.ClientPrincipalName && self.AuthenticationLevel == other.AuthenticationLevel && self.AuthenticationService == other.AuthenticationService && self.NullSession == other.NullSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V1_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V2_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V2_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
            && self.ClientIdentifierBufferLength == other.ClientIdentifierBufferLength
            && self.ClientIdentifier == other.ClientIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V3_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .field("ClientIdentifierBufferLength", &self.ClientIdentifierBufferLength)
            .field("ClientIdentifier", &self.ClientIdentifier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
            && self.ClientIdentifierBufferLength == other.ClientIdentifierBufferLength
            && self.ClientIdentifier == other.ClientIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V3_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .field("ClientIdentifierBufferLength", &self.ClientIdentifierBufferLength)
            .field("ClientIdentifier", &self.ClientIdentifier)
            .finish()
    }
}
impl ::core::default::Default for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.AddressFormat == other.AddressFormat
    }
}
impl ::core::cmp::Eq for RPC_CALL_LOCAL_ADDRESS_V1 {}
impl ::core::fmt::Debug for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_LOCAL_ADDRESS_V1").field("Version", &self.Version).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("AddressFormat", &self.AddressFormat).finish()
    }
}
impl ::core::default::Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_CLIENT_INFORMATION1 {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.ComputerName == other.ComputerName && self.Privilege == other.Privilege && self.AuthFlags == other.AuthFlags
    }
}
impl ::core::cmp::Eq for RPC_CLIENT_INFORMATION1 {}
impl ::core::fmt::Debug for RPC_CLIENT_INFORMATION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CLIENT_INFORMATION1").field("UserName", &self.UserName).field("ComputerName", &self.ComputerName).field("Privilege", &self.Privilege).field("AuthFlags", &self.AuthFlags).finish()
    }
}
impl ::core::default::Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_CLIENT_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.InterfaceId == other.InterfaceId && self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.RpcProtseqEndpointCount == other.RpcProtseqEndpointCount && self.RpcProtseqEndpoint == other.RpcProtseqEndpoint && self.Reserved == other.Reserved && self.InterpreterInfo == other.InterpreterInfo && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for RPC_CLIENT_INTERFACE {}
impl ::core::fmt::Debug for RPC_CLIENT_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CLIENT_INTERFACE").field("Length", &self.Length).field("InterfaceId", &self.InterfaceId).field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount).field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint).field("Reserved", &self.Reserved).field("InterpreterInfo", &self.InterpreterInfo).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for RPC_C_AUTHN_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_C_AUTHN_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RPC_C_HTTP_AUTHN_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_C_HTTP_AUTHN_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_HTTP_AUTHN_TARGET").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RPC_C_HTTP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_C_HTTP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_HTTP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_HTTP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_HTTP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {}
impl ::core::fmt::Debug for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR").field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for RPC_C_QOS_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_C_QOS_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_QOS_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_QOS_CAPABILITIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_QOS_CAPABILITIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RPC_C_QOS_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_C_QOS_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_QOS_IDENTITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for RPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_EE_INFO_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProtSeq == other.ProtSeq && self.Endpoint == other.Endpoint && self.SecurityDescriptor == other.SecurityDescriptor && self.Backlog == other.Backlog
    }
}
impl ::core::cmp::Eq for RPC_ENDPOINT_TEMPLATEA {}
impl ::core::fmt::Debug for RPC_ENDPOINT_TEMPLATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ENDPOINT_TEMPLATEA").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
impl ::core::default::Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEW {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProtSeq == other.ProtSeq && self.Endpoint == other.Endpoint && self.SecurityDescriptor == other.SecurityDescriptor && self.Backlog == other.Backlog
    }
}
impl ::core::cmp::Eq for RPC_ENDPOINT_TEMPLATEW {}
impl ::core::fmt::Debug for RPC_ENDPOINT_TEMPLATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ENDPOINT_TEMPLATEW").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
impl ::core::default::Default for RPC_ERROR_ENUM_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_ERROR_ENUM_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.CurrentPos == other.CurrentPos && self.Head == other.Head
    }
}
impl ::core::cmp::Eq for RPC_ERROR_ENUM_HANDLE {}
impl ::core::fmt::Debug for RPC_ERROR_ENUM_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ERROR_ENUM_HANDLE").field("Signature", &self.Signature).field("CurrentPos", &self.CurrentPos).field("Head", &self.Head).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_HTTP_REDIRECTOR_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_HTTP_REDIRECTOR_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_HTTP_REDIRECTOR_STAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_A").field("TransportCredentials", &self.TransportCredentials).field("Flags", &self.Flags).field("AuthenticationTarget", &self.AuthenticationTarget).field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes).field("AuthnSchemes", &self.AuthnSchemes).field("ServerCertificateSubject", &self.ServerCertificateSubject).finish()
    }
}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_W").field("TransportCredentials", &self.TransportCredentials).field("Flags", &self.Flags).field("AuthenticationTarget", &self.AuthenticationTarget).field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes).field("AuthnSchemes", &self.AuthnSchemes).field("ServerCertificateSubject", &self.ServerCertificateSubject).finish()
    }
}
impl ::core::default::Default for RPC_IF_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_IF_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Uuid == other.Uuid && self.VersMajor == other.VersMajor && self.VersMinor == other.VersMinor
    }
}
impl ::core::cmp::Eq for RPC_IF_ID {}
impl ::core::fmt::Debug for RPC_IF_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IF_ID").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
impl ::core::default::Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_IF_ID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.IfId == other.IfId
    }
}
impl ::core::cmp::Eq for RPC_IF_ID_VECTOR {}
impl ::core::fmt::Debug for RPC_IF_ID_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IF_ID_VECTOR").field("Count", &self.Count).field("IfId", &self.IfId).finish()
    }
}
impl ::core::default::Default for RPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_IMPORT_CONTEXT_P {
    fn eq(&self, other: &Self) -> bool {
        self.LookupContext == other.LookupContext && self.ProposedHandle == other.ProposedHandle && self.Bindings == other.Bindings
    }
}
impl ::core::cmp::Eq for RPC_IMPORT_CONTEXT_P {}
impl ::core::fmt::Debug for RPC_IMPORT_CONTEXT_P {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IMPORT_CONTEXT_P").field("LookupContext", &self.LookupContext).field("ProposedHandle", &self.ProposedHandle).field("Bindings", &self.Bindings).finish()
    }
}
impl ::core::default::Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.DataRepresentation == other.DataRepresentation && self.Buffer == other.Buffer && self.BufferLength == other.BufferLength && self.ProcNum == other.ProcNum && self.TransferSyntax == other.TransferSyntax && self.RpcInterfaceInformation == other.RpcInterfaceInformation && self.ReservedForRuntime == other.ReservedForRuntime && self.ManagerEpv == other.ManagerEpv && self.ImportContext == other.ImportContext && self.RpcFlags == other.RpcFlags
    }
}
impl ::core::cmp::Eq for RPC_MESSAGE {}
impl ::core::fmt::Debug for RPC_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_MESSAGE")
            .field("Handle", &self.Handle)
            .field("DataRepresentation", &self.DataRepresentation)
            .field("Buffer", &self.Buffer)
            .field("BufferLength", &self.BufferLength)
            .field("ProcNum", &self.ProcNum)
            .field("TransferSyntax", &self.TransferSyntax)
            .field("RpcInterfaceInformation", &self.RpcInterfaceInformation)
            .field("ReservedForRuntime", &self.ReservedForRuntime)
            .field("ManagerEpv", &self.ManagerEpv)
            .field("ImportContext", &self.ImportContext)
            .field("RpcFlags", &self.RpcFlags)
            .finish()
    }
}
impl ::core::default::Default for RPC_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_NOTIFICATIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RPC_NOTIFICATION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_NOTIFICATION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_NOTIFICATION_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for RPC_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.EndpointFlags == other.EndpointFlags && self.NICFlags == other.NICFlags
    }
}
impl ::core::cmp::Eq for RPC_POLICY {}
impl ::core::fmt::Debug for RPC_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_POLICY").field("Length", &self.Length).field("EndpointFlags", &self.EndpointFlags).field("NICFlags", &self.NICFlags).finish()
    }
}
impl ::core::default::Default for RPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.RpcProtocolSequence == other.RpcProtocolSequence && self.Endpoint == other.Endpoint
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_ENDPOINT {}
impl ::core::fmt::Debug for RPC_PROTSEQ_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_ENDPOINT").field("RpcProtocolSequence", &self.RpcProtocolSequence).field("Endpoint", &self.Endpoint).finish()
    }
}
impl ::core::default::Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_VECTORA {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Protseq == other.Protseq
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_VECTORA {}
impl ::core::fmt::Debug for RPC_PROTSEQ_VECTORA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_VECTORA").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
impl ::core::default::Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_VECTORW {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Protseq == other.Protseq
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_VECTORW {}
impl ::core::fmt::Debug for RPC_PROTSEQ_VECTORW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_VECTORW").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Capabilities == other.Capabilities && self.IdentityTracking == other.IdentityTracking && self.ImpersonationType == other.ImpersonationType
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_SECURITY_QOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SECURITY_QOS").field("Version", &self.Version).field("Capabilities", &self.Capabilities).field("IdentityTracking", &self.IdentityTracking).field("ImpersonationType", &self.ImpersonationType).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RPC_SEC_CONTEXT_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_SEC_CONTEXT_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EncryptAlgorithm == other.EncryptAlgorithm && self.KeySize == other.KeySize && self.SignatureAlgorithm == other.SignatureAlgorithm
    }
}
impl ::core::cmp::Eq for RPC_SEC_CONTEXT_KEY_INFO {}
impl ::core::fmt::Debug for RPC_SEC_CONTEXT_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SEC_CONTEXT_KEY_INFO").field("EncryptAlgorithm", &self.EncryptAlgorithm).field("KeySize", &self.KeySize).field("SignatureAlgorithm", &self.SignatureAlgorithm).finish()
    }
}
impl ::core::default::Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_SERVER_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.InterfaceId == other.InterfaceId && self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.RpcProtseqEndpointCount == other.RpcProtseqEndpointCount && self.RpcProtseqEndpoint == other.RpcProtseqEndpoint && self.DefaultManagerEpv == other.DefaultManagerEpv && self.InterpreterInfo == other.InterpreterInfo && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for RPC_SERVER_INTERFACE {}
impl ::core::fmt::Debug for RPC_SERVER_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SERVER_INTERFACE").field("Length", &self.Length).field("InterfaceId", &self.InterfaceId).field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount).field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint).field("DefaultManagerEpv", &self.DefaultManagerEpv).field("InterpreterInfo", &self.InterpreterInfo).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_STATS_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Stats == other.Stats
    }
}
impl ::core::cmp::Eq for RPC_STATS_VECTOR {}
impl ::core::fmt::Debug for RPC_STATS_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_STATS_VECTOR").field("Count", &self.Count).field("Stats", &self.Stats).finish()
    }
}
impl ::core::default::Default for RPC_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RPC_SYNTAX_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_SYNTAX_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.SyntaxGUID == other.SyntaxGUID && self.SyntaxVersion == other.SyntaxVersion
    }
}
impl ::core::cmp::Eq for RPC_SYNTAX_IDENTIFIER {}
impl ::core::fmt::Debug for RPC_SYNTAX_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SYNTAX_IDENTIFIER").field("SyntaxGUID", &self.SyntaxGUID).field("SyntaxVersion", &self.SyntaxVersion).finish()
    }
}
impl ::core::default::Default for RPC_TRANSFER_SYNTAX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_TRANSFER_SYNTAX {
    fn eq(&self, other: &Self) -> bool {
        self.Uuid == other.Uuid && self.VersMajor == other.VersMajor && self.VersMinor == other.VersMinor
    }
}
impl ::core::cmp::Eq for RPC_TRANSFER_SYNTAX {}
impl ::core::fmt::Debug for RPC_TRANSFER_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_TRANSFER_SYNTAX").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
impl ::core::default::Default for RPC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RPC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for RPC_VERSION {}
impl ::core::fmt::Debug for RPC_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::core::default::Default for RpcCallClientLocality {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RpcCallClientLocality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcCallClientLocality").field(&self.0).finish()
    }
}
impl ::core::default::Default for RpcCallType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RpcCallType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcCallType").field(&self.0).finish()
    }
}
impl ::core::default::Default for RpcLocalAddressFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RpcLocalAddressFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcLocalAddressFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for RpcPerfCounters {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RpcPerfCounters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcPerfCounters").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCONTEXT_QUEUE {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfObjects == other.NumberOfObjects && self.ArrayOfObjects == other.ArrayOfObjects
    }
}
impl ::core::cmp::Eq for SCONTEXT_QUEUE {}
impl ::core::fmt::Debug for SCONTEXT_QUEUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCONTEXT_QUEUE").field("NumberOfObjects", &self.NumberOfObjects).field("ArrayOfObjects", &self.ArrayOfObjects).finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEC_WINNT_AUTH_IDENTITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_A {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_A {}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_A").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_W {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_W {}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_W").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for STUB_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STUB_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STUB_PHASE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for USER_MARSHAL_CB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for USER_MARSHAL_CB {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pStubMsg == other.pStubMsg && self.pReserve == other.pReserve && self.Signature == other.Signature && self.CBType == other.CBType && self.pFormat == other.pFormat && self.pTypeFormat == other.pTypeFormat
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for USER_MARSHAL_CB {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for USER_MARSHAL_CB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MARSHAL_CB").field("Flags", &self.Flags).field("pStubMsg", &self.pStubMsg).field("pReserve", &self.pReserve).field("Signature", &self.Signature).field("CBType", &self.CBType).field("pFormat", &self.pFormat).field("pTypeFormat", &self.pTypeFormat).finish()
    }
}
impl ::core::default::Default for USER_MARSHAL_CB_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USER_MARSHAL_CB_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_MARSHAL_CB_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UUID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Uuid == other.Uuid
    }
}
impl ::core::cmp::Eq for UUID_VECTOR {}
impl ::core::fmt::Debug for UUID_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UUID_VECTOR").field("Count", &self.Count).field("Uuid", &self.Uuid).finish()
    }
}
impl ::core::default::Default for XLAT_SIDE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XLAT_SIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XLAT_SIDE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for XMIT_ROUTINE_QUINTUPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for _NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _NDR_SCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.pad == other.pad && self.userContext == other.userContext
    }
}
impl ::core::cmp::Eq for _NDR_SCONTEXT {}
impl ::core::fmt::Debug for _NDR_SCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_NDR_SCONTEXT").field("pad", &self.pad).field("userContext", &self.userContext).finish()
    }
}
impl ::core::default::Default for system_handle_t {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for system_handle_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("system_handle_t").field(&self.0).finish()
    }
}
