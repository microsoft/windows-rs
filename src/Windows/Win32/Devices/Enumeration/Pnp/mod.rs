#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
pub const FAULT_INVALID_ACTION: u32 = 401u32;
pub const FAULT_INVALID_ARG: u32 = 402u32;
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSWDEVICE(pub isize);
impl ::std::default::Default for HSWDEVICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HSWDEVICE {}
unsafe impl ::windows::runtime::Abi for HSWDEVICE {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPAddressFamilyControl(::windows::runtime::IUnknown);
impl IUPnPAddressFamilyControl {
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetAddressFamily(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPAddressFamilyControl {
    type Vtable = IUPnPAddressFamilyControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820970360, 26958, 17823, [165, 166, 25, 30, 160, 255, 161, 199]);
}
impl ::std::convert::From<IUPnPAddressFamilyControl> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPAddressFamilyControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPAddressFamilyControl> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPAddressFamilyControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAddressFamilyControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPAsyncResult(::windows::runtime::IUnknown);
impl IUPnPAsyncResult {
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullrequestid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPAsyncResult {
    type Vtable = IUPnPAsyncResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1298529544, 53566, 17012, [156, 139, 221, 141, 2, 140, 134, 68]);
}
impl ::std::convert::From<IUPnPAsyncResult> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPAsyncResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPAsyncResult> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPAsyncResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPAsyncResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPAsyncResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAsyncResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDescriptionDocument(::windows::runtime::IUnknown);
impl IUPnPDescriptionDocument {
    pub unsafe fn ReadyState(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, bstrurl: Param0, punkcallback: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), punkcallback.into_param().abi()).ok()
    }
    pub unsafe fn LoadResult(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RootDevice(&self) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceByUDN<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), bstrudn.into_param().abi(), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDescriptionDocument {
    type Vtable = IUPnPDescriptionDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298959282, 32170, 19614, [149, 149, 127, 130, 237, 32, 109, 30]);
}
impl ::std::convert::From<IUPnPDescriptionDocument> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDescriptionDocument> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IUPnPDescriptionDocument> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IUPnPDescriptionDocument> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plreadystate: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrerror: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppudrootdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDescriptionDocumentCallback(::windows::runtime::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    pub unsafe fn LoadComplete(&self, hrloadresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrloadresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDescriptionDocumentCallback {
    type Vtable = IUPnPDescriptionDocumentCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2000243817, 21638, 16598, [155, 195, 73, 145, 152, 62, 2, 218]);
}
impl ::std::convert::From<IUPnPDescriptionDocumentCallback> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDescriptionDocumentCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDescriptionDocumentCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDescriptionDocumentCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocumentCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrloadresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDevice(::windows::runtime::IUnknown);
impl IUPnPDevice {
    pub unsafe fn IsRootDevice(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn RootDevice(&self) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
    pub unsafe fn ParentDevice(&self) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
    pub unsafe fn HasChildren(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Children(&self) -> ::windows::runtime::Result<IUPnPDevices> {
        let mut result__: <IUPnPDevices as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevices>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueDeviceName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresentationURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ManufacturerName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ManufacturerURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModelName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModelNumber(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModelURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UPC(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SerialNumber(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IconURL<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrencodingformat: Param0, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrencodingformat.into_param().abi(), ::std::mem::transmute(lsizex), ::std::mem::transmute(lsizey), ::std::mem::transmute(lbitdepth), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Services(&self) -> ::windows::runtime::Result<IUPnPServices> {
        let mut result__: <IUPnPServices as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPServices>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDevice {
    type Vtable = IUPnPDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1027920081, 39113, 18569, [172, 209, 249, 214, 116, 191, 34, 33]);
}
impl ::std::convert::From<IUPnPDevice> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IUPnPDevice> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IUPnPDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IUPnPDevice> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IUPnPDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarb: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppudrootdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppuddeviceparent: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarb: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppudchildren: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrencodingformat: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppusservices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceControl(::windows::runtime::IUnknown);
impl IUPnPDeviceControl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrxmldesc: Param0, bstrdeviceidentifier: Param1, bstrinitstring: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), bstrdeviceidentifier.into_param().abi(), bstrinitstring.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetServiceObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0, bstrserviceid: Param1) -> ::windows::runtime::Result<super::super::super::System::Ole::Automation::IDispatch> {
        let mut result__: <super::super::super::System::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), bstrudn.into_param().abi(), bstrserviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceControl {
    type Vtable = IUPnPDeviceControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593786, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::std::convert::From<IUPnPDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrxmldesc: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceControlHttpHeaders(::windows::runtime::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceControlHttpHeaders {
    type Vtable = IUPnPDeviceControlHttpHeaders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593787, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::std::convert::From<IUPnPDeviceControlHttpHeaders> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceControlHttpHeaders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceControlHttpHeaders> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceControlHttpHeaders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlHttpHeaders_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrhttpresponseheaders: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceDocumentAccess(::windows::runtime::IUnknown);
impl IUPnPDeviceDocumentAccess {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceDocumentAccess {
    type Vtable = IUPnPDeviceDocumentAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3883345924, 12935, 16782, [144, 114, 207, 43, 71, 35, 137, 129]);
}
impl ::std::convert::From<IUPnPDeviceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccess) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccess) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocument: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceDocumentAccessEx(::windows::runtime::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocument(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceDocumentAccessEx {
    type Vtable = IUPnPDeviceDocumentAccessEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3300671568, 24952, 19409, [164, 184, 99, 152, 50, 31, 50, 71]);
}
impl ::std::convert::From<IUPnPDeviceDocumentAccessEx> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccessEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceDocumentAccessEx> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccessEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocument: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceFinder(::windows::runtime::IUnknown);
impl IUPnPDeviceFinder {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindByType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrtypeuri: Param0, dwflags: u32) -> ::windows::runtime::Result<IUPnPDevices> {
        let mut result__: <IUPnPDevices as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrtypeuri.into_param().abi(), ::std::mem::transmute(dwflags), &mut result__).from_abi::<IUPnPDevices>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAsyncFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, bstrtypeuri: Param0, dwflags: u32, punkdevicefindercallback: Param2) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrtypeuri.into_param().abi(), ::std::mem::transmute(dwflags), punkdevicefindercallback.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(lfinddata)).ok()
    }
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lfinddata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindByUDN<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), bstrudn.into_param().abi(), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceFinder {
    type Vtable = IUPnPDeviceFinder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2916760917, 28530, 17177, [191, 249, 24, 96, 10, 83, 155, 16]);
}
impl ::std::convert::From<IUPnPDeviceFinder> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceFinder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceFinder> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IUPnPDeviceFinder> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IUPnPDeviceFinder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IUPnPDeviceFinder> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtypeuri: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtypeuri: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: ::windows::runtime::RawPtr, plfinddata: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface(::windows::runtime::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    pub unsafe fn DeviceAddedWithInterface<'a, Param1: ::windows::runtime::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1, pguidinterface: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lfinddata), pdevice.into_param().abi(), ::std::mem::transmute(pguidinterface)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceFinderAddCallbackWithInterface {
    type Vtable = IUPnPDeviceFinderAddCallbackWithInterface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2554199051, 6038, 17631, [137, 117, 202, 84, 91, 98, 14, 229]);
}
impl ::std::convert::From<IUPnPDeviceFinderAddCallbackWithInterface> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceFinderAddCallbackWithInterface> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32, pdevice: ::windows::runtime::RawPtr, pguidinterface: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceFinderCallback(::windows::runtime::IUnknown);
impl IUPnPDeviceFinderCallback {
    pub unsafe fn DeviceAdded<'a, Param1: ::windows::runtime::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lfinddata), pdevice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceRemoved<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, lfinddata: i32, bstrudn: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(lfinddata), bstrudn.into_param().abi()).ok()
    }
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(lfinddata)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceFinderCallback {
    type Vtable = IUPnPDeviceFinderCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1096456266, 34995, 18931, [146, 175, 5, 8, 190, 223, 13, 108]);
}
impl ::std::convert::From<IUPnPDeviceFinderCallback> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceFinderCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceFinderCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceFinderCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32, pdevice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32, bstrudn: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDeviceProvider(::windows::runtime::IUnknown);
impl IUPnPDeviceProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrinitstring: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrinitstring.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceProvider {
    type Vtable = IUPnPDeviceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593784, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::std::convert::From<IUPnPDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrinitstring: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPDevices(::windows::runtime::IUnknown);
impl IUPnPDevices {
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bstrudn.into_param().abi(), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDevices {
    type Vtable = IUPnPDevices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4256959603, 48547, 19558, [172, 79, 242, 217, 111, 218, 214, 140]);
}
impl ::std::convert::From<IUPnPDevices> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDevices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPDevices> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDevices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IUPnPDevices> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IUPnPDevices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IUPnPDevices> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IUPnPDevices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPEventSink(::windows::runtime::IUnknown);
impl IUPnPEventSink {
    pub unsafe fn OnStateChanged(&self, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cchanges), ::std::mem::transmute(rgdispidchanges)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn OnStateChangedSafe<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, varsadispidchanges: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), varsadispidchanges.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPEventSink {
    type Vtable = IUPnPEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593780, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::std::convert::From<IUPnPEventSink> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPEventSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPEventSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varsadispidchanges: ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPEventSource(::windows::runtime::IUnknown);
impl IUPnPEventSource {
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pessubscriber.into_param().abi()).ok()
    }
    pub unsafe fn Unadvise<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pessubscriber.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPEventSource {
    type Vtable = IUPnPEventSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593781, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::std::convert::From<IUPnPEventSource> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPEventSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPEventSource> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPEventSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pessubscriber: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pessubscriber: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPHttpHeaderControl(::windows::runtime::IUnknown);
impl IUPnPHttpHeaderControl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddRequestHeaders<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrhttpheaders: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrhttpheaders.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPHttpHeaderControl {
    type Vtable = IUPnPHttpHeaderControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(67481423, 35676, 17532, [128, 242, 183, 89, 132, 163, 31, 60]);
}
impl ::std::convert::From<IUPnPHttpHeaderControl> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPHttpHeaderControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPHttpHeaderControl> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPHttpHeaderControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPHttpHeaderControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrhttpheaders: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPRegistrar(::windows::runtime::IUnknown);
impl IUPnPRegistrar {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrxmldesc: Param0,
        bstrprogiddevicecontrolclass: Param1,
        bstrinitstring: Param2,
        bstrcontainerid: Param3,
        bstrresourcepath: Param4,
        nlifetime: i32,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::std::mem::transmute(nlifetime), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterRunningDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrxmldesc: Param0,
        punkdevicecontrol: Param1,
        bstrinitstring: Param2,
        bstrresourcepath: Param3,
        nlifetime: i32,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::std::mem::transmute(nlifetime), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrprovidername: Param0,
        bstrprogidproviderclass: Param1,
        bstrinitstring: Param2,
        bstrcontainerid: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), bstrprovidername.into_param().abi(), bstrprogidproviderclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUniqueDeviceName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrtemplateudn: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrtemplateudn.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, bstrdeviceidentifier: Param0, fpermanent: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), fpermanent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrprovidername: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrprovidername.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPRegistrar {
    type Vtable = IUPnPRegistrar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593782, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::std::convert::From<IUPnPRegistrar> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPRegistrar) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPRegistrar> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPRegistrar) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPRegistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPRegistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRegistrar_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrxmldesc: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrprogiddevicecontrolclass: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrinitstring: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrcontainerid: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrresourcepath: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        nlifetime: i32,
        pbstrdeviceidentifier: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrxmldesc: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: ::windows::runtime::RawPtr, bstrinitstring: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovidername: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdeviceidentifier: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdeviceidentifier: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovidername: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPRemoteEndpointInfo(::windows::runtime::IUnknown);
impl IUPnPRemoteEndpointInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDwordValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrvaluename.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), bstrvaluename.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGuidValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), bstrvaluename.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPRemoteEndpointInfo {
    type Vtable = IUPnPRemoteEndpointInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3375282275, 617, 19199, [156, 114, 117, 50, 27, 186, 41, 82]);
}
impl ::std::convert::From<IUPnPRemoteEndpointInfo> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPRemoteEndpointInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPRemoteEndpointInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPRemoteEndpointInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRemoteEndpointInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPReregistrar(::windows::runtime::IUnknown);
impl IUPnPReregistrar {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReregisterDevice<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>,
    >(
        &self,
        bstrdeviceidentifier: Param0,
        bstrxmldesc: Param1,
        bstrprogiddevicecontrolclass: Param2,
        bstrinitstring: Param3,
        bstrcontainerid: Param4,
        bstrresourcepath: Param5,
        nlifetime: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::std::mem::transmute(nlifetime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReregisterRunningDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrdeviceidentifier: Param0,
        bstrxmldesc: Param1,
        punkdevicecontrol: Param2,
        bstrinitstring: Param3,
        bstrresourcepath: Param4,
        nlifetime: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::std::mem::transmute(nlifetime)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPReregistrar {
    type Vtable = IUPnPReregistrar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593783, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::std::convert::From<IUPnPReregistrar> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPReregistrar) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPReregistrar> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPReregistrar) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPReregistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPReregistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPReregistrar_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdeviceidentifier: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrxmldesc: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrprogiddevicecontrolclass: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrinitstring: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrcontainerid: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrresourcepath: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        nlifetime: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdeviceidentifier: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: ::windows::runtime::RawPtr, bstrinitstring: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPService(::windows::runtime::IUnknown);
impl IUPnPService {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QueryStateVariable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvariablename: Param0) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrvariablename.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn InvokeAction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, bstractionname: Param0, vinactionargs: Param1, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), ::std::mem::transmute(pvoutactionargs), ::std::mem::transmute(pvretval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeIdentifier(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AddCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), punkcallback.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn LastTransportStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPService {
    type Vtable = IUPnPService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2727674268, 56421, 18397, [144, 220, 127, 233, 24, 161, 171, 68]);
}
impl ::std::convert::From<IUPnPService> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPService) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPService> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPService) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IUPnPService> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IUPnPService) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IUPnPService> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IUPnPService) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvariablename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstractionname: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvretval: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plvalue: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPServiceAsync(::windows::runtime::IUnknown);
impl IUPnPServiceAsync {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn BeginInvokeAction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, bstractionname: Param0, vinactionargs: Param1, pasyncresult: Param2) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullrequestid), ::std::mem::transmute(pvoutactionargs), ::std::mem::transmute(pvretval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginQueryStateVariable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, bstrvariablename: Param0, pasyncresult: Param1) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), bstrvariablename.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullrequestid), ::std::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn BeginSubscribeToEvents<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, punkcallback: Param0, pasyncresult: Param1) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), punkcallback.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullrequestid)).ok()
    }
    pub unsafe fn BeginSCPDDownload<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, pasyncresult: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullrequestid), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullrequestid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceAsync {
    type Vtable = IUPnPServiceAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(160160501, 24257, 18919, [162, 96, 179, 161, 29, 216, 104, 12]);
}
impl ::std::convert::From<IUPnPServiceAsync> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPServiceAsync> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPServiceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstractionname: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64, pvoutactionargs: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvretval: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvariablename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64, pvalue: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkcallback: ::windows::runtime::RawPtr, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64, pbstrscpddoc: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPServiceCallback(::windows::runtime::IUnknown);
impl IUPnPServiceCallback {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn StateVariableChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPService>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, pus: Param0, pcwszstatevarname: Param1, vavalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pus.into_param().abi(), pcwszstatevarname.into_param().abi(), vavalue.into_param().abi()).ok()
    }
    pub unsafe fn ServiceInstanceDied<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPService>>(&self, pus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pus.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceCallback {
    type Vtable = IUPnPServiceCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(838524073, 43891, 17995, [182, 125, 92, 29, 15, 131, 200, 184]);
}
impl ::std::convert::From<IUPnPServiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPServiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPServiceCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pus: ::windows::runtime::RawPtr, pcwszstatevarname: super::super::super::Foundation::PWSTR, vavalue: ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPServiceDocumentAccess(::windows::runtime::IUnknown);
impl IUPnPServiceDocumentAccess {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocument(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceDocumentAccess {
    type Vtable = IUPnPServiceDocumentAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(563107113, 2654, 17801, [130, 93, 126, 109, 135, 234, 105, 152]);
}
impl ::std::convert::From<IUPnPServiceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceDocumentAccess) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPServiceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceDocumentAccess) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceDocumentAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocurl: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdoc: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPServiceEnumProperty(::windows::runtime::IUnknown);
impl IUPnPServiceEnumProperty {
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmask)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceEnumProperty {
    type Vtable = IUPnPServiceEnumProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(948386615, 37307, 18932, [178, 73, 46, 142, 251, 184, 168, 22]);
}
impl ::std::convert::From<IUPnPServiceEnumProperty> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceEnumProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPServiceEnumProperty> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceEnumProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceEnumProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmask: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUPnPServices(::windows::runtime::IUnknown);
impl IUPnPServices {
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrserviceid: Param0) -> ::windows::runtime::Result<IUPnPService> {
        let mut result__: <IUPnPService as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bstrserviceid.into_param().abi(), &mut result__).from_abi::<IUPnPService>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServices {
    type Vtable = IUPnPServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1066176158, 39546, 19912, [188, 65, 255, 49, 250, 55, 73, 86]);
}
impl ::std::convert::From<IUPnPServices> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUPnPServices> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IUPnPServices> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: IUPnPServices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IUPnPServices> for super::super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IUPnPServices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::Automation::IDispatch> for &IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrserviceid: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(0i32);
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(1i32);
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(2i32);
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(4i32);
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(8i32);
impl ::std::convert::From<i32> for SW_DEVICE_CAPABILITIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SW_DEVICE_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type SW_DEVICE_CREATE_CALLBACK = unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: ::windows::runtime::HRESULT, pcontext: *const ::std::ffi::c_void, pszdeviceinstanceid: super::super::super::Foundation::PWSTR);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: super::super::super::Foundation::PWSTR,
    pub pszzHardwareIds: super::super::super::Foundation::PWSTR,
    pub pszzCompatibleIds: super::super::super::Foundation::PWSTR,
    pub pContainerId: *mut ::windows::runtime::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: super::super::super::Foundation::PWSTR,
    pub pszDeviceLocation: super::super::super::Foundation::PWSTR,
    pub pSecurityDescriptor: *mut super::super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for SW_DEVICE_CREATE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SW_DEVICE_CREATE_INFO")
            .field("cbSize", &self.cbSize)
            .field("pszInstanceId", &self.pszInstanceId)
            .field("pszzHardwareIds", &self.pszzHardwareIds)
            .field("pszzCompatibleIds", &self.pszzCompatibleIds)
            .field("pContainerId", &self.pContainerId)
            .field("CapabilityFlags", &self.CapabilityFlags)
            .field("pszDeviceDescription", &self.pszDeviceDescription)
            .field("pszDeviceLocation", &self.pszDeviceLocation)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for SW_DEVICE_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszInstanceId == other.pszInstanceId && self.pszzHardwareIds == other.pszzHardwareIds && self.pszzCompatibleIds == other.pszzCompatibleIds && self.pContainerId == other.pContainerId && self.CapabilityFlags == other.CapabilityFlags && self.pszDeviceDescription == other.pszDeviceDescription && self.pszDeviceLocation == other.pszDeviceLocation && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for SW_DEVICE_CREATE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SW_DEVICE_LIFETIME(pub i32);
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(0i32);
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(1i32);
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(2i32);
impl ::std::convert::From<i32> for SW_DEVICE_LIFETIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SW_DEVICE_LIFETIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn SwDeviceClose<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceClose(hswdevice: HSWDEVICE);
        }
        ::std::mem::transmute(SwDeviceClose(hswdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SwDeviceCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    pszenumeratorname: Param0,
    pszparentdeviceinstance: Param1,
    pcreateinfo: *const SW_DEVICE_CREATE_INFO,
    cpropertycount: u32,
    pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY,
    pcallback: ::std::option::Option<SW_DEVICE_CREATE_CALLBACK>,
    pcontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceCreate(pszenumeratorname: super::super::super::Foundation::PWSTR, pszparentdeviceinstance: super::super::super::Foundation::PWSTR, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY, pcallback: ::windows::runtime::RawPtr, pcontext: *const ::std::ffi::c_void, phswdevice: *mut isize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SwDeviceCreate(pszenumeratorname.into_param().abi(), pszparentdeviceinstance.into_param().abi(), ::std::mem::transmute(pcreateinfo), ::std::mem::transmute(cpropertycount), ::std::mem::transmute(pproperties), ::std::mem::transmute(pcallback), ::std::mem::transmute(pcontext), &mut result__).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SwDeviceGetLifetime<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) -> ::windows::runtime::Result<SW_DEVICE_LIFETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceGetLifetime(hswdevice: HSWDEVICE, plifetime: *mut SW_DEVICE_LIFETIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <SW_DEVICE_LIFETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SwDeviceGetLifetime(hswdevice.into_param().abi(), &mut result__).from_abi::<SW_DEVICE_LIFETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hswdevice: Param0, pszdeviceinterfaceid: Param1, cpropertycount: u32, pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfacePropertySet(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY) -> ::windows::runtime::HRESULT;
        }
        SwDeviceInterfacePropertySet(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), ::std::mem::transmute(cpropertycount), ::std::mem::transmute(pproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(
    hswdevice: Param0,
    pinterfaceclassguid: *const ::windows::runtime::GUID,
    pszreferencestring: Param2,
    cpropertycount: u32,
    pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY,
    fenabled: Param5,
) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfaceRegister(hswdevice: HSWDEVICE, pinterfaceclassguid: *const ::windows::runtime::GUID, pszreferencestring: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY, fenabled: super::super::super::Foundation::BOOL, ppszdeviceinterfaceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SwDeviceInterfaceRegister(hswdevice.into_param().abi(), ::std::mem::transmute(pinterfaceclassguid), pszreferencestring.into_param().abi(), ::std::mem::transmute(cpropertycount), ::std::mem::transmute(pproperties), fenabled.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwDeviceInterfaceSetState<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(hswdevice: Param0, pszdeviceinterfaceid: Param1, fenabled: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfaceSetState(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, fenabled: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        SwDeviceInterfaceSetState(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), fenabled.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SwDevicePropertySet<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, cpropertycount: u32, pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDevicePropertySet(hswdevice: HSWDEVICE, cpropertycount: u32, pproperties: *const super::super::super::System::SystemServices::DEVPROPERTY) -> ::windows::runtime::HRESULT;
        }
        SwDevicePropertySet(hswdevice.into_param().abi(), ::std::mem::transmute(cpropertycount), ::std::mem::transmute(pproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SwDeviceSetLifetime<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, lifetime: SW_DEVICE_LIFETIME) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> ::windows::runtime::HRESULT;
        }
        SwDeviceSetLifetime(hswdevice.into_param().abi(), ::std::mem::transmute(lifetime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SwMemFree(pmem: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwMemFree(pmem: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(SwMemFree(::std::mem::transmute(pmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220976i32 as _);
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220736i32 as _);
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220991i32 as _);
pub const UPNP_E_DEVICE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220972i32 as _);
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220988i32 as _);
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180494i32 as _);
pub const UPNP_E_DEVICE_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180495i32 as _);
pub const UPNP_E_DEVICE_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220969i32 as _);
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180511i32 as _);
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180510i32 as _);
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220970i32 as _);
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220223i32 as _);
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220987i32 as _);
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220986i32 as _);
pub const UPNP_E_INVALID_ACTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220985i32 as _);
pub const UPNP_E_INVALID_ARGUMENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220984i32 as _);
pub const UPNP_E_INVALID_DESCRIPTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180509i32 as _);
pub const UPNP_E_INVALID_DOCUMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220224i32 as _);
pub const UPNP_E_INVALID_ICON: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180507i32 as _);
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180505i32 as _);
pub const UPNP_E_INVALID_SERVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180508i32 as _);
pub const UPNP_E_INVALID_VARIABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220973i32 as _);
pub const UPNP_E_INVALID_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180506i32 as _);
pub const UPNP_E_OUT_OF_SYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220983i32 as _);
pub const UPNP_E_PROTOCOL_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220971i32 as _);
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180512i32 as _);
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220992i32 as _);
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220990i32 as _);
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220989i32 as _);
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180504i32 as _);
pub const UPNP_E_TRANSPORT_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220975i32 as _);
pub const UPNP_E_URLBASE_PRESENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180503i32 as _);
pub const UPNP_E_VALUE_TOO_LONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180496i32 as _);
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220974i32 as _);
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
pub const UPnPDescriptionDocument: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(495622983, 14888, 19682, [138, 75, 189, 52, 228, 91, 206, 235]);
pub const UPnPDescriptionDocumentEx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(872220003, 55322, 17299, [131, 204, 1, 149, 177, 218, 47, 145]);
pub const UPnPDevice: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2737132229, 47713, 17786, [181, 154, 162, 86, 30, 18, 94, 51]);
pub const UPnPDeviceFinder: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3792199464, 65207, 16458, [184, 231, 230, 89, 189, 234, 170, 2]);
pub const UPnPDeviceFinderEx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(404444412, 14347, 19061, [179, 241, 74, 196, 94, 150, 5, 176]);
pub const UPnPDevices: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3119009789, 44348, 16548, [184, 53, 8, 130, 235, 203, 170, 168]);
pub const UPnPRegistrar: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593785, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
pub const UPnPRemoteEndpointInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(777946345, 16457, 16964, [183, 40, 45, 36, 34, 113, 87, 199]);
pub const UPnPService: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3324295829, 64459, 17417, [140, 3, 140, 206, 236, 83, 62, 241]);
pub const UPnPServices: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3233565514, 41990, 20220, [147, 47, 184, 84, 107, 129, 0, 204]);
