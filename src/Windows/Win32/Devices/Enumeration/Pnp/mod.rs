#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const FAULT_INVALID_ACTION: u32 = 401u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const FAULT_INVALID_ARG: u32 = 402u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSWDEVICE(pub isize);
impl ::core::default::Default for HSWDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HSWDEVICE {}
unsafe impl ::windows::runtime::Abi for HSWDEVICE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPAddressFamilyControl(pub ::windows::runtime::IUnknown);
impl IUPnPAddressFamilyControl {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn GetAddressFamily(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPAddressFamilyControl {
    type Vtable = IUPnPAddressFamilyControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820970360, 26958, 17823, [165, 166, 25, 30, 160, 255, 161, 199]);
}
impl ::core::convert::From<IUPnPAddressFamilyControl> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPAddressFamilyControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPAddressFamilyControl> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPAddressFamilyControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPAsyncResult(pub ::windows::runtime::IUnknown);
impl IUPnPAsyncResult {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPAsyncResult {
    type Vtable = IUPnPAsyncResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1298529544, 53566, 17012, [156, 139, 221, 141, 2, 140, 134, 68]);
}
impl ::core::convert::From<IUPnPAsyncResult> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPAsyncResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPAsyncResult> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPAsyncResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPAsyncResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPAsyncResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDescriptionDocument(pub ::windows::runtime::IUnknown);
impl IUPnPDescriptionDocument {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn ReadyState(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn LoadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, bstrurl: Param0, punkcallback: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrurl.into_param().abi(), punkcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn LoadResult(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn RootDevice(&self) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn DeviceByUDN<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDescriptionDocument {
    type Vtable = IUPnPDescriptionDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298959282, 32170, 19614, [149, 149, 127, 130, 237, 32, 109, 30]);
}
impl ::core::convert::From<IUPnPDescriptionDocument> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDescriptionDocument> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDescriptionDocument> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDescriptionDocument> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plreadystate: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrerror: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppudrootdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDescriptionDocumentCallback(pub ::windows::runtime::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn LoadComplete(&self, hrloadresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrloadresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDescriptionDocumentCallback {
    type Vtable = IUPnPDescriptionDocumentCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2000243817, 21638, 16598, [155, 195, 73, 145, 152, 62, 2, 218]);
}
impl ::core::convert::From<IUPnPDescriptionDocumentCallback> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDescriptionDocumentCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDescriptionDocumentCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDescriptionDocumentCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDevice(pub ::windows::runtime::IUnknown);
impl IUPnPDevice {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn IsRootDevice(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn RootDevice(&self) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn ParentDevice(&self) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn HasChildren(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Children(&self) -> ::windows::runtime::Result<IUPnPDevices> {
        let mut result__: <IUPnPDevices as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPDevices>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn UniqueDeviceName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn PresentationURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn ManufacturerName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn ManufacturerURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn ModelName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn ModelNumber(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn ModelURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn UPC(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn SerialNumber(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn IconURL<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrencodingformat: Param0, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), bstrencodingformat.into_param().abi(), ::core::mem::transmute(lsizex), ::core::mem::transmute(lsizey), ::core::mem::transmute(lbitdepth), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Services(&self) -> ::windows::runtime::Result<IUPnPServices> {
        let mut result__: <IUPnPServices as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUPnPServices>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDevice {
    type Vtable = IUPnPDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1027920081, 39113, 18569, [172, 209, 249, 214, 116, 191, 34, 33]);
}
impl ::core::convert::From<IUPnPDevice> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDevice> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDevice> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarb: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppudrootdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppuddeviceparent: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarb: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppudchildren: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrencodingformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppusservices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceControl(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceControl {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrxmldesc: Param0, bstrdeviceidentifier: Param1, bstrinitstring: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), bstrdeviceidentifier.into_param().abi(), bstrinitstring.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetServiceObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0, bstrserviceid: Param1) -> ::windows::runtime::Result<super::super::super::System::Com::IDispatch> {
        let mut result__: <super::super::super::System::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), bstrserviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceControl {
    type Vtable = IUPnPDeviceControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593786, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::core::convert::From<IUPnPDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceControlHttpHeaders(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceControlHttpHeaders {
    type Vtable = IUPnPDeviceControlHttpHeaders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593787, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::core::convert::From<IUPnPDeviceControlHttpHeaders> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceControlHttpHeaders) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceControlHttpHeaders> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceControlHttpHeaders) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlHttpHeaders_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrhttpresponseheaders: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceDocumentAccess(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceDocumentAccess {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetDocumentURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceDocumentAccess {
    type Vtable = IUPnPDeviceDocumentAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3883345924, 12935, 16782, [144, 114, 207, 43, 71, 35, 137, 129]);
}
impl ::core::convert::From<IUPnPDeviceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocument: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceDocumentAccessEx(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetDocument(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceDocumentAccessEx {
    type Vtable = IUPnPDeviceDocumentAccessEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3300671568, 24952, 19409, [164, 184, 99, 152, 50, 31, 50, 71]);
}
impl ::core::convert::From<IUPnPDeviceDocumentAccessEx> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccessEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceDocumentAccessEx> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccessEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocument: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceFinder(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceFinder {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn FindByType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrtypeuri: Param0, dwflags: u32) -> ::windows::runtime::Result<IUPnPDevices> {
        let mut result__: <IUPnPDevices as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrtypeuri.into_param().abi(), ::core::mem::transmute(dwflags), &mut result__).from_abi::<IUPnPDevices>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn CreateAsyncFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, bstrtypeuri: Param0, dwflags: u32, punkdevicefindercallback: Param2) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrtypeuri.into_param().abi(), ::core::mem::transmute(dwflags), punkdevicefindercallback.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn FindByUDN<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceFinder {
    type Vtable = IUPnPDeviceFinder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2916760917, 28530, 17177, [191, 249, 24, 96, 10, 83, 155, 16]);
}
impl ::core::convert::From<IUPnPDeviceFinder> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceFinder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceFinder> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDeviceFinder> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDeviceFinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDeviceFinder> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: ::windows::runtime::RawPtr, plfinddata: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn DeviceAddedWithInterface<'a, Param1: ::windows::runtime::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1, pguidinterface: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata), pdevice.into_param().abi(), ::core::mem::transmute(pguidinterface)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceFinderAddCallbackWithInterface {
    type Vtable = IUPnPDeviceFinderAddCallbackWithInterface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2554199051, 6038, 17631, [137, 117, 202, 84, 91, 98, 14, 229]);
}
impl ::core::convert::From<IUPnPDeviceFinderAddCallbackWithInterface> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceFinderAddCallbackWithInterface> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceFinderCallback(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceFinderCallback {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn DeviceAdded<'a, Param1: ::windows::runtime::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata), pdevice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn DeviceRemoved<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, lfinddata: i32, bstrudn: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata), bstrudn.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceFinderCallback {
    type Vtable = IUPnPDeviceFinderCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1096456266, 34995, 18931, [146, 175, 5, 8, 190, 223, 13, 108]);
}
impl ::core::convert::From<IUPnPDeviceFinderCallback> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceFinderCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceFinderCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceFinderCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32, pdevice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfinddata: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDeviceProvider(pub ::windows::runtime::IUnknown);
impl IUPnPDeviceProvider {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Start<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrinitstring: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrinitstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDeviceProvider {
    type Vtable = IUPnPDeviceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593784, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::core::convert::From<IUPnPDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDeviceProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDeviceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPDevices(pub ::windows::runtime::IUnknown);
impl IUPnPDevices {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::runtime::Result<IUPnPDevice> {
        let mut result__: <IUPnPDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), &mut result__).from_abi::<IUPnPDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPDevices {
    type Vtable = IUPnPDevices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4256959603, 48547, 19558, [172, 79, 242, 217, 111, 218, 214, 140]);
}
impl ::core::convert::From<IUPnPDevices> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPDevices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPDevices> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPDevices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDevices> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDevices> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPEventSink(pub ::windows::runtime::IUnknown);
impl IUPnPEventSink {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn OnStateChanged(&self, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchanges), ::core::mem::transmute(rgdispidchanges)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn OnStateChangedSafe<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, varsadispidchanges: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), varsadispidchanges.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPEventSink {
    type Vtable = IUPnPEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593780, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::core::convert::From<IUPnPEventSink> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, varsadispidchanges: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPEventSource(pub ::windows::runtime::IUnknown);
impl IUPnPEventSource {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pessubscriber.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Unadvise<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pessubscriber.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPEventSource {
    type Vtable = IUPnPEventSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593781, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::core::convert::From<IUPnPEventSource> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPEventSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPEventSource> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPEventSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPHttpHeaderControl(pub ::windows::runtime::IUnknown);
impl IUPnPHttpHeaderControl {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn AddRequestHeaders<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrhttpheaders: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrhttpheaders.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPHttpHeaderControl {
    type Vtable = IUPnPHttpHeaderControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(67481423, 35676, 17532, [128, 242, 183, 89, 132, 163, 31, 60]);
}
impl ::core::convert::From<IUPnPHttpHeaderControl> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPHttpHeaderControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPHttpHeaderControl> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPHttpHeaderControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPHttpHeaderControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrhttpheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPRegistrar(pub ::windows::runtime::IUnknown);
impl IUPnPRegistrar {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn RegisterDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrxmldesc: Param0,
        bstrprogiddevicecontrolclass: Param1,
        bstrinitstring: Param2,
        bstrcontainerid: Param3,
        bstrresourcepath: Param4,
        nlifetime: i32,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn RegisterRunningDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrxmldesc: Param0,
        punkdevicecontrol: Param1,
        bstrinitstring: Param2,
        bstrresourcepath: Param3,
        nlifetime: i32,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrprovidername: Param0,
        bstrprogidproviderclass: Param1,
        bstrinitstring: Param2,
        bstrcontainerid: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrprovidername.into_param().abi(), bstrprogidproviderclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueDeviceName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrtemplateudn: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrtemplateudn.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn UnregisterDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, bstrdeviceidentifier: Param0, fpermanent: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), fpermanent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrprovidername: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrprovidername.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPRegistrar {
    type Vtable = IUPnPRegistrar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593782, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::core::convert::From<IUPnPRegistrar> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPRegistrar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPRegistrar> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPRegistrar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPRegistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPRegistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
        bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        nlifetime: i32,
        pbstrdeviceidentifier: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        punkdevicecontrol: ::windows::runtime::RawPtr,
        bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        nlifetime: i32,
        pbstrdeviceidentifier: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPRemoteEndpointInfo(pub ::windows::runtime::IUnknown);
impl IUPnPRemoteEndpointInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetDwordValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrvaluename.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetStringValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrvaluename.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetGuidValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrvaluename.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPRemoteEndpointInfo {
    type Vtable = IUPnPRemoteEndpointInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3375282275, 617, 19199, [156, 114, 117, 50, 27, 186, 41, 82]);
}
impl ::core::convert::From<IUPnPRemoteEndpointInfo> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPRemoteEndpointInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPRemoteEndpointInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPRemoteEndpointInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRemoteEndpointInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPReregistrar(pub ::windows::runtime::IUnknown);
impl IUPnPReregistrar {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
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
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn ReregisterRunningDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(
        &self,
        bstrdeviceidentifier: Param0,
        bstrxmldesc: Param1,
        punkdevicecontrol: Param2,
        bstrinitstring: Param3,
        bstrresourcepath: Param4,
        nlifetime: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPReregistrar {
    type Vtable = IUPnPReregistrar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541593783, 29618, 4564, [191, 66, 0, 176, 208, 17, 139, 86]);
}
impl ::core::convert::From<IUPnPReregistrar> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPReregistrar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPReregistrar> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPReregistrar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPReregistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPReregistrar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
        bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
        nlifetime: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: ::windows::runtime::RawPtr, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPService(pub ::windows::runtime::IUnknown);
impl IUPnPService {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn QueryStateVariable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvariablename: Param0) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrvariablename.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn InvokeAction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, bstractionname: Param0, vinactionargs: Param1, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), ::core::mem::transmute(pvoutactionargs), ::core::mem::transmute(pvretval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn ServiceTypeIdentifier(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn AddCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), punkcallback.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn LastTransportStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPService {
    type Vtable = IUPnPService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2727674268, 56421, 18397, [144, 220, 127, 233, 24, 161, 171, 68]);
}
impl ::core::convert::From<IUPnPService> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPService> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPService> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPService> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvretval: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plvalue: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPServiceAsync(pub ::windows::runtime::IUnknown);
impl IUPnPServiceAsync {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn BeginInvokeAction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, bstractionname: Param0, vinactionargs: Param1, pasyncresult: Param2) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(pvoutactionargs), ::core::mem::transmute(pvretval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn BeginQueryStateVariable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, bstrvariablename: Param0, pasyncresult: Param1) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrvariablename.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn BeginSubscribeToEvents<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, punkcallback: Param0, pasyncresult: Param1) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), punkcallback.into_param().abi(), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn BeginSCPDDownload<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPAsyncResult>>(&self, pasyncresult: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pasyncresult.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceAsync {
    type Vtable = IUPnPServiceAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(160160501, 24257, 18919, [162, 96, 179, 161, 29, 216, 104, 12]);
}
impl ::core::convert::From<IUPnPServiceAsync> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceAsync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPServiceAsync> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceAsync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPServiceAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64, pvoutactionargs: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvretval: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64, pvalue: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkcallback: ::windows::runtime::RawPtr, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pasyncresult: ::windows::runtime::RawPtr, pullrequestid: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64, pbstrscpddoc: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullrequestid: u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPServiceCallback(pub ::windows::runtime::IUnknown);
impl IUPnPServiceCallback {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn StateVariableChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPService>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, pus: Param0, pcwszstatevarname: Param1, vavalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pus.into_param().abi(), pcwszstatevarname.into_param().abi(), vavalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn ServiceInstanceDied<'a, Param0: ::windows::runtime::IntoParam<'a, IUPnPService>>(&self, pus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pus.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceCallback {
    type Vtable = IUPnPServiceCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(838524073, 43891, 17995, [182, 125, 92, 29, 15, 131, 200, 184]);
}
impl ::core::convert::From<IUPnPServiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPServiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPServiceCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pus: ::windows::runtime::RawPtr, pcwszstatevarname: super::super::super::Foundation::PWSTR, vavalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPServiceDocumentAccess(pub ::windows::runtime::IUnknown);
impl IUPnPServiceDocumentAccess {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetDocumentURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn GetDocument(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceDocumentAccess {
    type Vtable = IUPnPServiceDocumentAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(563107113, 2654, 17801, [130, 93, 126, 109, 135, 234, 105, 152]);
}
impl ::core::convert::From<IUPnPServiceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceDocumentAccess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPServiceDocumentAccess> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceDocumentAccess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceDocumentAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocurl: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdoc: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPServiceEnumProperty(pub ::windows::runtime::IUnknown);
impl IUPnPServiceEnumProperty {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServiceEnumProperty {
    type Vtable = IUPnPServiceEnumProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(948386615, 37307, 18932, [178, 73, 46, 142, 251, 184, 168, 22]);
}
impl ::core::convert::From<IUPnPServiceEnumProperty> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServiceEnumProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPServiceEnumProperty> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServiceEnumProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPServices(pub ::windows::runtime::IUnknown);
impl IUPnPServices {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrserviceid: Param0) -> ::windows::runtime::Result<IUPnPService> {
        let mut result__: <IUPnPService as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrserviceid.into_param().abi(), &mut result__).from_abi::<IUPnPService>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPServices {
    type Vtable = IUPnPServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1066176158, 39546, 19912, [188, 65, 255, 49, 250, 55, 73, 86]);
}
impl ::core::convert::From<IUPnPServices> for ::windows::runtime::IUnknown {
    fn from(value: IUPnPServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPServices> for ::windows::runtime::IUnknown {
    fn from(value: &IUPnPServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPServices> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPServices> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(0i32);
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(1i32);
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(2i32);
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(4i32);
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(8i32);
impl ::core::convert::From<i32> for SW_DEVICE_CAPABILITIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SW_DEVICE_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SW_DEVICE_CREATE_CALLBACK = unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: ::windows::runtime::HRESULT, pcontext: *const ::core::ffi::c_void, pszdeviceinstanceid: super::super::super::Foundation::PWSTR);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`, `Win32_Security`*"]
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
impl ::core::default::Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for SW_DEVICE_CREATE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for SW_DEVICE_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszInstanceId == other.pszInstanceId && self.pszzHardwareIds == other.pszzHardwareIds && self.pszzCompatibleIds == other.pszzCompatibleIds && self.pContainerId == other.pContainerId && self.CapabilityFlags == other.CapabilityFlags && self.pszDeviceDescription == other.pszDeviceDescription && self.pszDeviceLocation == other.pszDeviceLocation && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for SW_DEVICE_CREATE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SW_DEVICE_LIFETIME(pub i32);
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(0i32);
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(1i32);
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(2i32);
impl ::core::convert::From<i32> for SW_DEVICE_LIFETIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SW_DEVICE_LIFETIME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[inline]
pub unsafe fn SwDeviceClose<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceClose(hswdevice: HSWDEVICE);
        }
        ::core::mem::transmute(SwDeviceClose(hswdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SwDeviceCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    pszenumeratorname: Param0,
    pszparentdeviceinstance: Param1,
    pcreateinfo: *const SW_DEVICE_CREATE_INFO,
    cpropertycount: u32,
    pproperties: *const super::super::Properties::DEVPROPERTY,
    pcallback: ::core::option::Option<SW_DEVICE_CREATE_CALLBACK>,
    pcontext: *const ::core::ffi::c_void,
) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceCreate(pszenumeratorname: super::super::super::Foundation::PWSTR, pszparentdeviceinstance: super::super::super::Foundation::PWSTR, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, pcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void, phswdevice: *mut isize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        SwDeviceCreate(pszenumeratorname.into_param().abi(), pszparentdeviceinstance.into_param().abi(), ::core::mem::transmute(pcreateinfo), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), &mut result__).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[inline]
pub unsafe fn SwDeviceGetLifetime<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) -> ::windows::runtime::Result<SW_DEVICE_LIFETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceGetLifetime(hswdevice: HSWDEVICE, plifetime: *mut SW_DEVICE_LIFETIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <SW_DEVICE_LIFETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        SwDeviceGetLifetime(hswdevice.into_param().abi(), &mut result__).from_abi::<SW_DEVICE_LIFETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hswdevice: Param0, pszdeviceinterfaceid: Param1, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfacePropertySet(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::runtime::HRESULT;
        }
        SwDeviceInterfacePropertySet(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(
    hswdevice: Param0,
    pinterfaceclassguid: *const ::windows::runtime::GUID,
    pszreferencestring: Param2,
    cpropertycount: u32,
    pproperties: *const super::super::Properties::DEVPROPERTY,
    fenabled: Param5,
) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfaceRegister(hswdevice: HSWDEVICE, pinterfaceclassguid: *const ::windows::runtime::GUID, pszreferencestring: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, fenabled: super::super::super::Foundation::BOOL, ppszdeviceinterfaceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        SwDeviceInterfaceRegister(hswdevice.into_param().abi(), ::core::mem::transmute(pinterfaceclassguid), pszreferencestring.into_param().abi(), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties), fenabled.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDevicePropertySet<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDevicePropertySet(hswdevice: HSWDEVICE, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::runtime::HRESULT;
        }
        SwDevicePropertySet(hswdevice.into_param().abi(), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[inline]
pub unsafe fn SwDeviceSetLifetime<'a, Param0: ::windows::runtime::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, lifetime: SW_DEVICE_LIFETIME) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> ::windows::runtime::HRESULT;
        }
        SwDeviceSetLifetime(hswdevice.into_param().abi(), ::core::mem::transmute(lifetime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
#[inline]
pub unsafe fn SwMemFree(pmem: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwMemFree(pmem: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(SwMemFree(::core::mem::transmute(pmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220976i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220736i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DEVICE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220972i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220988i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180494i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DEVICE_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180495i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DEVICE_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220969i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180511i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180510i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220970i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220223i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220987i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220986i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_ACTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220985i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_ARGUMENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220984i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_DESCRIPTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180509i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_DOCUMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220224i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_ICON: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180507i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180505i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_SERVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180508i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_VARIABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220973i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_INVALID_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180506i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_OUT_OF_SYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220983i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_PROTOCOL_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220971i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180512i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220992i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220990i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220989i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180504i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_TRANSPORT_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220975i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_URLBASE_PRESENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180503i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_VALUE_TOO_LONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180496i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220974i32 as _);
#[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
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
