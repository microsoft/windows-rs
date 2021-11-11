#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintManagerInterop(pub ::windows::runtime::IUnknown);
impl IPrintManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`*"]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintManagerInterop {
    type Vtable = IPrintManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc5435a42_8d43_4e7b_a68a_ef311e392087);
}
impl ::core::convert::From<IPrintManagerInterop> for ::windows::runtime::IUnknown {
    fn from(value: IPrintManagerInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintManagerInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintManagerInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintManagerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintManagerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintWorkflowConfigurationNative(pub ::windows::runtime::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[cfg(feature = "Win32_Graphics_Printing")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Graphics_Printing`*"]
    pub unsafe fn PrinterQueue(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Printing::IPrinterQueue> {
        let mut result__: <super::super::super::Graphics::Printing::IPrinterQueue as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Printing::IPrinterQueue>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Printing")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Graphics_Printing`*"]
    pub unsafe fn DriverProperties(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__: <super::super::super::Graphics::Printing::IPrinterPropertyBag as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Printing")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Graphics_Printing`*"]
    pub unsafe fn UserProperties(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__: <super::super::super::Graphics::Printing::IPrinterPropertyBag as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowConfigurationNative {
    type Vtable = IPrintWorkflowConfigurationNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc056be0a_9ee2_450a_9823_964f0006f2bb);
}
impl ::core::convert::From<IPrintWorkflowConfigurationNative> for ::windows::runtime::IUnknown {
    fn from(value: IPrintWorkflowConfigurationNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintWorkflowConfigurationNative> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintWorkflowConfigurationNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintWorkflowConfigurationNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintWorkflowConfigurationNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfigurationNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Printing"))] usize,
    #[cfg(feature = "Win32_Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Printing"))] usize,
    #[cfg(feature = "Win32_Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Printing"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(pub ::windows::runtime::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    #[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
    pub unsafe fn StartXpsOMGeneration<'a, Param0: ::windows::runtime::IntoParam<'a, IPrintWorkflowXpsReceiver>>(&self, receiver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), receiver.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Storage_Xps`*"]
    pub unsafe fn ObjectFactory(&self) -> ::windows::runtime::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1> {
        let mut result__: <super::super::super::Storage::Xps::IXpsOMObjectFactory1 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Storage::Xps::IXpsOMObjectFactory1>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowObjectModelSourceFileContentNative {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x68c9e477_993e_4052_8ac6_454eff58db9d);
}
impl ::core::convert::From<IPrintWorkflowObjectModelSourceFileContentNative> for ::windows::runtime::IUnknown {
    fn from(value: IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintWorkflowObjectModelSourceFileContentNative> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintWorkflowObjectModelSourceFileContentNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintWorkflowObjectModelSourceFileContentNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(pub ::windows::runtime::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[cfg(feature = "Win32_Storage_Xps")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Storage_Xps`*"]
    pub unsafe fn DocumentPackageTarget(&self) -> ::windows::runtime::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        let mut result__: <super::super::super::Storage::Xps::IXpsDocumentPackageTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    type Vtable = IPrintWorkflowXpsObjectModelTargetPackageNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
impl ::core::convert::From<IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows::runtime::IUnknown {
    fn from(value: IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Xps")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintWorkflowXpsReceiver(pub ::windows::runtime::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_System_Com`*"]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`*"]
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, documentsequenceuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    pub unsafe fn AddPage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), ::core::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowXpsReceiver {
    type Vtable = IPrintWorkflowXpsReceiver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x04097374_77b8_47f6_8167_aae29d4cf84b);
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver> for ::windows::runtime::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintWorkflowXpsReceiver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintWorkflowXpsReceiver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceprintticket: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceuri: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, documentprintticket: ::windows::runtime::RawPtr, documenturi: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, pageid: u32, pagereference: ::windows::runtime::RawPtr, pageuri: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintWorkflowXpsReceiver2(pub ::windows::runtime::IUnknown);
impl IPrintWorkflowXpsReceiver2 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_System_Com`*"]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`*"]
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, documentsequenceuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    pub unsafe fn AddPage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(documentid), ::core::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
    pub unsafe fn Failed(&self, xpserror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(xpserror)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowXpsReceiver2 {
    type Vtable = IPrintWorkflowXpsReceiver2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x023bcc0c_dfab_4a61_b074_490c6995580d);
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver2> for ::windows::runtime::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver2> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPrintWorkflowXpsReceiver> for IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintWorkflowXpsReceiver> for &IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceprintticket: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceuri: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, documentprintticket: ::windows::runtime::RawPtr, documenturi: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, pageid: u32, pagereference: ::windows::runtime::RawPtr, pageuri: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpserror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrinting3DManagerInterop(pub ::windows::runtime::IUnknown);
impl IPrinting3DManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Printing`, `Win32_Foundation`*"]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrinting3DManagerInterop {
    type Vtable = IPrinting3DManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9ca31010_1484_4587_b26b_dddf9f9caecd);
}
impl ::core::convert::From<IPrinting3DManagerInterop> for ::windows::runtime::IUnknown {
    fn from(value: IPrinting3DManagerInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrinting3DManagerInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IPrinting3DManagerInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
