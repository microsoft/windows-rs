#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsAddRef();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsBoolToBoolean();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsBooleanToBool();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCallFunction();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCollectGarbage();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConstructObject();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToBoolean();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToNumber();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToObject();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsConvertValueToString();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateArray();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsCreateContext();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateError();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateExternalObject();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateFunction();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateObject();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateRangeError();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateReferenceError();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateRuntime();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateSyntaxError();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateTypeError();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsCreateURIError();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDefineProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDeleteIndexedProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDeleteProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDisableRuntimeExecution();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDisposeRuntime();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsDoubleToNumber();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsEnableRuntimeExecution();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsEnumerateHeap();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsEquals();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetAndClearException();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetCurrentContext();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetExtensionAllowed();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetExternalData();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetFalseValue();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetGlobalObject();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetIndexedProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetNullValue();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetOwnPropertyDescriptor();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetOwnPropertyNames();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetProperty();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsGetPropertyIdFromName();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetPropertyNameFromId();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetPrototype();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetRuntime();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetRuntimeMemoryLimit();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetRuntimeMemoryUsage();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetStringLength();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetTrueValue();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetUndefinedValue();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsGetValueType();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasException();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasExternalData();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasIndexedProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsHasProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIdle();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIntToNumber();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIsEnumeratingHeap();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsIsRuntimeExecutionDisabled();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsNumberToDouble();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsParseScript();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsParseSerializedScript();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsPointerToString();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsPreventExtension();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsRelease();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsRunScript();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsRunSerializedScript();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JsSerializeScript();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetCurrentContext();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetException();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetExternalData();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetIndexedProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetProperty();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetPrototype();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetRuntimeBeforeCollectCallback();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetRuntimeMemoryAllocationCallback();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsSetRuntimeMemoryLimit();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartDebugging();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn JsStartProfiling();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsStopProfiling();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsStrictEquals();
    #[doc = "*Required features: `Win32_System_Js`*"]
    pub fn JsStringToPointer();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsValueToVariant();
    #[doc = "*Required features: `Win32_System_Js`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn JsVariantToValue();
}
