#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn JsAddRef(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    pub fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsBooleanToBool(value: *const ::core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode;
    pub fn JsCallFunction(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCollectGarbage(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsConstructObject(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsConvertValueToBoolean(value: *const ::core::ffi::c_void, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsConvertValueToNumber(value: *const ::core::ffi::c_void, numbervalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsConvertValueToObject(value: *const ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsConvertValueToString(value: *const ::core::ffi::c_void, stringvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateArray(length: u32, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: super::Diagnostics::Debug::IDebugApplication64, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: super::Diagnostics::Debug::IDebugApplication32, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateExternalObject(data: *const ::core::ffi::c_void, finalizecallback: JsFinalizeCallback, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateFunction(nativefunction: JsNativeFunction, callbackstate: *const ::core::ffi::c_void, function: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateObject(object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateRangeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateReferenceError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: JsThreadServiceCallback, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateSyntaxError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateTypeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsCreateURIError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsDefineProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    pub fn JsDeleteIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsDeleteProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, usestrictrules: u8, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsDisableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsDisposeRuntime(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsEnableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsEnumerateHeap(enumerator: *mut super::Diagnostics::Debug::IActiveScriptProfilerHeapEnum) -> JsErrorCode;
    pub fn JsEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    pub fn JsGetAndClearException(exception: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetCurrentContext(currentcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetExtensionAllowed(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    pub fn JsGetExternalData(object: *const ::core::ffi::c_void, externaldata: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetFalseValue(falsevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetGlobalObject(globalobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetNullValue(nullvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetOwnPropertyDescriptor(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetOwnPropertyNames(object: *const ::core::ffi::c_void, propertynames: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsGetPropertyIdFromName(name: super::super::Foundation::PWSTR, propertyid: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetPropertyNameFromId(propertyid: *const ::core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode;
    pub fn JsGetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetRuntime(context: *const ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode;
    pub fn JsGetRuntimeMemoryUsage(runtime: *const ::core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode;
    pub fn JsGetStringLength(stringvalue: *const ::core::ffi::c_void, length: *mut i32) -> JsErrorCode;
    pub fn JsGetTrueValue(truevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsGetValueType(value: *const ::core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode;
    pub fn JsHasException(hasexception: *mut bool) -> JsErrorCode;
    pub fn JsHasExternalData(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    pub fn JsHasIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    pub fn JsHasProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode;
    pub fn JsIdle(nextidletick: *mut u32) -> JsErrorCode;
    pub fn JsIntToNumber(intvalue: i32, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode;
    pub fn JsIsRuntimeExecutionDisabled(runtime: *const ::core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode;
    pub fn JsNumberToDouble(value: *const ::core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode;
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsParseScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsParseSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsPointerToString(stringvalue: super::super::Foundation::PWSTR, stringlength: usize, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsPreventExtension(object: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsRelease(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsRunScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsRunSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsSerializeScript(script: super::super::Foundation::PWSTR, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode;
    pub fn JsSetCurrentContext(context: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsSetException(exception: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsSetExternalData(object: *const ::core::ffi::c_void, externaldata: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsSetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsSetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, usestrictrules: u8) -> JsErrorCode;
    pub fn JsSetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *const ::core::ffi::c_void) -> JsErrorCode;
    pub fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, beforecollectcallback: JsBeforeCollectCallback) -> JsErrorCode;
    pub fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, allocationcallback: JsMemoryAllocationCallback) -> JsErrorCode;
    pub fn JsSetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: usize) -> JsErrorCode;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging(debugapplication: super::Diagnostics::Debug::IDebugApplication64) -> JsErrorCode;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging(debugapplication: super::Diagnostics::Debug::IDebugApplication32) -> JsErrorCode;
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartProfiling(callback: super::Diagnostics::Debug::IActiveScriptProfilerCallback, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode;
    pub fn JsStopProfiling(reason: ::windows_sys::core::HRESULT) -> JsErrorCode;
    pub fn JsStrictEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    pub fn JsStringToPointer(value: *const ::core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsValueToVariant(object: *const ::core::ffi::c_void, variant: *mut super::Com::VARIANT) -> JsErrorCode;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsVariantToValue(variant: *const super::Com::VARIANT, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
}
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
#[repr(C)]
pub struct JsBackgroundWorkItemCallback(i32);
#[repr(C)]
pub struct JsBeforeCollectCallback(i32);
#[repr(transparent)]
pub struct JsErrorCode(pub u32);
pub const JsNoError: JsErrorCode = JsErrorCode(0u32);
pub const JsErrorCategoryUsage: JsErrorCode = JsErrorCode(65536u32);
pub const JsErrorInvalidArgument: JsErrorCode = JsErrorCode(65537u32);
pub const JsErrorNullArgument: JsErrorCode = JsErrorCode(65538u32);
pub const JsErrorNoCurrentContext: JsErrorCode = JsErrorCode(65539u32);
pub const JsErrorInExceptionState: JsErrorCode = JsErrorCode(65540u32);
pub const JsErrorNotImplemented: JsErrorCode = JsErrorCode(65541u32);
pub const JsErrorWrongThread: JsErrorCode = JsErrorCode(65542u32);
pub const JsErrorRuntimeInUse: JsErrorCode = JsErrorCode(65543u32);
pub const JsErrorBadSerializedScript: JsErrorCode = JsErrorCode(65544u32);
pub const JsErrorInDisabledState: JsErrorCode = JsErrorCode(65545u32);
pub const JsErrorCannotDisableExecution: JsErrorCode = JsErrorCode(65546u32);
pub const JsErrorHeapEnumInProgress: JsErrorCode = JsErrorCode(65547u32);
pub const JsErrorArgumentNotObject: JsErrorCode = JsErrorCode(65548u32);
pub const JsErrorInProfileCallback: JsErrorCode = JsErrorCode(65549u32);
pub const JsErrorInThreadServiceCallback: JsErrorCode = JsErrorCode(65550u32);
pub const JsErrorCannotSerializeDebugScript: JsErrorCode = JsErrorCode(65551u32);
pub const JsErrorAlreadyDebuggingContext: JsErrorCode = JsErrorCode(65552u32);
pub const JsErrorAlreadyProfilingContext: JsErrorCode = JsErrorCode(65553u32);
pub const JsErrorIdleNotEnabled: JsErrorCode = JsErrorCode(65554u32);
pub const JsErrorCategoryEngine: JsErrorCode = JsErrorCode(131072u32);
pub const JsErrorOutOfMemory: JsErrorCode = JsErrorCode(131073u32);
pub const JsErrorCategoryScript: JsErrorCode = JsErrorCode(196608u32);
pub const JsErrorScriptException: JsErrorCode = JsErrorCode(196609u32);
pub const JsErrorScriptCompile: JsErrorCode = JsErrorCode(196610u32);
pub const JsErrorScriptTerminated: JsErrorCode = JsErrorCode(196611u32);
pub const JsErrorScriptEvalDisabled: JsErrorCode = JsErrorCode(196612u32);
pub const JsErrorCategoryFatal: JsErrorCode = JsErrorCode(262144u32);
pub const JsErrorFatal: JsErrorCode = JsErrorCode(262145u32);
#[repr(C)]
pub struct JsFinalizeCallback(i32);
#[repr(C)]
pub struct JsMemoryAllocationCallback(i32);
#[repr(transparent)]
pub struct JsMemoryEventType(pub i32);
pub const JsMemoryAllocate: JsMemoryEventType = JsMemoryEventType(0i32);
pub const JsMemoryFree: JsMemoryEventType = JsMemoryEventType(1i32);
pub const JsMemoryFailure: JsMemoryEventType = JsMemoryEventType(2i32);
#[repr(C)]
pub struct JsNativeFunction(i32);
#[repr(transparent)]
pub struct JsRuntimeAttributes(pub i32);
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = JsRuntimeAttributes(0i32);
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = JsRuntimeAttributes(1i32);
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = JsRuntimeAttributes(2i32);
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = JsRuntimeAttributes(4i32);
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = JsRuntimeAttributes(8i32);
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = JsRuntimeAttributes(16i32);
#[repr(transparent)]
pub struct JsRuntimeVersion(pub i32);
pub const JsRuntimeVersion10: JsRuntimeVersion = JsRuntimeVersion(0i32);
pub const JsRuntimeVersion11: JsRuntimeVersion = JsRuntimeVersion(1i32);
pub const JsRuntimeVersionEdge: JsRuntimeVersion = JsRuntimeVersion(-1i32);
#[repr(C)]
pub struct JsThreadServiceCallback(i32);
#[repr(transparent)]
pub struct JsValueType(pub i32);
pub const JsUndefined: JsValueType = JsValueType(0i32);
pub const JsNull: JsValueType = JsValueType(1i32);
pub const JsNumber: JsValueType = JsValueType(2i32);
pub const JsString: JsValueType = JsValueType(3i32);
pub const JsBoolean: JsValueType = JsValueType(4i32);
pub const JsObject: JsValueType = JsValueType(5i32);
pub const JsFunction: JsValueType = JsValueType(6i32);
pub const JsError: JsValueType = JsValueType(7i32);
pub const JsArray: JsValueType = JsValueType(8i32);
