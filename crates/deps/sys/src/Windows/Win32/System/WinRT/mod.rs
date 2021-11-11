#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_WinRT_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_System_WinRT_Composition")]
pub mod Composition;
#[cfg(feature = "Win32_System_WinRT_CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Win32_System_WinRT_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_System_WinRT_Display")]
pub mod Display;
#[cfg(feature = "Win32_System_WinRT_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_System_WinRT_Holographic")]
pub mod Holographic;
#[cfg(feature = "Win32_System_WinRT_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_System_WinRT_ML")]
pub mod ML;
#[cfg(feature = "Win32_System_WinRT_Media")]
pub mod Media;
#[cfg(feature = "Win32_System_WinRT_Pdf")]
pub mod Pdf;
#[cfg(feature = "Win32_System_WinRT_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_System_WinRT_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_System_WinRT_Storage")]
pub mod Storage;
#[cfg(feature = "Win32_System_WinRT_Xaml")]
pub mod Xaml;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `System`*"]
    #[cfg(feature = "System")]
    pub fn CreateDispatcherQueueController(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRandomAccessStreamOnFile(filepath: super::super::Foundation::PWSTR, accessmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateRandomAccessStreamOverStream(stream: ::windows::runtime::RawPtr, options: BSOS_OPTIONS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn CreateStreamOverRandomAccessStream(randomaccessstream: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserFree(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserFree64(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> u32;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> u32;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn MetaDataGetDispenser(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoActivateInstance(activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, instance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoCaptureErrorContext(hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoClearError();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoFailFastWithErrorContext(hrerror: ::windows::runtime::HRESULT);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetActivationFactory(activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, iid: *const ::windows::runtime::GUID, factory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, ppagilereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetApartmentIdentifier(apartmentidentifier: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com_Marshal`*"]
    #[cfg(feature = "Win32_System_Com_Marshal")]
    pub fn RoGetBufferMarshaler(buffermarshaler: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetErrorReportingFlags(pflags: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows::runtime::HRESULT, pprestrictederrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoGetParameterizedTypeInstanceIID(nameelementcount: u32, nameelements: *const super::super::Foundation::PWSTR, metadatalocator: ::windows::runtime::RawPtr, iid: *mut ::windows::runtime::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoGetServerActivatableClasses(servername: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, activatableclassids: *mut *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, count: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateError(error: ::windows::runtime::HRESULT, message: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateErrorW(error: ::windows::runtime::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateLanguageException(error: ::windows::runtime::HRESULT, message: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, languageexception: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRegisterActivationFactories(activatableclassids: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRegisterForApartmentShutdown(callbackobject: ::windows::runtime::RawPtr, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoReportFailedDelegate(punkdelegate: ::windows::runtime::RawPtr, prestrictederrorinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoReportUnhandledError(prestrictederrorinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoResolveRestrictedErrorInfoReference(reference: super::super::Foundation::PWSTR, pprestrictederrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoRevokeActivationFactories(cookie: isize);
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoSetErrorReportingFlags(flags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformError(olderror: ::windows::runtime::HRESULT, newerror: ::windows::runtime::HRESULT, message: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformErrorW(olderror: ::windows::runtime::HRESULT, newerror: ::windows::runtime::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoUninitialize();
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn SetRestrictedErrorInfo(prestrictederrorinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsCompareStringOrdinal(string1: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, string2: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsConcatString(string1: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, string2: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateString(sourcestring: super::super::Foundation::PWSTR, length: u32, string: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsCreateStringReference(sourcestring: super::super::Foundation::PWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDeleteString(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDeleteStringBuffer(bufferhandle: HSTRING_BUFFER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsDuplicateString(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsGetStringLen(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> u32;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsGetStringRawBuffer(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, length: *mut u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsInspectString(targethstring: usize, machine: u16, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsInspectString2(targethstring: u64, machine: u16, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsIsStringEmpty(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER, string: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsReplaceString(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, stringreplaced: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, stringreplacewith: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsStringHasEmbeddedNull(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, hasembednull: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsSubstring(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, newstring: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsSubstringWithSpecifiedLength(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, length: u32, newstring: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsTrimStringEnd(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, trimstring: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub fn WindowsTrimStringStart(string: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, trimstring: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
}
