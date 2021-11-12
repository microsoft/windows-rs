#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_System_Js`*"]
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsAddRef(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsBooleanToBool(value: *const ::core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCallFunction(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCollectGarbage(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConstructObject(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToBoolean(value: *const ::core::ffi::c_void, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToNumber(value: *const ::core::ffi::c_void, numbervalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToObject(value: *const ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToString(value: *const ::core::ffi::c_void, stringvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateArray(length: u32, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: super::Diagnostics::Debug::IDebugApplication64, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: super::Diagnostics::Debug::IDebugApplication32, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateExternalObject(data: *const ::core::ffi::c_void, finalizecallback: JsFinalizeCallback, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateFunction(nativefunction: JsNativeFunction, callbackstate: *const ::core::ffi::c_void, function: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateObject(object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateRangeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateReferenceError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: JsThreadServiceCallback, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateSyntaxError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateTypeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateURIError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDefineProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDeleteIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDeleteProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, usestrictrules: u8, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDisableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDisposeRuntime(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsEnableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsEnumerateHeap(enumerator: *mut super::Diagnostics::Debug::IActiveScriptProfilerHeapEnum) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetAndClearException(exception: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetCurrentContext(currentcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetExtensionAllowed(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetExternalData(object: *const ::core::ffi::c_void, externaldata: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetFalseValue(falsevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetGlobalObject(globalobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetNullValue(nullvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetOwnPropertyDescriptor(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetOwnPropertyNames(object: *const ::core::ffi::c_void, propertynames: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsGetPropertyIdFromName(name: super::super::Foundation::PWSTR, propertyid: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetPropertyNameFromId(propertyid: *const ::core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetRuntime(context: *const ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetRuntimeMemoryUsage(runtime: *const ::core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetStringLength(stringvalue: *const ::core::ffi::c_void, length: *mut i32) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetTrueValue(truevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetValueType(value: *const ::core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasException(hasexception: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasExternalData(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIdle(nextidletick: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIntToNumber(intvalue: i32, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIsRuntimeExecutionDisabled(runtime: *const ::core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsNumberToDouble(value: *const ::core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsParseScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsParseSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsPointerToString(stringvalue: super::super::Foundation::PWSTR, stringlength: usize, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsPreventExtension(object: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsRelease(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsRunScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsRunSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsSerializeScript(script: super::super::Foundation::PWSTR, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetCurrentContext(context: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetException(exception: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetExternalData(object: *const ::core::ffi::c_void, externaldata: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, usestrictrules: u8) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *const ::core::ffi::c_void) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, beforecollectcallback: JsBeforeCollectCallback) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, allocationcallback: JsMemoryAllocationCallback) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: usize) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging(debugapplication: super::Diagnostics::Debug::IDebugApplication64) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging(debugapplication: super::Diagnostics::Debug::IDebugApplication32) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartProfiling(callback: super::Diagnostics::Debug::IActiveScriptProfilerCallback, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsStopProfiling(reason: ::windows_sys::core::HRESULT) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsStrictEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsStringToPointer(value: *const ::core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsValueToVariant(object: *const ::core::ffi::c_void, variant: *mut super::Com::VARIANT) -> JsErrorCode;
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsVariantToValue(variant: *const super::Com::VARIANT, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
}
