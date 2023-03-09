#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDummyMBNUCMExt(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDummyMBNUCMExt {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IDummyMBNUCMExt, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDummyMBNUCMExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDummyMBNUCMExt {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDummyMBNUCMExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDummyMBNUCMExt").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDummyMBNUCMExt {
    type Vtable = IDummyMBNUCMExt_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDummyMBNUCMExt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IDummyMBNUCMExt {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_ffff_4bbb_aaee_338e368af6fa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDummyMBNUCMExt_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnection(::windows::core::IUnknown);
impl IMbnConnection {
    pub unsafe fn ConnectionID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ConnectionID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).InterfaceID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Connect<P0>(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), connectionmode, strprofile.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionState)(::windows::core::Interface::as_raw(self), connectionstate, ::core::mem::transmute(profilename)).ok()
    }
    pub unsafe fn GetVoiceCallState(&self) -> ::windows::core::Result<MBN_VOICE_CALL_STATE> {
        let mut result__ = ::windows::core::zeroed::<MBN_VOICE_CALL_STATE>();
        (::windows::core::Interface::vtable(self).GetVoiceCallState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActivationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActivationNetworkError)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnConnection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnection {}
impl ::core::fmt::Debug for IMbnConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnection {
    type Vtable = IMbnConnection_Vtbl;
}
impl ::core::clone::Clone for IMbnConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200d_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnectionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetVoiceCallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT,
    pub GetActivationNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionContext(::windows::core::IUnknown);
impl IMbnConnectionContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProvisionedContexts(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetProvisionedContexts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProvisionedContext<P0>(&self, provisionedcontexts: MBN_CONTEXT, providerid: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetProvisionedContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(provisionedcontexts), providerid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionContext {}
impl ::core::fmt::Debug for IMbnConnectionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionContext {
    type Vtable = IMbnConnectionContext_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200b_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProvisionedContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProvisionedContexts: usize,
    pub SetProvisionedContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisionedcontexts: MBN_CONTEXT, providerid: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionContextEvents(::windows::core::IUnknown);
impl IMbnConnectionContextEvents {
    pub unsafe fn OnProvisionedContextListChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnectionContext>,
    {
        (::windows::core::Interface::vtable(self).OnProvisionedContextListChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetProvisionedContextComplete<P0>(&self, newinterface: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnectionContext>,
    {
        (::windows::core::Interface::vtable(self).OnSetProvisionedContextComplete)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi(), requestid, status).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionContextEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionContextEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionContextEvents {}
impl ::core::fmt::Debug for IMbnConnectionContextEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionContextEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionContextEvents {
    type Vtable = IMbnConnectionContextEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionContextEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionContextEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200c_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContextEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnProvisionedContextListChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSetProvisionedContextComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionEvents(::windows::core::IUnknown);
impl IMbnConnectionEvents {
    pub unsafe fn OnConnectComplete<P0>(&self, newconnection: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnection>,
    {
        (::windows::core::Interface::vtable(self).OnConnectComplete)(::windows::core::Interface::as_raw(self), newconnection.into_param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnDisconnectComplete<P0>(&self, newconnection: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnection>,
    {
        (::windows::core::Interface::vtable(self).OnDisconnectComplete)(::windows::core::Interface::as_raw(self), newconnection.into_param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnConnectStateChange<P0>(&self, newconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnection>,
    {
        (::windows::core::Interface::vtable(self).OnConnectStateChange)(::windows::core::Interface::as_raw(self), newconnection.into_param().abi()).ok()
    }
    pub unsafe fn OnVoiceCallStateChange<P0>(&self, newconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnection>,
    {
        (::windows::core::Interface::vtable(self).OnVoiceCallStateChange)(::windows::core::Interface::as_raw(self), newconnection.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionEvents {}
impl ::core::fmt::Debug for IMbnConnectionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionEvents {
    type Vtable = IMbnConnectionEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200e_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnConnectComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnDisconnectComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnConnectStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnVoiceCallStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionManager(::windows::core::IUnknown);
impl IMbnConnectionManager {
    pub unsafe fn GetConnection<P0>(&self, connectionid: P0) -> ::windows::core::Result<IMbnConnection>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMbnConnection>();
        (::windows::core::Interface::vtable(self).GetConnection)(::windows::core::Interface::as_raw(self), connectionid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnections(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetConnections)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionManager {}
impl ::core::fmt::Debug for IMbnConnectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionManager {
    type Vtable = IMbnConnectionManager_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201d_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::PCWSTR, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnections: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionManagerEvents(::windows::core::IUnknown);
impl IMbnConnectionManagerEvents {
    pub unsafe fn OnConnectionArrival<P0>(&self, newconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnection>,
    {
        (::windows::core::Interface::vtable(self).OnConnectionArrival)(::windows::core::Interface::as_raw(self), newconnection.into_param().abi()).ok()
    }
    pub unsafe fn OnConnectionRemoval<P0>(&self, oldconnection: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnection>,
    {
        (::windows::core::Interface::vtable(self).OnConnectionRemoval)(::windows::core::Interface::as_raw(self), oldconnection.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionManagerEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionManagerEvents {}
impl ::core::fmt::Debug for IMbnConnectionManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionManagerEvents {
    type Vtable = IMbnConnectionManagerEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionManagerEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201e_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManagerEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnConnectionArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnConnectionRemoval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfile(::windows::core::IUnknown);
impl IMbnConnectionProfile {
    pub unsafe fn GetProfileXmlData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetProfileXmlData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UpdateProfile<P0>(&self, strprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UpdateProfile)(::windows::core::Interface::as_raw(self), strprofile.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionProfile, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionProfile {}
impl ::core::fmt::Debug for IMbnConnectionProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfile {
    type Vtable = IMbnConnectionProfile_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionProfile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2010_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfile_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProfileXmlData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiledata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UpdateProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprofile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileEvents(::windows::core::IUnknown);
impl IMbnConnectionProfileEvents {
    pub unsafe fn OnProfileUpdate<P0>(&self, newprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnectionProfile>,
    {
        (::windows::core::Interface::vtable(self).OnProfileUpdate)(::windows::core::Interface::as_raw(self), newprofile.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionProfileEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionProfileEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionProfileEvents {}
impl ::core::fmt::Debug for IMbnConnectionProfileEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionProfileEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfileEvents {
    type Vtable = IMbnConnectionProfileEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionProfileEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionProfileEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2011_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnProfileUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileManager(::windows::core::IUnknown);
impl IMbnConnectionProfileManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionProfiles<P0>(&self, mbninterface: P0) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetConnectionProfiles)(::windows::core::Interface::as_raw(self), mbninterface.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectionProfile<P0, P1>(&self, mbninterface: P0, profilename: P1) -> ::windows::core::Result<IMbnConnectionProfile>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMbnConnectionProfile>();
        (::windows::core::Interface::vtable(self).GetConnectionProfile)(::windows::core::Interface::as_raw(self), mbninterface.into_param().abi(), profilename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateConnectionProfile<P0>(&self, xmlprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateConnectionProfile)(::windows::core::Interface::as_raw(self), xmlprofile.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionProfileManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionProfileManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionProfileManager {}
impl ::core::fmt::Debug for IMbnConnectionProfileManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionProfileManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfileManager {
    type Vtable = IMbnConnectionProfileManager_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionProfileManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionProfileManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnectionProfiles: usize,
    pub GetConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, profilename: ::windows::core::PCWSTR, connectionprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlprofile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileManagerEvents(::windows::core::IUnknown);
impl IMbnConnectionProfileManagerEvents {
    pub unsafe fn OnConnectionProfileArrival<P0>(&self, newconnectionprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnectionProfile>,
    {
        (::windows::core::Interface::vtable(self).OnConnectionProfileArrival)(::windows::core::Interface::as_raw(self), newconnectionprofile.into_param().abi()).ok()
    }
    pub unsafe fn OnConnectionProfileRemoval<P0>(&self, oldconnectionprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnConnectionProfile>,
    {
        (::windows::core::Interface::vtable(self).OnConnectionProfileRemoval)(::windows::core::Interface::as_raw(self), oldconnectionprofile.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnConnectionProfileManagerEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnConnectionProfileManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnConnectionProfileManagerEvents {}
impl ::core::fmt::Debug for IMbnConnectionProfileManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnConnectionProfileManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfileManagerEvents {
    type Vtable = IMbnConnectionProfileManagerEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnConnectionProfileManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnConnectionProfileManagerEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManagerEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnConnectionProfileArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnectionprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnConnectionProfileRemoval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldconnectionprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceService(::windows::core::IUnknown);
impl IMbnDeviceService {
    pub unsafe fn QuerySupportedCommands(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).QuerySupportedCommands)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).OpenCommandSession)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CloseCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CloseCommandSession)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetCommand)(::windows::core::Interface::as_raw(self), commandid, deviceservicedata, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).QueryCommand)(::windows::core::Interface::as_raw(self), commandid, deviceservicedata, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).OpenDataSession)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CloseDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CloseDataSession)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).WriteData)(::windows::core::Interface::as_raw(self), deviceservicedata, &mut result__).from_abi(result__)
    }
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).InterfaceID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceServiceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).DeviceServiceID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCommandSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsCommandSessionOpen)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDataSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsDataSessionOpen)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnDeviceService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnDeviceService {}
impl ::core::fmt::Debug for IMbnDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnDeviceService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceService {
    type Vtable = IMbnDeviceService_Vtbl;
}
impl ::core::clone::Clone for IMbnDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnDeviceService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3bb9a71_dc70_4be9_a4da_7886ae8b191b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QuerySupportedCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub OpenCommandSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub CloseCommandSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCommand: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryCommand: usize,
    pub OpenDataSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub CloseDataSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteData: usize,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DeviceServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCommandSessionOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCommandSessionOpen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDataSessionOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDataSessionOpen: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceServiceStateEvents(::windows::core::IUnknown);
impl IMbnDeviceServiceStateEvents {
    pub unsafe fn OnSessionsStateChange<P0>(&self, interfaceid: P0, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).OnSessionsStateChange)(::windows::core::Interface::as_raw(self), interfaceid.into_param().abi(), statechange).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnDeviceServiceStateEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnDeviceServiceStateEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnDeviceServiceStateEvents {}
impl ::core::fmt::Debug for IMbnDeviceServiceStateEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnDeviceServiceStateEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServiceStateEvents {
    type Vtable = IMbnDeviceServiceStateEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnDeviceServiceStateEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnDeviceServiceStateEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d3ff196_89ee_49d8_8b60_33ffddffc58d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServiceStateEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSessionsStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesContext(::windows::core::IUnknown);
impl IMbnDeviceServicesContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateDeviceServices(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).EnumerateDeviceServices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceService<P0>(&self, deviceserviceid: P0) -> ::windows::core::Result<IMbnDeviceService>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMbnDeviceService>();
        (::windows::core::Interface::vtable(self).GetDeviceService)(::windows::core::Interface::as_raw(self), deviceserviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn MaxCommandSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).MaxCommandSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MaxDataSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).MaxDataSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnDeviceServicesContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnDeviceServicesContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnDeviceServicesContext {}
impl ::core::fmt::Debug for IMbnDeviceServicesContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnDeviceServicesContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServicesContext {
    type Vtable = IMbnDeviceServicesContext_Vtbl;
}
impl ::core::clone::Clone for IMbnDeviceServicesContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnDeviceServicesContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc5ac347_1592_4068_80bb_6a57580150d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateDeviceServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateDeviceServices: usize,
    pub GetDeviceService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, mbndeviceservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxCommandSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT,
    pub MaxDataSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesEvents(::windows::core::IUnknown);
impl IMbnDeviceServicesEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuerySupportedCommandsComplete<P0>(&self, deviceservice: P0, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnQuerySupportedCommandsComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), commandidlist, status, requestid).ok()
    }
    pub unsafe fn OnOpenCommandSessionComplete<P0>(&self, deviceservice: P0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnOpenCommandSessionComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), status, requestid).ok()
    }
    pub unsafe fn OnCloseCommandSessionComplete<P0>(&self, deviceservice: P0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnCloseCommandSessionComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), status, requestid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetCommandComplete<P0>(&self, deviceservice: P0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnSetCommandComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), responseid, deviceservicedata, status, requestid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQueryCommandComplete<P0>(&self, deviceservice: P0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnQueryCommandComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), responseid, deviceservicedata, status, requestid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<P0>(&self, deviceservice: P0, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnEventNotification)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), eventid, deviceservicedata).ok()
    }
    pub unsafe fn OnOpenDataSessionComplete<P0>(&self, deviceservice: P0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnOpenDataSessionComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), status, requestid).ok()
    }
    pub unsafe fn OnCloseDataSessionComplete<P0>(&self, deviceservice: P0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnCloseDataSessionComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), status, requestid).ok()
    }
    pub unsafe fn OnWriteDataComplete<P0>(&self, deviceservice: P0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnWriteDataComplete)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), status, requestid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReadData<P0>(&self, deviceservice: P0, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnDeviceService>,
    {
        (::windows::core::Interface::vtable(self).OnReadData)(::windows::core::Interface::as_raw(self), deviceservice.into_param().abi(), deviceservicedata).ok()
    }
    pub unsafe fn OnInterfaceStateChange<P0>(&self, interfaceid: P0, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).OnInterfaceStateChange)(::windows::core::Interface::as_raw(self), interfaceid.into_param().abi(), statechange).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnDeviceServicesEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnDeviceServicesEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnDeviceServicesEvents {}
impl ::core::fmt::Debug for IMbnDeviceServicesEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnDeviceServicesEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServicesEvents {
    type Vtable = IMbnDeviceServicesEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnDeviceServicesEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnDeviceServicesEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a900c19_6824_4e97_b76e_cf239d0ca642);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQuerySupportedCommandsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQuerySupportedCommandsComplete: usize,
    pub OnOpenCommandSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub OnCloseCommandSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetCommandComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQueryCommandComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQueryCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    pub OnOpenDataSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub OnCloseDataSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub OnWriteDataComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnReadData: usize,
    pub OnInterfaceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesManager(::windows::core::IUnknown);
impl IMbnDeviceServicesManager {
    pub unsafe fn GetDeviceServicesContext<P0>(&self, networkinterfaceid: P0) -> ::windows::core::Result<IMbnDeviceServicesContext>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMbnDeviceServicesContext>();
        (::windows::core::Interface::vtable(self).GetDeviceServicesContext)(::windows::core::Interface::as_raw(self), networkinterfaceid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnDeviceServicesManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnDeviceServicesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnDeviceServicesManager {}
impl ::core::fmt::Debug for IMbnDeviceServicesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnDeviceServicesManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServicesManager {
    type Vtable = IMbnDeviceServicesManager_Vtbl;
}
impl ::core::clone::Clone for IMbnDeviceServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnDeviceServicesManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20a26258_6811_4478_ac1d_13324e45e41c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDeviceServicesContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkinterfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, mbndevicescontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterface(::windows::core::IUnknown);
impl IMbnInterface {
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).InterfaceID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInterfaceCapability(&self, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInterfaceCapability)(::windows::core::Interface::as_raw(self), interfacecaps).ok()
    }
    pub unsafe fn GetSubscriberInformation(&self) -> ::windows::core::Result<IMbnSubscriberInformation> {
        let mut result__ = ::windows::core::zeroed::<IMbnSubscriberInformation>();
        (::windows::core::Interface::vtable(self).GetSubscriberInformation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReadyState(&self) -> ::windows::core::Result<MBN_READY_STATE> {
        let mut result__ = ::windows::core::zeroed::<MBN_READY_STATE>();
        (::windows::core::Interface::vtable(self).GetReadyState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InEmergencyMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).InEmergencyMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetHomeProvider(&self) -> ::windows::core::Result<MBN_PROVIDER> {
        let mut result__ = ::windows::core::zeroed::<MBN_PROVIDER>();
        (::windows::core::Interface::vtable(self).GetHomeProvider)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetPreferredProviders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetPreferredProviders)(::windows::core::Interface::as_raw(self), preferredproviders, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVisibleProviders)(::windows::core::Interface::as_raw(self), age, visibleproviders).ok()
    }
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).ScanNetwork)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMbnConnection> {
        let mut result__ = ::windows::core::zeroed::<IMbnConnection>();
        (::windows::core::Interface::vtable(self).GetConnection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnInterface, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnInterface {}
impl ::core::fmt::Debug for IMbnInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnInterface {
    type Vtable = IMbnInterface_Vtbl;
}
impl ::core::clone::Clone for IMbnInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnInterface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2001_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetInterfaceCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT,
    pub GetSubscriberInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriberinformation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InEmergencyMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emergencymode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InEmergencyMode: usize,
    pub GetHomeProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPreferredProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPreferredProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleProviders: usize,
    pub ScanNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterfaceEvents(::windows::core::IUnknown);
impl IMbnInterfaceEvents {
    pub unsafe fn OnInterfaceCapabilityAvailable<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnInterfaceCapabilityAvailable)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSubscriberInformationChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnSubscriberInformationChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnReadyStateChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnReadyStateChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnEmergencyModeChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnEmergencyModeChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnHomeProviderAvailable<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnHomeProviderAvailable)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnPreferredProvidersChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnPreferredProvidersChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetPreferredProvidersComplete<P0>(&self, newinterface: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnSetPreferredProvidersComplete)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnScanNetworkComplete<P0>(&self, newinterface: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnScanNetworkComplete)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi(), requestid, status).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnInterfaceEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnInterfaceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnInterfaceEvents {}
impl ::core::fmt::Debug for IMbnInterfaceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnInterfaceEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnInterfaceEvents {
    type Vtable = IMbnInterfaceEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnInterfaceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnInterfaceEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2002_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnInterfaceCapabilityAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSubscriberInformationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnReadyStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEmergencyModeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnHomeProviderAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSetPreferredProvidersComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterfaceManager(::windows::core::IUnknown);
impl IMbnInterfaceManager {
    pub unsafe fn GetInterface<P0>(&self, interfaceid: P0) -> ::windows::core::Result<IMbnInterface>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMbnInterface>();
        (::windows::core::Interface::vtable(self).GetInterface)(::windows::core::Interface::as_raw(self), interfaceid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetInterfaces)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnInterfaceManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnInterfaceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnInterfaceManager {}
impl ::core::fmt::Debug for IMbnInterfaceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnInterfaceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnInterfaceManager {
    type Vtable = IMbnInterfaceManager_Vtbl;
}
impl ::core::clone::Clone for IMbnInterfaceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnInterfaceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201b_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::windows::core::PCWSTR, mbninterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInterfaces: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterfaceManagerEvents(::windows::core::IUnknown);
impl IMbnInterfaceManagerEvents {
    pub unsafe fn OnInterfaceArrival<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnInterfaceArrival)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnInterfaceRemoval<P0>(&self, oldinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnInterface>,
    {
        (::windows::core::Interface::vtable(self).OnInterfaceRemoval)(::windows::core::Interface::as_raw(self), oldinterface.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnInterfaceManagerEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnInterfaceManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnInterfaceManagerEvents {}
impl ::core::fmt::Debug for IMbnInterfaceManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnInterfaceManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnInterfaceManagerEvents {
    type Vtable = IMbnInterfaceManagerEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnInterfaceManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnInterfaceManagerEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201c_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManagerEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnInterfaceArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnInterfaceRemoval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnMultiCarrier(::windows::core::IUnknown);
impl IMbnMultiCarrier {
    pub unsafe fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetHomeProvider)(::windows::core::Interface::as_raw(self), homeprovider, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetPreferredProviders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVisibleProviders)(::windows::core::Interface::as_raw(self), age, visibleproviders).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedCellularClasses(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetSupportedCellularClasses)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentCellularClass(&self) -> ::windows::core::Result<MBN_CELLULAR_CLASS> {
        let mut result__ = ::windows::core::zeroed::<MBN_CELLULAR_CLASS>();
        (::windows::core::Interface::vtable(self).GetCurrentCellularClass)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).ScanNetwork)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnMultiCarrier, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnMultiCarrier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnMultiCarrier {}
impl ::core::fmt::Debug for IMbnMultiCarrier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnMultiCarrier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnMultiCarrier {
    type Vtable = IMbnMultiCarrier_Vtbl;
}
impl ::core::clone::Clone for IMbnMultiCarrier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnMultiCarrier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2020_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrier_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetHomeProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPreferredProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSupportedCellularClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSupportedCellularClasses: usize,
    pub GetCurrentCellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT,
    pub ScanNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnMultiCarrierEvents(::windows::core::IUnknown);
impl IMbnMultiCarrierEvents {
    pub unsafe fn OnSetHomeProviderComplete<P0>(&self, mbninterface: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnMultiCarrier>,
    {
        (::windows::core::Interface::vtable(self).OnSetHomeProviderComplete)(::windows::core::Interface::as_raw(self), mbninterface.into_param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnCurrentCellularClassChange<P0>(&self, mbninterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnMultiCarrier>,
    {
        (::windows::core::Interface::vtable(self).OnCurrentCellularClassChange)(::windows::core::Interface::as_raw(self), mbninterface.into_param().abi()).ok()
    }
    pub unsafe fn OnPreferredProvidersChange<P0>(&self, mbninterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnMultiCarrier>,
    {
        (::windows::core::Interface::vtable(self).OnPreferredProvidersChange)(::windows::core::Interface::as_raw(self), mbninterface.into_param().abi()).ok()
    }
    pub unsafe fn OnScanNetworkComplete<P0>(&self, mbninterface: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnMultiCarrier>,
    {
        (::windows::core::Interface::vtable(self).OnScanNetworkComplete)(::windows::core::Interface::as_raw(self), mbninterface.into_param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnInterfaceCapabilityChange<P0>(&self, mbninterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnMultiCarrier>,
    {
        (::windows::core::Interface::vtable(self).OnInterfaceCapabilityChange)(::windows::core::Interface::as_raw(self), mbninterface.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnMultiCarrierEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnMultiCarrierEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnMultiCarrierEvents {}
impl ::core::fmt::Debug for IMbnMultiCarrierEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnMultiCarrierEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnMultiCarrierEvents {
    type Vtable = IMbnMultiCarrierEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnMultiCarrierEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnMultiCarrierEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_2021_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrierEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSetHomeProviderComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnCurrentCellularClassChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnInterfaceCapabilityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPin(::windows::core::IUnknown);
impl IMbnPin {
    pub unsafe fn PinType(&self) -> ::windows::core::Result<MBN_PIN_TYPE> {
        let mut result__ = ::windows::core::zeroed::<MBN_PIN_TYPE>();
        (::windows::core::Interface::vtable(self).PinType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PinFormat(&self) -> ::windows::core::Result<MBN_PIN_FORMAT> {
        let mut result__ = ::windows::core::zeroed::<MBN_PIN_FORMAT>();
        (::windows::core::Interface::vtable(self).PinFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PinLengthMin(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).PinLengthMin)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PinLengthMax(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).PinLengthMax)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PinMode(&self) -> ::windows::core::Result<MBN_PIN_MODE> {
        let mut result__ = ::windows::core::zeroed::<MBN_PIN_MODE>();
        (::windows::core::Interface::vtable(self).PinMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Enable<P0>(&self, pin: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Enable)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Disable<P0>(&self, pin: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Disable)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Enter<P0>(&self, pin: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Enter)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Change<P0, P1>(&self, pin: P0, newpin: P1) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Change)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unblock<P0, P1>(&self, puk: P0, newpin: P1) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Unblock)(::windows::core::Interface::as_raw(self), puk.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPinManager(&self) -> ::windows::core::Result<IMbnPinManager> {
        let mut result__ = ::windows::core::zeroed::<IMbnPinManager>();
        (::windows::core::Interface::vtable(self).GetPinManager)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnPin, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnPin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnPin {}
impl ::core::fmt::Debug for IMbnPin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnPin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnPin {
    type Vtable = IMbnPin_Vtbl;
}
impl ::core::clone::Clone for IMbnPin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnPin {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2007_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPin_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PinType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT,
    pub PinFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT,
    pub PinLengthMin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows::core::HRESULT,
    pub PinLengthMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows::core::HRESULT,
    pub PinMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub Enter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub Change: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, newpin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub Unblock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puk: ::windows::core::PCWSTR, newpin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub GetPinManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPinEvents(::windows::core::IUnknown);
impl IMbnPinEvents {
    pub unsafe fn OnEnableComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnPin>,
    {
        (::windows::core::Interface::vtable(self).OnEnableComplete)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnDisableComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnPin>,
    {
        (::windows::core::Interface::vtable(self).OnDisableComplete)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnEnterComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnPin>,
    {
        (::windows::core::Interface::vtable(self).OnEnterComplete)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnChangeComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnPin>,
    {
        (::windows::core::Interface::vtable(self).OnChangeComplete)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnUnblockComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnPin>,
    {
        (::windows::core::Interface::vtable(self).OnUnblockComplete)(::windows::core::Interface::as_raw(self), pin.into_param().abi(), pininfo, requestid, status).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnPinEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnPinEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnPinEvents {}
impl ::core::fmt::Debug for IMbnPinEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnPinEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnPinEvents {
    type Vtable = IMbnPinEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnPinEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnPinEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2008_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnEnableComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnDisableComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnEnterComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnChangeComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnUnblockComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPinManager(::windows::core::IUnknown);
impl IMbnPinManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPinList(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetPinList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPin(&self, pintype: MBN_PIN_TYPE) -> ::windows::core::Result<IMbnPin> {
        let mut result__ = ::windows::core::zeroed::<IMbnPin>();
        (::windows::core::Interface::vtable(self).GetPin)(::windows::core::Interface::as_raw(self), pintype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPinState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPinState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnPinManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnPinManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnPinManager {}
impl ::core::fmt::Debug for IMbnPinManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnPinManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnPinManager {
    type Vtable = IMbnPinManager_Vtbl;
}
impl ::core::clone::Clone for IMbnPinManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnPinManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2005_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPinList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPinList: usize,
    pub GetPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPinState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPinManagerEvents(::windows::core::IUnknown);
impl IMbnPinManagerEvents {
    pub unsafe fn OnPinListAvailable<P0>(&self, pinmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnPinManager>,
    {
        (::windows::core::Interface::vtable(self).OnPinListAvailable)(::windows::core::Interface::as_raw(self), pinmanager.into_param().abi()).ok()
    }
    pub unsafe fn OnGetPinStateComplete<P0>(&self, pinmanager: P0, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnPinManager>,
    {
        (::windows::core::Interface::vtable(self).OnGetPinStateComplete)(::windows::core::Interface::as_raw(self), pinmanager.into_param().abi(), ::core::mem::transmute(pininfo), requestid, status).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnPinManagerEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnPinManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnPinManagerEvents {}
impl ::core::fmt::Debug for IMbnPinManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnPinManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnPinManagerEvents {
    type Vtable = IMbnPinManagerEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnPinManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnPinManagerEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2006_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManagerEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnPinListAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnGetPinStateComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: *mut ::core::ffi::c_void, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnRadio(::windows::core::IUnknown);
impl IMbnRadio {
    pub unsafe fn SoftwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__ = ::windows::core::zeroed::<MBN_RADIO>();
        (::windows::core::Interface::vtable(self).SoftwareRadioState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn HardwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__ = ::windows::core::zeroed::<MBN_RADIO>();
        (::windows::core::Interface::vtable(self).HardwareRadioState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetSoftwareRadioState)(::windows::core::Interface::as_raw(self), radiostate, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnRadio, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnRadio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnRadio {}
impl ::core::fmt::Debug for IMbnRadio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnRadio").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnRadio {
    type Vtable = IMbnRadio_Vtbl;
}
impl ::core::clone::Clone for IMbnRadio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnRadio {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdccccab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadio_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SoftwareRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub HardwareRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub SetSoftwareRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnRadioEvents(::windows::core::IUnknown);
impl IMbnRadioEvents {
    pub unsafe fn OnRadioStateChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnRadio>,
    {
        (::windows::core::Interface::vtable(self).OnRadioStateChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetSoftwareRadioStateComplete<P0>(&self, newinterface: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnRadio>,
    {
        (::windows::core::Interface::vtable(self).OnSetSoftwareRadioStateComplete)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi(), requestid, status).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnRadioEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnRadioEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnRadioEvents {}
impl ::core::fmt::Debug for IMbnRadioEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnRadioEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnRadioEvents {
    type Vtable = IMbnRadioEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnRadioEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnRadioEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadioEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnRadioStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSetSoftwareRadioStateComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnRegistration(::windows::core::IUnknown);
impl IMbnRegistration {
    pub unsafe fn GetRegisterState(&self) -> ::windows::core::Result<MBN_REGISTER_STATE> {
        let mut result__ = ::windows::core::zeroed::<MBN_REGISTER_STATE>();
        (::windows::core::Interface::vtable(self).GetRegisterState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRegisterMode(&self) -> ::windows::core::Result<MBN_REGISTER_MODE> {
        let mut result__ = ::windows::core::zeroed::<MBN_REGISTER_MODE>();
        (::windows::core::Interface::vtable(self).GetRegisterMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProviderID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetProviderID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetProviderName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRoamingText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetRoamingText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAvailableDataClasses(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetAvailableDataClasses)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentDataClass(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCurrentDataClass)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRegistrationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetRegistrationNetworkError)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPacketAttachNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPacketAttachNetworkError)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRegisterMode<P0>(&self, registermode: MBN_REGISTER_MODE, providerid: P0, dataclass: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetRegisterMode)(::windows::core::Interface::as_raw(self), registermode, providerid.into_param().abi(), dataclass, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnRegistration, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnRegistration {}
impl ::core::fmt::Debug for IMbnRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnRegistration {
    type Vtable = IMbnRegistration_Vtbl;
}
impl ::core::clone::Clone for IMbnRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2009_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRegisterState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT,
    pub GetRegisterMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT,
    pub GetProviderID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetRoamingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roamingtext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetAvailableDataClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows::core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows::core::HRESULT,
    pub GetRegistrationNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT,
    pub GetPacketAttachNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT,
    pub SetRegisterMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: ::windows::core::PCWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnRegistrationEvents(::windows::core::IUnknown);
impl IMbnRegistrationEvents {
    pub unsafe fn OnRegisterModeAvailable<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnRegistration>,
    {
        (::windows::core::Interface::vtable(self).OnRegisterModeAvailable)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnRegisterStateChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnRegistration>,
    {
        (::windows::core::Interface::vtable(self).OnRegisterStateChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnPacketServiceStateChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnRegistration>,
    {
        (::windows::core::Interface::vtable(self).OnPacketServiceStateChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetRegisterModeComplete<P0>(&self, newinterface: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnRegistration>,
    {
        (::windows::core::Interface::vtable(self).OnSetRegisterModeComplete)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi(), requestid, status).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnRegistrationEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnRegistrationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnRegistrationEvents {}
impl ::core::fmt::Debug for IMbnRegistrationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnRegistrationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnRegistrationEvents {
    type Vtable = IMbnRegistrationEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnRegistrationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnRegistrationEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200a_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistrationEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnRegisterModeAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnRegisterStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnPacketServiceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSetRegisterModeComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnServiceActivation(::windows::core::IUnknown);
impl IMbnServiceActivation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), vendorspecificdata, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnServiceActivation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnServiceActivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnServiceActivation {}
impl ::core::fmt::Debug for IMbnServiceActivation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnServiceActivation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnServiceActivation {
    type Vtable = IMbnServiceActivation_Vtbl;
}
impl ::core::clone::Clone for IMbnServiceActivation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnServiceActivation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2017_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activate: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnServiceActivationEvents(::windows::core::IUnknown);
impl IMbnServiceActivationEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivationComplete<P0>(&self, serviceactivation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnServiceActivation>,
    {
        (::windows::core::Interface::vtable(self).OnActivationComplete)(::windows::core::Interface::as_raw(self), serviceactivation.into_param().abi(), vendorspecificdata, requestid, status, networkerror).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnServiceActivationEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnServiceActivationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnServiceActivationEvents {}
impl ::core::fmt::Debug for IMbnServiceActivationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnServiceActivationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnServiceActivationEvents {
    type Vtable = IMbnServiceActivationEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnServiceActivationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnServiceActivationEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2018_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivationEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceactivation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivationComplete: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSignal(::windows::core::IUnknown);
impl IMbnSignal {
    pub unsafe fn GetSignalStrength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSignalStrength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignalError(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSignalError)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnSignal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSignal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSignal {}
impl ::core::fmt::Debug for IMbnSignal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSignal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSignal {
    type Vtable = IMbnSignal_Vtbl;
}
impl ::core::clone::Clone for IMbnSignal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSignal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2003_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSignalStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignalError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSignalEvents(::windows::core::IUnknown);
impl IMbnSignalEvents {
    pub unsafe fn OnSignalStateChange<P0>(&self, newinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSignal>,
    {
        (::windows::core::Interface::vtable(self).OnSignalStateChange)(::windows::core::Interface::as_raw(self), newinterface.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnSignalEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSignalEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSignalEvents {}
impl ::core::fmt::Debug for IMbnSignalEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSignalEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSignalEvents {
    type Vtable = IMbnSignalEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnSignalEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSignalEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2004_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignalEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSignalStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSms(::windows::core::IUnknown);
impl IMbnSms {
    pub unsafe fn GetSmsConfiguration(&self) -> ::windows::core::Result<IMbnSmsConfiguration> {
        let mut result__ = ::windows::core::zeroed::<IMbnSmsConfiguration>();
        (::windows::core::Interface::vtable(self).GetSmsConfiguration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSmsConfiguration<P0>(&self, smsconfiguration: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IMbnSmsConfiguration>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetSmsConfiguration)(::windows::core::Interface::as_raw(self), smsconfiguration.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SmsSendPdu<P0>(&self, pdudata: P0, size: u8) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SmsSendPdu)(::windows::core::Interface::as_raw(self), pdudata.into_param().abi(), size, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdma<P0>(&self, address: P0, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SmsSendCdma)(::windows::core::Interface::as_raw(self), address.into_param().abi(), encoding, language, sizeincharacters, message, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SmsSendCdmaPdu)(::windows::core::Interface::as_raw(self), message, &mut result__).from_abi(result__)
    }
    pub unsafe fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SmsRead)(::windows::core::Interface::as_raw(self), smsfilter, smsformat, &mut result__).from_abi(result__)
    }
    pub unsafe fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SmsDelete)(::windows::core::Interface::as_raw(self), smsfilter, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSmsStatus(&self) -> ::windows::core::Result<MBN_SMS_STATUS_INFO> {
        let mut result__ = ::windows::core::zeroed::<MBN_SMS_STATUS_INFO>();
        (::windows::core::Interface::vtable(self).GetSmsStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnSms, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSms {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSms {}
impl ::core::fmt::Debug for IMbnSms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSms").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSms {
    type Vtable = IMbnSms_Vtbl;
}
impl ::core::clone::Clone for IMbnSms {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSms {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2015_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSms_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSmsConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSmsConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub SmsSendPdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdudata: ::windows::core::PCWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SmsSendCdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::windows::core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmsSendCdma: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SmsSendCdmaPdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmsSendCdmaPdu: usize,
    pub SmsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub SmsDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub GetSmsStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSmsConfiguration(::windows::core::IUnknown);
impl IMbnSmsConfiguration {
    pub unsafe fn ServiceCenterAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ServiceCenterAddress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceCenterAddress<P0>(&self, scaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetServiceCenterAddress)(::windows::core::Interface::as_raw(self), scaddress.into_param().abi()).ok()
    }
    pub unsafe fn MaxMessageIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).MaxMessageIndex)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CdmaShortMsgSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CdmaShortMsgSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SmsFormat(&self) -> ::windows::core::Result<MBN_SMS_FORMAT> {
        let mut result__ = ::windows::core::zeroed::<MBN_SMS_FORMAT>();
        (::windows::core::Interface::vtable(self).SmsFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSmsFormat)(::windows::core::Interface::as_raw(self), smsformat).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnSmsConfiguration, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSmsConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSmsConfiguration {}
impl ::core::fmt::Debug for IMbnSmsConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSmsConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsConfiguration {
    type Vtable = IMbnSmsConfiguration_Vtbl;
}
impl ::core::clone::Clone for IMbnSmsConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSmsConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2012_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsConfiguration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ServiceCenterAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetServiceCenterAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaddress: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub MaxMessageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub CdmaShortMsgSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows::core::HRESULT,
    pub SmsFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT,
    pub SetSmsFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSmsEvents(::windows::core::IUnknown);
impl IMbnSmsEvents {
    pub unsafe fn OnSmsConfigurationChange<P0>(&self, sms: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSms>,
    {
        (::windows::core::Interface::vtable(self).OnSmsConfigurationChange)(::windows::core::Interface::as_raw(self), sms.into_param().abi()).ok()
    }
    pub unsafe fn OnSetSmsConfigurationComplete<P0>(&self, sms: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSms>,
    {
        (::windows::core::Interface::vtable(self).OnSetSmsConfigurationComplete)(::windows::core::Interface::as_raw(self), sms.into_param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnSmsSendComplete<P0>(&self, sms: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSms>,
    {
        (::windows::core::Interface::vtable(self).OnSmsSendComplete)(::windows::core::Interface::as_raw(self), sms.into_param().abi(), requestid, status).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnSmsReadComplete<P0, P1>(&self, sms: P0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: P1, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSms>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnSmsReadComplete)(::windows::core::Interface::as_raw(self), sms.into_param().abi(), smsformat, readmsgs, moremsgs.into_param().abi(), requestid, status).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsNewClass0Message<P0>(&self, sms: P0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSms>,
    {
        (::windows::core::Interface::vtable(self).OnSmsNewClass0Message)(::windows::core::Interface::as_raw(self), sms.into_param().abi(), smsformat, readmsgs).ok()
    }
    pub unsafe fn OnSmsDeleteComplete<P0>(&self, sms: P0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSms>,
    {
        (::windows::core::Interface::vtable(self).OnSmsDeleteComplete)(::windows::core::Interface::as_raw(self), sms.into_param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnSmsStatusChange<P0>(&self, sms: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnSms>,
    {
        (::windows::core::Interface::vtable(self).OnSmsStatusChange)(::windows::core::Interface::as_raw(self), sms.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnSmsEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSmsEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSmsEvents {}
impl ::core::fmt::Debug for IMbnSmsEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSmsEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsEvents {
    type Vtable = IMbnSmsEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnSmsEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSmsEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2016_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSmsConfigurationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSetSmsConfigurationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnSmsSendComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnSmsReadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnSmsReadComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSmsNewClass0Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSmsNewClass0Message: usize,
    pub OnSmsDeleteComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnSmsStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSmsReadMsgPdu(::windows::core::IUnknown);
impl IMbnSmsReadMsgPdu {
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Index)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__ = ::windows::core::zeroed::<MBN_MSG_STATUS>();
        (::windows::core::Interface::vtable(self).Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PduData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).PduData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).Message)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnSmsReadMsgPdu, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSmsReadMsgPdu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSmsReadMsgPdu {}
impl ::core::fmt::Debug for IMbnSmsReadMsgPdu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSmsReadMsgPdu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsReadMsgPdu {
    type Vtable = IMbnSmsReadMsgPdu_Vtbl;
}
impl ::core::clone::Clone for IMbnSmsReadMsgPdu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSmsReadMsgPdu {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2013_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgPdu_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    pub PduData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdudata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSmsReadMsgTextCdma(::windows::core::IUnknown);
impl IMbnSmsReadMsgTextCdma {
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Index)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__ = ::windows::core::zeroed::<MBN_MSG_STATUS>();
        (::windows::core::Interface::vtable(self).Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Address(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Address)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Timestamp)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EncodingID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_ENCODING> {
        let mut result__ = ::windows::core::zeroed::<MBN_SMS_CDMA_ENCODING>();
        (::windows::core::Interface::vtable(self).EncodingID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LanguageID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_LANG> {
        let mut result__ = ::windows::core::zeroed::<MBN_SMS_CDMA_LANG>();
        (::windows::core::Interface::vtable(self).LanguageID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SizeInCharacters(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SizeInCharacters)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).Message)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnSmsReadMsgTextCdma, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSmsReadMsgTextCdma {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSmsReadMsgTextCdma {}
impl ::core::fmt::Debug for IMbnSmsReadMsgTextCdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSmsReadMsgTextCdma").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsReadMsgTextCdma {
    type Vtable = IMbnSmsReadMsgTextCdma_Vtbl;
}
impl ::core::clone::Clone for IMbnSmsReadMsgTextCdma {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSmsReadMsgTextCdma {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2014_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgTextCdma_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub EncodingID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT,
    pub LanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT,
    pub SizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSubscriberInformation(::windows::core::IUnknown);
impl IMbnSubscriberInformation {
    pub unsafe fn SubscriberID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).SubscriberID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SimIccID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).SimIccID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TelephoneNumbers(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::super::System::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).TelephoneNumbers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnSubscriberInformation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnSubscriberInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnSubscriberInformation {}
impl ::core::fmt::Debug for IMbnSubscriberInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnSubscriberInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnSubscriberInformation {
    type Vtable = IMbnSubscriberInformation_Vtbl;
}
impl ::core::clone::Clone for IMbnSubscriberInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnSubscriberInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x459ecc43_bcf5_11dc_a8a8_001321f1405f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSubscriberInformation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SubscriberID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriberid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SimIccID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simiccid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TelephoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TelephoneNumbers: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnVendorSpecificEvents(::windows::core::IUnknown);
impl IMbnVendorSpecificEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<P0>(&self, vendoroperation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnVendorSpecificOperation>,
    {
        (::windows::core::Interface::vtable(self).OnEventNotification)(::windows::core::Interface::as_raw(self), vendoroperation.into_param().abi(), vendorspecificdata).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetVendorSpecificComplete<P0>(&self, vendoroperation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMbnVendorSpecificOperation>,
    {
        (::windows::core::Interface::vtable(self).OnSetVendorSpecificComplete)(::windows::core::Interface::as_raw(self), vendoroperation.into_param().abi(), vendorspecificdata, requestid).ok()
    }
}
::windows::imp::interface_hierarchy!(IMbnVendorSpecificEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnVendorSpecificEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnVendorSpecificEvents {}
impl ::core::fmt::Debug for IMbnVendorSpecificEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnVendorSpecificEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnVendorSpecificEvents {
    type Vtable = IMbnVendorSpecificEvents_Vtbl;
}
impl ::core::clone::Clone for IMbnVendorSpecificEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnVendorSpecificEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201a_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetVendorSpecificComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetVendorSpecificComplete: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnVendorSpecificOperation(::windows::core::IUnknown);
impl IMbnVendorSpecificOperation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).SetVendorSpecific)(::windows::core::Interface::as_raw(self), vendorspecificdata, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMbnVendorSpecificOperation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMbnVendorSpecificOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMbnVendorSpecificOperation {}
impl ::core::fmt::Debug for IMbnVendorSpecificOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMbnVendorSpecificOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMbnVendorSpecificOperation {
    type Vtable = IMbnVendorSpecificOperation_Vtbl;
}
impl ::core::clone::Clone for IMbnVendorSpecificOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMbnVendorSpecificOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2019_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificOperation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetVendorSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetVendorSpecific: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnConnectionManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05c_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnConnectionProfileManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05a_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnDeviceServicesManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2269daa3_2a9f_4165_a501_ce00a6f7a75b);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnInterfaceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05b_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_ACTIVATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_NONE: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_ACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_ACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(4i32);
impl ::core::marker::Copy for MBN_ACTIVATION_STATE {}
impl ::core::clone::Clone for MBN_ACTIVATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_ACTIVATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_ACTIVATION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_ACTIVATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_ACTIVATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_AUTH_PROTOCOL(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_NONE: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_PAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_CHAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(3i32);
impl ::core::marker::Copy for MBN_AUTH_PROTOCOL {}
impl ::core::clone::Clone for MBN_AUTH_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_AUTH_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_AUTH_PROTOCOL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_AUTH_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_AUTH_PROTOCOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_BAND_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_NONE: MBN_BAND_CLASS = MBN_BAND_CLASS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_0: MBN_BAND_CLASS = MBN_BAND_CLASS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_I: MBN_BAND_CLASS = MBN_BAND_CLASS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_II: MBN_BAND_CLASS = MBN_BAND_CLASS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_III: MBN_BAND_CLASS = MBN_BAND_CLASS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_IV: MBN_BAND_CLASS = MBN_BAND_CLASS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_V: MBN_BAND_CLASS = MBN_BAND_CLASS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VI: MBN_BAND_CLASS = MBN_BAND_CLASS(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VII: MBN_BAND_CLASS = MBN_BAND_CLASS(128i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VIII: MBN_BAND_CLASS = MBN_BAND_CLASS(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_IX: MBN_BAND_CLASS = MBN_BAND_CLASS(512i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_X: MBN_BAND_CLASS = MBN_BAND_CLASS(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XI: MBN_BAND_CLASS = MBN_BAND_CLASS(2048i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XII: MBN_BAND_CLASS = MBN_BAND_CLASS(4096i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XIII: MBN_BAND_CLASS = MBN_BAND_CLASS(8192i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XIV: MBN_BAND_CLASS = MBN_BAND_CLASS(16384i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XV: MBN_BAND_CLASS = MBN_BAND_CLASS(32768i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XVI: MBN_BAND_CLASS = MBN_BAND_CLASS(65536i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XVII: MBN_BAND_CLASS = MBN_BAND_CLASS(131072i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_CUSTOM: MBN_BAND_CLASS = MBN_BAND_CLASS(-2147483648i32);
impl ::core::marker::Copy for MBN_BAND_CLASS {}
impl ::core::clone::Clone for MBN_BAND_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_BAND_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_BAND_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_BAND_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_BAND_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_CELLULAR_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_NONE: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_GSM: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_CDMA: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(2i32);
impl ::core::marker::Copy for MBN_CELLULAR_CLASS {}
impl ::core::clone::Clone for MBN_CELLULAR_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_CELLULAR_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_CELLULAR_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_CELLULAR_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CELLULAR_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_COMPRESSION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_COMPRESSION_NONE: MBN_COMPRESSION = MBN_COMPRESSION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_COMPRESSION_ENABLE: MBN_COMPRESSION = MBN_COMPRESSION(1i32);
impl ::core::marker::Copy for MBN_COMPRESSION {}
impl ::core::clone::Clone for MBN_COMPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_COMPRESSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_COMPRESSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_COMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_COMPRESSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_CONNECTION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONNECTION_MODE_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONNECTION_MODE_TMP_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(1i32);
impl ::core::marker::Copy for MBN_CONNECTION_MODE {}
impl ::core::clone::Clone for MBN_CONNECTION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_CONNECTION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_CONNECTION_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_CONNECTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CONNECTION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_CONTEXT_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACCESSSTRING_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(100i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_USERNAME_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PASSWORD_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_ID_APPEND: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(-1i32);
impl ::core::marker::Copy for MBN_CONTEXT_CONSTANTS {}
impl ::core::clone::Clone for MBN_CONTEXT_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_CONTEXT_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_CONTEXT_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_CONTEXT_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CONTEXT_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_CONTEXT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_NONE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_INTERNET: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VPN: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VOICE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_CUSTOM: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_PURCHASE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(6i32);
impl ::core::marker::Copy for MBN_CONTEXT_TYPE {}
impl ::core::clone::Clone for MBN_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_CONTEXT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_CTRL_CAPS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_NONE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_REG_MANUAL: MBN_CTRL_CAPS = MBN_CTRL_CAPS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: MBN_CTRL_CAPS = MBN_CTRL_CAPS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: MBN_CTRL_CAPS = MBN_CTRL_CAPS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: MBN_CTRL_CAPS = MBN_CTRL_CAPS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_USSD: MBN_CTRL_CAPS = MBN_CTRL_CAPS(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_MULTI_MODE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(128i32);
impl ::core::marker::Copy for MBN_CTRL_CAPS {}
impl ::core::clone::Clone for MBN_CTRL_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_CTRL_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_CTRL_CAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_CTRL_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CTRL_CAPS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_DATA_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_NONE: MBN_DATA_CLASS = MBN_DATA_CLASS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_GPRS: MBN_DATA_CLASS = MBN_DATA_CLASS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_EDGE: MBN_DATA_CLASS = MBN_DATA_CLASS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_UMTS: MBN_DATA_CLASS = MBN_DATA_CLASS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_HSDPA: MBN_DATA_CLASS = MBN_DATA_CLASS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_HSUPA: MBN_DATA_CLASS = MBN_DATA_CLASS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_LTE: MBN_DATA_CLASS = MBN_DATA_CLASS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_5G_NSA: MBN_DATA_CLASS = MBN_DATA_CLASS(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_5G_SA: MBN_DATA_CLASS = MBN_DATA_CLASS(128i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XRTT: MBN_DATA_CLASS = MBN_DATA_CLASS(65536i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO: MBN_DATA_CLASS = MBN_DATA_CLASS(131072i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO_REVA: MBN_DATA_CLASS = MBN_DATA_CLASS(262144i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDV: MBN_DATA_CLASS = MBN_DATA_CLASS(524288i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_3XRTT: MBN_DATA_CLASS = MBN_DATA_CLASS(1048576i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO_REVB: MBN_DATA_CLASS = MBN_DATA_CLASS(2097152i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_UMB: MBN_DATA_CLASS = MBN_DATA_CLASS(4194304i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_CUSTOM: MBN_DATA_CLASS = MBN_DATA_CLASS(-2147483648i32);
impl ::core::marker::Copy for MBN_DATA_CLASS {}
impl ::core::clone::Clone for MBN_DATA_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_DATA_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_DATA_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_DATA_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_DATA_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_DEVICE_SERVICES_INTERFACE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(1i32);
impl ::core::marker::Copy for MBN_DEVICE_SERVICES_INTERFACE_STATE {}
impl ::core::clone::Clone for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_DEVICE_SERVICES_INTERFACE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_DEVICE_SERVICE_SESSIONS_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: MBN_DEVICE_SERVICE_SESSIONS_STATE = MBN_DEVICE_SERVICE_SESSIONS_STATE(0i32);
impl ::core::marker::Copy for MBN_DEVICE_SERVICE_SESSIONS_STATE {}
impl ::core::clone::Clone for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_DEVICE_SERVICE_SESSIONS_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_INTERFACE_CAPS_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICEID_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(18i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MANUFACTURER_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MODEL_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_FIRMWARE_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
impl ::core::marker::Copy for MBN_INTERFACE_CAPS_CONSTANTS {}
impl ::core::clone::Clone for MBN_INTERFACE_CAPS_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_INTERFACE_CAPS_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_INTERFACE_CAPS_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_INTERFACE_CAPS_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_INTERFACE_CAPS_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_MSG_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_NEW: MBN_MSG_STATUS = MBN_MSG_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_OLD: MBN_MSG_STATUS = MBN_MSG_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_DRAFT: MBN_MSG_STATUS = MBN_MSG_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_SENT: MBN_MSG_STATUS = MBN_MSG_STATUS(3i32);
impl ::core::marker::Copy for MBN_MSG_STATUS {}
impl ::core::clone::Clone for MBN_MSG_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_MSG_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_MSG_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_MSG_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_MSG_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_PIN_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_LENGTH_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
impl ::core::marker::Copy for MBN_PIN_CONSTANTS {}
impl ::core::clone::Clone for MBN_PIN_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_PIN_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_PIN_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_PIN_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_PIN_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_NONE: MBN_PIN_FORMAT = MBN_PIN_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_NUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_ALPHANUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(2i32);
impl ::core::marker::Copy for MBN_PIN_FORMAT {}
impl ::core::clone::Clone for MBN_PIN_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_PIN_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_PIN_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_PIN_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_PIN_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_MODE_ENABLED: MBN_PIN_MODE = MBN_PIN_MODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_MODE_DISABLED: MBN_PIN_MODE = MBN_PIN_MODE(2i32);
impl ::core::marker::Copy for MBN_PIN_MODE {}
impl ::core::clone::Clone for MBN_PIN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_PIN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_PIN_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_PIN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_PIN_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_NONE: MBN_PIN_STATE = MBN_PIN_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_ENTER: MBN_PIN_STATE = MBN_PIN_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_UNBLOCK: MBN_PIN_STATE = MBN_PIN_STATE(2i32);
impl ::core::marker::Copy for MBN_PIN_STATE {}
impl ::core::clone::Clone for MBN_PIN_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_PIN_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_PIN_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_PIN_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_PIN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NONE: MBN_PIN_TYPE = MBN_PIN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_CUSTOM: MBN_PIN_TYPE = MBN_PIN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_PIN1: MBN_PIN_TYPE = MBN_PIN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_PIN2: MBN_PIN_TYPE = MBN_PIN_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_DEVICE_SIM_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NETWORK_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NETWORK_SUBSET_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_SVC_PROVIDER_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_CORPORATE_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_SUBSIDY_LOCK: MBN_PIN_TYPE = MBN_PIN_TYPE(10i32);
impl ::core::marker::Copy for MBN_PIN_TYPE {}
impl ::core::clone::Clone for MBN_PIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_PIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_PIN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_PIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_PROVIDER_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDERNAME_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(20i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDERID_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(6i32);
impl ::core::marker::Copy for MBN_PROVIDER_CONSTANTS {}
impl ::core::clone::Clone for MBN_PROVIDER_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_PROVIDER_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_PROVIDER_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_PROVIDER_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PROVIDER_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_PROVIDER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_NONE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_HOME: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_FORBIDDEN: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_PREFERRED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_VISIBLE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_REGISTERED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(32i32);
impl ::core::marker::Copy for MBN_PROVIDER_STATE {}
impl ::core::clone::Clone for MBN_PROVIDER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_PROVIDER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_PROVIDER_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_PROVIDER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PROVIDER_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_RADIO(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RADIO_OFF: MBN_RADIO = MBN_RADIO(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RADIO_ON: MBN_RADIO = MBN_RADIO(1i32);
impl ::core::marker::Copy for MBN_RADIO {}
impl ::core::clone::Clone for MBN_RADIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_RADIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_RADIO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_RADIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_RADIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_READY_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_OFF: MBN_READY_STATE = MBN_READY_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_INITIALIZED: MBN_READY_STATE = MBN_READY_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_SIM_NOT_INSERTED: MBN_READY_STATE = MBN_READY_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_BAD_SIM: MBN_READY_STATE = MBN_READY_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_FAILURE: MBN_READY_STATE = MBN_READY_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_NOT_ACTIVATED: MBN_READY_STATE = MBN_READY_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_DEVICE_LOCKED: MBN_READY_STATE = MBN_READY_STATE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_DEVICE_BLOCKED: MBN_READY_STATE = MBN_READY_STATE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_NO_ESIM_PROFILE: MBN_READY_STATE = MBN_READY_STATE(8i32);
impl ::core::marker::Copy for MBN_READY_STATE {}
impl ::core::clone::Clone for MBN_READY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_READY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_READY_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_READY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_READY_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_REGISTER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_NONE: MBN_REGISTER_MODE = MBN_REGISTER_MODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_AUTOMATIC: MBN_REGISTER_MODE = MBN_REGISTER_MODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_MANUAL: MBN_REGISTER_MODE = MBN_REGISTER_MODE(2i32);
impl ::core::marker::Copy for MBN_REGISTER_MODE {}
impl ::core::clone::Clone for MBN_REGISTER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_REGISTER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_REGISTER_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_REGISTER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_REGISTER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_REGISTER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_NONE: MBN_REGISTER_STATE = MBN_REGISTER_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_DEREGISTERED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_SEARCHING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_HOME: MBN_REGISTER_STATE = MBN_REGISTER_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_ROAMING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_PARTNER: MBN_REGISTER_STATE = MBN_REGISTER_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_DENIED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(6i32);
impl ::core::marker::Copy for MBN_REGISTER_STATE {}
impl ::core::clone::Clone for MBN_REGISTER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_REGISTER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_REGISTER_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_REGISTER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_REGISTER_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_REGISTRATION_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ROAMTEXT_LEN: MBN_REGISTRATION_CONSTANTS = MBN_REGISTRATION_CONSTANTS(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_DEFAULT_PROVIDER_ID: MBN_REGISTRATION_CONSTANTS = MBN_REGISTRATION_CONSTANTS(0i32);
impl ::core::marker::Copy for MBN_REGISTRATION_CONSTANTS {}
impl ::core::clone::Clone for MBN_REGISTRATION_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_REGISTRATION_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_REGISTRATION_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_REGISTRATION_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_REGISTRATION_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_SIGNAL_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_DEFAULT: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(-1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_DISABLE: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ERROR_RATE_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
impl ::core::marker::Copy for MBN_SIGNAL_CONSTANTS {}
impl ::core::clone::Clone for MBN_SIGNAL_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_SIGNAL_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_SIGNAL_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_SIGNAL_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SIGNAL_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_SMS_CAPS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_NONE: MBN_SMS_CAPS = MBN_SMS_CAPS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_PDU_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_PDU_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_TEXT_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_TEXT_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(8i32);
impl ::core::marker::Copy for MBN_SMS_CAPS {}
impl ::core::clone::Clone for MBN_SMS_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_SMS_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_SMS_CAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_SMS_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_CAPS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_SMS_CDMA_ENCODING(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_OCTET: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_EPM: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_7BIT_ASCII: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_IA5: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_UNICODE: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_SHIFT_JIS: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_KOREAN: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN_HEBREW: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_GSM_7BIT: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(9i32);
impl ::core::marker::Copy for MBN_SMS_CDMA_ENCODING {}
impl ::core::clone::Clone for MBN_SMS_CDMA_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_SMS_CDMA_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_SMS_CDMA_ENCODING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_SMS_CDMA_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_CDMA_ENCODING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_SMS_CDMA_LANG(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_NONE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_ENGLISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_FRENCH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_SPANISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_JAPANESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_KOREAN: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_CHINESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_HEBREW: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(7i32);
impl ::core::marker::Copy for MBN_SMS_CDMA_LANG {}
impl ::core::clone::Clone for MBN_SMS_CDMA_LANG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_SMS_CDMA_LANG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_SMS_CDMA_LANG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_SMS_CDMA_LANG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_CDMA_LANG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_SMS_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_ALL: MBN_SMS_FLAG = MBN_SMS_FLAG(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_INDEX: MBN_SMS_FLAG = MBN_SMS_FLAG(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NEW: MBN_SMS_FLAG = MBN_SMS_FLAG(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_OLD: MBN_SMS_FLAG = MBN_SMS_FLAG(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_SENT: MBN_SMS_FLAG = MBN_SMS_FLAG(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_DRAFT: MBN_SMS_FLAG = MBN_SMS_FLAG(5i32);
impl ::core::marker::Copy for MBN_SMS_FLAG {}
impl ::core::clone::Clone for MBN_SMS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_SMS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_SMS_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_SMS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_SMS_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_NONE: MBN_SMS_FORMAT = MBN_SMS_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_PDU: MBN_SMS_FORMAT = MBN_SMS_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_TEXT: MBN_SMS_FORMAT = MBN_SMS_FORMAT(2i32);
impl ::core::marker::Copy for MBN_SMS_FORMAT {}
impl ::core::clone::Clone for MBN_SMS_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_SMS_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_SMS_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_SMS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_SMS_STATUS_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NONE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NEW_MESSAGE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(2i32);
impl ::core::marker::Copy for MBN_SMS_STATUS_FLAG {}
impl ::core::clone::Clone for MBN_SMS_STATUS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_SMS_STATUS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_SMS_STATUS_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_SMS_STATUS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_STATUS_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_VOICE_CALL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_NONE: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_HANGUP: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(2i32);
impl ::core::marker::Copy for MBN_VOICE_CALL_STATE {}
impl ::core::clone::Clone for MBN_VOICE_CALL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_VOICE_CALL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_VOICE_CALL_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_VOICE_CALL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_VOICE_CALL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MBN_VOICE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_NONE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_NO_VOICE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(3i32);
impl ::core::marker::Copy for MBN_VOICE_CLASS {}
impl ::core::clone::Clone for MBN_VOICE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MBN_VOICE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MBN_VOICE_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MBN_VOICE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_VOICE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WWAEXT_SMS_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MESSAGE_INDEX_NONE: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_MAX: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(160i32);
impl ::core::marker::Copy for WWAEXT_SMS_CONSTANTS {}
impl ::core::clone::Clone for WWAEXT_SMS_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WWAEXT_SMS_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WWAEXT_SMS_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WWAEXT_SMS_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WWAEXT_SMS_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub userName: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub password: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
impl ::core::clone::Clone for MBN_CONTEXT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for MBN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_CONTEXT").field("contextID", &self.contextID).field("contextType", &self.contextType).field("accessString", &self.accessString).field("userName", &self.userName).field("password", &self.password).field("compression", &self.compression).field("authType", &self.authType).finish()
    }
}
impl ::windows::core::TypeKind for MBN_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MBN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.contextID == other.contextID && self.contextType == other.contextType && self.accessString == other.accessString && self.userName == other.userName && self.password == other.password && self.compression == other.compression && self.authType == other.authType
    }
}
impl ::core::cmp::Eq for MBN_CONTEXT {}
impl ::core::default::Default for MBN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub dataWriteSupported: super::super::Foundation::VARIANT_BOOL,
    pub dataReadSupported: super::super::Foundation::VARIANT_BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_DEVICE_SERVICE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_DEVICE_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_DEVICE_SERVICE").field("deviceServiceID", &self.deviceServiceID).field("dataWriteSupported", &self.dataWriteSupported).field("dataReadSupported", &self.dataReadSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MBN_DEVICE_SERVICE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_DEVICE_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.deviceServiceID == other.deviceServiceID && self.dataWriteSupported == other.dataWriteSupported && self.dataReadSupported == other.dataReadSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_DEVICE_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_DEVICE_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_INTERFACE_CAPS {
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub voiceClass: MBN_VOICE_CLASS,
    pub dataClass: u32,
    pub customDataClass: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub gsmBandClass: u32,
    pub cdmaBandClass: u32,
    pub customBandClass: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub smsCaps: u32,
    pub controlCaps: u32,
    pub deviceID: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub manufacturer: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub model: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub firmwareInfo: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
}
impl ::core::clone::Clone for MBN_INTERFACE_CAPS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for MBN_INTERFACE_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_INTERFACE_CAPS")
            .field("cellularClass", &self.cellularClass)
            .field("voiceClass", &self.voiceClass)
            .field("dataClass", &self.dataClass)
            .field("customDataClass", &self.customDataClass)
            .field("gsmBandClass", &self.gsmBandClass)
            .field("cdmaBandClass", &self.cdmaBandClass)
            .field("customBandClass", &self.customBandClass)
            .field("smsCaps", &self.smsCaps)
            .field("controlCaps", &self.controlCaps)
            .field("deviceID", &self.deviceID)
            .field("manufacturer", &self.manufacturer)
            .field("model", &self.model)
            .field("firmwareInfo", &self.firmwareInfo)
            .finish()
    }
}
impl ::windows::core::TypeKind for MBN_INTERFACE_CAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MBN_INTERFACE_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.cellularClass == other.cellularClass && self.voiceClass == other.voiceClass && self.dataClass == other.dataClass && self.customDataClass == other.customDataClass && self.gsmBandClass == other.gsmBandClass && self.cdmaBandClass == other.cdmaBandClass && self.customBandClass == other.customBandClass && self.smsCaps == other.smsCaps && self.controlCaps == other.controlCaps && self.deviceID == other.deviceID && self.manufacturer == other.manufacturer && self.model == other.model && self.firmwareInfo == other.firmwareInfo
    }
}
impl ::core::cmp::Eq for MBN_INTERFACE_CAPS {}
impl ::core::default::Default for MBN_INTERFACE_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_PIN_INFO {
    pub pinState: MBN_PIN_STATE,
    pub pinType: MBN_PIN_TYPE,
    pub attemptsRemaining: u32,
}
impl ::core::marker::Copy for MBN_PIN_INFO {}
impl ::core::clone::Clone for MBN_PIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MBN_PIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_PIN_INFO").field("pinState", &self.pinState).field("pinType", &self.pinType).field("attemptsRemaining", &self.attemptsRemaining).finish()
    }
}
impl ::windows::core::TypeKind for MBN_PIN_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MBN_PIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pinState == other.pinState && self.pinType == other.pinType && self.attemptsRemaining == other.attemptsRemaining
    }
}
impl ::core::cmp::Eq for MBN_PIN_INFO {}
impl ::core::default::Default for MBN_PIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_PROVIDER {
    pub providerID: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub providerState: u32,
    pub providerName: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub dataClass: u32,
}
impl ::core::clone::Clone for MBN_PROVIDER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for MBN_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_PROVIDER").field("providerID", &self.providerID).field("providerState", &self.providerState).field("providerName", &self.providerName).field("dataClass", &self.dataClass).finish()
    }
}
impl ::windows::core::TypeKind for MBN_PROVIDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MBN_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.providerID == other.providerID && self.providerState == other.providerState && self.providerName == other.providerName && self.dataClass == other.dataClass
    }
}
impl ::core::cmp::Eq for MBN_PROVIDER {}
impl ::core::default::Default for MBN_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
impl ::core::clone::Clone for MBN_PROVIDER2 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for MBN_PROVIDER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_PROVIDER2").field("provider", &self.provider).field("cellularClass", &self.cellularClass).field("signalStrength", &self.signalStrength).field("signalError", &self.signalError).finish()
    }
}
impl ::windows::core::TypeKind for MBN_PROVIDER2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MBN_PROVIDER2 {
    fn eq(&self, other: &Self) -> bool {
        self.provider == other.provider && self.cellularClass == other.cellularClass && self.signalStrength == other.signalStrength && self.signalError == other.signalError
    }
}
impl ::core::cmp::Eq for MBN_PROVIDER2 {}
impl ::core::default::Default for MBN_PROVIDER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_SMS_FILTER {
    pub flag: MBN_SMS_FLAG,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_FILTER {}
impl ::core::clone::Clone for MBN_SMS_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MBN_SMS_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_SMS_FILTER").field("flag", &self.flag).field("messageIndex", &self.messageIndex).finish()
    }
}
impl ::windows::core::TypeKind for MBN_SMS_FILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MBN_SMS_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag && self.messageIndex == other.messageIndex
    }
}
impl ::core::cmp::Eq for MBN_SMS_FILTER {}
impl ::core::default::Default for MBN_SMS_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_SMS_STATUS_INFO {
    pub flag: u32,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_STATUS_INFO {}
impl ::core::clone::Clone for MBN_SMS_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MBN_SMS_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_SMS_STATUS_INFO").field("flag", &self.flag).field("messageIndex", &self.messageIndex).finish()
    }
}
impl ::windows::core::TypeKind for MBN_SMS_STATUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MBN_SMS_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag && self.messageIndex == other.messageIndex
    }
}
impl ::core::cmp::Eq for MBN_SMS_STATUS_INFO {}
impl ::core::default::Default for MBN_SMS_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct __DummyPinType__ {
    pub pinType: u32,
}
impl ::core::marker::Copy for __DummyPinType__ {}
impl ::core::clone::Clone for __DummyPinType__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for __DummyPinType__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("__DummyPinType__").field("pinType", &self.pinType).finish()
    }
}
impl ::windows::core::TypeKind for __DummyPinType__ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for __DummyPinType__ {
    fn eq(&self, other: &Self) -> bool {
        self.pinType == other.pinType
    }
}
impl ::core::cmp::Eq for __DummyPinType__ {}
impl ::core::default::Default for __DummyPinType__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct __mbnapi_ReferenceRemainingTypes__ {
    pub bandClass: MBN_BAND_CLASS,
    pub contextConstants: MBN_CONTEXT_CONSTANTS,
    pub ctrlCaps: MBN_CTRL_CAPS,
    pub dataClass: MBN_DATA_CLASS,
    pub interfaceCapsConstants: MBN_INTERFACE_CAPS_CONSTANTS,
    pub pinConstants: MBN_PIN_CONSTANTS,
    pub providerConstants: MBN_PROVIDER_CONSTANTS,
    pub providerState: MBN_PROVIDER_STATE,
    pub registrationConstants: MBN_REGISTRATION_CONSTANTS,
    pub signalConstants: MBN_SIGNAL_CONSTANTS,
    pub smsCaps: MBN_SMS_CAPS,
    pub smsConstants: WWAEXT_SMS_CONSTANTS,
    pub wwaextSmsConstants: WWAEXT_SMS_CONSTANTS,
    pub smsStatusFlag: MBN_SMS_STATUS_FLAG,
}
impl ::core::marker::Copy for __mbnapi_ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __mbnapi_ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for __mbnapi_ReferenceRemainingTypes__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("__mbnapi_ReferenceRemainingTypes__")
            .field("bandClass", &self.bandClass)
            .field("contextConstants", &self.contextConstants)
            .field("ctrlCaps", &self.ctrlCaps)
            .field("dataClass", &self.dataClass)
            .field("interfaceCapsConstants", &self.interfaceCapsConstants)
            .field("pinConstants", &self.pinConstants)
            .field("providerConstants", &self.providerConstants)
            .field("providerState", &self.providerState)
            .field("registrationConstants", &self.registrationConstants)
            .field("signalConstants", &self.signalConstants)
            .field("smsCaps", &self.smsCaps)
            .field("smsConstants", &self.smsConstants)
            .field("wwaextSmsConstants", &self.wwaextSmsConstants)
            .field("smsStatusFlag", &self.smsStatusFlag)
            .finish()
    }
}
impl ::windows::core::TypeKind for __mbnapi_ReferenceRemainingTypes__ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for __mbnapi_ReferenceRemainingTypes__ {
    fn eq(&self, other: &Self) -> bool {
        self.bandClass == other.bandClass && self.contextConstants == other.contextConstants && self.ctrlCaps == other.ctrlCaps && self.dataClass == other.dataClass && self.interfaceCapsConstants == other.interfaceCapsConstants && self.pinConstants == other.pinConstants && self.providerConstants == other.providerConstants && self.providerState == other.providerState && self.registrationConstants == other.registrationConstants && self.signalConstants == other.signalConstants && self.smsCaps == other.smsCaps && self.smsConstants == other.smsConstants && self.wwaextSmsConstants == other.wwaextSmsConstants && self.smsStatusFlag == other.smsStatusFlag
    }
}
impl ::core::cmp::Eq for __mbnapi_ReferenceRemainingTypes__ {}
impl ::core::default::Default for __mbnapi_ReferenceRemainingTypes__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
