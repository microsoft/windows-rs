pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
pub const JsArray: JsValueType = 8i32;
pub const JsBoolean: JsValueType = 4i32;
pub const JsError: JsValueType = 7i32;
pub const JsErrorAlreadyDebuggingContext: JsErrorCode = 65552u32;
pub const JsErrorAlreadyProfilingContext: JsErrorCode = 65553u32;
pub const JsErrorArgumentNotObject: JsErrorCode = 65548u32;
pub const JsErrorBadSerializedScript: JsErrorCode = 65544u32;
pub const JsErrorCannotDisableExecution: JsErrorCode = 65546u32;
pub const JsErrorCannotSerializeDebugScript: JsErrorCode = 65551u32;
pub const JsErrorCategoryEngine: JsErrorCode = 131072u32;
pub const JsErrorCategoryFatal: JsErrorCode = 262144u32;
pub const JsErrorCategoryScript: JsErrorCode = 196608u32;
pub const JsErrorCategoryUsage: JsErrorCode = 65536u32;
pub const JsErrorFatal: JsErrorCode = 262145u32;
pub const JsErrorHeapEnumInProgress: JsErrorCode = 65547u32;
pub const JsErrorIdleNotEnabled: JsErrorCode = 65554u32;
pub const JsErrorInDisabledState: JsErrorCode = 65545u32;
pub const JsErrorInExceptionState: JsErrorCode = 65540u32;
pub const JsErrorInProfileCallback: JsErrorCode = 65549u32;
pub const JsErrorInThreadServiceCallback: JsErrorCode = 65550u32;
pub const JsErrorInvalidArgument: JsErrorCode = 65537u32;
pub const JsErrorNoCurrentContext: JsErrorCode = 65539u32;
pub const JsErrorNotImplemented: JsErrorCode = 65541u32;
pub const JsErrorNullArgument: JsErrorCode = 65538u32;
pub const JsErrorOutOfMemory: JsErrorCode = 131073u32;
pub const JsErrorRuntimeInUse: JsErrorCode = 65543u32;
pub const JsErrorScriptCompile: JsErrorCode = 196610u32;
pub const JsErrorScriptEvalDisabled: JsErrorCode = 196612u32;
pub const JsErrorScriptException: JsErrorCode = 196609u32;
pub const JsErrorScriptTerminated: JsErrorCode = 196611u32;
pub const JsErrorWrongThread: JsErrorCode = 65542u32;
pub const JsFunction: JsValueType = 6i32;
pub const JsMemoryAllocate: JsMemoryEventType = 0i32;
pub const JsMemoryFailure: JsMemoryEventType = 2i32;
pub const JsMemoryFree: JsMemoryEventType = 1i32;
pub const JsNoError: JsErrorCode = 0u32;
pub const JsNull: JsValueType = 1i32;
pub const JsNumber: JsValueType = 2i32;
pub const JsObject: JsValueType = 5i32;
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = 2i32;
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = 1i32;
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = 16i32;
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = 8i32;
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = 4i32;
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = 0i32;
pub const JsRuntimeVersion10: JsRuntimeVersion = 0i32;
pub const JsRuntimeVersion11: JsRuntimeVersion = 1i32;
pub const JsRuntimeVersionEdge: JsRuntimeVersion = -1i32;
pub const JsString: JsValueType = 3i32;
pub const JsUndefined: JsValueType = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsErrorCode(pub u32);
impl windows_core::TypeKind for JsErrorCode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsMemoryEventType(pub i32);
impl windows_core::TypeKind for JsMemoryEventType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsRuntimeAttributes(pub i32);
impl windows_core::TypeKind for JsRuntimeAttributes {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsRuntimeVersion(pub i32);
impl windows_core::TypeKind for JsRuntimeVersion {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsValueType(pub i32);
impl windows_core::TypeKind for JsValueType {
    type TypeKind = windows_core::CopyType;
}
pub type JsBackgroundWorkItemCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void)>;
pub type JsBeforeCollectCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void)>;
pub type JsFinalizeCallback = Option<unsafe extern "system" fn(data: *const core::ffi::c_void)>;
pub type JsMemoryAllocationCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool>;
pub type JsNativeFunction = Option<unsafe extern "system" fn(callee: *const core::ffi::c_void, isconstructcall: bool, arguments: *const *const core::ffi::c_void, argumentcount: u16, callbackstate: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type JsThreadServiceCallback = Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackstate: *const core::ffi::c_void) -> bool>;
