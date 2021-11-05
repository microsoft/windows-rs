#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceDiscoveryMechanism(pub i32);
pub const MulticastDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(0i32);
pub const DirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(1i32);
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(2i32);
impl ::std::convert::From<i32> for DeviceDiscoveryMechanism {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceDiscoveryMechanism {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDAddress(pub ::windows::runtime::IUnknown);
impl IWSDAddress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Serialize<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Deserialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDAddress {
    type Vtable = IWSDAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3109506156, 4774, 20340, [147, 161, 51, 24, 255, 96, 87, 89]);
}
impl ::std::convert::From<IWSDAddress> for ::windows::runtime::IUnknown {
    fn from(value: IWSDAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDAddress> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDAsyncCallback(pub ::windows::runtime::IUnknown);
impl IWSDAsyncCallback {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn AsyncOperationComplete<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pasyncresult: Param0, pasyncstate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pasyncresult.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDAsyncCallback {
    type Vtable = IWSDAsyncCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2789085341, 52850, 18914, [186, 152, 232, 69, 245, 238, 22, 102]);
}
impl ::std::convert::From<IWSDAsyncCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWSDAsyncCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDAsyncCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDAsyncCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDAsyncCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDAsyncCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pasyncresult: ::windows::runtime::RawPtr, pasyncstate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDAsyncResult(pub ::windows::runtime::IUnknown);
impl IWSDAsyncResult {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAsyncCallback>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcallback: Param0, pasyncstate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcallback.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetWaitHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hwaithandle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwaithandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn HasCompleted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetAsyncState(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetEvent(&self) -> ::windows::runtime::Result<WSD_EVENT> {
        let mut result__: <WSD_EVENT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WSD_EVENT>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::runtime::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDAsyncResult {
    type Vtable = IWSDAsyncResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(296322346, 36312, 16958, [181, 55, 147, 86, 219, 79, 191, 184]);
}
impl ::std::convert::From<IWSDAsyncResult> for ::windows::runtime::IUnknown {
    fn from(value: IWSDAsyncResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDAsyncResult> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDAsyncResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDAsyncResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDAsyncResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pasyncstate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppasyncstate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pevent: *mut ::std::mem::ManuallyDrop<WSD_EVENT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppendpoint: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDAttachment(pub ::windows::runtime::IUnknown);
impl IWSDAttachment {}
unsafe impl ::windows::runtime::Interface for IWSDAttachment {
    type Vtable = IWSDAttachment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1565894166, 40440, 19209, [177, 86, 155, 163, 81, 164, 139, 118]);
}
impl ::std::convert::From<IWSDAttachment> for ::windows::runtime::IUnknown {
    fn from(value: IWSDAttachment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDAttachment> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDAttachment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDDeviceHost(pub ::windows::runtime::IUnknown);
impl IWSDDeviceHost {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Init<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(&self, pszlocalid: Param0, pcontext: Param1, pphostaddresses: *const ::std::option::Option<IWSDAddress>, dwhostaddresscount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::std::mem::transmute(pphostaddresses), ::std::mem::transmute(dwhostaddresscount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Start<'a, Param2: ::windows::runtime::IntoParam<'a, IWSDDeviceHostNotify>>(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullinstanceid), ::std::mem::transmute(pscopelist), pnotificationsink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Terminate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pporttype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pthismodelmetadata), ::std::mem::transmute(pthisdevicemetadata), ::std::mem::transmute(phostmetadata), ::std::mem::transmute(pcustommetadata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn RegisterService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pszserviceid: Param0, pservice: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi(), pservice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn RetireService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn AddDynamicService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pszserviceid: Param0, pszendpointaddress: Param1, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi(), pszendpointaddress.into_param().abi(), ::std::mem::transmute(pporttype), ::std::mem::transmute(pportname), ::std::mem::transmute(pany), pservice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn RemoveDynamicService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetServiceDiscoverable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszserviceid: Param0, fdiscoverable: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi(), fdiscoverable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SignalEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0, pbody: *const ::std::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi(), ::std::mem::transmute(pbody), ::std::mem::transmute(poperation)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDDeviceHost {
    type Vtable = IWSDDeviceHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2441078929, 15635, 16696, [152, 9, 147, 76, 138, 190, 177, 44]);
}
impl ::std::convert::From<IWSDDeviceHost> for ::windows::runtime::IUnknown {
    fn from(value: IWSDDeviceHost) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDDeviceHost> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDDeviceHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDDeviceHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDDeviceHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, pphostaddresses: *const ::windows::runtime::RawPtr, dwhostaddresscount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pporttype: *const WSD_PORT_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR, pservice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR, pszendpointaddress: super::super::Foundation::PWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR, pbody: *const ::std::ffi::c_void, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDDeviceHostNotify(pub ::windows::runtime::IUnknown);
impl IWSDDeviceHostNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDDeviceHostNotify {
    type Vtable = IWSDDeviceHostNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3049187833, 61146, 16894, [150, 247, 244, 94, 20, 153, 15, 176]);
}
impl ::std::convert::From<IWSDDeviceHostNotify> for ::windows::runtime::IUnknown {
    fn from(value: IWSDDeviceHostNotify) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDDeviceHostNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDDeviceHostNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDDeviceHostNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDDeviceHostNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR, ppservice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDDeviceProxy(pub ::windows::runtime::IUnknown);
impl IWSDDeviceProxy {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Init<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWSDAddress>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, IWSDXMLContext>, Param4: ::windows::runtime::IntoParam<'a, IWSDDeviceProxy>>(&self, pszdeviceid: Param0, pdeviceaddress: Param1, pszlocalid: Param2, pcontext: Param3, psponsor: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), psponsor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), presult.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetHostMetadata(&self) -> ::windows::runtime::Result<*mut WSD_HOST_METADATA> {
        let mut result__: <*mut WSD_HOST_METADATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_HOST_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetThisModelMetadata(&self) -> ::windows::runtime::Result<*mut WSD_THIS_MODEL_METADATA> {
        let mut result__: <*mut WSD_THIS_MODEL_METADATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_THIS_MODEL_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetThisDeviceMetadata(&self) -> ::windows::runtime::Result<*mut WSD_THIS_DEVICE_METADATA> {
        let mut result__: <*mut WSD_THIS_DEVICE_METADATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_THIS_DEVICE_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetAllMetadata(&self) -> ::windows::runtime::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetServiceProxyById<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszserviceid: Param0) -> ::windows::runtime::Result<IWSDServiceProxy> {
        let mut result__: <IWSDServiceProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszserviceid.into_param().abi(), &mut result__).from_abi::<IWSDServiceProxy>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> ::windows::runtime::Result<IWSDServiceProxy> {
        let mut result__: <IWSDServiceProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype), &mut result__).from_abi::<IWSDServiceProxy>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::runtime::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDDeviceProxy {
    type Vtable = IWSDDeviceProxy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4007706673, 50552, 19470, [154, 59, 151, 60, 53, 244, 9, 219]);
}
impl ::std::convert::From<IWSDDeviceProxy> for ::windows::runtime::IUnknown {
    fn from(value: IWSDDeviceProxy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDDeviceProxy> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDDeviceProxy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDDeviceProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDDeviceProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::runtime::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, psponsor: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszserviceid: super::super::Foundation::PWSTR, ppserviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *const WSDXML_NAME, ppserviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDEndpointProxy(pub ::windows::runtime::IUnknown);
impl IWSDEndpointProxy {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SendOneWayRequest(&self, pbody: *const ::std::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbody), ::std::mem::transmute(poperation)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const ::std::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbody), ::std::mem::transmute(poperation), ::std::mem::transmute(presponsecontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SendTwoWayRequestAsync<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, IWSDAsyncCallback>>(&self, pbody: *const ::std::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: Param2, pcallback: Param3) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbody), ::std::mem::transmute(poperation), pasyncstate.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn AbortAsyncOperation<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, pasyncresult: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pasyncresult.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfault)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetErrorInfo(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetFaultInfo(&self) -> ::windows::runtime::Result<*mut WSD_SOAP_FAULT> {
        let mut result__: <*mut WSD_SOAP_FAULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDEndpointProxy {
    type Vtable = IWSDEndpointProxy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408998960, 45644, 18805, [159, 144, 219, 179, 155, 170, 36, 236]);
}
impl ::std::convert::From<IWSDEndpointProxy> for ::windows::runtime::IUnknown {
    fn from(value: IWSDEndpointProxy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDEndpointProxy> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDEndpointProxy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDEndpointProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDEndpointProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbody: *const ::std::ffi::c_void, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbody: *const ::std::ffi::c_void, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, presponsecontext: *const ::std::mem::ManuallyDrop<WSD_SYNCHRONOUS_RESPONSE_CONTEXT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbody: *const ::std::ffi::c_void, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, pasyncstate: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, presult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pasyncresult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszerrorinfo: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDEventingStatus(pub ::windows::runtime::IUnknown);
impl IWSDEventingStatus {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SubscriptionRenewed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubscriptionaction: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszsubscriptionaction.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SubscriptionRenewalFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubscriptionaction: Param0, hr: ::windows::runtime::HRESULT) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszsubscriptionaction.into_param().abi(), ::std::mem::transmute(hr)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SubscriptionEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubscriptionaction: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszsubscriptionaction.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IWSDEventingStatus {
    type Vtable = IWSDEventingStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1236369234, 25466, 16506, [174, 153, 251, 232, 42, 77, 56, 192]);
}
impl ::std::convert::From<IWSDEventingStatus> for ::windows::runtime::IUnknown {
    fn from(value: IWSDEventingStatus) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDEventingStatus> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDEventingStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDEventingStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDEventingStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubscriptionaction: super::super::Foundation::PWSTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubscriptionaction: super::super::Foundation::PWSTR, hr: ::windows::runtime::HRESULT),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubscriptionaction: super::super::Foundation::PWSTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDHttpAddress(pub ::windows::runtime::IUnknown);
impl IWSDHttpAddress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Serialize<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Deserialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetPort(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransportAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsafe: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fsafe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszaddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetSecure(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetSecure<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsecure: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), fsecure.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDHttpAddress {
    type Vtable = IWSDHttpAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3499804605, 10814, 19333, [134, 5, 39, 55, 255, 62, 78, 160]);
}
impl ::std::convert::From<IWSDHttpAddress> for ::windows::runtime::IUnknown {
    fn from(value: IWSDHttpAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDHttpAddress> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDHttpAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDHttpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDHttpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDTransportAddress> for IWSDHttpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDTransportAddress> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDTransportAddress> for &IWSDHttpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDTransportAddress> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IWSDHttpAddress> for IWSDAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDHttpAddress> for IWSDAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAddress> for IWSDHttpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAddress> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAddress> for &IWSDHttpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAddress> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwport: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wport: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszaddress: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsecure: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDHttpAuthParameters(pub ::windows::runtime::IUnknown);
impl IWSDHttpAuthParameters {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetClientAccessToken(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetAuthType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDHttpAuthParameters {
    type Vtable = IWSDHttpAuthParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(189230576, 36268, 18445, [176, 92, 153, 120, 26, 88, 132, 170]);
}
impl ::std::convert::From<IWSDHttpAuthParameters> for ::windows::runtime::IUnknown {
    fn from(value: IWSDHttpAuthParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDHttpAuthParameters> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDHttpAuthParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDHttpAuthParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDHttpAuthParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pauthtype: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDHttpMessageParameters(pub ::windows::runtime::IUnknown);
impl IWSDHttpMessageParameters {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetLocalAddress(&self) -> ::windows::runtime::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::runtime::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetLowerParameters(&self) -> ::windows::runtime::Result<IWSDMessageParameters> {
        let mut result__: <IWSDMessageParameters as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDMessageParameters>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetInboundHttpHeaders<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszheaders: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pszheaders.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetInboundHttpHeaders(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetOutboundHttpHeaders<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszheaders: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszheaders.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetOutboundHttpHeaders(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetID(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetContext(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDHttpMessageParameters {
    type Vtable = IWSDHttpMessageParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1410060578, 23683, 19948, [179, 150, 234, 98, 162, 105, 127, 223]);
}
impl ::std::convert::From<IWSDHttpMessageParameters> for ::windows::runtime::IUnknown {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDHttpMessageParameters> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMessageParameters> for IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMessageParameters> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMessageParameters> for &IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMessageParameters> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paddress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paddress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptxparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszheaders: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszheaders: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDInboundAttachment(pub ::windows::runtime::IUnknown);
impl IWSDInboundAttachment {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Read(&self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer), ::std::mem::transmute(dwbytestoread), ::std::mem::transmute(pdwnumberofbytesread)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDInboundAttachment {
    type Vtable = IWSDInboundAttachment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1540803173, 9020, 20408, [159, 122, 38, 65, 97, 150, 85, 201]);
}
impl ::std::convert::From<IWSDInboundAttachment> for ::windows::runtime::IUnknown {
    fn from(value: IWSDInboundAttachment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDInboundAttachment> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDInboundAttachment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDInboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDInboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: IWSDInboundAttachment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDInboundAttachment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAttachment> for IWSDInboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAttachment> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAttachment> for &IWSDInboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAttachment> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDMessageParameters(pub ::windows::runtime::IUnknown);
impl IWSDMessageParameters {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetLocalAddress(&self) -> ::windows::runtime::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::runtime::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetLowerParameters(&self) -> ::windows::runtime::Result<IWSDMessageParameters> {
        let mut result__: <IWSDMessageParameters as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDMessageParameters>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDMessageParameters {
    type Vtable = IWSDMessageParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(531622050, 59132, 19328, [182, 207, 183, 212, 92, 65, 109, 124]);
}
impl ::std::convert::From<IWSDMessageParameters> for ::windows::runtime::IUnknown {
    fn from(value: IWSDMessageParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDMessageParameters> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDMessageParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paddress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paddress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptxparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDMetadataExchange(pub ::windows::runtime::IUnknown);
impl IWSDMetadataExchange {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetMetadata(&self) -> ::windows::runtime::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDMetadataExchange {
    type Vtable = IWSDMetadataExchange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(110718295, 7527, 18728, [147, 7, 61, 120, 51, 253, 184, 70]);
}
impl ::std::convert::From<IWSDMetadataExchange> for ::windows::runtime::IUnknown {
    fn from(value: IWSDMetadataExchange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDMetadataExchange> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDMetadataExchange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDMetadataExchange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDMetadataExchange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDOutboundAttachment(pub ::windows::runtime::IUnknown);
impl IWSDOutboundAttachment {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Write(&self, pbuffer: *const u8, dwbytestowrite: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer), ::std::mem::transmute(dwbytestowrite), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDOutboundAttachment {
    type Vtable = IWSDOutboundAttachment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2855284621, 23074, 19365, [179, 146, 170, 132, 134, 244, 193, 93]);
}
impl ::std::convert::From<IWSDOutboundAttachment> for ::windows::runtime::IUnknown {
    fn from(value: IWSDOutboundAttachment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDOutboundAttachment> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: IWSDOutboundAttachment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAttachment> for IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAttachment> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAttachment> for &IWSDOutboundAttachment {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAttachment> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDSSLClientCertificate(pub ::windows::runtime::IUnknown);
impl IWSDSSLClientCertificate {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn GetClientCertificate(&self) -> ::windows::runtime::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT> {
        let mut result__: <*mut super::super::Security::Cryptography::CERT_CONTEXT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::Security::Cryptography::CERT_CONTEXT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetMappedAccessToken(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDSSLClientCertificate {
    type Vtable = IWSDSSLClientCertificate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3725614727, 41178, 16782, [152, 173, 39, 185, 238, 216, 123, 220]);
}
impl ::std::convert::From<IWSDSSLClientCertificate> for ::windows::runtime::IUnknown {
    fn from(value: IWSDSSLClientCertificate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDSSLClientCertificate> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDSSLClientCertificate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDSSLClientCertificate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDSSLClientCertificate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDScopeMatchingRule(pub ::windows::runtime::IUnknown);
impl IWSDScopeMatchingRule {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetScopeRule(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn MatchScopes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszscope1: Param0, pszscope2: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszscope1.into_param().abi(), pszscope2.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDScopeMatchingRule {
    type Vtable = IWSDScopeMatchingRule_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4239385636, 65269, 18458, [189, 159, 51, 206, 5, 116, 37, 111]);
}
impl ::std::convert::From<IWSDScopeMatchingRule> for ::windows::runtime::IUnknown {
    fn from(value: IWSDScopeMatchingRule) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDScopeMatchingRule> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDScopeMatchingRule) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDScopeMatchingRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDScopeMatchingRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszscopematchingrule: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszscope1: super::super::Foundation::PWSTR, pszscope2: super::super::Foundation::PWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDServiceMessaging(pub ::windows::runtime::IUnknown);
impl IWSDServiceMessaging {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SendResponse<'a, Param2: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>>(&self, pbody: *const ::std::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbody), ::std::mem::transmute(poperation), pmessageparameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn FaultRequest<'a, Param1: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>>(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: Param1, pfault: *const WSD_SOAP_FAULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(prequestheader), pmessageparameters.into_param().abi(), ::std::mem::transmute(pfault)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDServiceMessaging {
    type Vtable = IWSDServiceMessaging_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2492943604, 3243, 17933, [163, 246, 122, 10, 214, 35, 192, 230]);
}
impl ::std::convert::From<IWSDServiceMessaging> for ::windows::runtime::IUnknown {
    fn from(value: IWSDServiceMessaging) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDServiceMessaging> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDServiceMessaging) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDServiceMessaging {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDServiceMessaging {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbody: *const ::std::ffi::c_void, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, pmessageparameters: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::windows::runtime::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDServiceProxy(pub ::windows::runtime::IUnknown);
impl IWSDServiceProxy {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetMetadata(&self) -> ::windows::runtime::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows::runtime::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), presult.into_param().abi(), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::runtime::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__: <*mut WSD_SERVICE_METADATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SubscribeToOperation<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, poperation: *const WSD_OPERATION, punknown: Param1, pany: *const WSDXML_ELEMENT) -> ::windows::runtime::Result<*mut WSDXML_ELEMENT> {
        let mut result__: <*mut WSDXML_ELEMENT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperation), punknown.into_param().abi(), ::std::mem::transmute(pany), &mut result__).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperation)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetEventingStatusCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDEventingStatus>>(&self, pstatus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::runtime::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDServiceProxy {
    type Vtable = IWSDServiceProxy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3569875868, 939, 16757, [157, 103, 9, 79, 175, 235, 244, 135]);
}
impl ::std::convert::From<IWSDServiceProxy> for ::windows::runtime::IUnknown {
    fn from(value: IWSDServiceProxy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDServiceProxy> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDServiceProxy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDServiceProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDServiceProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMetadataExchange> for IWSDServiceProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMetadataExchange> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMetadataExchange> for &IWSDServiceProxy {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMetadataExchange> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: ::windows::runtime::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, punknown: ::windows::runtime::RawPtr, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDServiceProxyEventing(pub ::windows::runtime::IUnknown);
impl IWSDServiceProxyEventing {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetMetadata(&self) -> ::windows::runtime::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows::runtime::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__: <*mut WSD_METADATA_SECTION_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), presult.into_param().abi(), &mut result__).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::runtime::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__: <*mut WSD_SERVICE_METADATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SubscribeToOperation<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, poperation: *const WSD_OPERATION, punknown: Param1, pany: *const WSDXML_ELEMENT) -> ::windows::runtime::Result<*mut WSDXML_ELEMENT> {
        let mut result__: <*mut WSDXML_ELEMENT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperation), punknown.into_param().abi(), ::std::mem::transmute(pany), &mut result__).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperation)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetEventingStatusCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDEventingStatus>>(&self, pstatus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::runtime::Result<IWSDEndpointProxy> {
        let mut result__: <IWSDEndpointProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDEndpointProxy>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SubscribeToMultipleOperations<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: Param2, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), punknown.into_param().abi(), ::std::mem::transmute(pexpires), ::std::mem::transmute(pany), ::std::mem::transmute(ppexpires), ::std::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn BeginSubscribeToMultipleOperations<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param6: ::windows::runtime::IntoParam<'a, IWSDAsyncCallback>>(
        &self,
        poperations: *const WSD_OPERATION,
        dwoperationcount: u32,
        punknown: Param2,
        pexpires: *const WSD_EVENTING_EXPIRES,
        pany: *const WSDXML_ELEMENT,
        pasyncstate: Param5,
        pasynccallback: Param6,
    ) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), punknown.into_param().abi(), ::std::mem::transmute(pexpires), ::std::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn EndSubscribeToMultipleOperations<'a, Param2: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), presult.into_param().abi(), ::std::mem::transmute(ppexpires), ::std::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), ::std::mem::transmute(pany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn BeginUnsubscribeToMultipleOperations<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param4: ::windows::runtime::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: Param3, pasynccallback: Param4) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), ::std::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn EndUnsubscribeToMultipleOperations<'a, Param2: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), presult.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn RenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), ::std::mem::transmute(pexpires), ::std::mem::transmute(pany), ::std::mem::transmute(ppexpires), ::std::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn BeginRenewMultipleOperations<'a, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param5: ::windows::runtime::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: Param4, pasynccallback: Param5) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), ::std::mem::transmute(pexpires), ::std::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn EndRenewMultipleOperations<'a, Param2: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), presult.into_param().abi(), ::std::mem::transmute(ppexpires), ::std::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), ::std::mem::transmute(pany), ::std::mem::transmute(ppexpires), ::std::mem::transmute(ppany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn BeginGetStatusForMultipleOperations<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param4: ::windows::runtime::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: Param3, pasynccallback: Param4) -> ::windows::runtime::Result<IWSDAsyncResult> {
        let mut result__: <IWSDAsyncResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), ::std::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), &mut result__).from_abi::<IWSDAsyncResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn EndGetStatusForMultipleOperations<'a, Param2: ::windows::runtime::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(poperations), ::std::mem::transmute(dwoperationcount), presult.into_param().abi(), ::std::mem::transmute(ppexpires), ::std::mem::transmute(ppany)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDServiceProxyEventing {
    type Vtable = IWSDServiceProxyEventing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4180122989, 4114, 19092, [184, 204, 253, 53, 210, 32, 43, 254]);
}
impl ::std::convert::From<IWSDServiceProxyEventing> for ::windows::runtime::IUnknown {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDServiceProxyEventing> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDServiceProxy> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDServiceProxy> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDServiceProxy> for &IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDServiceProxy> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMetadataExchange> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMetadataExchange> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMetadataExchange> for &IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMetadataExchange> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: ::windows::runtime::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, punknown: ::windows::runtime::RawPtr, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperation: *const ::std::mem::ManuallyDrop<WSD_OPERATION>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, punknown: ::windows::runtime::RawPtr, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, punknown: ::windows::runtime::RawPtr, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::runtime::RawPtr, pasynccallback: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::runtime::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::runtime::RawPtr, pasynccallback: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::runtime::RawPtr, pasynccallback: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::runtime::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::windows::runtime::RawPtr, pasynccallback: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poperations: *const ::std::mem::ManuallyDrop<WSD_OPERATION>, dwoperationcount: u32, presult: ::windows::runtime::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDSignatureProperty(pub ::windows::runtime::IUnknown);
impl IWSDSignatureProperty {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn IsMessageSigned(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn IsMessageSignatureTrusted(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbkeyinfo), ::std::mem::transmute(pdwkeyinfosize)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetSignature(&self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbsignature), ::std::mem::transmute(pdwsignaturesize)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbsignedinfohash), ::std::mem::transmute(pdwhashsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDSignatureProperty {
    type Vtable = IWSDSignatureProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(63840426, 29124, 17890, [179, 46, 55, 102, 198, 28, 121, 15]);
}
impl ::std::convert::From<IWSDSignatureProperty> for ::windows::runtime::IUnknown {
    fn from(value: IWSDSignatureProperty) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDSignatureProperty> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDSignatureProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDSignatureProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDSignatureProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDTransportAddress(pub ::windows::runtime::IUnknown);
impl IWSDTransportAddress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Serialize<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Deserialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetPort(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransportAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsafe: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fsafe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszaddress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDTransportAddress {
    type Vtable = IWSDTransportAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1892824216, 20198, 17216, [163, 223, 216, 69, 210, 35, 84, 103]);
}
impl ::std::convert::From<IWSDTransportAddress> for ::windows::runtime::IUnknown {
    fn from(value: IWSDTransportAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDTransportAddress> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDTransportAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDTransportAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDTransportAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDTransportAddress> for IWSDAddress {
    fn from(value: IWSDTransportAddress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDTransportAddress> for IWSDAddress {
    fn from(value: &IWSDTransportAddress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAddress> for IWSDTransportAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAddress> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAddress> for &IWSDTransportAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAddress> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwport: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wport: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszaddress: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDUdpAddress(pub ::windows::runtime::IUnknown);
impl IWSDUdpAddress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Serialize<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(cchlength), fsafe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Deserialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetPort(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransportAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsafe: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fsafe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszaddress.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(psockaddr)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    pub unsafe fn GetSockaddr(&self) -> ::windows::runtime::Result<super::super::Networking::WinSock::SOCKADDR_STORAGE> {
        let mut result__: <super::super::Networking::WinSock::SOCKADDR_STORAGE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Networking::WinSock::SOCKADDR_STORAGE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetExclusive<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fexclusive: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), fexclusive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetExclusive(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(messagetype)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetMessageType(&self) -> ::windows::runtime::Result<WSDUdpMessageType> {
        let mut result__: <WSDUdpMessageType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WSDUdpMessageType>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetTTL(&self, dwttl: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwttl)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetTTL(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetAlias(&self, palias: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(palias)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetAlias(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDUdpAddress {
    type Vtable = IWSDUdpAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1960186442, 42049, 20344, [161, 235, 151, 168, 209, 153, 104, 147]);
}
impl ::std::convert::From<IWSDUdpAddress> for ::windows::runtime::IUnknown {
    fn from(value: IWSDUdpAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDUdpAddress> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDUdpAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDUdpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDUdpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDTransportAddress> for IWSDUdpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDTransportAddress> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDTransportAddress> for &IWSDUdpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDTransportAddress> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IWSDUdpAddress> for IWSDAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDUdpAddress> for IWSDAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAddress> for IWSDUdpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAddress> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDAddress> for &IWSDUdpAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDAddress> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwport: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wport: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszaddress: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fexclusive: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: WSDUdpMessageType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessagetype: *mut WSDUdpMessageType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwttl: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwttl: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, palias: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, palias: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDUdpMessageParameters(pub ::windows::runtime::IUnknown);
impl IWSDUdpMessageParameters {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetLocalAddress(&self) -> ::windows::runtime::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::runtime::Result<IWSDAddress> {
        let mut result__: <IWSDAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDAddress>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), paddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetLowerParameters(&self) -> ::windows::runtime::Result<IWSDMessageParameters> {
        let mut result__: <IWSDMessageParameters as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDMessageParameters>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pparams)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetRetransmitParams(&self) -> ::windows::runtime::Result<WSDUdpRetransmitParams> {
        let mut result__: <WSDUdpRetransmitParams as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WSDUdpRetransmitParams>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDUdpMessageParameters {
    type Vtable = IWSDUdpMessageParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2570327199, 36620, 17531, [170, 11, 115, 18, 75, 12, 167, 240]);
}
impl ::std::convert::From<IWSDUdpMessageParameters> for ::windows::runtime::IUnknown {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDUdpMessageParameters> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMessageParameters> for IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMessageParameters> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWSDMessageParameters> for &IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWSDMessageParameters> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paddress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paddress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptxparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pparams: *const WSDUdpRetransmitParams) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pparams: *mut WSDUdpRetransmitParams) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDXMLContext(pub ::windows::runtime::IUnknown);
impl IWSDXMLContext {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn AddNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszuri: Param0, pszsuggestedprefix: Param1) -> ::windows::runtime::Result<*mut WSDXML_NAMESPACE> {
        let mut result__: <*mut WSDXML_NAMESPACE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszuri.into_param().abi(), pszsuggestedprefix.into_param().abi(), &mut result__).from_abi::<*mut WSDXML_NAMESPACE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn AddNameToNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszuri: Param0, pszname: Param1) -> ::windows::runtime::Result<*mut WSDXML_NAME> {
        let mut result__: <*mut WSDXML_NAME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszuri.into_param().abi(), pszname.into_param().abi(), &mut result__).from_abi::<*mut WSDXML_NAME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetNamespaces(&self, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pnamespaces), ::std::mem::transmute(wnamespacescount), ::std::mem::transmute(blayernumber)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetTypes(&self, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptypes), ::std::mem::transmute(dwtypescount), ::std::mem::transmute(blayernumber)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDXMLContext {
    type Vtable = IWSDXMLContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1977152494, 15962, 17332, [161, 90, 188, 246, 136, 116, 96, 192]);
}
impl ::std::convert::From<IWSDXMLContext> for ::windows::runtime::IUnknown {
    fn from(value: IWSDXMLContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDXMLContext> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDXMLContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDXMLContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDXMLContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszuri: super::super::Foundation::PWSTR, pszsuggestedprefix: super::super::Foundation::PWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszuri: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDiscoveredService(pub ::windows::runtime::IUnknown);
impl IWSDiscoveredService {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetEndpointReference(&self) -> ::windows::runtime::Result<*mut WSD_ENDPOINT_REFERENCE> {
        let mut result__: <*mut WSD_ENDPOINT_REFERENCE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_ENDPOINT_REFERENCE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetTypes(&self) -> ::windows::runtime::Result<*mut WSD_NAME_LIST> {
        let mut result__: <*mut WSD_NAME_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_NAME_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetScopes(&self) -> ::windows::runtime::Result<*mut WSD_URI_LIST> {
        let mut result__: <*mut WSD_URI_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetXAddrs(&self) -> ::windows::runtime::Result<*mut WSD_URI_LIST> {
        let mut result__: <*mut WSD_URI_LIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetMetadataVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppheaderany), ::std::mem::transmute(ppbodyany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetProbeResolveTag(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetRemoteTransportAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetLocalTransportAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetLocalInterfaceGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetInstanceId(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDiscoveredService {
    type Vtable = IWSDiscoveredService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1269664315, 45940, 17440, [150, 50, 170, 201, 69, 179, 116, 170]);
}
impl ::std::convert::From<IWSDiscoveredService> for ::windows::runtime::IUnknown {
    fn from(value: IWSDiscoveredService) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDiscoveredService> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDiscoveredService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDiscoveredService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDiscoveredService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pullmetadataversion: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsztag: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszremotetransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszlocaltransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pullinstanceid: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDiscoveryProvider(pub ::windows::runtime::IUnknown);
impl IWSDiscoveryProvider {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaddressfamily)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Attach<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDiscoveryProviderNotify>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Detach(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SearchById<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0, psztag: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszid.into_param().abi(), psztag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SearchByAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszaddress: Param0, psztag: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszaddress.into_param().abi(), psztag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SearchByType<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: Param2, psztag: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptypeslist), ::std::mem::transmute(pscopeslist), pszmatchby.into_param().abi(), psztag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetXMLContext(&self) -> ::windows::runtime::Result<IWSDXMLContext> {
        let mut result__: <IWSDXMLContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDXMLContext>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDiscoveryProvider {
    type Vtable = IWSDiscoveryProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2415693397, 61675, 18447, [136, 183, 180, 53, 221, 40, 29, 69]);
}
impl ::std::convert::From<IWSDiscoveryProvider> for ::windows::runtime::IUnknown {
    fn from(value: IWSDiscoveryProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDiscoveryProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDiscoveryProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDiscoveryProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDiscoveryProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaddressfamily: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszid: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszaddress: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDiscoveryProviderNotify(pub ::windows::runtime::IUnknown);
impl IWSDiscoveryProviderNotify {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDiscoveredService>>(&self, pservice: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pservice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDiscoveredService>>(&self, pservice: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pservice.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SearchFailed<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hr: ::windows::runtime::HRESULT, psztag: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr), psztag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn SearchComplete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psztag: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), psztag.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDiscoveryProviderNotify {
    type Vtable = IWSDiscoveryProviderNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1944993005, 46822, 17193, [165, 70, 62, 138, 212, 101, 99, 210]);
}
impl ::std::convert::From<IWSDiscoveryProviderNotify> for ::windows::runtime::IUnknown {
    fn from(value: IWSDiscoveryProviderNotify) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDiscoveryProviderNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDiscoveryProviderNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDiscoveryProviderNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDiscoveryProviderNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pservice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pservice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT, psztag: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psztag: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDiscoveryPublisher(pub ::windows::runtime::IUnknown);
impl IWSDiscoveryPublisher {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaddressfamily)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn RegisterNotificationSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDiscoveryPublisherNotify>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn UnRegisterNotificationSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDiscoveryPublisherNotify>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn Publish<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param4, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pszid.into_param().abi(),
            ::std::mem::transmute(ullmetadataversion),
            ::std::mem::transmute(ullinstanceid),
            ::std::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::std::mem::transmute(ptypeslist),
            ::std::mem::transmute(pscopeslist),
            ::std::mem::transmute(pxaddrslist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn UnPublish<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszid: Param0, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param3, pany: *const WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszid.into_param().abi(), ::std::mem::transmute(ullinstanceid), ::std::mem::transmute(ullmessagenumber), pszsessionid.into_param().abi(), ::std::mem::transmute(pany)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn MatchProbe<'a, Param1: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pprobemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pprobemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::std::mem::transmute(ullmetadataversion),
            ::std::mem::transmute(ullinstanceid),
            ::std::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::std::mem::transmute(ptypeslist),
            ::std::mem::transmute(pscopeslist),
            ::std::mem::transmute(pxaddrslist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn MatchResolve<'a, Param1: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        presolvemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(presolvemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::std::mem::transmute(ullmetadataversion),
            ::std::mem::transmute(ullinstanceid),
            ::std::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::std::mem::transmute(ptypeslist),
            ::std::mem::transmute(pscopeslist),
            ::std::mem::transmute(pxaddrslist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn PublishEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pszid: Param0,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param4,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pszid.into_param().abi(),
            ::std::mem::transmute(ullmetadataversion),
            ::std::mem::transmute(ullinstanceid),
            ::std::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::std::mem::transmute(ptypeslist),
            ::std::mem::transmute(pscopeslist),
            ::std::mem::transmute(pxaddrslist),
            ::std::mem::transmute(pheaderany),
            ::std::mem::transmute(preferenceparameterany),
            ::std::mem::transmute(ppolicyany),
            ::std::mem::transmute(pendpointreferenceany),
            ::std::mem::transmute(pany),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn MatchProbeEx<'a, Param1: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        pprobemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pprobemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::std::mem::transmute(ullmetadataversion),
            ::std::mem::transmute(ullinstanceid),
            ::std::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::std::mem::transmute(ptypeslist),
            ::std::mem::transmute(pscopeslist),
            ::std::mem::transmute(pxaddrslist),
            ::std::mem::transmute(pheaderany),
            ::std::mem::transmute(preferenceparameterany),
            ::std::mem::transmute(ppolicyany),
            ::std::mem::transmute(pendpointreferenceany),
            ::std::mem::transmute(pany),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn MatchResolveEx<'a, Param1: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
        &self,
        presolvemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: Param1,
        pszid: Param2,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: Param6,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(presolvemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::std::mem::transmute(ullmetadataversion),
            ::std::mem::transmute(ullinstanceid),
            ::std::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::std::mem::transmute(ptypeslist),
            ::std::mem::transmute(pscopeslist),
            ::std::mem::transmute(pxaddrslist),
            ::std::mem::transmute(pheaderany),
            ::std::mem::transmute(preferenceparameterany),
            ::std::mem::transmute(ppolicyany),
            ::std::mem::transmute(pendpointreferenceany),
            ::std::mem::transmute(pany),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn RegisterScopeMatchingRule<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDScopeMatchingRule>>(&self, pscopematchingrule: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pscopematchingrule.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn UnRegisterScopeMatchingRule<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDScopeMatchingRule>>(&self, pscopematchingrule: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pscopematchingrule.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub unsafe fn GetXMLContext(&self) -> ::windows::runtime::Result<IWSDXMLContext> {
        let mut result__: <IWSDXMLContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWSDXMLContext>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWSDiscoveryPublisher {
    type Vtable = IWSDiscoveryPublisher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2919358888, 16377, 16712, [129, 22, 5, 124, 198, 22, 254, 19]);
}
impl ::std::convert::From<IWSDiscoveryPublisher> for ::windows::runtime::IUnknown {
    fn from(value: IWSDiscoveryPublisher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDiscoveryPublisher> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDiscoveryPublisher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDiscoveryPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDiscoveryPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaddressfamily: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszid: super::super::Foundation::PWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::runtime::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::runtime::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszid: super::super::Foundation::PWSTR,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: super::super::Foundation::PWSTR,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pprobemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: ::windows::runtime::RawPtr,
        pszid: super::super::Foundation::PWSTR,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: super::super::Foundation::PWSTR,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        presolvemessage: *const WSD_SOAP_MESSAGE,
        pmessageparameters: ::windows::runtime::RawPtr,
        pszid: super::super::Foundation::PWSTR,
        ullmetadataversion: u64,
        ullinstanceid: u64,
        ullmessagenumber: u64,
        pszsessionid: super::super::Foundation::PWSTR,
        ptypeslist: *const WSD_NAME_LIST,
        pscopeslist: *const WSD_URI_LIST,
        pxaddrslist: *const WSD_URI_LIST,
        pheaderany: *const WSDXML_ELEMENT,
        preferenceparameterany: *const WSDXML_ELEMENT,
        ppolicyany: *const WSDXML_ELEMENT,
        pendpointreferenceany: *const WSDXML_ELEMENT,
        pany: *const WSDXML_ELEMENT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pscopematchingrule: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pscopematchingrule: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWSDiscoveryPublisherNotify(pub ::windows::runtime::IUnknown);
impl IWSDiscoveryPublisherNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn ProbeHandler<'a, Param1: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(psoap), pmessageparameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    pub unsafe fn ResolveHandler<'a, Param1: ::windows::runtime::IntoParam<'a, IWSDMessageParameters>>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(psoap), pmessageparameters.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWSDiscoveryPublisherNotify {
    type Vtable = IWSDiscoveryPublisherNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3866513840, 13178, 19260, [151, 88, 115, 51, 136, 86, 130, 81]);
}
impl ::std::convert::From<IWSDiscoveryPublisherNotify> for ::windows::runtime::IUnknown {
    fn from(value: IWSDiscoveryPublisherNotify) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWSDiscoveryPublisherNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IWSDiscoveryPublisherNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWSDiscoveryPublisherNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWSDiscoveryPublisherNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWSD_SOAP_MESSAGE_HANDLER = unsafe extern "system" fn(thisunknown: ::windows::runtime::RawPtr, event: *mut ::std::mem::ManuallyDrop<WSD_EVENT>) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REQUESTBODY_GetStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REQUESTBODY_GetStatus").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REQUESTBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REQUESTBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REQUESTBODY_GetStatus {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REQUESTBODY_Renew {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REQUESTBODY_Renew").field("Expires", &self.Expires).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REQUESTBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.Expires == other.Expires && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REQUESTBODY_Renew {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REQUESTBODY_Subscribe {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REQUESTBODY_Subscribe").field("EndTo", &self.EndTo).field("Delivery", &self.Delivery).field("Expires", &self.Expires).field("Filter", &self.Filter).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REQUESTBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.EndTo == other.EndTo && self.Delivery == other.Delivery && self.Expires == other.Expires && self.Filter == other.Filter && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REQUESTBODY_Subscribe {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQUESTBODY_Unsubscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REQUESTBODY_Unsubscribe {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REQUESTBODY_Unsubscribe").field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REQUESTBODY_Unsubscribe {
    fn eq(&self, other: &Self) -> bool {
        self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REQUESTBODY_Unsubscribe {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REQUESTBODY_Unsubscribe {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_GetMetadata {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESPONSEBODY_GetMetadata {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_GetMetadata").field("Metadata", &self.Metadata).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESPONSEBODY_GetMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.Metadata == other.Metadata
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESPONSEBODY_GetMetadata {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESPONSEBODY_GetMetadata {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESPONSEBODY_GetStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_GetStatus").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESPONSEBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESPONSEBODY_GetStatus {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESPONSEBODY_Renew {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_Renew").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESPONSEBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESPONSEBODY_Renew {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESPONSEBODY_Subscribe {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_Subscribe").field("SubscriptionManager", &self.SubscriptionManager).field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESPONSEBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESPONSEBODY_Subscribe {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: super::super::Foundation::PWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl RESPONSEBODY_SubscriptionEnd {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESPONSEBODY_SubscriptionEnd {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RESPONSEBODY_SubscriptionEnd").field("SubscriptionManager", &self.SubscriptionManager).field("Status", &self.Status).field("Reason", &self.Reason).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESPONSEBODY_SubscriptionEnd {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.Status == other.Status && self.Reason == other.Reason && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESPONSEBODY_SubscriptionEnd {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESPONSEBODY_SubscriptionEnd {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut ::std::ffi::c_void, cbsize: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDAllocateLinkedMemory(pparent: *mut ::std::ffi::c_void, cbsize: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(WSDAllocateLinkedMemory(::std::mem::transmute(pparent), ::std::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut ::std::ffi::c_void, pchild: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDAttachLinkedMemory(pparent: *mut ::std::ffi::c_void, pchild: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(WSDAttachLinkedMemory(::std::mem::transmute(pparent), ::std::mem::transmute(pchild)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceHost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1) -> ::windows::runtime::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHost(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppdevicehost: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDDeviceHost as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDeviceHost(pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceHost2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::runtime::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHost2(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDDeviceHost as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDeviceHost2(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::std::mem::transmute(pconfigparams), ::std::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1, pphostaddresses: *const ::std::option::Option<IWSDAddress>, dwhostaddresscount: u32) -> ::windows::runtime::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHostAdvanced(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, pphostaddresses: *const ::windows::runtime::RawPtr, dwhostaddresscount: u32, ppdevicehost: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDDeviceHost as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDeviceHostAdvanced(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::std::mem::transmute(pphostaddresses), ::std::mem::transmute(dwhostaddresscount), &mut result__).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceProxy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pszlocalid: Param1, pcontext: Param2) -> ::windows::runtime::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxy(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppdeviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDDeviceProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDeviceProxy(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pszlocalid: Param1, pcontext: Param2, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::runtime::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxy2(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDDeviceProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDeviceProxy2(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::std::mem::transmute(pconfigparams), ::std::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWSDAddress>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pdeviceaddress: Param1, pszlocalid: Param2, pcontext: Param3) -> ::windows::runtime::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxyAdvanced(pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::runtime::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppdeviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDDeviceProxy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDeviceProxyAdvanced(pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0) -> ::windows::runtime::Result<IWSDiscoveryProvider> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryProvider(pcontext: ::windows::runtime::RawPtr, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDiscoveryProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDiscoveryProvider(pcontext.into_param().abi(), &mut result__).from_abi::<IWSDiscoveryProvider>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::runtime::Result<IWSDiscoveryProvider> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryProvider2(pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDiscoveryProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDiscoveryProvider2(pcontext.into_param().abi(), ::std::mem::transmute(pconfigparams), ::std::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDiscoveryProvider>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0) -> ::windows::runtime::Result<IWSDiscoveryPublisher> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryPublisher(pcontext: ::windows::runtime::RawPtr, pppublisher: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDiscoveryPublisher as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDiscoveryPublisher(pcontext.into_param().abi(), &mut result__).from_abi::<IWSDiscoveryPublisher>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<'a, Param0: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32) -> ::windows::runtime::Result<IWSDiscoveryPublisher> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryPublisher2(pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDiscoveryPublisher as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateDiscoveryPublisher2(pcontext.into_param().abi(), ::std::mem::transmute(pconfigparams), ::std::mem::transmute(dwconfigparamcount), &mut result__).from_abi::<IWSDiscoveryPublisher>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> ::windows::runtime::Result<IWSDHttpAddress> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateHttpAddress(ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDHttpAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateHttpAddress(&mut result__).from_abi::<IWSDHttpAddress>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> ::windows::runtime::Result<IWSDHttpMessageParameters> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateHttpMessageParameters(pptxparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDHttpMessageParameters as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateHttpMessageParameters(&mut result__).from_abi::<IWSDHttpMessageParameters>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> ::windows::runtime::Result<IWSDOutboundAttachment> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateOutboundAttachment(ppattachment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDOutboundAttachment as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateOutboundAttachment(&mut result__).from_abi::<IWSDOutboundAttachment>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> ::windows::runtime::Result<IWSDUdpAddress> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateUdpAddress(ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDUdpAddress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateUdpAddress(&mut result__).from_abi::<IWSDUdpAddress>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters() -> ::windows::runtime::Result<IWSDUdpMessageParameters> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateUdpMessageParameters(pptxparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDUdpMessageParameters as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDCreateUdpMessageParameters(&mut result__).from_abi::<IWSDUdpMessageParameters>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDDetachLinkedMemory(pvoid: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(WSDDetachLinkedMemory(::std::mem::transmute(pvoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSDEventType(pub i32);
pub const WSDET_NONE: WSDEventType = WSDEventType(0i32);
pub const WSDET_INCOMING_MESSAGE: WSDEventType = WSDEventType(1i32);
pub const WSDET_INCOMING_FAULT: WSDEventType = WSDEventType(2i32);
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = WSDEventType(3i32);
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = WSDEventType(4i32);
impl ::std::convert::From<i32> for WSDEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSDEventType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDFreeLinkedMemory(pvoid: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(WSDFreeLinkedMemory(::std::mem::transmute(pvoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDGenerateFault<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, IWSDXMLContext>>(
    pszcode: Param0,
    pszsubcode: Param1,
    pszreason: Param2,
    pszdetail: Param3,
    pcontext: Param4,
) -> ::windows::runtime::Result<*mut WSD_SOAP_FAULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGenerateFault(pszcode: super::super::Foundation::PWSTR, pszsubcode: super::super::Foundation::PWSTR, pszreason: super::super::Foundation::PWSTR, pszdetail: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut WSD_SOAP_FAULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDGenerateFault(pszcode.into_param().abi(), pszsubcode.into_param().abi(), pszreason.into_param().abi(), pszdetail.into_param().abi(), pcontext.into_param().abi(), &mut result__).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDGenerateFaultEx<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: Param3) -> ::windows::runtime::Result<*mut WSD_SOAP_FAULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: super::super::Foundation::PWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut WSD_SOAP_FAULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDGenerateFaultEx(::std::mem::transmute(pcode), ::std::mem::transmute(psubcode), ::std::mem::transmute(preasons), pszdetail.into_param().abi(), &mut result__).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::std::ffi::c_void, cboutbuffer: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::std::ffi::c_void, cboutbuffer: u32) -> ::windows::runtime::HRESULT;
        }
        WSDGetConfigurationOption(::std::mem::transmute(dwoption), ::std::mem::transmute(pvoid), ::std::mem::transmute(cboutbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::std::ffi::c_void, cbinbuffer: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::std::ffi::c_void, cbinbuffer: u32) -> ::windows::runtime::HRESULT;
        }
        WSDSetConfigurationOption(::std::mem::transmute(dwoption), ::std::mem::transmute(pvoid), ::std::mem::transmute(cbinbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSDUdpMessageType(pub i32);
pub const ONE_WAY: WSDUdpMessageType = WSDUdpMessageType(0i32);
pub const TWO_WAY: WSDUdpMessageType = WSDUdpMessageType(1i32);
impl ::std::convert::From<i32> for WSDUdpMessageType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSDUdpMessageType {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
impl WSDUdpRetransmitParams {}
impl ::std::default::Default for WSDUdpRetransmitParams {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSDUdpRetransmitParams {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDUdpRetransmitParams").field("ulSendDelay", &self.ulSendDelay).field("ulRepeat", &self.ulRepeat).field("ulRepeatMinDelay", &self.ulRepeatMinDelay).field("ulRepeatMaxDelay", &self.ulRepeatMaxDelay).field("ulRepeatUpperDelay", &self.ulRepeatUpperDelay).finish()
    }
}
impl ::std::cmp::PartialEq for WSDUdpRetransmitParams {
    fn eq(&self, other: &Self) -> bool {
        self.ulSendDelay == other.ulSendDelay && self.ulRepeat == other.ulRepeat && self.ulRepeatMinDelay == other.ulRepeatMinDelay && self.ulRepeatMaxDelay == other.ulRepeatMaxDelay && self.ulRepeatUpperDelay == other.ulRepeatUpperDelay
    }
}
impl ::std::cmp::Eq for WSDUdpRetransmitParams {}
unsafe impl ::windows::runtime::Abi for WSDUdpRetransmitParams {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDUriDecode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(source: Param0, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDUriDecode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WSDUriDecode(source.into_param().abi(), ::std::mem::transmute(cchsource), ::std::mem::transmute(destout), ::std::mem::transmute(cchdestout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDUriEncode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(source: Param0, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDUriEncode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WSDUriEncode(source.into_param().abi(), ::std::mem::transmute(cchsource), ::std::mem::transmute(destout), ::std::mem::transmute(cchdestout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
        }
        WSDXMLAddChild(::std::mem::transmute(pparent), ::std::mem::transmute(pchild)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
        }
        WSDXMLAddSibling(::std::mem::transmute(pfirst), ::std::mem::transmute(psecond)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pelementname: *mut WSDXML_NAME, psztext: Param1, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: super::super::Foundation::PWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
        }
        WSDXMLBuildAnyForSingleElement(::std::mem::transmute(pelementname), psztext.into_param().abi(), ::std::mem::transmute(ppany)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
        }
        WSDXMLCleanupElement(::std::mem::transmute(pany)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[inline]
pub unsafe fn WSDXMLCreateContext() -> ::windows::runtime::Result<IWSDXMLContext> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLCreateContext(ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWSDXMLContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDXMLCreateContext(&mut result__).from_abi::<IWSDXMLContext>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(psznamespace: Param0, pszname: Param1) -> ::windows::runtime::Result<*mut WSDXML_NAME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut WSDXML_NAME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WSDXMLGetNameFromBuiltinNamespace(psznamespace.into_param().abi(), pszname.into_param().abi(), &mut result__).from_abi::<*mut WSDXML_NAME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(psznamespace: Param0, pszname: Param1, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLGetValueFromAny(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WSDXMLGetValueFromAny(psznamespace.into_param().abi(), pszname.into_param().abi(), ::std::mem::transmute(pany), ::std::mem::transmute(ppszvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_ATTRIBUTE").field("Element", &self.Element).field("Next", &self.Next).field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.Next == other.Next && self.Name == other.Name && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_ATTRIBUTE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_ELEMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_ELEMENT").field("Node", &self.Node).field("Name", &self.Name).field("FirstAttribute", &self.FirstAttribute).field("FirstChild", &self.FirstChild).field("PrefixMappings", &self.PrefixMappings).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Name == other.Name && self.FirstAttribute == other.FirstAttribute && self.FirstChild == other.FirstChild && self.PrefixMappings == other.PrefixMappings
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_ELEMENT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_ELEMENT_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_ELEMENT_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_ELEMENT_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_ELEMENT_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_ELEMENT_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_NAME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_NAME").field("Space", &self.Space).field("LocalName", &self.LocalName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Space == other.Space && self.LocalName == other.LocalName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_NAME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_NAMESPACE {
    pub Uri: super::super::Foundation::PWSTR,
    pub PreferredPrefix: super::super::Foundation::PWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_NAMESPACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_NAMESPACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_NAMESPACE").field("Uri", &self.Uri).field("PreferredPrefix", &self.PreferredPrefix).field("Names", &self.Names).field("NamesCount", &self.NamesCount).field("Encoding", &self.Encoding).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_NAMESPACE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.PreferredPrefix == other.PreferredPrefix && self.Names == other.Names && self.NamesCount == other.NamesCount && self.Encoding == other.Encoding
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_NAMESPACE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_NAMESPACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_NODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_NODE").field("Type", &self.Type).field("Parent", &self.Parent).field("Next", &self.Next).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Parent == other.Parent && self.Next == other.Next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_NODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_NODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSDXML_OP(pub i32);
pub const OpNone: WSDXML_OP = WSDXML_OP(0i32);
pub const OpEndOfTable: WSDXML_OP = WSDXML_OP(1i32);
pub const OpBeginElement_: WSDXML_OP = WSDXML_OP(2i32);
pub const OpBeginAnyElement: WSDXML_OP = WSDXML_OP(3i32);
pub const OpEndElement: WSDXML_OP = WSDXML_OP(4i32);
pub const OpElement_: WSDXML_OP = WSDXML_OP(5i32);
pub const OpAnyElement: WSDXML_OP = WSDXML_OP(6i32);
pub const OpAnyElements: WSDXML_OP = WSDXML_OP(7i32);
pub const OpAnyText: WSDXML_OP = WSDXML_OP(8i32);
pub const OpAttribute_: WSDXML_OP = WSDXML_OP(9i32);
pub const OpBeginChoice: WSDXML_OP = WSDXML_OP(10i32);
pub const OpEndChoice: WSDXML_OP = WSDXML_OP(11i32);
pub const OpBeginSequence: WSDXML_OP = WSDXML_OP(12i32);
pub const OpEndSequence: WSDXML_OP = WSDXML_OP(13i32);
pub const OpBeginAll: WSDXML_OP = WSDXML_OP(14i32);
pub const OpEndAll: WSDXML_OP = WSDXML_OP(15i32);
pub const OpAnything: WSDXML_OP = WSDXML_OP(16i32);
pub const OpAnyNumber: WSDXML_OP = WSDXML_OP(17i32);
pub const OpOneOrMore: WSDXML_OP = WSDXML_OP(18i32);
pub const OpOptional: WSDXML_OP = WSDXML_OP(19i32);
pub const OpFormatBool_: WSDXML_OP = WSDXML_OP(20i32);
pub const OpFormatInt8_: WSDXML_OP = WSDXML_OP(21i32);
pub const OpFormatInt16_: WSDXML_OP = WSDXML_OP(22i32);
pub const OpFormatInt32_: WSDXML_OP = WSDXML_OP(23i32);
pub const OpFormatInt64_: WSDXML_OP = WSDXML_OP(24i32);
pub const OpFormatUInt8_: WSDXML_OP = WSDXML_OP(25i32);
pub const OpFormatUInt16_: WSDXML_OP = WSDXML_OP(26i32);
pub const OpFormatUInt32_: WSDXML_OP = WSDXML_OP(27i32);
pub const OpFormatUInt64_: WSDXML_OP = WSDXML_OP(28i32);
pub const OpFormatUnicodeString_: WSDXML_OP = WSDXML_OP(29i32);
pub const OpFormatDom_: WSDXML_OP = WSDXML_OP(30i32);
pub const OpFormatStruct_: WSDXML_OP = WSDXML_OP(31i32);
pub const OpFormatUri_: WSDXML_OP = WSDXML_OP(32i32);
pub const OpFormatUuidUri_: WSDXML_OP = WSDXML_OP(33i32);
pub const OpFormatName_: WSDXML_OP = WSDXML_OP(34i32);
pub const OpFormatListInsertTail_: WSDXML_OP = WSDXML_OP(35i32);
pub const OpFormatType_: WSDXML_OP = WSDXML_OP(36i32);
pub const OpFormatDynamicType_: WSDXML_OP = WSDXML_OP(37i32);
pub const OpFormatLookupType_: WSDXML_OP = WSDXML_OP(38i32);
pub const OpFormatDuration_: WSDXML_OP = WSDXML_OP(39i32);
pub const OpFormatDateTime_: WSDXML_OP = WSDXML_OP(40i32);
pub const OpFormatFloat_: WSDXML_OP = WSDXML_OP(41i32);
pub const OpFormatDouble_: WSDXML_OP = WSDXML_OP(42i32);
pub const OpProcess_: WSDXML_OP = WSDXML_OP(43i32);
pub const OpQualifiedAttribute_: WSDXML_OP = WSDXML_OP(44i32);
pub const OpFormatXMLDeclaration_: WSDXML_OP = WSDXML_OP(45i32);
pub const OpFormatMax: WSDXML_OP = WSDXML_OP(46i32);
impl ::std::convert::From<i32> for WSDXML_OP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSDXML_OP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_PREFIX_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_PREFIX_MAPPING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_PREFIX_MAPPING").field("Refs", &self.Refs).field("Next", &self.Next).field("Space", &self.Space).field("Prefix", &self.Prefix).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_PREFIX_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.Refs == other.Refs && self.Next == other.Next && self.Space == other.Space && self.Prefix == other.Prefix
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_PREFIX_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_PREFIX_MAPPING {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_TEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_TEXT").field("Node", &self.Node).field("Text", &self.Text).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Text == other.Text
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_TEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_TEXT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSDXML_TYPE {
    pub Uri: super::super::Foundation::PWSTR,
    pub Table: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WSDXML_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSDXML_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSDXML_TYPE").field("Uri", &self.Uri).field("Table", &self.Table).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSDXML_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.Table == other.Table
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSDXML_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSDXML_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: super::super::Foundation::PWSTR,
    pub MessageNumber: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_APP_SEQUENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_APP_SEQUENCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_APP_SEQUENCE").field("InstanceId", &self.InstanceId).field("SequenceId", &self.SequenceId).field("MessageNumber", &self.MessageNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_APP_SEQUENCE {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceId == other.InstanceId && self.SequenceId == other.SequenceId && self.MessageNumber == other.MessageNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_APP_SEQUENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_APP_SEQUENCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_BYE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_BYE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_BYE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_BYE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_BYE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_BYE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_BYE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut ::std::option::Option<IWSDAddress>,
    pub dwAddressCount: u32,
}
impl WSD_CONFIG_ADDRESSES {}
impl ::std::default::Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSD_CONFIG_ADDRESSES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_CONFIG_ADDRESSES").field("addresses", &self.addresses).field("dwAddressCount", &self.dwAddressCount).finish()
    }
}
impl ::std::cmp::PartialEq for WSD_CONFIG_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.addresses == other.addresses && self.dwAddressCount == other.dwAddressCount
    }
}
impl ::std::cmp::Eq for WSD_CONFIG_ADDRESSES {}
unsafe impl ::windows::runtime::Abi for WSD_CONFIG_ADDRESSES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut ::std::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl WSD_CONFIG_PARAM {}
impl ::std::default::Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WSD_CONFIG_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_CONFIG_PARAM").field("configParamType", &self.configParamType).field("pConfigData", &self.pConfigData).field("dwConfigDataSize", &self.dwConfigDataSize).finish()
    }
}
impl ::std::cmp::PartialEq for WSD_CONFIG_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.configParamType == other.configParamType && self.pConfigData == other.pConfigData && self.dwConfigDataSize == other.dwConfigDataSize
    }
}
impl ::std::cmp::Eq for WSD_CONFIG_PARAM {}
unsafe impl ::windows::runtime::Abi for WSD_CONFIG_PARAM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSD_CONFIG_PARAM_TYPE(pub i32);
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(1i32);
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(2i32);
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(3i32);
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(4i32);
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(5i32);
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(6i32);
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(7i32);
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(8i32);
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(9i32);
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(10i32);
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(11i32);
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(12i32);
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(13i32);
impl ::std::convert::From<i32> for WSD_CONFIG_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSD_CONFIG_PARAM_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_DATETIME {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: super::super::Foundation::BOOL,
    pub TZIsPositive: super::super::Foundation::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_DATETIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_DATETIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_DATETIME")
            .field("isPositive", &self.isPositive)
            .field("year", &self.year)
            .field("month", &self.month)
            .field("day", &self.day)
            .field("hour", &self.hour)
            .field("minute", &self.minute)
            .field("second", &self.second)
            .field("millisecond", &self.millisecond)
            .field("TZIsLocal", &self.TZIsLocal)
            .field("TZIsPositive", &self.TZIsPositive)
            .field("TZHour", &self.TZHour)
            .field("TZMinute", &self.TZMinute)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond && self.TZIsLocal == other.TZIsLocal && self.TZIsPositive == other.TZIsPositive && self.TZHour == other.TZHour && self.TZMinute == other.TZMinute
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_DATETIME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_DURATION {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_DURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_DURATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_DURATION").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_DURATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: super::super::Foundation::PWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_ENDPOINT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_ENDPOINT_REFERENCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_ENDPOINT_REFERENCE").field("Address", &self.Address).field("ReferenceProperties", &self.ReferenceProperties).field("ReferenceParameters", &self.ReferenceParameters).field("PortType", &self.PortType).field("ServiceName", &self.ServiceName).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_ENDPOINT_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.ReferenceProperties == other.ReferenceProperties && self.ReferenceParameters == other.ReferenceParameters && self.PortType == other.PortType && self.ServiceName == other.ServiceName && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_ENDPOINT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_ENDPOINT_REFERENCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_ENDPOINT_REFERENCE_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_ENDPOINT_REFERENCE_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_ENDPOINT_REFERENCE_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_ENDPOINT_REFERENCE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_ENDPOINT_REFERENCE_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_ENDPOINT_REFERENCE_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_EVENT {
    pub Hr: ::windows::runtime::HRESULT,
    pub EventType: u32,
    pub DispatchTag: super::super::Foundation::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: ::std::option::Option<IWSDMessageParameters>,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_EVENT").field("Hr", &self.Hr).field("EventType", &self.EventType).field("DispatchTag", &self.DispatchTag).field("HandlerContext", &self.HandlerContext).field("Soap", &self.Soap).field("Operation", &self.Operation).field("MessageParameters", &self.MessageParameters).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Hr == other.Hr && self.EventType == other.EventType && self.DispatchTag == other.DispatchTag && self.HandlerContext == other.HandlerContext && self.Soap == other.Soap && self.Operation == other.Operation && self.MessageParameters == other.MessageParameters
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_EVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_EVENT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: super::super::Foundation::PWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_DELIVERY_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_EVENTING_DELIVERY_MODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_DELIVERY_MODE").field("Mode", &self.Mode).field("Push", &self.Push).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Push == other.Push && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_EVENTING_DELIVERY_MODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_EVENTING_DELIVERY_MODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_DELIVERY_MODE_PUSH {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_DELIVERY_MODE_PUSH").field("NotifyTo", &self.NotifyTo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn eq(&self, other: &Self) -> bool {
        self.NotifyTo == other.NotifyTo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_EVENTING_DELIVERY_MODE_PUSH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_EVENTING_DELIVERY_MODE_PUSH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_EVENTING_EXPIRES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_EXPIRES").field("Duration", &self.Duration).field("DateTime", &self.DateTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_EVENTING_EXPIRES {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration && self.DateTime == other.DateTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_EVENTING_EXPIRES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: super::super::Foundation::PWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_EVENTING_FILTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_FILTER").field("Dialect", &self.Dialect).field("FilterAction", &self.FilterAction).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_EVENTING_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.FilterAction == other.FilterAction && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_EVENTING_FILTER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_EVENTING_FILTER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_EVENTING_FILTER_ACTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_EVENTING_FILTER_ACTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_EVENTING_FILTER_ACTION").field("Actions", &self.Actions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_EVENTING_FILTER_ACTION {
    fn eq(&self, other: &Self) -> bool {
        self.Actions == other.Actions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_EVENTING_FILTER_ACTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_EVENTING_FILTER_ACTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: ::std::option::Option<PWSD_SOAP_MESSAGE_HANDLER>,
    pub PVoid: *mut ::std::ffi::c_void,
    pub Unknown: ::std::option::Option<::windows::runtime::IUnknown>,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HANDLER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_HANDLER_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_HANDLER_CONTEXT").field("PVoid", &self.PVoid).field("Unknown", &self.Unknown).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_HANDLER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Handler.map(|f| f as usize) == other.Handler.map(|f| f as usize) && self.PVoid == other.PVoid && self.Unknown == other.Unknown
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_HANDLER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_HANDLER_CONTEXT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HEADER_RELATESTO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_HEADER_RELATESTO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_HEADER_RELATESTO").field("RelationshipType", &self.RelationshipType).field("MessageID", &self.MessageID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_HEADER_RELATESTO {
    fn eq(&self, other: &Self) -> bool {
        self.RelationshipType == other.RelationshipType && self.MessageID == other.MessageID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_HEADER_RELATESTO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_HEADER_RELATESTO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HELLO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_HELLO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_HELLO").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_HELLO {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_HELLO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_HELLO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_HOST_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_HOST_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_HOST_METADATA").field("Host", &self.Host).field("Hosted", &self.Hosted).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_HOST_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Host == other.Host && self.Hosted == other.Hosted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_HOST_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_HOST_METADATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_LOCALIZED_STRING {
    pub lang: super::super::Foundation::PWSTR,
    pub String: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_LOCALIZED_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_LOCALIZED_STRING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_LOCALIZED_STRING").field("lang", &self.lang).field("String", &self.String).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_LOCALIZED_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang && self.String == other.String
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_LOCALIZED_STRING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_LOCALIZED_STRING {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_LOCALIZED_STRING_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_LOCALIZED_STRING_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_LOCALIZED_STRING_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_LOCALIZED_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_LOCALIZED_STRING_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_LOCALIZED_STRING_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_METADATA_SECTION {
    pub Dialect: super::super::Foundation::PWSTR,
    pub Identifier: super::super::Foundation::PWSTR,
    pub Data: *mut ::std::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_METADATA_SECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_METADATA_SECTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_METADATA_SECTION").field("Dialect", &self.Dialect).field("Identifier", &self.Identifier).field("Data", &self.Data).field("MetadataReference", &self.MetadataReference).field("Location", &self.Location).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_METADATA_SECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.Identifier == other.Identifier && self.Data == other.Data && self.MetadataReference == other.MetadataReference && self.Location == other.Location && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_METADATA_SECTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_METADATA_SECTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_METADATA_SECTION_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_METADATA_SECTION_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_METADATA_SECTION_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_METADATA_SECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_METADATA_SECTION_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_METADATA_SECTION_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_NAME_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_NAME_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_NAME_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_NAME_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_NAME_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: ::std::option::Option<WSD_STUB_FUNCTION>,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_OPERATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_OPERATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_OPERATION").field("RequestType", &self.RequestType).field("ResponseType", &self.ResponseType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        self.RequestType == other.RequestType && self.ResponseType == other.ResponseType && self.RequestStubFunction.map(|f| f as usize) == other.RequestStubFunction.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_OPERATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_OPERATION {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PORT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_PORT_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_PORT_TYPE").field("EncodedName", &self.EncodedName).field("OperationCount", &self.OperationCount).field("Operations", &self.Operations).field("ProtocolType", &self.ProtocolType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_PORT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.EncodedName == other.EncodedName && self.OperationCount == other.OperationCount && self.Operations == other.Operations && self.ProtocolType == other.ProtocolType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_PORT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_PORT_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_PROBE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_PROBE").field("Types", &self.Types).field("Scopes", &self.Scopes).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_PROBE {
    fn eq(&self, other: &Self) -> bool {
        self.Types == other.Types && self.Scopes == other.Scopes && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_PROBE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_PROBE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_PROBE_MATCH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_PROBE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_PROBE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_PROBE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_PROBE_MATCH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_PROBE_MATCHES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_PROBE_MATCHES").field("ProbeMatch", &self.ProbeMatch).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_PROBE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ProbeMatch == other.ProbeMatch && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_PROBE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_PROBE_MATCHES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_PROBE_MATCH_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_PROBE_MATCH_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_PROBE_MATCH_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_PROBE_MATCH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_PROBE_MATCH_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_PROBE_MATCH_LIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSD_PROTOCOL_TYPE(pub i32);
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(0i32);
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(1i32);
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(2i32);
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(4i32);
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(255i32);
impl ::std::convert::From<i32> for WSD_PROTOCOL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSD_PROTOCOL_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_REFERENCE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_REFERENCE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_REFERENCE_PARAMETERS").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_REFERENCE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_REFERENCE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_REFERENCE_PARAMETERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_REFERENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_REFERENCE_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_REFERENCE_PROPERTIES").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_REFERENCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_REFERENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_REFERENCE_PROPERTIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: super::super::Foundation::PWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RELATIONSHIP_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_RELATIONSHIP_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_RELATIONSHIP_METADATA").field("Type", &self.Type).field("Data", &self.Data).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_RELATIONSHIP_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Data == other.Data && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_RELATIONSHIP_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_RELATIONSHIP_METADATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RESOLVE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_RESOLVE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_RESOLVE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_RESOLVE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_RESOLVE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_RESOLVE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RESOLVE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_RESOLVE_MATCH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_RESOLVE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_RESOLVE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_RESOLVE_MATCH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_RESOLVE_MATCH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_RESOLVE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_RESOLVE_MATCHES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_RESOLVE_MATCHES").field("ResolveMatch", &self.ResolveMatch).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_RESOLVE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ResolveMatch == other.ResolveMatch && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_RESOLVE_MATCHES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_RESOLVE_MATCHES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SCOPES {
    pub MatchBy: super::super::Foundation::PWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SCOPES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SCOPES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SCOPES").field("MatchBy", &self.MatchBy).field("Scopes", &self.Scopes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SCOPES {
    fn eq(&self, other: &Self) -> bool {
        self.MatchBy == other.MatchBy && self.Scopes == other.Scopes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SCOPES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SCOPES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: *mut ::std::ffi::c_void,
    pub hCertIssuerStore: *mut ::std::ffi::c_void,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: super::super::Foundation::PWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for WSD_SECURITY_CERT_VALIDATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SECURITY_CERT_VALIDATION")
            .field("certMatchArray", &self.certMatchArray)
            .field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount)
            .field("hCertMatchStore", &self.hCertMatchStore)
            .field("hCertIssuerStore", &self.hCertIssuerStore)
            .field("dwCertCheckOptions", &self.dwCertCheckOptions)
            .field("pszCNGHashAlgId", &self.pszCNGHashAlgId)
            .field("pbCertHash", &self.pbCertHash)
            .field("dwCertHashSize", &self.dwCertHashSize)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions && self.pszCNGHashAlgId == other.pszCNGHashAlgId && self.pbCertHash == other.pbCertHash && self.dwCertHashSize == other.dwCertHashSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WSD_SECURITY_CERT_VALIDATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: *mut ::std::ffi::c_void,
    pub hCertIssuerStore: *mut ::std::ffi::c_void,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SECURITY_CERT_VALIDATION_V1")
            .field("certMatchArray", &self.certMatchArray)
            .field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount)
            .field("hCertMatchStore", &self.hCertMatchStore)
            .field("hCertIssuerStore", &self.hCertIssuerStore)
            .field("dwCertCheckOptions", &self.dwCertCheckOptions)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WSD_SECURITY_CERT_VALIDATION_V1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: *mut ::std::ffi::c_void,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SECURITY_SIGNATURE_VALIDATION").field("signingCertArray", &self.signingCertArray).field("dwSigningCertArrayCount", &self.dwSigningCertArrayCount).field("hSigningCertStore", &self.hSigningCertStore).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.signingCertArray == other.signingCertArray && self.dwSigningCertArrayCount == other.dwSigningCertArrayCount && self.hSigningCertStore == other.hSigningCertStore && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WSD_SECURITY_SIGNATURE_VALIDATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SERVICE_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SERVICE_METADATA").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("ServiceId", &self.ServiceId).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.ServiceId == other.ServiceId && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SERVICE_METADATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SERVICE_METADATA_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SERVICE_METADATA_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SERVICE_METADATA_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SERVICE_METADATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SERVICE_METADATA_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SERVICE_METADATA_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: super::super::Foundation::PWSTR,
    pub Role: super::super::Foundation::PWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SOAP_FAULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT").field("Code", &self.Code).field("Reason", &self.Reason).field("Node", &self.Node).field("Role", &self.Role).field("Detail", &self.Detail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SOAP_FAULT {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code && self.Reason == other.Reason && self.Node == other.Node && self.Role == other.Role && self.Detail == other.Detail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SOAP_FAULT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SOAP_FAULT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SOAP_FAULT_CODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT_CODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SOAP_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SOAP_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SOAP_FAULT_CODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT_REASON {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SOAP_FAULT_REASON {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT_REASON").field("Text", &self.Text).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SOAP_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.Text == other.Text
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SOAP_FAULT_REASON {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SOAP_FAULT_REASON {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_FAULT_SUBCODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SOAP_FAULT_SUBCODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SOAP_FAULT_SUBCODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SOAP_FAULT_SUBCODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SOAP_FAULT_SUBCODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SOAP_FAULT_SUBCODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SOAP_HEADER {
    pub To: super::super::Foundation::PWSTR,
    pub Action: super::super::Foundation::PWSTR,
    pub MessageID: super::super::Foundation::PWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SOAP_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SOAP_HEADER")
            .field("To", &self.To)
            .field("Action", &self.Action)
            .field("MessageID", &self.MessageID)
            .field("RelatesTo", &self.RelatesTo)
            .field("ReplyTo", &self.ReplyTo)
            .field("From", &self.From)
            .field("FaultTo", &self.FaultTo)
            .field("AppSequence", &self.AppSequence)
            .field("AnyHeaders", &self.AnyHeaders)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SOAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.To == other.To && self.Action == other.Action && self.MessageID == other.MessageID && self.RelatesTo == other.RelatesTo && self.ReplyTo == other.ReplyTo && self.From == other.From && self.FaultTo == other.FaultTo && self.AppSequence == other.AppSequence && self.AnyHeaders == other.AnyHeaders
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SOAP_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SOAP_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut ::std::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SOAP_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SOAP_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SOAP_MESSAGE").field("Header", &self.Header).field("Body", &self.Body).field("BodyType", &self.BodyType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SOAP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Body == other.Body && self.BodyType == other.BodyType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SOAP_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SOAP_MESSAGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSD_STUB_FUNCTION = unsafe extern "system" fn(server: ::windows::runtime::RawPtr, session: ::windows::runtime::RawPtr, event: *mut ::std::mem::ManuallyDrop<WSD_EVENT>) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: ::windows::runtime::HRESULT,
    pub eventHandle: super::super::Foundation::HANDLE,
    pub messageParameters: ::std::option::Option<IWSDMessageParameters>,
    pub results: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_SYNCHRONOUS_RESPONSE_CONTEXT").field("hr", &self.hr).field("eventHandle", &self.eventHandle).field("messageParameters", &self.messageParameters).field("results", &self.results).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.eventHandle == other.eventHandle && self.messageParameters == other.messageParameters && self.results == other.results
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: super::super::Foundation::PWSTR,
    pub SerialNumber: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_THIS_DEVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_THIS_DEVICE_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_THIS_DEVICE_METADATA").field("FriendlyName", &self.FriendlyName).field("FirmwareVersion", &self.FirmwareVersion).field("SerialNumber", &self.SerialNumber).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_THIS_DEVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.FriendlyName == other.FriendlyName && self.FirmwareVersion == other.FirmwareVersion && self.SerialNumber == other.SerialNumber && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_THIS_DEVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_THIS_DEVICE_METADATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: super::super::Foundation::PWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: super::super::Foundation::PWSTR,
    pub ModelUrl: super::super::Foundation::PWSTR,
    pub PresentationUrl: super::super::Foundation::PWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_THIS_MODEL_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_THIS_MODEL_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_THIS_MODEL_METADATA")
            .field("Manufacturer", &self.Manufacturer)
            .field("ManufacturerUrl", &self.ManufacturerUrl)
            .field("ModelName", &self.ModelName)
            .field("ModelNumber", &self.ModelNumber)
            .field("ModelUrl", &self.ModelUrl)
            .field("PresentationUrl", &self.PresentationUrl)
            .field("Any", &self.Any)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_THIS_MODEL_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.ManufacturerUrl == other.ManufacturerUrl && self.ModelName == other.ModelName && self.ModelNumber == other.ModelNumber && self.ModelUrl == other.ModelUrl && self.PresentationUrl == other.PresentationUrl && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_THIS_MODEL_METADATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_THIS_MODEL_METADATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_UNKNOWN_LOOKUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_UNKNOWN_LOOKUP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_UNKNOWN_LOOKUP").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_UNKNOWN_LOOKUP {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_UNKNOWN_LOOKUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_UNKNOWN_LOOKUP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WSD_URI_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WSD_URI_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WSD_URI_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WSD_URI_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WSD_URI_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WSD_URI_LIST {
    type Abi = Self;
}
