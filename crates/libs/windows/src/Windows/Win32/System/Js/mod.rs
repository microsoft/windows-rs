#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsAddRef(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsAddRef(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    }
    JsAddRef(::core::mem::transmute(r#ref), ::core::mem::transmute(count))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsBackgroundWorkItemCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsBeforeCollectCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsBoolToBoolean(value, ::core::mem::transmute(booleanvalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsBooleanToBool(value: *const ::core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsBooleanToBool(value: *const ::core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode;
    }
    JsBooleanToBool(::core::mem::transmute(value), ::core::mem::transmute(boolvalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCallFunction(function: *const ::core::ffi::c_void, arguments: &[*const ::core::ffi::c_void], result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCallFunction(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCallFunction(::core::mem::transmute(function), ::core::mem::transmute(::windows::core::as_ptr_or_null(arguments)), arguments.len() as _, ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCollectGarbage(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCollectGarbage(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCollectGarbage(::core::mem::transmute(runtime))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsConstructObject(function: *const ::core::ffi::c_void, arguments: &[*const ::core::ffi::c_void], result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsConstructObject(function: *const ::core::ffi::c_void, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsConstructObject(::core::mem::transmute(function), ::core::mem::transmute(::windows::core::as_ptr_or_null(arguments)), arguments.len() as _, ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsConvertValueToBoolean(value: *const ::core::ffi::c_void, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsConvertValueToBoolean(value: *const ::core::ffi::c_void, booleanvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsConvertValueToBoolean(::core::mem::transmute(value), ::core::mem::transmute(booleanvalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsConvertValueToNumber(value: *const ::core::ffi::c_void, numbervalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsConvertValueToNumber(value: *const ::core::ffi::c_void, numbervalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsConvertValueToNumber(::core::mem::transmute(value), ::core::mem::transmute(numbervalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsConvertValueToObject(value: *const ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsConvertValueToObject(value: *const ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsConvertValueToObject(::core::mem::transmute(value), ::core::mem::transmute(object))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsConvertValueToString(value: *const ::core::ffi::c_void, stringvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsConvertValueToString(value: *const ::core::ffi::c_void, stringvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsConvertValueToString(::core::mem::transmute(value), ::core::mem::transmute(stringvalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateArray(length: u32, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateArray(length: u32, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateArray(length, ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsCreateContext<'a, P0>(runtime: *const ::core::ffi::c_void, debugapplication: P0, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Diagnostics::Debug::IDebugApplication64>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: *mut ::core::ffi::c_void, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateContext(::core::mem::transmute(runtime), debugapplication.into().abi(), ::core::mem::transmute(newcontext))
}
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsCreateContext<'a, P0>(runtime: *const ::core::ffi::c_void, debugapplication: P0, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Diagnostics::Debug::IDebugApplication32>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateContext(runtime: *const ::core::ffi::c_void, debugapplication: *mut ::core::ffi::c_void, newcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateContext(::core::mem::transmute(runtime), debugapplication.into().abi(), ::core::mem::transmute(newcontext))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateError(::core::mem::transmute(message), ::core::mem::transmute(error))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateExternalObject(data: *const ::core::ffi::c_void, finalizecallback: JsFinalizeCallback, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateExternalObject(data: *const ::core::ffi::c_void, finalizecallback: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateExternalObject(::core::mem::transmute(data), ::core::mem::transmute(finalizecallback), ::core::mem::transmute(object))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateFunction(nativefunction: JsNativeFunction, callbackstate: *const ::core::ffi::c_void, function: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateFunction(nativefunction: *mut ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, function: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateFunction(::core::mem::transmute(nativefunction), ::core::mem::transmute(callbackstate), ::core::mem::transmute(function))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateObject(object: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateObject(object: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateObject(::core::mem::transmute(object))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateRangeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateRangeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateRangeError(::core::mem::transmute(message), ::core::mem::transmute(error))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateReferenceError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateReferenceError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateReferenceError(::core::mem::transmute(message), ::core::mem::transmute(error))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: JsThreadServiceCallback, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: *mut ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateRuntime(attributes, runtimeversion, ::core::mem::transmute(threadservice), ::core::mem::transmute(runtime))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateSyntaxError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateSyntaxError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateSyntaxError(::core::mem::transmute(message), ::core::mem::transmute(error))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateTypeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateTypeError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateTypeError(::core::mem::transmute(message), ::core::mem::transmute(error))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsCreateURIError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsCreateURIError(message: *const ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsCreateURIError(::core::mem::transmute(message), ::core::mem::transmute(error))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsDefineProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsDefineProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    }
    JsDefineProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(propertydescriptor), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsDeleteIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsDeleteIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsDeleteIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsDeleteProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, usestrictrules: u8, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsDeleteProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, usestrictrules: u8, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsDeleteProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), usestrictrules, ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsDisableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsDisableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsDisableRuntimeExecution(::core::mem::transmute(runtime))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsDisposeRuntime(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsDisposeRuntime(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsDisposeRuntime(::core::mem::transmute(runtime))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsDoubleToNumber(doublevalue, ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsEnableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsEnableRuntimeExecution(runtime: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsEnableRuntimeExecution(::core::mem::transmute(runtime))
}
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsEnumerateHeap(enumerator: *mut ::core::option::Option<super::Diagnostics::Debug::IActiveScriptProfilerHeapEnum>) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsEnumerateHeap(enumerator: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsEnumerateHeap(::core::mem::transmute(enumerator))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    }
    JsEquals(::core::mem::transmute(object1), ::core::mem::transmute(object2), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsErrorCode(pub u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNoError: JsErrorCode = JsErrorCode(0u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryUsage: JsErrorCode = JsErrorCode(65536u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInvalidArgument: JsErrorCode = JsErrorCode(65537u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNullArgument: JsErrorCode = JsErrorCode(65538u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNoCurrentContext: JsErrorCode = JsErrorCode(65539u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInExceptionState: JsErrorCode = JsErrorCode(65540u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNotImplemented: JsErrorCode = JsErrorCode(65541u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorWrongThread: JsErrorCode = JsErrorCode(65542u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorRuntimeInUse: JsErrorCode = JsErrorCode(65543u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorBadSerializedScript: JsErrorCode = JsErrorCode(65544u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInDisabledState: JsErrorCode = JsErrorCode(65545u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCannotDisableExecution: JsErrorCode = JsErrorCode(65546u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorHeapEnumInProgress: JsErrorCode = JsErrorCode(65547u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorArgumentNotObject: JsErrorCode = JsErrorCode(65548u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInProfileCallback: JsErrorCode = JsErrorCode(65549u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInThreadServiceCallback: JsErrorCode = JsErrorCode(65550u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCannotSerializeDebugScript: JsErrorCode = JsErrorCode(65551u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorAlreadyDebuggingContext: JsErrorCode = JsErrorCode(65552u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorAlreadyProfilingContext: JsErrorCode = JsErrorCode(65553u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorIdleNotEnabled: JsErrorCode = JsErrorCode(65554u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryEngine: JsErrorCode = JsErrorCode(131072u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorOutOfMemory: JsErrorCode = JsErrorCode(131073u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryScript: JsErrorCode = JsErrorCode(196608u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptException: JsErrorCode = JsErrorCode(196609u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptCompile: JsErrorCode = JsErrorCode(196610u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptTerminated: JsErrorCode = JsErrorCode(196611u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptEvalDisabled: JsErrorCode = JsErrorCode(196612u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryFatal: JsErrorCode = JsErrorCode(262144u32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorFatal: JsErrorCode = JsErrorCode(262145u32);
impl ::core::marker::Copy for JsErrorCode {}
impl ::core::clone::Clone for JsErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JsErrorCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsErrorCode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsFinalizeCallback = ::core::option::Option<unsafe extern "system" fn(data: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetAndClearException(exception: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetAndClearException(exception: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetAndClearException(::core::mem::transmute(exception))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetCurrentContext(currentcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetCurrentContext(currentcontext: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetCurrentContext(::core::mem::transmute(currentcontext))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetExtensionAllowed(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetExtensionAllowed(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    }
    JsGetExtensionAllowed(::core::mem::transmute(object), ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetExternalData(object: *const ::core::ffi::c_void, externaldata: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetExternalData(object: *const ::core::ffi::c_void, externaldata: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetExternalData(::core::mem::transmute(object), ::core::mem::transmute(externaldata))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetFalseValue(falsevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetFalseValue(falsevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetFalseValue(::core::mem::transmute(falsevalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetGlobalObject(globalobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetGlobalObject(globalobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetGlobalObject(::core::mem::transmute(globalobject))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetNullValue(nullvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetNullValue(nullvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetNullValue(::core::mem::transmute(nullvalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetOwnPropertyDescriptor(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetOwnPropertyDescriptor(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, propertydescriptor: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetOwnPropertyDescriptor(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(propertydescriptor))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetOwnPropertyNames(object: *const ::core::ffi::c_void, propertynames: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetOwnPropertyNames(object: *const ::core::ffi::c_void, propertynames: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetOwnPropertyNames(::core::mem::transmute(object), ::core::mem::transmute(propertynames))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetPropertyIdFromName<'a, P0>(name: P0, propertyid: *mut *mut ::core::ffi::c_void) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetPropertyIdFromName(name: ::windows::core::PCWSTR, propertyid: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetPropertyIdFromName(name.into(), ::core::mem::transmute(propertyid))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetPropertyNameFromId(propertyid: *const ::core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetPropertyNameFromId(propertyid: *const ::core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode;
    }
    JsGetPropertyNameFromId(::core::mem::transmute(propertyid), ::core::mem::transmute(name))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetPrototype(::core::mem::transmute(object), ::core::mem::transmute(prototypeobject))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetRuntime(context: *const ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetRuntime(context: *const ::core::ffi::c_void, runtime: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetRuntime(::core::mem::transmute(context), ::core::mem::transmute(runtime))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode;
    }
    JsGetRuntimeMemoryLimit(::core::mem::transmute(runtime), ::core::mem::transmute(memorylimit))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetRuntimeMemoryUsage(runtime: *const ::core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetRuntimeMemoryUsage(runtime: *const ::core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode;
    }
    JsGetRuntimeMemoryUsage(::core::mem::transmute(runtime), ::core::mem::transmute(memoryusage))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetStringLength(stringvalue: *const ::core::ffi::c_void, length: *mut i32) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetStringLength(stringvalue: *const ::core::ffi::c_void, length: *mut i32) -> JsErrorCode;
    }
    JsGetStringLength(::core::mem::transmute(stringvalue), ::core::mem::transmute(length))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetTrueValue(truevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetTrueValue(truevalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetTrueValue(::core::mem::transmute(truevalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsGetUndefinedValue(::core::mem::transmute(undefinedvalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsGetValueType(value: *const ::core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsGetValueType(value: *const ::core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode;
    }
    JsGetValueType(::core::mem::transmute(value), ::core::mem::transmute(r#type))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsHasException(hasexception: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsHasException(hasexception: *mut bool) -> JsErrorCode;
    }
    JsHasException(::core::mem::transmute(hasexception))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsHasExternalData(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsHasExternalData(object: *const ::core::ffi::c_void, value: *mut bool) -> JsErrorCode;
    }
    JsHasExternalData(::core::mem::transmute(object), ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsHasIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsHasIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    }
    JsHasIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsHasProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsHasProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode;
    }
    JsHasProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(hasproperty))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsIdle(nextidletick: *mut u32) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsIdle(nextidletick: *mut u32) -> JsErrorCode;
    }
    JsIdle(::core::mem::transmute(nextidletick))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsIntToNumber(intvalue: i32, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsIntToNumber(intvalue: i32, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsIntToNumber(intvalue, ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode;
    }
    JsIsEnumeratingHeap(::core::mem::transmute(isenumeratingheap))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsIsRuntimeExecutionDisabled(runtime: *const ::core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsIsRuntimeExecutionDisabled(runtime: *const ::core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode;
    }
    JsIsRuntimeExecutionDisabled(::core::mem::transmute(runtime), ::core::mem::transmute(isdisabled))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsMemoryAllocationCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsMemoryEventType(pub i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryAllocate: JsMemoryEventType = JsMemoryEventType(0i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryFree: JsMemoryEventType = JsMemoryEventType(1i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryFailure: JsMemoryEventType = JsMemoryEventType(2i32);
impl ::core::marker::Copy for JsMemoryEventType {}
impl ::core::clone::Clone for JsMemoryEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsMemoryEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JsMemoryEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsMemoryEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsMemoryEventType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsNativeFunction = ::core::option::Option<unsafe extern "system" fn(callee: *const ::core::ffi::c_void, isconstructcall: bool, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, callbackstate: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsNumberToDouble(value: *const ::core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsNumberToDouble(value: *const ::core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode;
    }
    JsNumberToDouble(::core::mem::transmute(value), ::core::mem::transmute(doublevalue))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsParseScript<'a, P0, P1>(script: P0, sourcecontext: usize, sourceurl: P1, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsParseScript(script: ::windows::core::PCWSTR, sourcecontext: usize, sourceurl: ::windows::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsParseScript(script.into(), sourcecontext, sourceurl.into(), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsParseSerializedScript<'a, P0, P1>(script: P0, buffer: *const u8, sourcecontext: usize, sourceurl: P1, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsParseSerializedScript(script: ::windows::core::PCWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: ::windows::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsParseSerializedScript(script.into(), ::core::mem::transmute(buffer), sourcecontext, sourceurl.into(), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsPointerToString(stringvalue: &[u16], value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsPointerToString(stringvalue: ::windows::core::PCWSTR, stringlength: usize, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsPointerToString(::core::mem::transmute(::windows::core::as_ptr_or_null(stringvalue)), stringvalue.len() as _, ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsPreventExtension(object: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsPreventExtension(object: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsPreventExtension(::core::mem::transmute(object))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsRelease(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsRelease(r#ref: *const ::core::ffi::c_void, count: *mut u32) -> JsErrorCode;
    }
    JsRelease(::core::mem::transmute(r#ref), ::core::mem::transmute(count))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsRunScript<'a, P0, P1>(script: P0, sourcecontext: usize, sourceurl: P1, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsRunScript(script: ::windows::core::PCWSTR, sourcecontext: usize, sourceurl: ::windows::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsRunScript(script.into(), sourcecontext, sourceurl.into(), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsRunSerializedScript<'a, P0, P1>(script: P0, buffer: *const u8, sourcecontext: usize, sourceurl: P1, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsRunSerializedScript(script: ::windows::core::PCWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: ::windows::core::PCWSTR, result: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsRunSerializedScript(script.into(), ::core::mem::transmute(buffer), sourcecontext, sourceurl.into(), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsRuntimeAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = JsRuntimeAttributes(0i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = JsRuntimeAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = JsRuntimeAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = JsRuntimeAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = JsRuntimeAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = JsRuntimeAttributes(16i32);
impl ::core::marker::Copy for JsRuntimeAttributes {}
impl ::core::clone::Clone for JsRuntimeAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsRuntimeAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JsRuntimeAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsRuntimeAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsRuntimeAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsRuntimeVersion(pub i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersion10: JsRuntimeVersion = JsRuntimeVersion(0i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersion11: JsRuntimeVersion = JsRuntimeVersion(1i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersionEdge: JsRuntimeVersion = JsRuntimeVersion(-1i32);
impl ::core::marker::Copy for JsRuntimeVersion {}
impl ::core::clone::Clone for JsRuntimeVersion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsRuntimeVersion {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JsRuntimeVersion {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsRuntimeVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsRuntimeVersion").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSerializeScript<'a, P0>(script: P0, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSerializeScript(script: ::windows::core::PCWSTR, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode;
    }
    JsSerializeScript(script.into(), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetCurrentContext(context: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetCurrentContext(context: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsSetCurrentContext(::core::mem::transmute(context))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetException(exception: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetException(exception: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsSetException(::core::mem::transmute(exception))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetExternalData(object: *const ::core::ffi::c_void, externaldata: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetExternalData(object: *const ::core::ffi::c_void, externaldata: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsSetExternalData(::core::mem::transmute(object), ::core::mem::transmute(externaldata))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetIndexedProperty(object: *const ::core::ffi::c_void, index: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsSetIndexedProperty(::core::mem::transmute(object), ::core::mem::transmute(index), ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, usestrictrules: u8) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetProperty(object: *const ::core::ffi::c_void, propertyid: *const ::core::ffi::c_void, value: *const ::core::ffi::c_void, usestrictrules: u8) -> JsErrorCode;
    }
    JsSetProperty(::core::mem::transmute(object), ::core::mem::transmute(propertyid), ::core::mem::transmute(value), usestrictrules)
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *const ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetPrototype(object: *const ::core::ffi::c_void, prototypeobject: *const ::core::ffi::c_void) -> JsErrorCode;
    }
    JsSetPrototype(::core::mem::transmute(object), ::core::mem::transmute(prototypeobject))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, beforecollectcallback: JsBeforeCollectCallback) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, beforecollectcallback: *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsSetRuntimeBeforeCollectCallback(::core::mem::transmute(runtime), ::core::mem::transmute(callbackstate), ::core::mem::transmute(beforecollectcallback))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, allocationcallback: JsMemoryAllocationCallback) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::core::ffi::c_void, callbackstate: *const ::core::ffi::c_void, allocationcallback: *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsSetRuntimeMemoryAllocationCallback(::core::mem::transmute(runtime), ::core::mem::transmute(callbackstate), ::core::mem::transmute(allocationcallback))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsSetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: usize) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsSetRuntimeMemoryLimit(runtime: *const ::core::ffi::c_void, memorylimit: usize) -> JsErrorCode;
    }
    JsSetRuntimeMemoryLimit(::core::mem::transmute(runtime), memorylimit)
}
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartDebugging<'a, P0>(debugapplication: P0) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Diagnostics::Debug::IDebugApplication64>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsStartDebugging(debugapplication: *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsStartDebugging(debugapplication.into().abi())
}
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartDebugging<'a, P0>(debugapplication: P0) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Diagnostics::Debug::IDebugApplication32>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsStartDebugging(debugapplication: *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsStartDebugging(debugapplication.into().abi())
}
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartProfiling<'a, P0>(callback: P0, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Diagnostics::Debug::IActiveScriptProfilerCallback>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsStartProfiling(callback: *mut ::core::ffi::c_void, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode;
    }
    JsStartProfiling(callback.into().abi(), eventmask, context)
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsStopProfiling(reason: ::windows::core::HRESULT) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsStopProfiling(reason: ::windows::core::HRESULT) -> JsErrorCode;
    }
    JsStopProfiling(reason)
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsStrictEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsStrictEquals(object1: *const ::core::ffi::c_void, object2: *const ::core::ffi::c_void, result: *mut bool) -> JsErrorCode;
    }
    JsStrictEquals(::core::mem::transmute(object1), ::core::mem::transmute(object2), ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[inline]
pub unsafe fn JsStringToPointer(value: *const ::core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsStringToPointer(value: *const ::core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode;
    }
    JsStringToPointer(::core::mem::transmute(value), ::core::mem::transmute(stringvalue), ::core::mem::transmute(stringlength))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsThreadServiceCallback = ::core::option::Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackstate: *const ::core::ffi::c_void) -> bool>;
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn JsValueToVariant(object: *const ::core::ffi::c_void, variant: *mut super::Com::VARIANT) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsValueToVariant(object: *const ::core::ffi::c_void, variant: *mut super::Com::VARIANT) -> JsErrorCode;
    }
    JsValueToVariant(::core::mem::transmute(object), ::core::mem::transmute(variant))
}
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsValueType(pub i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsUndefined: JsValueType = JsValueType(0i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNull: JsValueType = JsValueType(1i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNumber: JsValueType = JsValueType(2i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsString: JsValueType = JsValueType(3i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsBoolean: JsValueType = JsValueType(4i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsObject: JsValueType = JsValueType(5i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsFunction: JsValueType = JsValueType(6i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsError: JsValueType = JsValueType(7i32);
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsArray: JsValueType = JsValueType(8i32);
impl ::core::marker::Copy for JsValueType {}
impl ::core::clone::Clone for JsValueType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsValueType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JsValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsValueType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn JsVariantToValue(variant: *const super::Com::VARIANT, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn JsVariantToValue(variant: *const super::Com::VARIANT, value: *mut *mut ::core::ffi::c_void) -> JsErrorCode;
    }
    JsVariantToValue(::core::mem::transmute(variant), ::core::mem::transmute(value))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
