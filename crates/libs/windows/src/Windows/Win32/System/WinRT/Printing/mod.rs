#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintManagerInterop(::windows::core::IUnknown);
impl IPrintManagerInterop {
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, appwindow: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, appwindow: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPrintManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IPrintManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrintManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IPrintManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintManagerInterop {}
impl ::core::fmt::Debug for IPrintManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintManagerInterop {
    type Vtable = IPrintManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5435a42_8d43_4e7b_a68a_ef311e392087);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowConfigurationNative(::windows::core::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn PrinterQueue(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterQueue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PrinterQueue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Printing::IPrinterQueue>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn DriverProperties(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DriverProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn UserProperties(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowConfigurationNative> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowConfigurationNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowConfigurationNative> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowConfigurationNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintWorkflowConfigurationNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintWorkflowConfigurationNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowConfigurationNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowConfigurationNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowConfigurationNative {}
impl ::core::fmt::Debug for IPrintWorkflowConfigurationNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowConfigurationNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowConfigurationNative {
    type Vtable = IPrintWorkflowConfigurationNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc056be0a_9ee2_450a_9823_964f0006f2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfigurationNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub PrinterQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    PrinterQueue: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub DriverProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    DriverProperties: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub UserProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    UserProperties: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(::windows::core::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
    pub unsafe fn StartXpsOMGeneration<'a, Param0: ::windows::core::IntoParam<'a, IPrintWorkflowXpsReceiver>>(&self, receiver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartXpsOMGeneration)(::core::mem::transmute_copy(self), receiver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn ObjectFactory(&self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ObjectFactory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Storage::Xps::IXpsOMObjectFactory1>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowObjectModelSourceFileContentNative> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowObjectModelSourceFileContentNative> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintWorkflowObjectModelSourceFileContentNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintWorkflowObjectModelSourceFileContentNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContentNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowObjectModelSourceFileContentNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowObjectModelSourceFileContentNative {}
impl ::core::fmt::Debug for IPrintWorkflowObjectModelSourceFileContentNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowObjectModelSourceFileContentNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowObjectModelSourceFileContentNative {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68c9e477_993e_4052_8ac6_454eff58db9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub StartXpsOMGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(::windows::core::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn DocumentPackageTarget(&self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DocumentPackageTarget)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsObjectModelTargetPackageNative {}
impl ::core::fmt::Debug for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsObjectModelTargetPackageNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    type Vtable = IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver(::windows::core::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows::core::IntoParam<'a, super::super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDocumentSequencePrintTicket)(::core::mem::transmute_copy(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, documentsequenceuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDocumentSequenceUri)(::core::mem::transmute_copy(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows::core::IntoParam<'a, super::super::Com::IStream>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDocumentData)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), ::core::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintWorkflowXpsReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintWorkflowXpsReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsReceiver {}
impl ::core::fmt::Debug for IPrintWorkflowXpsReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowXpsReceiver {
    type Vtable = IPrintWorkflowXpsReceiver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04097374_77b8_47f6_8167_aae29d4cf84b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentSequencePrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentSequencePrintTicket: usize,
    pub SetDocumentSequenceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDocumentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: ::windows::core::RawPtr, documenturi: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDocumentData: usize,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: ::windows::core::RawPtr, pageuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver2(::windows::core::IUnknown);
impl IPrintWorkflowXpsReceiver2 {
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows::core::IntoParam<'a, super::super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDocumentSequencePrintTicket)(::core::mem::transmute_copy(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, documentsequenceuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDocumentSequenceUri)(::core::mem::transmute_copy(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows::core::IntoParam<'a, super::super::Com::IStream>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddDocumentData)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), ::core::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
    pub unsafe fn Failed(&self, xpserror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Failed)(::core::mem::transmute_copy(self), ::core::mem::transmute(xpserror)).ok()
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver2> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver2> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintWorkflowXpsReceiver> for IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintWorkflowXpsReceiver> for &'a IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsReceiver2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsReceiver2 {}
impl ::core::fmt::Debug for IPrintWorkflowXpsReceiver2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsReceiver2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowXpsReceiver2 {
    type Vtable = IPrintWorkflowXpsReceiver2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023bcc0c_dfab_4a61_b074_490c6995580d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver2_Vtbl {
    pub base: IPrintWorkflowXpsReceiver_Vtbl,
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpserror: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrinting3DManagerInterop(::windows::core::IUnknown);
impl IPrinting3DManagerInterop {
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, appwindow: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, appwindow: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPrinting3DManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrinting3DManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IPrinting3DManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrinting3DManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrinting3DManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IPrinting3DManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrinting3DManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrinting3DManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrinting3DManagerInterop {}
impl ::core::fmt::Debug for IPrinting3DManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinting3DManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrinting3DManagerInterop {
    type Vtable = IPrinting3DManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ca31010_1484_4587_b26b_dddf9f9caecd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
