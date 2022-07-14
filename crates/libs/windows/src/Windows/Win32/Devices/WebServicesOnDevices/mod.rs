#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceDiscoveryMechanism(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const MulticastDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const DirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(2i32);
impl ::core::marker::Copy for DeviceDiscoveryMechanism {}
impl ::core::clone::Clone for DeviceDiscoveryMechanism {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceDiscoveryMechanism {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceDiscoveryMechanism {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceDiscoveryMechanism {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDiscoveryMechanism").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAddress(::windows::core::IUnknown);
impl IWSDAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<'a, P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Deserialize)(::windows::core::Interface::as_raw(self), pszbuffer.into()).ok()
    }
}
impl ::core::convert::From<IWSDAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDAddress> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAddress {}
impl ::core::fmt::Debug for IWSDAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDAddress {
    type Vtable = IWSDAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9574c6c_12a6_4f74_93a1_3318ff605759);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows::core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Serialize: usize,
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAsyncCallback(::windows::core::IUnknown);
impl IWSDAsyncCallback {
    pub unsafe fn AsyncOperationComplete<'a, P0, P1>(&self, pasyncresult: P0, pasyncstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).AsyncOperationComplete)(::windows::core::Interface::as_raw(self), pasyncresult.into().abi(), pasyncstate.into().abi()).ok()
    }
}
impl ::core::convert::From<IWSDAsyncCallback> for ::windows::core::IUnknown {
    fn from(value: IWSDAsyncCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDAsyncCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDAsyncCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAsyncCallback> for ::windows::core::IUnknown {
    fn from(value: &IWSDAsyncCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDAsyncCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAsyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAsyncCallback {}
impl ::core::fmt::Debug for IWSDAsyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAsyncCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDAsyncCallback {
    type Vtable = IWSDAsyncCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa63e109d_ce72_49e2_ba98_e845f5ee1666);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAsyncResult(::windows::core::IUnknown);
impl IWSDAsyncResult {
    pub unsafe fn SetCallback<'a, P0, P1>(&self, pcallback: P0, pasyncstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetCallback)(::windows::core::Interface::as_raw(self), pcallback.into().abi(), pasyncstate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWaitHandle<'a, P0>(&self, hwaithandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).SetWaitHandle)(::windows::core::Interface::as_raw(self), hwaithandle.into()).ok()
    }
    pub unsafe fn HasCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HasCompleted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAsyncState(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAsyncState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetEvent(&self) -> ::windows::core::Result<WSD_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEvent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSD_EVENT>(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEndpointProxy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
}
impl ::core::convert::From<IWSDAsyncResult> for ::windows::core::IUnknown {
    fn from(value: IWSDAsyncResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDAsyncResult> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDAsyncResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAsyncResult> for ::windows::core::IUnknown {
    fn from(value: &IWSDAsyncResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDAsyncResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAsyncResult {}
impl ::core::fmt::Debug for IWSDAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAsyncResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDAsyncResult {
    type Vtable = IWSDAsyncResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11a9852a_8dd8_423e_b537_9356db4fbfb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWaitHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWaitHandle: usize,
    pub HasCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAsyncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAttachment(::windows::core::IUnknown);
impl IWSDAttachment {}
impl ::core::convert::From<IWSDAttachment> for ::windows::core::IUnknown {
    fn from(value: IWSDAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDAttachment> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAttachment> for ::windows::core::IUnknown {
    fn from(value: &IWSDAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAttachment {}
impl ::core::fmt::Debug for IWSDAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDAttachment {
    type Vtable = IWSDAttachment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d55a616_9df8_4b09_b156_9ba351a48b76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDDeviceHost(::windows::core::IUnknown);
impl IWSDDeviceHost {
    pub unsafe fn Init<'a, P0, P1>(&self, pszlocalid: P0, pcontext: P1, pphostaddresses: &[::core::option::Option<IWSDAddress>]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), pszlocalid.into(), pcontext.into().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pphostaddresses)), pphostaddresses.len() as _).ok()
    }
    pub unsafe fn Start<'a, P0>(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDDeviceHostNotify>>,
    {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), ullinstanceid, ::core::mem::transmute(pscopelist), pnotificationsink.into().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterPortType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pporttype)).ok()
    }
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pthismodelmetadata), ::core::mem::transmute(pthisdevicemetadata), ::core::mem::transmute(phostmetadata), ::core::mem::transmute(pcustommetadata)).ok()
    }
    pub unsafe fn RegisterService<'a, P0, P1>(&self, pszserviceid: P0, pservice: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).RegisterService)(::windows::core::Interface::as_raw(self), pszserviceid.into(), pservice.into().abi()).ok()
    }
    pub unsafe fn RetireService<'a, P0>(&self, pszserviceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RetireService)(::windows::core::Interface::as_raw(self), pszserviceid.into()).ok()
    }
    pub unsafe fn AddDynamicService<'a, P0, P1, P2>(&self, pszserviceid: P0, pszendpointaddress: P1, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).AddDynamicService)(::windows::core::Interface::as_raw(self), pszserviceid.into(), pszendpointaddress.into(), ::core::mem::transmute(pporttype), ::core::mem::transmute(pportname), ::core::mem::transmute(pany), pservice.into().abi()).ok()
    }
    pub unsafe fn RemoveDynamicService<'a, P0>(&self, pszserviceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveDynamicService)(::windows::core::Interface::as_raw(self), pszserviceid.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceDiscoverable<'a, P0, P1>(&self, pszserviceid: P0, fdiscoverable: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetServiceDiscoverable)(::windows::core::Interface::as_raw(self), pszserviceid.into(), fdiscoverable.into()).ok()
    }
    pub unsafe fn SignalEvent<'a, P0>(&self, pszserviceid: P0, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SignalEvent)(::windows::core::Interface::as_raw(self), pszserviceid.into(), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation)).ok()
    }
}
impl ::core::convert::From<IWSDDeviceHost> for ::windows::core::IUnknown {
    fn from(value: IWSDDeviceHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDDeviceHost> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDDeviceHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDDeviceHost> for ::windows::core::IUnknown {
    fn from(value: &IWSDDeviceHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDDeviceHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceHost {}
impl ::core::fmt::Debug for IWSDDeviceHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDDeviceHost {
    type Vtable = IWSDDeviceHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917fe891_3d13_4138_9809_934c8abeb12c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pphostaddresses: *const *mut ::core::ffi::c_void, dwhostaddresscount: u32) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterPortType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    pub RegisterService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RetireService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, pszendpointaddress: ::windows::core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceDiscoverable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceDiscoverable: usize,
    pub SignalEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDDeviceHostNotify(::windows::core::IUnknown);
impl IWSDDeviceHostNotify {
    pub unsafe fn GetService<'a, P0>(&self, pszserviceid: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetService)(::windows::core::Interface::as_raw(self), pszserviceid.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IWSDDeviceHostNotify> for ::windows::core::IUnknown {
    fn from(value: IWSDDeviceHostNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDDeviceHostNotify> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDDeviceHostNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDDeviceHostNotify> for ::windows::core::IUnknown {
    fn from(value: &IWSDDeviceHostNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDDeviceHostNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceHostNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceHostNotify {}
impl ::core::fmt::Debug for IWSDDeviceHostNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceHostNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDDeviceHostNotify {
    type Vtable = IWSDDeviceHostNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDDeviceProxy(::windows::core::IUnknown);
impl IWSDDeviceProxy {
    pub unsafe fn Init<'a, P0, P1, P2, P3, P4>(&self, pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3, psponsor: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, IWSDDeviceProxy>>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), pszdeviceid.into(), pdeviceaddress.into().abi(), pszlocalid.into(), pcontext.into().abi(), psponsor.into().abi()).ok()
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginGetMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetMetadata<'a, P0>(&self, presult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        (::windows::core::Interface::vtable(self).EndGetMetadata)(::windows::core::Interface::as_raw(self), presult.into().abi()).ok()
    }
    pub unsafe fn GetHostMetadata(&self) -> ::windows::core::Result<*mut WSD_HOST_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHostMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_HOST_METADATA>(result__)
    }
    pub unsafe fn GetThisModelMetadata(&self) -> ::windows::core::Result<*mut WSD_THIS_MODEL_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetThisModelMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_THIS_MODEL_METADATA>(result__)
    }
    pub unsafe fn GetThisDeviceMetadata(&self) -> ::windows::core::Result<*mut WSD_THIS_DEVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetThisDeviceMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_THIS_DEVICE_METADATA>(result__)
    }
    pub unsafe fn GetAllMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAllMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn GetServiceProxyById<'a, P0>(&self, pszserviceid: P0) -> ::windows::core::Result<IWSDServiceProxy>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetServiceProxyById)(::windows::core::Interface::as_raw(self), pszserviceid.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDServiceProxy>(result__)
    }
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> ::windows::core::Result<IWSDServiceProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetServiceProxyByType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDServiceProxy>(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEndpointProxy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
}
impl ::core::convert::From<IWSDDeviceProxy> for ::windows::core::IUnknown {
    fn from(value: IWSDDeviceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDDeviceProxy> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDDeviceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDDeviceProxy> for ::windows::core::IUnknown {
    fn from(value: &IWSDDeviceProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDDeviceProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceProxy {}
impl ::core::fmt::Debug for IWSDDeviceProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDDeviceProxy {
    type Vtable = IWSDDeviceProxy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeee0c031_c578_4c0e_9a3b_973c35f409db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdeviceid: ::windows::core::PCWSTR, pdeviceaddress: *mut ::core::ffi::c_void, pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, psponsor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHostMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows::core::HRESULT,
    pub GetThisModelMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows::core::HRESULT,
    pub GetThisDeviceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows::core::HRESULT,
    pub GetAllMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    pub GetServiceProxyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetServiceProxyByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDEndpointProxy(::windows::core::IUnknown);
impl IWSDEndpointProxy {
    pub unsafe fn SendOneWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOneWayRequest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendTwoWayRequest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), ::core::mem::transmute(presponsecontext)).ok()
    }
    pub unsafe fn SendTwoWayRequestAsync<'a, P0, P1>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: P0, pcallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SendTwoWayRequestAsync)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), pasyncstate.into().abi(), pcallback.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn AbortAsyncOperation<'a, P0>(&self, pasyncresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        (::windows::core::Interface::vtable(self).AbortAsyncOperation)(::windows::core::Interface::as_raw(self), pasyncresult.into().abi()).ok()
    }
    pub unsafe fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessFault)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfault)).ok()
    }
    pub unsafe fn GetErrorInfo(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetFaultInfo(&self) -> ::windows::core::Result<*mut WSD_SOAP_FAULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFaultInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
}
impl ::core::convert::From<IWSDEndpointProxy> for ::windows::core::IUnknown {
    fn from(value: IWSDEndpointProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDEndpointProxy> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDEndpointProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDEndpointProxy> for ::windows::core::IUnknown {
    fn from(value: &IWSDEndpointProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDEndpointProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDEndpointProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDEndpointProxy {}
impl ::core::fmt::Debug for IWSDEndpointProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDEndpointProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDEndpointProxy {
    type Vtable = IWSDEndpointProxy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1860d430_b24c_4975_9f90_dbb39baa24ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SendOneWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendTwoWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendTwoWayRequest: usize,
    pub SendTwoWayRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, presult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AbortAsyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessFault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetFaultInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDEventingStatus(::windows::core::IUnknown);
impl IWSDEventingStatus {
    pub unsafe fn SubscriptionRenewed<'a, P0>(&self, pszsubscriptionaction: P0)
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SubscriptionRenewed)(::windows::core::Interface::as_raw(self), pszsubscriptionaction.into())
    }
    pub unsafe fn SubscriptionRenewalFailed<'a, P0>(&self, pszsubscriptionaction: P0, hr: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SubscriptionRenewalFailed)(::windows::core::Interface::as_raw(self), pszsubscriptionaction.into(), hr)
    }
    pub unsafe fn SubscriptionEnded<'a, P0>(&self, pszsubscriptionaction: P0)
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SubscriptionEnded)(::windows::core::Interface::as_raw(self), pszsubscriptionaction.into())
    }
}
impl ::core::convert::From<IWSDEventingStatus> for ::windows::core::IUnknown {
    fn from(value: IWSDEventingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDEventingStatus> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDEventingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDEventingStatus> for ::windows::core::IUnknown {
    fn from(value: &IWSDEventingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDEventingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDEventingStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDEventingStatus {}
impl ::core::fmt::Debug for IWSDEventingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDEventingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDEventingStatus {
    type Vtable = IWSDEventingStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SubscriptionRenewed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows::core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows::core::PCWSTR, hr: ::windows::core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows::core::PCWSTR),
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDHttpAddress(::windows::core::IUnknown);
impl IWSDHttpAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<'a, P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Deserialize)(::windows::core::Interface::as_raw(self), pszbuffer.into()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPort)(::windows::core::Interface::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransportAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<'a, P0>(&self, fsafe: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransportAddressEx)(::windows::core::Interface::as_raw(self), fsafe.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetTransportAddress<'a, P0>(&self, pszaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetTransportAddress)(::windows::core::Interface::as_raw(self), pszaddress.into()).ok()
    }
    pub unsafe fn GetSecure(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSecure)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecure<'a, P0>(&self, fsecure: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetSecure)(::windows::core::Interface::as_raw(self), fsecure.into()).ok()
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetPath<'a, P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPath)(::windows::core::Interface::as_raw(self), pszpath.into()).ok()
    }
}
impl ::core::convert::From<IWSDHttpAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDHttpAddress> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDHttpAddress> for IWSDAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDHttpAddress> for &'a IWSDAddress {
    fn from(value: &'a IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for IWSDAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDHttpAddress> for &'a IWSDTransportAddress {
    fn from(value: &'a IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDHttpAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDHttpAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpAddress {}
impl ::core::fmt::Debug for IWSDHttpAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDHttpAddress {
    type Vtable = IWSDHttpAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd09ac7bd_2a3e_4b85_8605_2737ff3e4ea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    pub GetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecure: usize,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDHttpAuthParameters(::windows::core::IUnknown);
impl IWSDHttpAuthParameters {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientAccessToken(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetClientAccessToken)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn GetAuthType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAuthType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWSDHttpAuthParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDHttpAuthParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDHttpAuthParameters> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDHttpAuthParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAuthParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDHttpAuthParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDHttpAuthParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDHttpAuthParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpAuthParameters {}
impl ::core::fmt::Debug for IWSDHttpAuthParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpAuthParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDHttpAuthParameters {
    type Vtable = IWSDHttpAuthParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b476df0_8dac_480d_b05c_99781a5884aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientAccessToken: usize,
    pub GetAuthType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDHttpMessageParameters(::windows::core::IUnknown);
impl IWSDHttpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLocalAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLocalAddress)(::windows::core::Interface::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetRemoteAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRemoteAddress)(::windows::core::Interface::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLowerParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDMessageParameters>(result__)
    }
    pub unsafe fn SetInboundHttpHeaders<'a, P0>(&self, pszheaders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetInboundHttpHeaders)(::windows::core::Interface::as_raw(self), pszheaders.into()).ok()
    }
    pub unsafe fn GetInboundHttpHeaders(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInboundHttpHeaders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetOutboundHttpHeaders<'a, P0>(&self, pszheaders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetOutboundHttpHeaders)(::windows::core::Interface::as_raw(self), pszheaders.into()).ok()
    }
    pub unsafe fn GetOutboundHttpHeaders(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutboundHttpHeaders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetID<'a, P0>(&self, pszid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetID)(::windows::core::Interface::as_raw(self), pszid.into()).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetContext<'a, P0>(&self, pcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetContext)(::windows::core::Interface::as_raw(self), pcontext.into().abi()).ok()
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWSDHttpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDHttpMessageParameters> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDHttpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDHttpMessageParameters> for &'a IWSDMessageParameters {
    fn from(value: &'a IWSDHttpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDHttpMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDHttpMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpMessageParameters {}
impl ::core::fmt::Debug for IWSDHttpMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpMessageParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDHttpMessageParameters {
    type Vtable = IWSDHttpMessageParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x540bd122_5c83_4dec_b396_ea62a2697fdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDInboundAttachment(::windows::core::IUnknown);
impl IWSDInboundAttachment {
    pub unsafe fn Read(&self, pbuffer: &mut [u8], pdwnumberofbytesread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(pdwnumberofbytesread)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWSDInboundAttachment> for ::windows::core::IUnknown {
    fn from(value: IWSDInboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDInboundAttachment> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDInboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDInboundAttachment> for ::windows::core::IUnknown {
    fn from(value: &IWSDInboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: IWSDInboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDInboundAttachment> for &'a IWSDAttachment {
    fn from(value: &'a IWSDInboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDInboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDInboundAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDInboundAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDInboundAttachment {}
impl ::core::fmt::Debug for IWSDInboundAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDInboundAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDInboundAttachment {
    type Vtable = IWSDInboundAttachment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDMessageParameters(::windows::core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLocalAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
    {
        (::windows::core::Interface::vtable(self).SetLocalAddress)(::windows::core::Interface::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRemoteAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
    {
        (::windows::core::Interface::vtable(self).SetRemoteAddress)(::windows::core::Interface::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLowerParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDMessageParameters>(result__)
    }
}
impl ::core::convert::From<IWSDMessageParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDMessageParameters> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDMessageParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDMessageParameters {}
impl ::core::fmt::Debug for IWSDMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDMessageParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDMessageParameters {
    type Vtable = IWSDMessageParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLowerParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDMetadataExchange(::windows::core::IUnknown);
impl IWSDMetadataExchange {
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
}
impl ::core::convert::From<IWSDMetadataExchange> for ::windows::core::IUnknown {
    fn from(value: IWSDMetadataExchange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDMetadataExchange> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDMetadataExchange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDMetadataExchange> for ::windows::core::IUnknown {
    fn from(value: &IWSDMetadataExchange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDMetadataExchange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDMetadataExchange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDMetadataExchange {}
impl ::core::fmt::Debug for IWSDMetadataExchange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDMetadataExchange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDMetadataExchange {
    type Vtable = IWSDMetadataExchange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06996d57_1d67_4928_9307_3d7833fdb846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDOutboundAttachment(::windows::core::IUnknown);
impl IWSDOutboundAttachment {
    pub unsafe fn Write(&self, pbuffer: &[u8]) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWSDOutboundAttachment> for ::windows::core::IUnknown {
    fn from(value: IWSDOutboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDOutboundAttachment> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDOutboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDOutboundAttachment> for ::windows::core::IUnknown {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: IWSDOutboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDOutboundAttachment> for &'a IWSDAttachment {
    fn from(value: &'a IWSDOutboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDOutboundAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDOutboundAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDOutboundAttachment {}
impl ::core::fmt::Debug for IWSDOutboundAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDOutboundAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDOutboundAttachment {
    type Vtable = IWSDOutboundAttachment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDSSLClientCertificate(::windows::core::IUnknown);
impl IWSDSSLClientCertificate {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn GetClientCertificate(&self) -> ::windows::core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetClientCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::Security::Cryptography::CERT_CONTEXT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMappedAccessToken(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMappedAccessToken)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<IWSDSSLClientCertificate> for ::windows::core::IUnknown {
    fn from(value: IWSDSSLClientCertificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDSSLClientCertificate> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDSSLClientCertificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDSSLClientCertificate> for ::windows::core::IUnknown {
    fn from(value: &IWSDSSLClientCertificate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDSSLClientCertificate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDSSLClientCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDSSLClientCertificate {}
impl ::core::fmt::Debug for IWSDSSLClientCertificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDSSLClientCertificate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDSSLClientCertificate {
    type Vtable = IWSDSSLClientCertificate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde105e87_a0da_418e_98ad_27b9eed87bdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub GetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    GetClientCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMappedAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMappedAccessToken: usize,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDScopeMatchingRule(::windows::core::IUnknown);
impl IWSDScopeMatchingRule {
    pub unsafe fn GetScopeRule(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetScopeRule)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchScopes<'a, P0, P1>(&self, pszscope1: P0, pszscope2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MatchScopes)(::windows::core::Interface::as_raw(self), pszscope1.into(), pszscope2.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWSDScopeMatchingRule> for ::windows::core::IUnknown {
    fn from(value: IWSDScopeMatchingRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDScopeMatchingRule> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDScopeMatchingRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDScopeMatchingRule> for ::windows::core::IUnknown {
    fn from(value: &IWSDScopeMatchingRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDScopeMatchingRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDScopeMatchingRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDScopeMatchingRule {}
impl ::core::fmt::Debug for IWSDScopeMatchingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDScopeMatchingRule").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDScopeMatchingRule {
    type Vtable = IWSDScopeMatchingRule_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcafe424_fef5_481a_bd9f_33ce0574256f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetScopeRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszscope1: ::windows::core::PCWSTR, pszscope2: ::windows::core::PCWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchScopes: usize,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDServiceMessaging(::windows::core::IUnknown);
impl IWSDServiceMessaging {
    pub unsafe fn SendResponse<'a, P0>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
    {
        (::windows::core::Interface::vtable(self).SendResponse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), pmessageparameters.into().abi()).ok()
    }
    pub unsafe fn FaultRequest<'a, P0>(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: P0, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
    {
        (::windows::core::Interface::vtable(self).FaultRequest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prequestheader), pmessageparameters.into().abi(), ::core::mem::transmute(pfault)).ok()
    }
}
impl ::core::convert::From<IWSDServiceMessaging> for ::windows::core::IUnknown {
    fn from(value: IWSDServiceMessaging) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDServiceMessaging> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDServiceMessaging) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceMessaging> for ::windows::core::IUnknown {
    fn from(value: &IWSDServiceMessaging) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDServiceMessaging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDServiceMessaging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceMessaging {}
impl ::core::fmt::Debug for IWSDServiceMessaging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceMessaging").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDServiceMessaging {
    type Vtable = IWSDServiceMessaging_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94974cf4_0cab_460d_a3f6_7a0ad623c0e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SendResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FaultRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDServiceProxy(::windows::core::IUnknown);
impl IWSDServiceProxy {
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginGetMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetMetadata<'a, P0>(&self, presult: P0) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EndGetMetadata)(::windows::core::Interface::as_raw(self), presult.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetServiceMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    pub unsafe fn SubscribeToOperation<'a, P0>(&self, poperation: *const WSD_OPERATION, punknown: P0, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<*mut WSDXML_ELEMENT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SubscribeToOperation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(poperation), punknown.into().abi(), ::core::mem::transmute(pany), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnsubscribeToOperation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(poperation)).ok()
    }
    pub unsafe fn SetEventingStatusCallback<'a, P0>(&self, pstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDEventingStatus>>,
    {
        (::windows::core::Interface::vtable(self).SetEventingStatusCallback)(::windows::core::Interface::as_raw(self), pstatus.into().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEndpointProxy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
}
impl ::core::convert::From<IWSDServiceProxy> for ::windows::core::IUnknown {
    fn from(value: IWSDServiceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDServiceProxy> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDServiceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxy> for ::windows::core::IUnknown {
    fn from(value: &IWSDServiceProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDServiceProxy> for &'a IWSDMetadataExchange {
    fn from(value: &'a IWSDServiceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDServiceProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDServiceProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceProxy {}
impl ::core::fmt::Debug for IWSDServiceProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDServiceProxy {
    type Vtable = IWSDServiceProxy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4c7fb9c_03ab_4175_9d67_094fafebf487);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_Vtbl {
    pub base__: IWSDMetadataExchange_Vtbl,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    pub GetServiceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::core::HRESULT,
    pub SubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub UnsubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT,
    pub SetEventingStatusCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDServiceProxyEventing(::windows::core::IUnknown);
impl IWSDServiceProxyEventing {
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.BeginGetMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetMetadata<'a, P0>(&self, presult: P0) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.EndGetMetadata)(::windows::core::Interface::as_raw(self), presult.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetServiceMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    pub unsafe fn SubscribeToOperation<'a, P0>(&self, poperation: *const WSD_OPERATION, punknown: P0, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<*mut WSDXML_ELEMENT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SubscribeToOperation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(poperation), punknown.into().abi(), ::core::mem::transmute(pany), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnsubscribeToOperation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(poperation)).ok()
    }
    pub unsafe fn SetEventingStatusCallback<'a, P0>(&self, pstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDEventingStatus>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEventingStatusCallback)(::windows::core::Interface::as_raw(self), pstatus.into().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetEndpointProxy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscribeToMultipleOperations<'a, P0>(&self, poperations: &[WSD_OPERATION], punknown: P0, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SubscribeToMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, punknown.into().abi(), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginSubscribeToMultipleOperations<'a, P0, P1, P2>(&self, poperations: &[WSD_OPERATION], punknown: P0, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: P1, pasynccallback: P2) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginSubscribeToMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, punknown.into().abi(), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), pasyncstate.into().abi(), pasynccallback.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndSubscribeToMultipleOperations<'a, P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        (::windows::core::Interface::vtable(self).EndSubscribeToMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnsubscribeToMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany)).ok()
    }
    pub unsafe fn BeginUnsubscribeToMultipleOperations<'a, P0, P1>(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT, pasyncstate: P0, pasynccallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginUnsubscribeToMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany), pasyncstate.into().abi(), pasynccallback.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndUnsubscribeToMultipleOperations<'a, P0>(&self, poperations: &[WSD_OPERATION], presult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        (::windows::core::Interface::vtable(self).EndUnsubscribeToMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenewMultipleOperations(&self, poperations: &[WSD_OPERATION], pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RenewMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginRenewMultipleOperations<'a, P0, P1>(&self, poperations: &[WSD_OPERATION], pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: P0, pasynccallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginRenewMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), pasyncstate.into().abi(), pasynccallback.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndRenewMultipleOperations<'a, P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        (::windows::core::Interface::vtable(self).EndRenewMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatusForMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    pub unsafe fn BeginGetStatusForMultipleOperations<'a, P0, P1>(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT, pasyncstate: P0, pasynccallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginGetStatusForMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany), pasyncstate.into().abi(), pasynccallback.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndGetStatusForMultipleOperations<'a, P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAsyncResult>>,
    {
        (::windows::core::Interface::vtable(self).EndGetStatusForMultipleOperations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for ::windows::core::IUnknown {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDServiceProxyEventing> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for ::windows::core::IUnknown {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDServiceProxyEventing> for &'a IWSDMetadataExchange {
    fn from(value: &'a IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDServiceProxyEventing> for &'a IWSDServiceProxy {
    fn from(value: &'a IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDServiceProxyEventing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDServiceProxyEventing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceProxyEventing {}
impl ::core::fmt::Debug for IWSDServiceProxyEventing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceProxyEventing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDServiceProxyEventing {
    type Vtable = IWSDServiceProxyEventing_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9279d6d_1012_4a94_b8cc_fd35d2202bfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_Vtbl {
    pub base__: IWSDServiceProxy_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscribeToMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginSubscribeToMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndSubscribeToMultipleOperations: usize,
    pub UnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub BeginUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginRenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndRenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStatusForMultipleOperations: usize,
    pub BeginGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EndGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndGetStatusForMultipleOperations: usize,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDSignatureProperty(::windows::core::IUnknown);
impl IWSDSignatureProperty {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMessageSigned(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsMessageSigned)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMessageSignatureTrusted(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsMessageSignatureTrusted)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKeyInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbkeyinfo), ::core::mem::transmute(pdwkeyinfosize)).ok()
    }
    pub unsafe fn GetSignature(&self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignature)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsignature), ::core::mem::transmute(pdwsignaturesize)).ok()
    }
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignedInfoHash)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsignedinfohash), ::core::mem::transmute(pdwhashsize)).ok()
    }
}
impl ::core::convert::From<IWSDSignatureProperty> for ::windows::core::IUnknown {
    fn from(value: IWSDSignatureProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDSignatureProperty> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDSignatureProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDSignatureProperty> for ::windows::core::IUnknown {
    fn from(value: &IWSDSignatureProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDSignatureProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDSignatureProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDSignatureProperty {}
impl ::core::fmt::Debug for IWSDSignatureProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDSignatureProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDSignatureProperty {
    type Vtable = IWSDSignatureProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03ce20aa_71c4_45e2_b32e_3766c61c790f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMessageSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMessageSigned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMessageSignatureTrusted: usize,
    pub GetKeyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignedInfoHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDTransportAddress(::windows::core::IUnknown);
impl IWSDTransportAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<'a, P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Deserialize)(::windows::core::Interface::as_raw(self), pszbuffer.into()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPort)(::windows::core::Interface::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransportAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<'a, P0>(&self, fsafe: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransportAddressEx)(::windows::core::Interface::as_raw(self), fsafe.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetTransportAddress<'a, P0>(&self, pszaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetTransportAddress)(::windows::core::Interface::as_raw(self), pszaddress.into()).ok()
    }
}
impl ::core::convert::From<IWSDTransportAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDTransportAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDTransportAddress> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDTransportAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDTransportAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDTransportAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDTransportAddress> for IWSDAddress {
    fn from(value: IWSDTransportAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDTransportAddress> for &'a IWSDAddress {
    fn from(value: &'a IWSDTransportAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDTransportAddress> for IWSDAddress {
    fn from(value: &IWSDTransportAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDTransportAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDTransportAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDTransportAddress {}
impl ::core::fmt::Debug for IWSDTransportAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDTransportAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDTransportAddress {
    type Vtable = IWSDTransportAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70d23498_4ee6_4340_a3df_d845d2235467);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_Vtbl {
    pub base__: IWSDAddress_Vtbl,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows::core::HRESULT,
    pub GetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTransportAddressEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTransportAddressEx: usize,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDUdpAddress(::windows::core::IUnknown);
impl IWSDUdpAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<'a, P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Deserialize)(::windows::core::Interface::as_raw(self), pszbuffer.into()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPort)(::windows::core::Interface::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransportAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<'a, P0>(&self, fsafe: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTransportAddressEx)(::windows::core::Interface::as_raw(self), fsafe.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetTransportAddress<'a, P0>(&self, pszaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetTransportAddress)(::windows::core::Interface::as_raw(self), pszaddress.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSockaddr)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psockaddr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub unsafe fn GetSockaddr(&self) -> ::windows::core::Result<super::super::Networking::WinSock::SOCKADDR_STORAGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSockaddr)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Networking::WinSock::SOCKADDR_STORAGE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExclusive<'a, P0>(&self, fexclusive: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetExclusive)(::windows::core::Interface::as_raw(self), fexclusive.into()).ok()
    }
    pub unsafe fn GetExclusive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExclusive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageType)(::windows::core::Interface::as_raw(self), messagetype).ok()
    }
    pub unsafe fn GetMessageType(&self) -> ::windows::core::Result<WSDUdpMessageType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMessageType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSDUdpMessageType>(result__)
    }
    pub unsafe fn SetTTL(&self, dwttl: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTTL)(::windows::core::Interface::as_raw(self), dwttl).ok()
    }
    pub unsafe fn GetTTL(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTTL)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAlias(&self, palias: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlias)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(palias)).ok()
    }
    pub unsafe fn GetAlias(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAlias)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IWSDUdpAddress> for ::windows::core::IUnknown {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDUdpAddress> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for ::windows::core::IUnknown {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDUdpAddress> for IWSDAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDUdpAddress> for &'a IWSDAddress {
    fn from(value: &'a IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for IWSDAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDUdpAddress> for &'a IWSDTransportAddress {
    fn from(value: &'a IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDUdpAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDUdpAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDUdpAddress {}
impl ::core::fmt::Debug for IWSDUdpAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDUdpAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDUdpAddress {
    type Vtable = IWSDUdpAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d6124a_a441_4f78_a1eb_97a8d1996893);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub SetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))]
    SetSockaddr: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub GetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))]
    GetSockaddr: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExclusive: usize,
    pub GetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows::core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows::core::HRESULT,
    pub SetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows::core::HRESULT,
    pub GetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows::core::HRESULT,
    pub SetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDUdpMessageParameters(::windows::core::IUnknown);
impl IWSDUdpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLocalAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLocalAddress)(::windows::core::Interface::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetRemoteAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRemoteAddress)(::windows::core::Interface::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLowerParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDMessageParameters>(result__)
    }
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRetransmitParams)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pparams)).ok()
    }
    pub unsafe fn GetRetransmitParams(&self) -> ::windows::core::Result<WSDUdpRetransmitParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRetransmitParams)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSDUdpRetransmitParams>(result__)
    }
}
impl ::core::convert::From<IWSDUdpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDUdpMessageParameters> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDUdpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpMessageParameters> for ::windows::core::IUnknown {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDUdpMessageParameters> for &'a IWSDMessageParameters {
    fn from(value: &'a IWSDUdpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDUdpMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDUdpMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDUdpMessageParameters {}
impl ::core::fmt::Debug for IWSDUdpMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDUdpMessageParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDUdpMessageParameters {
    type Vtable = IWSDUdpMessageParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9934149f_8f0c_447b_aa0b_73124b0ca7f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::HRESULT,
    pub GetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDXMLContext(::windows::core::IUnknown);
impl IWSDXMLContext {
    pub unsafe fn AddNamespace<'a, P0, P1>(&self, pszuri: P0, pszsuggestedprefix: P1) -> ::windows::core::Result<*mut WSDXML_NAMESPACE>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddNamespace)(::windows::core::Interface::as_raw(self), pszuri.into(), pszsuggestedprefix.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_NAMESPACE>(result__)
    }
    pub unsafe fn AddNameToNamespace<'a, P0, P1>(&self, pszuri: P0, pszname: P1) -> ::windows::core::Result<*mut WSDXML_NAME>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddNameToNamespace)(::windows::core::Interface::as_raw(self), pszuri.into(), pszname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_NAME>(result__)
    }
    pub unsafe fn SetNamespaces(&self, pnamespaces: &[*const WSDXML_NAMESPACE], blayernumber: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNamespaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pnamespaces)), pnamespaces.len() as _, blayernumber).ok()
    }
    pub unsafe fn SetTypes(&self, ptypes: &[*const WSDXML_TYPE], blayernumber: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ptypes)), ptypes.len() as _, blayernumber).ok()
    }
}
impl ::core::convert::From<IWSDXMLContext> for ::windows::core::IUnknown {
    fn from(value: IWSDXMLContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDXMLContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDXMLContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDXMLContext> for ::windows::core::IUnknown {
    fn from(value: &IWSDXMLContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDXMLContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDXMLContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDXMLContext {}
impl ::core::fmt::Debug for IWSDXMLContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDXMLContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDXMLContext {
    type Vtable = IWSDXMLContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows::core::PCWSTR, pszsuggestedprefix: ::windows::core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows::core::HRESULT,
    pub AddNameToNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows::core::PCWSTR, pszname: ::windows::core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT,
    pub SetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::HRESULT,
    pub SetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveredService(::windows::core::IUnknown);
impl IWSDiscoveredService {
    pub unsafe fn GetEndpointReference(&self) -> ::windows::core::Result<*mut WSD_ENDPOINT_REFERENCE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEndpointReference)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_ENDPOINT_REFERENCE>(result__)
    }
    pub unsafe fn GetTypes(&self) -> ::windows::core::Result<*mut WSD_NAME_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_NAME_LIST>(result__)
    }
    pub unsafe fn GetScopes(&self) -> ::windows::core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetScopes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    pub unsafe fn GetXAddrs(&self) -> ::windows::core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetXAddrs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    pub unsafe fn GetMetadataVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMetadataVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExtendedDiscoXML)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppheaderany), ::core::mem::transmute(ppbodyany)).ok()
    }
    pub unsafe fn GetProbeResolveTag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProbeResolveTag)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetRemoteTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRemoteTransportAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLocalTransportAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalInterfaceGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLocalInterfaceGUID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWSDiscoveredService> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveredService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDiscoveredService> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDiscoveredService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveredService> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveredService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDiscoveredService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveredService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveredService {}
impl ::core::fmt::Debug for IWSDiscoveredService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveredService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveredService {
    type Vtable = IWSDiscoveredService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bad8a3b_b374_4420_9632_aac945b374aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetEndpointReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows::core::HRESULT,
    pub GetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows::core::HRESULT,
    pub GetScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub GetXAddrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub GetMetadataVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows::core::HRESULT,
    pub GetExtendedDiscoXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub GetProbeResolveTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztag: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetRemoteTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetLocalTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetLocalInterfaceGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryProvider(::windows::core::IUnknown);
impl IWSDiscoveryProvider {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAddressFamily)(::windows::core::Interface::as_raw(self), dwaddressfamily).ok()
    }
    pub unsafe fn Attach<'a, P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDiscoveryProviderNotify>>,
    {
        (::windows::core::Interface::vtable(self).Attach)(::windows::core::Interface::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Detach)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SearchById<'a, P0, P1>(&self, pszid: P0, psztag: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SearchById)(::windows::core::Interface::as_raw(self), pszid.into(), psztag.into()).ok()
    }
    pub unsafe fn SearchByAddress<'a, P0, P1>(&self, pszaddress: P0, psztag: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SearchByAddress)(::windows::core::Interface::as_raw(self), pszaddress.into(), psztag.into()).ok()
    }
    pub unsafe fn SearchByType<'a, P0, P1>(&self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: P0, psztag: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SearchByType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), pszmatchby.into(), psztag.into()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows::core::Result<IWSDXMLContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetXMLContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDXMLContext>(result__)
    }
}
impl ::core::convert::From<IWSDiscoveryProvider> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDiscoveryProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDiscoveryProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryProvider> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDiscoveryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryProvider {}
impl ::core::fmt::Debug for IWSDiscoveryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryProvider {
    type Vtable = IWSDiscoveryProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ffc8e55_f0eb_480f_88b7_b435dd281d45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SearchById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SearchByAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows::core::PCWSTR, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SearchByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: ::windows::core::PCWSTR, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryProviderNotify(::windows::core::IUnknown);
impl IWSDiscoveryProviderNotify {
    pub unsafe fn Add<'a, P0>(&self, pservice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDiscoveredService>>,
    {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), pservice.into().abi()).ok()
    }
    pub unsafe fn Remove<'a, P0>(&self, pservice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDiscoveredService>>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), pservice.into().abi()).ok()
    }
    pub unsafe fn SearchFailed<'a, P0>(&self, hr: ::windows::core::HRESULT, psztag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SearchFailed)(::windows::core::Interface::as_raw(self), hr, psztag.into()).ok()
    }
    pub unsafe fn SearchComplete<'a, P0>(&self, psztag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SearchComplete)(::windows::core::Interface::as_raw(self), psztag.into()).ok()
    }
}
impl ::core::convert::From<IWSDiscoveryProviderNotify> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryProviderNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDiscoveryProviderNotify> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDiscoveryProviderNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryProviderNotify> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryProviderNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDiscoveryProviderNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryProviderNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryProviderNotify {}
impl ::core::fmt::Debug for IWSDiscoveryProviderNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryProviderNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryProviderNotify {
    type Vtable = IWSDiscoveryProviderNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73ee3ced_b6e6_4329_a546_3e8ad46563d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SearchFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryPublisher(::windows::core::IUnknown);
impl IWSDiscoveryPublisher {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAddressFamily)(::windows::core::Interface::as_raw(self), dwaddressfamily).ok()
    }
    pub unsafe fn RegisterNotificationSink<'a, P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDiscoveryPublisherNotify>>,
    {
        (::windows::core::Interface::vtable(self).RegisterNotificationSink)(::windows::core::Interface::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn UnRegisterNotificationSink<'a, P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDiscoveryPublisherNotify>>,
    {
        (::windows::core::Interface::vtable(self).UnRegisterNotificationSink)(::windows::core::Interface::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn Publish<'a, P0, P1>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Publish)(::windows::core::Interface::as_raw(self), pszid.into(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into(), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), ::core::mem::transmute(pxaddrslist)).ok()
    }
    pub unsafe fn UnPublish<'a, P0, P1>(&self, pszid: P0, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UnPublish)(::windows::core::Interface::as_raw(self), pszid.into(), ullinstanceid, ullmessagenumber, pszsessionid.into(), ::core::mem::transmute(pany)).ok()
    }
    pub unsafe fn MatchProbe<'a, P0, P1, P2>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MatchProbe)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprobemessage), pmessageparameters.into().abi(), pszid.into(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into(), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), ::core::mem::transmute(pxaddrslist)).ok()
    }
    pub unsafe fn MatchResolve<'a, P0, P1, P2>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MatchResolve)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(presolvemessage), pmessageparameters.into().abi(), pszid.into(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into(), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), ::core::mem::transmute(pxaddrslist)).ok()
    }
    pub unsafe fn PublishEx<'a, P0, P1>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).PublishEx)(::windows::core::Interface::as_raw(self), pszid.into(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into(), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), ::core::mem::transmute(pxaddrslist), ::core::mem::transmute(pheaderany), ::core::mem::transmute(preferenceparameterany), ::core::mem::transmute(ppolicyany), ::core::mem::transmute(pendpointreferenceany), ::core::mem::transmute(pany)).ok()
    }
    pub unsafe fn MatchProbeEx<'a, P0, P1, P2>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MatchProbeEx)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(pprobemessage),
            pmessageparameters.into().abi(),
            pszid.into(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    pub unsafe fn MatchResolveEx<'a, P0, P1, P2>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MatchResolveEx)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(presolvemessage),
            pmessageparameters.into().abi(),
            pszid.into(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    pub unsafe fn RegisterScopeMatchingRule<'a, P0>(&self, pscopematchingrule: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDScopeMatchingRule>>,
    {
        (::windows::core::Interface::vtable(self).RegisterScopeMatchingRule)(::windows::core::Interface::as_raw(self), pscopematchingrule.into().abi()).ok()
    }
    pub unsafe fn UnRegisterScopeMatchingRule<'a, P0>(&self, pscopematchingrule: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDScopeMatchingRule>>,
    {
        (::windows::core::Interface::vtable(self).UnRegisterScopeMatchingRule)(::windows::core::Interface::as_raw(self), pscopematchingrule.into().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows::core::Result<IWSDXMLContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetXMLContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDXMLContext>(result__)
    }
}
impl ::core::convert::From<IWSDiscoveryPublisher> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDiscoveryPublisher> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDiscoveryPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryPublisher> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDiscoveryPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryPublisher {}
impl ::core::fmt::Debug for IWSDiscoveryPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryPublisher {
    type Vtable = IWSDiscoveryPublisher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae01e1a8_3ff9_4148_8116_057cc616fe13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT,
    pub RegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnRegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Publish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub UnPublish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub MatchProbe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub MatchResolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub PublishEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub MatchProbeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub MatchResolveEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub RegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnRegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryPublisherNotify(::windows::core::IUnknown);
impl IWSDiscoveryPublisherNotify {
    pub unsafe fn ProbeHandler<'a, P0>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
    {
        (::windows::core::Interface::vtable(self).ProbeHandler)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psoap), pmessageparameters.into().abi()).ok()
    }
    pub unsafe fn ResolveHandler<'a, P0>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDMessageParameters>>,
    {
        (::windows::core::Interface::vtable(self).ResolveHandler)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psoap), pmessageparameters.into().abi()).ok()
    }
}
impl ::core::convert::From<IWSDiscoveryPublisherNotify> for ::windows::core::IUnknown {
    fn from(value: IWSDiscoveryPublisherNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWSDiscoveryPublisherNotify> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWSDiscoveryPublisherNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryPublisherNotify> for ::windows::core::IUnknown {
    fn from(value: &IWSDiscoveryPublisherNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWSDiscoveryPublisherNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryPublisherNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryPublisherNotify {}
impl ::core::fmt::Debug for IWSDiscoveryPublisherNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryPublisherNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWSDiscoveryPublisherNotify {
    type Vtable = IWSDiscoveryPublisherNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe67651b0_337a_4b3c_9758_733388568251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ProbeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResolveHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type PWSD_SOAP_MESSAGE_HANDLER = ::core::option::Option<unsafe extern "system" fn(thisunknown: ::core::option::Option<::windows::core::IUnknown>, event: *mut WSD_EVENT) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_GetStatus {}
impl ::core::clone::Clone for REQUESTBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_GetStatus").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for REQUESTBODY_GetStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUESTBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_GetStatus>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUESTBODY_GetStatus {}
impl ::core::default::Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQUESTBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Renew").field("Expires", &self.Expires).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_Renew {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_Renew>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQUESTBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Subscribe").field("EndTo", &self.EndTo).field("Delivery", &self.Delivery).field("Expires", &self.Expires).field("Filter", &self.Filter).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_Subscribe {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_Subscribe>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Unsubscribe {}
impl ::core::clone::Clone for REQUESTBODY_Unsubscribe {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_Unsubscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Unsubscribe").field("any", &self.any).finish()
    }
}
unsafe impl ::windows::core::Abi for REQUESTBODY_Unsubscribe {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUESTBODY_Unsubscribe {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_Unsubscribe>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Unsubscribe {}
impl ::core::default::Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
impl ::core::marker::Copy for RESPONSEBODY_GetMetadata {}
impl ::core::clone::Clone for RESPONSEBODY_GetMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_GetMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetMetadata").field("Metadata", &self.Metadata).finish()
    }
}
unsafe impl ::windows::core::Abi for RESPONSEBODY_GetMetadata {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_GetMetadata {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_GetMetadata>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_GetMetadata {}
impl ::core::default::Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetStatus").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_GetStatus {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_GetStatus>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Renew").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_Renew {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_Renew>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Subscribe").field("SubscriptionManager", &self.SubscriptionManager).field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_Subscribe {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_Subscribe>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: ::windows::core::PCWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_SubscriptionEnd {}
impl ::core::clone::Clone for RESPONSEBODY_SubscriptionEnd {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_SubscriptionEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_SubscriptionEnd").field("SubscriptionManager", &self.SubscriptionManager).field("Status", &self.Status).field("Reason", &self.Reason).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for RESPONSEBODY_SubscriptionEnd {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_SubscriptionEnd {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_SubscriptionEnd>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_SubscriptionEnd {}
impl ::core::default::Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void;
    }
    WSDAllocateLinkedMemory(::core::mem::transmute(pparent), cbsize)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void);
    }
    WSDAttachLinkedMemory(::core::mem::transmute(pparent), ::core::mem::transmute(pchild))
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceHost<'a, P0, P1>(pszlocalid: P0, pcontext: P1) -> ::windows::core::Result<IWSDDeviceHost>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDeviceHost(pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, ppdevicehost: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceHost(pszlocalid.into(), pcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceHost>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceHost2<'a, P0, P1>(pszlocalid: P0, pcontext: P1, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows::core::Result<IWSDDeviceHost>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDeviceHost2(pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceHost2(pszlocalid.into(), pcontext.into().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceHost>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<'a, P0, P1>(pszlocalid: P0, pcontext: P1, pphostaddresses: &[::core::option::Option<IWSDAddress>]) -> ::windows::core::Result<IWSDDeviceHost>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDeviceHostAdvanced(pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pphostaddresses: *const *mut ::core::ffi::c_void, dwhostaddresscount: u32, ppdevicehost: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceHostAdvanced(pszlocalid.into(), pcontext.into().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pphostaddresses)), pphostaddresses.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceHost>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceProxy<'a, P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2) -> ::windows::core::Result<IWSDDeviceProxy>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDeviceProxy(pszdeviceid: ::windows::core::PCWSTR, pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, ppdeviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceProxy(pszdeviceid.into(), pszlocalid.into(), pcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceProxy>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<'a, P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows::core::Result<IWSDDeviceProxy>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDeviceProxy2(pszdeviceid: ::windows::core::PCWSTR, pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceProxy2(pszdeviceid.into(), pszlocalid.into(), pcontext.into().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceProxy>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<'a, P0, P1, P2, P3>(pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3) -> ::windows::core::Result<IWSDDeviceProxy>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IWSDAddress>>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDeviceProxyAdvanced(pszdeviceid: ::windows::core::PCWSTR, pdeviceaddress: *mut ::core::ffi::c_void, pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, ppdeviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceProxyAdvanced(pszdeviceid.into(), pdeviceaddress.into().abi(), pszlocalid.into(), pcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceProxy>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<'a, P0>(pcontext: P0) -> ::windows::core::Result<IWSDiscoveryProvider>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDiscoveryProvider(pcontext: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryProvider(pcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryProvider>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<'a, P0>(pcontext: P0, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows::core::Result<IWSDiscoveryProvider>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDiscoveryProvider2(pcontext: *mut ::core::ffi::c_void, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryProvider2(pcontext.into().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryProvider>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<'a, P0>(pcontext: P0) -> ::windows::core::Result<IWSDiscoveryPublisher>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDiscoveryPublisher(pcontext: *mut ::core::ffi::c_void, pppublisher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryPublisher(pcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryPublisher>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<'a, P0>(pcontext: P0, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows::core::Result<IWSDiscoveryPublisher>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateDiscoveryPublisher2(pcontext: *mut ::core::ffi::c_void, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryPublisher2(pcontext.into().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryPublisher>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> ::windows::core::Result<IWSDHttpAddress> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateHttpAddress(ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateHttpAddress(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDHttpAddress>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> ::windows::core::Result<IWSDHttpMessageParameters> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateHttpMessageParameters(pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateHttpMessageParameters(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDHttpMessageParameters>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> ::windows::core::Result<IWSDOutboundAttachment> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateOutboundAttachment(ppattachment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateOutboundAttachment(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDOutboundAttachment>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> ::windows::core::Result<IWSDUdpAddress> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateUdpAddress(ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateUdpAddress(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDUdpAddress>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters() -> ::windows::core::Result<IWSDUdpMessageParameters> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDCreateUdpMessageParameters(pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateUdpMessageParameters(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDUdpMessageParameters>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    }
    WSDDetachLinkedMemory(::core::mem::transmute(pvoid))
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSDEventType(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_NONE: WSDEventType = WSDEventType(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_INCOMING_MESSAGE: WSDEventType = WSDEventType(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_INCOMING_FAULT: WSDEventType = WSDEventType(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = WSDEventType(3i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = WSDEventType(4i32);
impl ::core::marker::Copy for WSDEventType {}
impl ::core::clone::Clone for WSDEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSDEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSDEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSDEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDEventType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    }
    WSDFreeLinkedMemory(::core::mem::transmute(pvoid))
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDGenerateFault<'a, P0, P1, P2, P3, P4>(pszcode: P0, pszsubcode: P1, pszreason: P2, pszdetail: P3, pcontext: P4) -> ::windows::core::Result<*mut WSD_SOAP_FAULT>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::InParam<'a, IWSDXMLContext>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDGenerateFault(pszcode: ::windows::core::PCWSTR, pszsubcode: ::windows::core::PCWSTR, pszreason: ::windows::core::PCWSTR, pszdetail: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDGenerateFault(pszcode.into(), pszsubcode.into(), pszreason.into(), pszdetail.into(), pcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SOAP_FAULT>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDGenerateFaultEx<'a, P0>(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: P0) -> ::windows::core::Result<*mut WSD_SOAP_FAULT>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: ::windows::core::PCWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDGenerateFaultEx(::core::mem::transmute(pcode), ::core::mem::transmute(psubcode), ::core::mem::transmute(preasons), pszdetail.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SOAP_FAULT>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows::core::HRESULT;
    }
    WSDGetConfigurationOption(dwoption, ::core::mem::transmute(pvoid), cboutbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows::core::HRESULT;
    }
    WSDSetConfigurationOption(dwoption, ::core::mem::transmute(pvoid), cbinbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSDUdpMessageType(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const ONE_WAY: WSDUdpMessageType = WSDUdpMessageType(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const TWO_WAY: WSDUdpMessageType = WSDUdpMessageType(1i32);
impl ::core::marker::Copy for WSDUdpMessageType {}
impl ::core::clone::Clone for WSDUdpMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSDUdpMessageType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSDUdpMessageType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSDUdpMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDUdpMessageType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
impl ::core::marker::Copy for WSDUdpRetransmitParams {}
impl ::core::clone::Clone for WSDUdpRetransmitParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDUdpRetransmitParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDUdpRetransmitParams").field("ulSendDelay", &self.ulSendDelay).field("ulRepeat", &self.ulRepeat).field("ulRepeatMinDelay", &self.ulRepeatMinDelay).field("ulRepeatMaxDelay", &self.ulRepeatMaxDelay).field("ulRepeatUpperDelay", &self.ulRepeatUpperDelay).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDUdpRetransmitParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDUdpRetransmitParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDUdpRetransmitParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDUdpRetransmitParams {}
impl ::core::default::Default for WSDUdpRetransmitParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDUriDecode(source: &[u16], destout: *mut ::windows::core::PWSTR, cchdestout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDUriDecode(source: ::windows::core::PCWSTR, cchsource: u32, destout: *mut ::windows::core::PWSTR, cchdestout: *mut u32) -> ::windows::core::HRESULT;
    }
    WSDUriDecode(::core::mem::transmute(::windows::core::as_ptr_or_null(source)), source.len() as _, ::core::mem::transmute(destout), ::core::mem::transmute(cchdestout)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDUriEncode(source: &[u16], destout: *mut ::windows::core::PWSTR, cchdestout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDUriEncode(source: ::windows::core::PCWSTR, cchsource: u32, destout: *mut ::windows::core::PWSTR, cchdestout: *mut u32) -> ::windows::core::HRESULT;
    }
    WSDUriEncode(::core::mem::transmute(::windows::core::as_ptr_or_null(source)), source.len() as _, ::core::mem::transmute(destout), ::core::mem::transmute(cchdestout)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
    }
    WSDXMLAddChild(::core::mem::transmute(pparent), ::core::mem::transmute(pchild)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
    }
    WSDXMLAddSibling(::core::mem::transmute(pfirst), ::core::mem::transmute(psecond)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<'a, P0>(pelementname: *mut WSDXML_NAME, psztext: P0, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: ::windows::core::PCWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
    }
    WSDXMLBuildAnyForSingleElement(::core::mem::transmute(pelementname), psztext.into(), ::core::mem::transmute(ppany)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT;
    }
    WSDXMLCleanupElement(::core::mem::transmute(pany)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLCreateContext() -> ::windows::core::Result<IWSDXMLContext> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDXMLCreateContext(ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDXMLCreateContext(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDXMLContext>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<'a, P0, P1>(psznamespace: P0, pszname: P1) -> ::windows::core::Result<*mut WSDXML_NAME>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: ::windows::core::PCWSTR, pszname: ::windows::core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDXMLGetNameFromBuiltinNamespace(psznamespace.into(), pszname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_NAME>(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<'a, P0, P1>(psznamespace: P0, pszname: P1, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WSDXMLGetValueFromAny(psznamespace: ::windows::core::PCWSTR, pszname: ::windows::core::PCWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    WSDXMLGetValueFromAny(psznamespace.into(), pszname.into(), ::core::mem::transmute(pany), ::core::mem::transmute(ppszvalue)).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_ATTRIBUTE {}
impl ::core::clone::Clone for WSDXML_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ATTRIBUTE").field("Element", &self.Element).field("Next", &self.Next).field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_ATTRIBUTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_ATTRIBUTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_ATTRIBUTE {}
impl ::core::default::Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl ::core::marker::Copy for WSDXML_ELEMENT {}
impl ::core::clone::Clone for WSDXML_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT").field("Node", &self.Node).field("Name", &self.Name).field("FirstAttribute", &self.FirstAttribute).field("FirstChild", &self.FirstChild).field("PrefixMappings", &self.PrefixMappings).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_ELEMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_ELEMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT {}
impl ::core::default::Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSDXML_ELEMENT_LIST {}
impl ::core::clone::Clone for WSDXML_ELEMENT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ELEMENT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_ELEMENT_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_ELEMENT_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT_LIST {}
impl ::core::default::Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_NAME {}
impl ::core::clone::Clone for WSDXML_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAME").field("Space", &self.Space).field("LocalName", &self.LocalName).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_NAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_NAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_NAME {}
impl ::core::default::Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NAMESPACE {
    pub Uri: ::windows::core::PCWSTR,
    pub PreferredPrefix: ::windows::core::PCWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl ::core::marker::Copy for WSDXML_NAMESPACE {}
impl ::core::clone::Clone for WSDXML_NAMESPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NAMESPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAMESPACE").field("Uri", &self.Uri).field("PreferredPrefix", &self.PreferredPrefix).field("Names", &self.Names).field("NamesCount", &self.NamesCount).field("Encoding", &self.Encoding).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_NAMESPACE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_NAMESPACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_NAMESPACE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_NAMESPACE {}
impl ::core::default::Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
impl ::core::marker::Copy for WSDXML_NODE {}
impl ::core::clone::Clone for WSDXML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NODE").field("Type", &self.Type).field("Parent", &self.Parent).field("Next", &self.Next).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_NODE {}
impl ::core::default::Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSDXML_OP(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpNone: WSDXML_OP = WSDXML_OP(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndOfTable: WSDXML_OP = WSDXML_OP(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginElement_: WSDXML_OP = WSDXML_OP(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginAnyElement: WSDXML_OP = WSDXML_OP(3i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndElement: WSDXML_OP = WSDXML_OP(4i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpElement_: WSDXML_OP = WSDXML_OP(5i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyElement: WSDXML_OP = WSDXML_OP(6i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyElements: WSDXML_OP = WSDXML_OP(7i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyText: WSDXML_OP = WSDXML_OP(8i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAttribute_: WSDXML_OP = WSDXML_OP(9i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginChoice: WSDXML_OP = WSDXML_OP(10i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndChoice: WSDXML_OP = WSDXML_OP(11i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginSequence: WSDXML_OP = WSDXML_OP(12i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndSequence: WSDXML_OP = WSDXML_OP(13i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginAll: WSDXML_OP = WSDXML_OP(14i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndAll: WSDXML_OP = WSDXML_OP(15i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnything: WSDXML_OP = WSDXML_OP(16i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyNumber: WSDXML_OP = WSDXML_OP(17i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpOneOrMore: WSDXML_OP = WSDXML_OP(18i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpOptional: WSDXML_OP = WSDXML_OP(19i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatBool_: WSDXML_OP = WSDXML_OP(20i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt8_: WSDXML_OP = WSDXML_OP(21i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt16_: WSDXML_OP = WSDXML_OP(22i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt32_: WSDXML_OP = WSDXML_OP(23i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt64_: WSDXML_OP = WSDXML_OP(24i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt8_: WSDXML_OP = WSDXML_OP(25i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt16_: WSDXML_OP = WSDXML_OP(26i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt32_: WSDXML_OP = WSDXML_OP(27i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt64_: WSDXML_OP = WSDXML_OP(28i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUnicodeString_: WSDXML_OP = WSDXML_OP(29i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDom_: WSDXML_OP = WSDXML_OP(30i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatStruct_: WSDXML_OP = WSDXML_OP(31i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUri_: WSDXML_OP = WSDXML_OP(32i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUuidUri_: WSDXML_OP = WSDXML_OP(33i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatName_: WSDXML_OP = WSDXML_OP(34i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatListInsertTail_: WSDXML_OP = WSDXML_OP(35i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatType_: WSDXML_OP = WSDXML_OP(36i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDynamicType_: WSDXML_OP = WSDXML_OP(37i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatLookupType_: WSDXML_OP = WSDXML_OP(38i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDuration_: WSDXML_OP = WSDXML_OP(39i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDateTime_: WSDXML_OP = WSDXML_OP(40i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatFloat_: WSDXML_OP = WSDXML_OP(41i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDouble_: WSDXML_OP = WSDXML_OP(42i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpProcess_: WSDXML_OP = WSDXML_OP(43i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpQualifiedAttribute_: WSDXML_OP = WSDXML_OP(44i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatXMLDeclaration_: WSDXML_OP = WSDXML_OP(45i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatMax: WSDXML_OP = WSDXML_OP(46i32);
impl ::core::marker::Copy for WSDXML_OP {}
impl ::core::clone::Clone for WSDXML_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSDXML_OP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSDXML_OP {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSDXML_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDXML_OP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_PREFIX_MAPPING {}
impl ::core::clone::Clone for WSDXML_PREFIX_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_PREFIX_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_PREFIX_MAPPING").field("Refs", &self.Refs).field("Next", &self.Next).field("Space", &self.Space).field("Prefix", &self.Prefix).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_PREFIX_MAPPING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_PREFIX_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_PREFIX_MAPPING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_PREFIX_MAPPING {}
impl ::core::default::Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_TEXT {}
impl ::core::clone::Clone for WSDXML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TEXT").field("Node", &self.Node).field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_TEXT {}
impl ::core::default::Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_TYPE {
    pub Uri: ::windows::core::PCWSTR,
    pub Table: *const u8,
}
impl ::core::marker::Copy for WSDXML_TYPE {}
impl ::core::clone::Clone for WSDXML_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TYPE").field("Uri", &self.Uri).field("Table", &self.Table).finish()
    }
}
unsafe impl ::windows::core::Abi for WSDXML_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_TYPE {}
impl ::core::default::Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: ::windows::core::PCWSTR,
    pub MessageNumber: u64,
}
impl ::core::marker::Copy for WSD_APP_SEQUENCE {}
impl ::core::clone::Clone for WSD_APP_SEQUENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_APP_SEQUENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_APP_SEQUENCE").field("InstanceId", &self.InstanceId).field("SequenceId", &self.SequenceId).field("MessageNumber", &self.MessageNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_APP_SEQUENCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_APP_SEQUENCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_APP_SEQUENCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_APP_SEQUENCE {}
impl ::core::default::Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_BYE {}
impl ::core::clone::Clone for WSD_BYE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_BYE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_BYE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_BYE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_BYE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_BYE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_BYE {}
impl ::core::default::Default for WSD_BYE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut ::core::option::Option<IWSDAddress>,
    pub dwAddressCount: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_ADDRESSES {}
impl ::core::clone::Clone for WSD_CONFIG_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_ADDRESSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_ADDRESSES").field("addresses", &self.addresses).field("dwAddressCount", &self.dwAddressCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_CONFIG_ADDRESSES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_CONFIG_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_CONFIG_ADDRESSES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_ADDRESSES {}
impl ::core::default::Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut ::core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_PARAM {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_PARAM").field("configParamType", &self.configParamType).field("pConfigData", &self.pConfigData).field("dwConfigDataSize", &self.dwConfigDataSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_CONFIG_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_CONFIG_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_CONFIG_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_PARAM {}
impl ::core::default::Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSD_CONFIG_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(13i32);
impl ::core::marker::Copy for WSD_CONFIG_PARAM_TYPE {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSD_CONFIG_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSD_CONFIG_PARAM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_CONFIG_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DATETIME").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).field("TZIsLocal", &self.TZIsLocal).field("TZIsPositive", &self.TZIsPositive).field("TZHour", &self.TZHour).field("TZMinute", &self.TZMinute).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_DATETIME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_DATETIME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_EVENTING_ADDRESS: &str = "http://*:5357/";
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_HOSTING_ADDRESS: &str = "http://*:5357/";
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: &str = "https://*:5358/";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DURATION").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_DURATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_DURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_DURATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: ::windows::core::PCWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE").field("Address", &self.Address).field("ReferenceProperties", &self.ReferenceProperties).field("ReferenceParameters", &self.ReferenceParameters).field("PortType", &self.PortType).field("ServiceName", &self.ServiceName).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_ENDPOINT_REFERENCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_ENDPOINT_REFERENCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE {}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_ENDPOINT_REFERENCE_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_ENDPOINT_REFERENCE_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENT {
    pub Hr: ::windows::core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: ::windows::core::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: ::core::option::Option<IWSDMessageParameters>,
}
impl ::core::clone::Clone for WSD_EVENT {
    fn clone(&self) -> Self {
        Self {
            Hr: self.Hr,
            EventType: self.EventType,
            DispatchTag: self.DispatchTag,
            HandlerContext: self.HandlerContext.clone(),
            Soap: self.Soap,
            Operation: self.Operation,
            MessageParameters: self.MessageParameters.clone(),
        }
    }
}
impl ::core::fmt::Debug for WSD_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENT").field("Hr", &self.Hr).field("EventType", &self.EventType).field("DispatchTag", &self.DispatchTag).field("HandlerContext", &self.HandlerContext).field("Soap", &self.Soap).field("Operation", &self.Operation).field("MessageParameters", &self.MessageParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WSD_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Hr == other.Hr && self.EventType == other.EventType && self.DispatchTag == other.DispatchTag && self.HandlerContext == other.HandlerContext && self.Soap == other.Soap && self.Operation == other.Operation && self.MessageParameters == other.MessageParameters
    }
}
impl ::core::cmp::Eq for WSD_EVENT {}
impl ::core::default::Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: ::windows::core::PCWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE").field("Mode", &self.Mode).field("Push", &self.Push).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_DELIVERY_MODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_DELIVERY_MODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE_PUSH").field("NotifyTo", &self.NotifyTo).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_DELIVERY_MODE_PUSH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_DELIVERY_MODE_PUSH>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_EVENTING_EXPIRES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENTING_EXPIRES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_EXPIRES").field("Duration", &self.Duration).field("DateTime", &self.DateTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENTING_EXPIRES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENTING_EXPIRES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_EXPIRES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: ::windows::core::PCWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER").field("Dialect", &self.Dialect).field("FilterAction", &self.FilterAction).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_FILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_FILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER {}
impl ::core::default::Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER_ACTION {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER_ACTION").field("Actions", &self.Actions).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_FILTER_ACTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER_ACTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_FILTER_ACTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER_ACTION {}
impl ::core::default::Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut ::core::ffi::c_void,
    pub Unknown: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for WSD_HANDLER_CONTEXT {
    fn clone(&self) -> Self {
        Self { Handler: self.Handler, PVoid: self.PVoid, Unknown: self.Unknown.clone() }
    }
}
impl ::core::fmt::Debug for WSD_HANDLER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HANDLER_CONTEXT").field("Handler", &self.Handler.map(|f| f as usize)).field("PVoid", &self.PVoid).field("Unknown", &self.Unknown).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_HANDLER_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WSD_HANDLER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Handler.map(|f| f as usize) == other.Handler.map(|f| f as usize) && self.PVoid == other.PVoid && self.Unknown == other.Unknown
    }
}
impl ::core::cmp::Eq for WSD_HANDLER_CONTEXT {}
impl ::core::default::Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_HEADER_RELATESTO {}
impl ::core::clone::Clone for WSD_HEADER_RELATESTO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HEADER_RELATESTO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HEADER_RELATESTO").field("RelationshipType", &self.RelationshipType).field("MessageID", &self.MessageID).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_HEADER_RELATESTO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_HEADER_RELATESTO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_HEADER_RELATESTO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_HEADER_RELATESTO {}
impl ::core::default::Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_HELLO {}
impl ::core::clone::Clone for WSD_HELLO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HELLO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HELLO").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_HELLO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_HELLO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_HELLO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_HELLO {}
impl ::core::default::Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
impl ::core::marker::Copy for WSD_HOST_METADATA {}
impl ::core::clone::Clone for WSD_HOST_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HOST_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HOST_METADATA").field("Host", &self.Host).field("Hosted", &self.Hosted).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_HOST_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_HOST_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_HOST_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_HOST_METADATA {}
impl ::core::default::Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_LOCALIZED_STRING {
    pub lang: ::windows::core::PCWSTR,
    pub String: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING").field("lang", &self.lang).field("String", &self.String).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_LOCALIZED_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_LOCALIZED_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING {}
impl ::core::default::Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING_LIST {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_LOCALIZED_STRING_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_LOCALIZED_STRING_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING_LIST {}
impl ::core::default::Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_METADATA_SECTION {
    pub Dialect: ::windows::core::PCWSTR,
    pub Identifier: ::windows::core::PCWSTR,
    pub Data: *mut ::core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION {}
impl ::core::clone::Clone for WSD_METADATA_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_METADATA_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION").field("Dialect", &self.Dialect).field("Identifier", &self.Identifier).field("Data", &self.Data).field("MetadataReference", &self.MetadataReference).field("Location", &self.Location).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_METADATA_SECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_METADATA_SECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION {}
impl ::core::default::Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION_LIST {}
impl ::core::clone::Clone for WSD_METADATA_SECTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_METADATA_SECTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_METADATA_SECTION_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_METADATA_SECTION_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION_LIST {}
impl ::core::default::Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
impl ::core::marker::Copy for WSD_NAME_LIST {}
impl ::core::clone::Clone for WSD_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_NAME_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_NAME_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_NAME_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_NAME_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_NAME_LIST {}
impl ::core::default::Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
impl ::core::marker::Copy for WSD_OPERATION {}
impl ::core::clone::Clone for WSD_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_OPERATION").field("RequestType", &self.RequestType).field("ResponseType", &self.ResponseType).field("RequestStubFunction", &self.RequestStubFunction.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_OPERATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_OPERATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_OPERATION {}
impl ::core::default::Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
impl ::core::marker::Copy for WSD_PORT_TYPE {}
impl ::core::clone::Clone for WSD_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PORT_TYPE").field("EncodedName", &self.EncodedName).field("OperationCount", &self.OperationCount).field("Operations", &self.Operations).field("ProtocolType", &self.ProtocolType).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_PORT_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PORT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PORT_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PORT_TYPE {}
impl ::core::default::Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE {}
impl ::core::clone::Clone for WSD_PROBE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE").field("Types", &self.Types).field("Scopes", &self.Scopes).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE {}
impl ::core::default::Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH {}
impl ::core::clone::Clone for WSD_PROBE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE_MATCH>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH {}
impl ::core::default::Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCHES {}
impl ::core::clone::Clone for WSD_PROBE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCHES").field("ProbeMatch", &self.ProbeMatch).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCHES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE_MATCHES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCHES {}
impl ::core::default::Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH_LIST {}
impl ::core::clone::Clone for WSD_PROBE_MATCH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCH_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE_MATCH_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH_LIST {}
impl ::core::default::Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSD_PROTOCOL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(255i32);
impl ::core::marker::Copy for WSD_PROTOCOL_TYPE {}
impl ::core::clone::Clone for WSD_PROTOCOL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSD_PROTOCOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSD_PROTOCOL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSD_PROTOCOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_PROTOCOL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PARAMETERS {}
impl ::core::clone::Clone for WSD_REFERENCE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_REFERENCE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PARAMETERS").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_REFERENCE_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_REFERENCE_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PARAMETERS {}
impl ::core::default::Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PROPERTIES {}
impl ::core::clone::Clone for WSD_REFERENCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_REFERENCE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PROPERTIES").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_REFERENCE_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_REFERENCE_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PROPERTIES {}
impl ::core::default::Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: ::windows::core::PCWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RELATIONSHIP_METADATA {}
impl ::core::clone::Clone for WSD_RELATIONSHIP_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RELATIONSHIP_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RELATIONSHIP_METADATA").field("Type", &self.Type).field("Data", &self.Data).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_RELATIONSHIP_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RELATIONSHIP_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RELATIONSHIP_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RELATIONSHIP_METADATA {}
impl ::core::default::Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE {}
impl ::core::clone::Clone for WSD_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_RESOLVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RESOLVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE {}
impl ::core::default::Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCH {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_RESOLVE_MATCH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RESOLVE_MATCH>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCH {}
impl ::core::default::Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCHES {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCHES").field("ResolveMatch", &self.ResolveMatch).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_RESOLVE_MATCHES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RESOLVE_MATCHES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCHES {}
impl ::core::default::Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SCOPES {
    pub MatchBy: ::windows::core::PCWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_SCOPES {}
impl ::core::clone::Clone for WSD_SCOPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SCOPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SCOPES").field("MatchBy", &self.MatchBy).field("Scopes", &self.Scopes).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SCOPES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SCOPES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SCOPES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SCOPES {}
impl ::core::default::Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: ::windows::core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).field("pszCNGHashAlgId", &self.pszCNGHashAlgId).field("pbCertHash", &self.pbCertHash).field("dwCertHashSize", &self.dwCertHashSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_CERT_VALIDATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SECURITY_CERT_VALIDATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION_V1").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_CERT_VALIDATION_V1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SECURITY_CERT_VALIDATION_V1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_SIGNATURE_VALIDATION").field("signingCertArray", &self.signingCertArray).field("dwSigningCertArrayCount", &self.dwSigningCertArrayCount).field("hSigningCertStore", &self.hSigningCertStore).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_SIGNATURE_VALIDATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SECURITY_SIGNATURE_VALIDATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("ServiceId", &self.ServiceId).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SERVICE_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SERVICE_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA {}
impl ::core::default::Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA_LIST {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SERVICE_METADATA_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SERVICE_METADATA_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA_LIST {}
impl ::core::default::Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: ::windows::core::PCWSTR,
    pub Role: ::windows::core::PCWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT {}
impl ::core::clone::Clone for WSD_SOAP_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT").field("Code", &self.Code).field("Reason", &self.Reason).field("Node", &self.Node).field("Role", &self.Role).field("Detail", &self.Detail).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT {}
impl ::core::default::Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_CODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_CODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_CODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT_CODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_CODE {}
impl ::core::default::Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_REASON {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_REASON").field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_REASON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT_REASON>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_REASON {}
impl ::core::default::Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_SUBCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_SUBCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_SUBCODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_SUBCODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_SUBCODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT_SUBCODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::default::Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_HEADER {
    pub To: ::windows::core::PCWSTR,
    pub Action: ::windows::core::PCWSTR,
    pub MessageID: ::windows::core::PCWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_HEADER {}
impl ::core::clone::Clone for WSD_SOAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_HEADER").field("To", &self.To).field("Action", &self.Action).field("MessageID", &self.MessageID).field("RelatesTo", &self.RelatesTo).field("ReplyTo", &self.ReplyTo).field("From", &self.From).field("FaultTo", &self.FaultTo).field("AppSequence", &self.AppSequence).field("AnyHeaders", &self.AnyHeaders).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_HEADER {}
impl ::core::default::Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut ::core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
impl ::core::marker::Copy for WSD_SOAP_MESSAGE {}
impl ::core::clone::Clone for WSD_SOAP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_MESSAGE").field("Header", &self.Header).field("Body", &self.Body).field("BodyType", &self.BodyType).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_MESSAGE {}
impl ::core::default::Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSD_STUB_FUNCTION = ::core::option::Option<unsafe extern "system" fn(server: ::core::option::Option<::windows::core::IUnknown>, session: ::core::option::Option<IWSDServiceMessaging>, event: *mut WSD_EVENT) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: ::windows::core::HRESULT,
    pub eventHandle: super::super::Foundation::HANDLE,
    pub messageParameters: ::core::option::Option<IWSDMessageParameters>,
    pub results: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn clone(&self) -> Self {
        Self { hr: self.hr, eventHandle: self.eventHandle, messageParameters: self.messageParameters.clone(), results: self.results }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SYNCHRONOUS_RESPONSE_CONTEXT").field("hr", &self.hr).field("eventHandle", &self.eventHandle).field("messageParameters", &self.messageParameters).field("results", &self.results).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.eventHandle == other.eventHandle && self.messageParameters == other.messageParameters && self.results == other.results
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: ::windows::core::PCWSTR,
    pub SerialNumber: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_DEVICE_METADATA {}
impl ::core::clone::Clone for WSD_THIS_DEVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_THIS_DEVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_DEVICE_METADATA").field("FriendlyName", &self.FriendlyName).field("FirmwareVersion", &self.FirmwareVersion).field("SerialNumber", &self.SerialNumber).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_THIS_DEVICE_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_THIS_DEVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_THIS_DEVICE_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_THIS_DEVICE_METADATA {}
impl ::core::default::Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: ::windows::core::PCWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: ::windows::core::PCWSTR,
    pub ModelUrl: ::windows::core::PCWSTR,
    pub PresentationUrl: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_MODEL_METADATA {}
impl ::core::clone::Clone for WSD_THIS_MODEL_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_THIS_MODEL_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_MODEL_METADATA").field("Manufacturer", &self.Manufacturer).field("ManufacturerUrl", &self.ManufacturerUrl).field("ModelName", &self.ModelName).field("ModelNumber", &self.ModelNumber).field("ModelUrl", &self.ModelUrl).field("PresentationUrl", &self.PresentationUrl).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_THIS_MODEL_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_THIS_MODEL_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_THIS_MODEL_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_THIS_MODEL_METADATA {}
impl ::core::default::Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_UNKNOWN_LOOKUP {}
impl ::core::clone::Clone for WSD_UNKNOWN_LOOKUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_UNKNOWN_LOOKUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_UNKNOWN_LOOKUP").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_UNKNOWN_LOOKUP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_UNKNOWN_LOOKUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_UNKNOWN_LOOKUP>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_UNKNOWN_LOOKUP {}
impl ::core::default::Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_URI_LIST {}
impl ::core::clone::Clone for WSD_URI_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_URI_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_URI_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for WSD_URI_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_URI_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_URI_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_URI_LIST {}
impl ::core::default::Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
