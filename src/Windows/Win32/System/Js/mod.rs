#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
#[inline]
pub unsafe fn JsAddRef(r#ref: *const ::std::ffi::c_void, count: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsAddRef(r#ref: *const ::std::ffi::c_void, count: *mut u32) -> JsErrorCode;
        }
        ::std::mem::transmute(JsAddRef(::std::mem::transmute(r#ref), ::std::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type JsBackgroundWorkItemCallback = unsafe extern "system" fn(callbackstate: *const ::std::ffi::c_void);
pub type JsBeforeCollectCallback = unsafe extern "system" fn(callbackstate: *const ::std::ffi::c_void);
#[inline]
pub unsafe fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsBoolToBoolean(::std::mem::transmute(value), ::std::mem::transmute(booleanvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsBooleanToBool(value: *const ::std::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsBooleanToBool(value: *const ::std::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsBooleanToBool(::std::mem::transmute(value), ::std::mem::transmute(boolvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCallFunction(function: *const ::std::ffi::c_void, arguments: *const *const ::std::ffi::c_void, argumentcount: u16, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCallFunction(function: *const ::std::ffi::c_void, arguments: *const *const ::std::ffi::c_void, argumentcount: u16, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCallFunction(::std::mem::transmute(function), ::std::mem::transmute(arguments), ::std::mem::transmute(argumentcount), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCollectGarbage(runtime: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCollectGarbage(runtime: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCollectGarbage(::std::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConstructObject(function: *const ::std::ffi::c_void, arguments: *const *const ::std::ffi::c_void, argumentcount: u16, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConstructObject(function: *const ::std::ffi::c_void, arguments: *const *const ::std::ffi::c_void, argumentcount: u16, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsConstructObject(::std::mem::transmute(function), ::std::mem::transmute(arguments), ::std::mem::transmute(argumentcount), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToBoolean(value: *const ::std::ffi::c_void, booleanvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToBoolean(value: *const ::std::ffi::c_void, booleanvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsConvertValueToBoolean(::std::mem::transmute(value), ::std::mem::transmute(booleanvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToNumber(value: *const ::std::ffi::c_void, numbervalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToNumber(value: *const ::std::ffi::c_void, numbervalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsConvertValueToNumber(::std::mem::transmute(value), ::std::mem::transmute(numbervalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToObject(value: *const ::std::ffi::c_void, object: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToObject(value: *const ::std::ffi::c_void, object: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsConvertValueToObject(::std::mem::transmute(value), ::std::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsConvertValueToString(value: *const ::std::ffi::c_void, stringvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsConvertValueToString(value: *const ::std::ffi::c_void, stringvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsConvertValueToString(::std::mem::transmute(value), ::std::mem::transmute(stringvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateArray(length: u32, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateArray(length: u32, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateArray(::std::mem::transmute(length), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsCreateContext<'a, Param1: ::windows::runtime::IntoParam<'a, super::Diagnostics::Debug::IDebugApplication64>>(runtime: *const ::std::ffi::c_void, debugapplication: Param1, newcontext: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateContext(runtime: *const ::std::ffi::c_void, debugapplication: ::windows::runtime::RawPtr, newcontext: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateContext(::std::mem::transmute(runtime), debugapplication.into_param().abi(), ::std::mem::transmute(newcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateError(::std::mem::transmute(message), ::std::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateExternalObject(data: *const ::std::ffi::c_void, finalizecallback: ::std::option::Option<JsFinalizeCallback>, object: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateExternalObject(data: *const ::std::ffi::c_void, finalizecallback: ::windows::runtime::RawPtr, object: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateExternalObject(::std::mem::transmute(data), ::std::mem::transmute(finalizecallback), ::std::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateFunction(nativefunction: ::std::option::Option<JsNativeFunction>, callbackstate: *const ::std::ffi::c_void, function: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateFunction(nativefunction: ::windows::runtime::RawPtr, callbackstate: *const ::std::ffi::c_void, function: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateFunction(::std::mem::transmute(nativefunction), ::std::mem::transmute(callbackstate), ::std::mem::transmute(function)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateObject(object: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateObject(object: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateObject(::std::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateRangeError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateRangeError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateRangeError(::std::mem::transmute(message), ::std::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateReferenceError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateReferenceError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateReferenceError(::std::mem::transmute(message), ::std::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: ::std::option::Option<JsThreadServiceCallback>, runtime: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: ::windows::runtime::RawPtr, runtime: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateRuntime(::std::mem::transmute(attributes), ::std::mem::transmute(runtimeversion), ::std::mem::transmute(threadservice), ::std::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateSyntaxError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateSyntaxError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateSyntaxError(::std::mem::transmute(message), ::std::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateTypeError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateTypeError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateTypeError(::std::mem::transmute(message), ::std::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsCreateURIError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsCreateURIError(message: *const ::std::ffi::c_void, error: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsCreateURIError(::std::mem::transmute(message), ::std::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDefineProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, propertydescriptor: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDefineProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, propertydescriptor: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsDefineProperty(::std::mem::transmute(object), ::std::mem::transmute(propertyid), ::std::mem::transmute(propertydescriptor), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDeleteIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDeleteIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsDeleteIndexedProperty(::std::mem::transmute(object), ::std::mem::transmute(index)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDeleteProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, usestrictrules: u8, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDeleteProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, usestrictrules: u8, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsDeleteProperty(::std::mem::transmute(object), ::std::mem::transmute(propertyid), ::std::mem::transmute(usestrictrules), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDisableRuntimeExecution(runtime: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDisableRuntimeExecution(runtime: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsDisableRuntimeExecution(::std::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDisposeRuntime(runtime: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDisposeRuntime(runtime: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsDisposeRuntime(::std::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsDoubleToNumber(::std::mem::transmute(doublevalue), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsEnableRuntimeExecution(runtime: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsEnableRuntimeExecution(runtime: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsEnableRuntimeExecution(::std::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsEnumerateHeap(enumerator: *mut ::std::option::Option<super::Diagnostics::Debug::IActiveScriptProfilerHeapEnum>) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsEnumerateHeap(enumerator: *mut ::windows::runtime::RawPtr) -> JsErrorCode;
        }
        ::std::mem::transmute(JsEnumerateHeap(::std::mem::transmute(enumerator)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsEquals(object1: *const ::std::ffi::c_void, object2: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsEquals(object1: *const ::std::ffi::c_void, object2: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsEquals(::std::mem::transmute(object1), ::std::mem::transmute(object2), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for JsErrorCode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JsErrorCode {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for JsErrorCode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for JsErrorCode {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for JsErrorCode {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for JsErrorCode {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for JsErrorCode {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub type JsFinalizeCallback = unsafe extern "system" fn(data: *const ::std::ffi::c_void);
#[inline]
pub unsafe fn JsGetAndClearException(exception: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetAndClearException(exception: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetAndClearException(::std::mem::transmute(exception)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetCurrentContext(currentcontext: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetCurrentContext(currentcontext: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetCurrentContext(::std::mem::transmute(currentcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetExtensionAllowed(object: *const ::std::ffi::c_void, value: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetExtensionAllowed(object: *const ::std::ffi::c_void, value: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetExtensionAllowed(::std::mem::transmute(object), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetExternalData(object: *const ::std::ffi::c_void, externaldata: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetExternalData(object: *const ::std::ffi::c_void, externaldata: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetExternalData(::std::mem::transmute(object), ::std::mem::transmute(externaldata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetFalseValue(falsevalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetFalseValue(falsevalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetFalseValue(::std::mem::transmute(falsevalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetGlobalObject(globalobject: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetGlobalObject(globalobject: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetGlobalObject(::std::mem::transmute(globalobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetIndexedProperty(::std::mem::transmute(object), ::std::mem::transmute(index), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetNullValue(nullvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetNullValue(nullvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetNullValue(::std::mem::transmute(nullvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetOwnPropertyDescriptor(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, propertydescriptor: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetOwnPropertyDescriptor(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, propertydescriptor: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetOwnPropertyDescriptor(::std::mem::transmute(object), ::std::mem::transmute(propertyid), ::std::mem::transmute(propertydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetOwnPropertyNames(object: *const ::std::ffi::c_void, propertynames: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetOwnPropertyNames(object: *const ::std::ffi::c_void, propertynames: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetOwnPropertyNames(::std::mem::transmute(object), ::std::mem::transmute(propertynames)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetProperty(::std::mem::transmute(object), ::std::mem::transmute(propertyid), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsGetPropertyIdFromName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(name: Param0, propertyid: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetPropertyIdFromName(name: super::super::Foundation::PWSTR, propertyid: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetPropertyIdFromName(name.into_param().abi(), ::std::mem::transmute(propertyid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetPropertyNameFromId(propertyid: *const ::std::ffi::c_void, name: *mut *mut u16) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetPropertyNameFromId(propertyid: *const ::std::ffi::c_void, name: *mut *mut u16) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetPropertyNameFromId(::std::mem::transmute(propertyid), ::std::mem::transmute(name)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetPrototype(object: *const ::std::ffi::c_void, prototypeobject: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetPrototype(object: *const ::std::ffi::c_void, prototypeobject: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetPrototype(::std::mem::transmute(object), ::std::mem::transmute(prototypeobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetRuntime(context: *const ::std::ffi::c_void, runtime: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetRuntime(context: *const ::std::ffi::c_void, runtime: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetRuntime(::std::mem::transmute(context), ::std::mem::transmute(runtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryLimit(runtime: *const ::std::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetRuntimeMemoryLimit(runtime: *const ::std::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetRuntimeMemoryLimit(::std::mem::transmute(runtime), ::std::mem::transmute(memorylimit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryUsage(runtime: *const ::std::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetRuntimeMemoryUsage(runtime: *const ::std::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetRuntimeMemoryUsage(::std::mem::transmute(runtime), ::std::mem::transmute(memoryusage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetStringLength(stringvalue: *const ::std::ffi::c_void, length: *mut i32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetStringLength(stringvalue: *const ::std::ffi::c_void, length: *mut i32) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetStringLength(::std::mem::transmute(stringvalue), ::std::mem::transmute(length)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetTrueValue(truevalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetTrueValue(truevalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetTrueValue(::std::mem::transmute(truevalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetUndefinedValue(undefinedvalue: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetUndefinedValue(::std::mem::transmute(undefinedvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsGetValueType(value: *const ::std::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsGetValueType(value: *const ::std::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode;
        }
        ::std::mem::transmute(JsGetValueType(::std::mem::transmute(value), ::std::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasException(hasexception: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasException(hasexception: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsHasException(::std::mem::transmute(hasexception)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasExternalData(object: *const ::std::ffi::c_void, value: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasExternalData(object: *const ::std::ffi::c_void, value: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsHasExternalData(::std::mem::transmute(object), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsHasIndexedProperty(::std::mem::transmute(object), ::std::mem::transmute(index), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsHasProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsHasProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsHasProperty(::std::mem::transmute(object), ::std::mem::transmute(propertyid), ::std::mem::transmute(hasproperty)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIdle(nextidletick: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIdle(nextidletick: *mut u32) -> JsErrorCode;
        }
        ::std::mem::transmute(JsIdle(::std::mem::transmute(nextidletick)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIntToNumber(intvalue: i32, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIntToNumber(intvalue: i32, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsIntToNumber(::std::mem::transmute(intvalue), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsIsEnumeratingHeap(::std::mem::transmute(isenumeratingheap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsIsRuntimeExecutionDisabled(runtime: *const ::std::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsIsRuntimeExecutionDisabled(runtime: *const ::std::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsIsRuntimeExecutionDisabled(::std::mem::transmute(runtime), ::std::mem::transmute(isdisabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type JsMemoryAllocationCallback = unsafe extern "system" fn(callbackstate: *const ::std::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsMemoryEventType(pub i32);
pub const JsMemoryAllocate: JsMemoryEventType = JsMemoryEventType(0i32);
pub const JsMemoryFree: JsMemoryEventType = JsMemoryEventType(1i32);
pub const JsMemoryFailure: JsMemoryEventType = JsMemoryEventType(2i32);
impl ::std::convert::From<i32> for JsMemoryEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JsMemoryEventType {
    type Abi = Self;
    type DefaultType = Self;
}
pub type JsNativeFunction = unsafe extern "system" fn(callee: *const ::std::ffi::c_void, isconstructcall: bool, arguments: *const *const ::std::ffi::c_void, argumentcount: u16, callbackstate: *const ::std::ffi::c_void) -> *mut ::std::ffi::c_void;
#[inline]
pub unsafe fn JsNumberToDouble(value: *const ::std::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsNumberToDouble(value: *const ::std::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode;
        }
        ::std::mem::transmute(JsNumberToDouble(::std::mem::transmute(value), ::std::mem::transmute(doublevalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsParseScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, sourcecontext: usize, sourceurl: Param2, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsParseScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsParseScript(script.into_param().abi(), ::std::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsParseSerializedScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, buffer: *const u8, sourcecontext: usize, sourceurl: Param3, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsParseSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsParseSerializedScript(script.into_param().abi(), ::std::mem::transmute(buffer), ::std::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsPointerToString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(stringvalue: Param0, stringlength: usize, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsPointerToString(stringvalue: super::super::Foundation::PWSTR, stringlength: usize, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsPointerToString(stringvalue.into_param().abi(), ::std::mem::transmute(stringlength), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsPreventExtension(object: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsPreventExtension(object: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsPreventExtension(::std::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsRelease(r#ref: *const ::std::ffi::c_void, count: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsRelease(r#ref: *const ::std::ffi::c_void, count: *mut u32) -> JsErrorCode;
        }
        ::std::mem::transmute(JsRelease(::std::mem::transmute(r#ref), ::std::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsRunScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, sourcecontext: usize, sourceurl: Param2, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsRunScript(script: super::super::Foundation::PWSTR, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsRunScript(script.into_param().abi(), ::std::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsRunSerializedScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, buffer: *const u8, sourcecontext: usize, sourceurl: Param3, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsRunSerializedScript(script: super::super::Foundation::PWSTR, buffer: *const u8, sourcecontext: usize, sourceurl: super::super::Foundation::PWSTR, result: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsRunSerializedScript(script.into_param().abi(), ::std::mem::transmute(buffer), ::std::mem::transmute(sourcecontext), sourceurl.into_param().abi(), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsRuntimeAttributes(pub i32);
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = JsRuntimeAttributes(0i32);
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = JsRuntimeAttributes(1i32);
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = JsRuntimeAttributes(2i32);
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = JsRuntimeAttributes(4i32);
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = JsRuntimeAttributes(8i32);
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = JsRuntimeAttributes(16i32);
impl ::std::convert::From<i32> for JsRuntimeAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JsRuntimeAttributes {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct JsRuntimeVersion(pub i32);
pub const JsRuntimeVersion10: JsRuntimeVersion = JsRuntimeVersion(0i32);
pub const JsRuntimeVersion11: JsRuntimeVersion = JsRuntimeVersion(1i32);
pub const JsRuntimeVersionEdge: JsRuntimeVersion = JsRuntimeVersion(-1i32);
impl ::std::convert::From<i32> for JsRuntimeVersion {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JsRuntimeVersion {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn JsSerializeScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(script: Param0, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSerializeScript(script: super::super::Foundation::PWSTR, buffer: *mut u8, buffersize: *mut u32) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSerializeScript(script.into_param().abi(), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetCurrentContext(context: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetCurrentContext(context: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetCurrentContext(::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetException(exception: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetException(exception: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetException(::std::mem::transmute(exception)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetExternalData(object: *const ::std::ffi::c_void, externaldata: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetExternalData(object: *const ::std::ffi::c_void, externaldata: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetExternalData(::std::mem::transmute(object), ::std::mem::transmute(externaldata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void, value: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetIndexedProperty(object: *const ::std::ffi::c_void, index: *const ::std::ffi::c_void, value: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetIndexedProperty(::std::mem::transmute(object), ::std::mem::transmute(index), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, value: *const ::std::ffi::c_void, usestrictrules: u8) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetProperty(object: *const ::std::ffi::c_void, propertyid: *const ::std::ffi::c_void, value: *const ::std::ffi::c_void, usestrictrules: u8) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetProperty(::std::mem::transmute(object), ::std::mem::transmute(propertyid), ::std::mem::transmute(value), ::std::mem::transmute(usestrictrules)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetPrototype(object: *const ::std::ffi::c_void, prototypeobject: *const ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetPrototype(object: *const ::std::ffi::c_void, prototypeobject: *const ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetPrototype(::std::mem::transmute(object), ::std::mem::transmute(prototypeobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::std::ffi::c_void, callbackstate: *const ::std::ffi::c_void, beforecollectcallback: ::std::option::Option<JsBeforeCollectCallback>) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetRuntimeBeforeCollectCallback(runtime: *const ::std::ffi::c_void, callbackstate: *const ::std::ffi::c_void, beforecollectcallback: ::windows::runtime::RawPtr) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetRuntimeBeforeCollectCallback(::std::mem::transmute(runtime), ::std::mem::transmute(callbackstate), ::std::mem::transmute(beforecollectcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::std::ffi::c_void, callbackstate: *const ::std::ffi::c_void, allocationcallback: ::std::option::Option<JsMemoryAllocationCallback>) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetRuntimeMemoryAllocationCallback(runtime: *const ::std::ffi::c_void, callbackstate: *const ::std::ffi::c_void, allocationcallback: ::windows::runtime::RawPtr) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetRuntimeMemoryAllocationCallback(::std::mem::transmute(runtime), ::std::mem::transmute(callbackstate), ::std::mem::transmute(allocationcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryLimit(runtime: *const ::std::ffi::c_void, memorylimit: usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsSetRuntimeMemoryLimit(runtime: *const ::std::ffi::c_void, memorylimit: usize) -> JsErrorCode;
        }
        ::std::mem::transmute(JsSetRuntimeMemoryLimit(::std::mem::transmute(runtime), ::std::mem::transmute(memorylimit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartDebugging<'a, Param0: ::windows::runtime::IntoParam<'a, super::Diagnostics::Debug::IDebugApplication64>>(debugapplication: Param0) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStartDebugging(debugapplication: ::windows::runtime::RawPtr) -> JsErrorCode;
        }
        ::std::mem::transmute(JsStartDebugging(debugapplication.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn JsStartProfiling<'a, Param0: ::windows::runtime::IntoParam<'a, super::Diagnostics::Debug::IActiveScriptProfilerCallback>>(callback: Param0, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStartProfiling(callback: ::windows::runtime::RawPtr, eventmask: super::Diagnostics::Debug::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode;
        }
        ::std::mem::transmute(JsStartProfiling(callback.into_param().abi(), ::std::mem::transmute(eventmask), ::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsStopProfiling(reason: ::windows::runtime::HRESULT) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStopProfiling(reason: ::windows::runtime::HRESULT) -> JsErrorCode;
        }
        ::std::mem::transmute(JsStopProfiling(::std::mem::transmute(reason)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsStrictEquals(object1: *const ::std::ffi::c_void, object2: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStrictEquals(object1: *const ::std::ffi::c_void, object2: *const ::std::ffi::c_void, result: *mut bool) -> JsErrorCode;
        }
        ::std::mem::transmute(JsStrictEquals(::std::mem::transmute(object1), ::std::mem::transmute(object2), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn JsStringToPointer(value: *const ::std::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsStringToPointer(value: *const ::std::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode;
        }
        ::std::mem::transmute(JsStringToPointer(::std::mem::transmute(value), ::std::mem::transmute(stringvalue), ::std::mem::transmute(stringlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type JsThreadServiceCallback = unsafe extern "system" fn(callback: ::windows::runtime::RawPtr, callbackstate: *const ::std::ffi::c_void) -> bool;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn JsValueToVariant(object: *const ::std::ffi::c_void, variant: *mut super::Com::VARIANT) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsValueToVariant(object: *const ::std::ffi::c_void, variant: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> JsErrorCode;
        }
        ::std::mem::transmute(JsValueToVariant(::std::mem::transmute(object), ::std::mem::transmute(variant)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for JsValueType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JsValueType {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn JsVariantToValue(variant: *const super::Com::VARIANT, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JsVariantToValue(variant: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut *mut ::std::ffi::c_void) -> JsErrorCode;
        }
        ::std::mem::transmute(JsVariantToValue(::std::mem::transmute(variant), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
