#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintManagerInterop(::windows_core::IUnknown);
impl IPrintManagerInterop {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<P0, T>(&self, appwindow: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPrintManagerInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IPrintManagerInterop {
    type Vtable = IPrintManagerInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintManagerInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5435a42_8d43_4e7b_a68a_ef311e392087);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintWorkflowConfigurationNative(::windows_core::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[doc = "Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn PrinterQueue(&self) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterQueue> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrinterQueue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn DriverProperties(&self) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DriverProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn UserProperties(&self) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPrintWorkflowConfigurationNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowConfigurationNative {
    type Vtable = IPrintWorkflowConfigurationNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowConfigurationNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc056be0a_9ee2_450a_9823_964f0006f2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfigurationNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub PrinterQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    PrinterQueue: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub DriverProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    DriverProperties: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub UserProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    UserProperties: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(::windows_core::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    pub unsafe fn StartXpsOMGeneration<P0>(&self, receiver: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IPrintWorkflowXpsReceiver>,
    {
        (::windows_core::Interface::vtable(self).StartXpsOMGeneration)(::windows_core::Interface::as_raw(self), receiver.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Xps\"`"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn ObjectFactory(&self) -> ::windows_core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ObjectFactory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPrintWorkflowObjectModelSourceFileContentNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelSourceFileContentNative {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowObjectModelSourceFileContentNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68c9e477_993e_4052_8ac6_454eff58db9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub StartXpsOMGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(::windows_core::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[doc = "Required features: `\"Win32_Storage_Xps\"`"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn DocumentPackageTarget(&self) -> ::windows_core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DocumentPackageTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsObjectModelTargetPackageNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    type Vtable = IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintWorkflowXpsReceiver(::windows_core::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<P0>(&self, documentsequenceprintticket: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).SetDocumentSequencePrintTicket)(::windows_core::Interface::as_raw(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    pub unsafe fn SetDocumentSequenceUri<P0>(&self, documentsequenceuri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDocumentSequenceUri)(::windows_core::Interface::as_raw(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<P0, P1>(&self, documentid: u32, documentprintticket: P0, documenturi: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Com::IStream>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddDocumentData)(::windows_core::Interface::as_raw(self), documentid, documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Xps\"`"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<P0, P1>(&self, documentid: u32, pageid: u32, pagereference: P0, pageuri: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::IXpsOMPageReference>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddPage)(::windows_core::Interface::as_raw(self), documentid, pageid, pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsReceiver, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsReceiver {
    type Vtable = IPrintWorkflowXpsReceiver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowXpsReceiver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04097374_77b8_47f6_8167_aae29d4cf84b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentSequencePrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentSequencePrintTicket: usize,
    pub SetDocumentSequenceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDocumentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: *mut ::core::ffi::c_void, documenturi: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDocumentData: usize,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: *mut ::core::ffi::c_void, pageuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintWorkflowXpsReceiver2(::windows_core::IUnknown);
impl IPrintWorkflowXpsReceiver2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<P0>(&self, documentsequenceprintticket: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDocumentSequencePrintTicket)(::windows_core::Interface::as_raw(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    pub unsafe fn SetDocumentSequenceUri<P0>(&self, documentsequenceuri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDocumentSequenceUri)(::windows_core::Interface::as_raw(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<P0, P1>(&self, documentid: u32, documentprintticket: P0, documenturi: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Com::IStream>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.AddDocumentData)(::windows_core::Interface::as_raw(self), documentid, documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Xps\"`"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<P0, P1>(&self, documentid: u32, pageid: u32, pagereference: P0, pageuri: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::IXpsOMPageReference>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.AddPage)(::windows_core::Interface::as_raw(self), documentid, pageid, pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Failed(&self, xpserror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Failed)(::windows_core::Interface::as_raw(self), xpserror).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsReceiver2, ::windows_core::IUnknown, IPrintWorkflowXpsReceiver);
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsReceiver2 {
    type Vtable = IPrintWorkflowXpsReceiver2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowXpsReceiver2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x023bcc0c_dfab_4a61_b074_490c6995580d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver2_Vtbl {
    pub base__: IPrintWorkflowXpsReceiver_Vtbl,
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpserror: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrinting3DManagerInterop(::windows_core::IUnknown);
impl IPrinting3DManagerInterop {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<P0, T>(&self, appwindow: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPrinting3DManagerInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IPrinting3DManagerInterop {
    type Vtable = IPrinting3DManagerInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrinting3DManagerInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ca31010_1484_4587_b26b_dddf9f9caecd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
