#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDummyMBNUCMExt(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDummyMBNUCMExt {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDummyMBNUCMExt> for ::windows::core::IUnknown {
    fn from(value: IDummyMBNUCMExt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDummyMBNUCMExt> for ::windows::core::IUnknown {
    fn from(value: &IDummyMBNUCMExt) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDummyMBNUCMExt> for super::super::System::Com::IDispatch {
    fn from(value: IDummyMBNUCMExt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDummyMBNUCMExt> for super::super::System::Com::IDispatch {
    fn from(value: &IDummyMBNUCMExt) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDummyMBNUCMExt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_ffff_4bbb_aaee_338e368af6fa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDummyMBNUCMExt_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnection(::windows::core::IUnknown);
impl IMbnConnection {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectionID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ConnectionID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InterfaceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Connect<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Connect)(::core::mem::transmute_copy(self), ::core::mem::transmute(connectionmode), strprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionState)(::core::mem::transmute_copy(self), ::core::mem::transmute(connectionstate), ::core::mem::transmute(profilename)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetVoiceCallState(&self) -> ::windows::core::Result<MBN_VOICE_CALL_STATE> {
        let mut result__: MBN_VOICE_CALL_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVoiceCallState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_VOICE_CALL_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetActivationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetActivationNetworkError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnConnection> for ::windows::core::IUnknown {
    fn from(value: IMbnConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnection> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200d_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetConnectionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetConnectionState: usize,
    pub GetVoiceCallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT,
    pub GetActivationNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionContext(::windows::core::IUnknown);
impl IMbnConnectionContext {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProvisionedContexts(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProvisionedContexts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProvisionedContext<'a, Param0: ::windows::core::IntoParam<'a, MBN_CONTEXT>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, provisionedcontexts: Param0, providerid: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetProvisionedContext)(::core::mem::transmute_copy(self), provisionedcontexts.into_param().abi(), providerid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnConnectionContext> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionContext> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200b_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContext_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProvisionedContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProvisionedContexts: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProvisionedContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProvisionedContext: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionContextEvents(::windows::core::IUnknown);
impl IMbnConnectionContextEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnProvisionedContextListChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionContext>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnProvisionedContextListChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSetProvisionedContextComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionContext>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetProvisionedContextComplete)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IMbnConnectionContextEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionContextEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionContextEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionContextEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionContextEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionContextEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionContextEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200c_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContextEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnProvisionedContextListChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnSetProvisionedContextComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionEvents(::windows::core::IUnknown);
impl IMbnConnectionEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnConnectComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectComplete)(::core::mem::transmute_copy(self), newconnection.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnDisconnectComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDisconnectComplete)(::core::mem::transmute_copy(self), newconnection.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnConnectStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectStateChange)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnVoiceCallStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnVoiceCallStateChange)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnConnectionEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200e_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnConnectComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnDisconnectComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnConnectStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnVoiceCallStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionManager(::windows::core::IUnknown);
impl IMbnConnectionManager {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetConnection<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, connectionid: Param0) -> ::windows::core::Result<IMbnConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnection)(::core::mem::transmute_copy(self), connectionid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnConnection>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnections(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnections)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
impl ::core::convert::From<IMbnConnectionManager> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201d_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::PCWSTR, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnections: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionManagerEvents(::windows::core::IUnknown);
impl IMbnConnectionManagerEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnConnectionArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectionArrival)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnConnectionRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, oldconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectionRemoval)(::core::mem::transmute_copy(self), oldconnection.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnConnectionManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionManagerEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionManagerEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201e_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManagerEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnConnectionArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnConnectionRemoval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfile(::windows::core::IUnknown);
impl IMbnConnectionProfile {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProfileXmlData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProfileXmlData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn UpdateProfile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, strprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateProfile)(::core::mem::transmute_copy(self), strprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IMbnConnectionProfile> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionProfile> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2010_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfile_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProfileXmlData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiledata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProfileXmlData: usize,
    pub UpdateProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprofile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileEvents(::windows::core::IUnknown);
impl IMbnConnectionProfileEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnProfileUpdate<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, newprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnProfileUpdate)(::core::mem::transmute_copy(self), newprofile.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnConnectionProfileEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfileEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionProfileEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfileEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfileEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfileEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionProfileEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2011_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnProfileUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileManager(::windows::core::IUnknown);
impl IMbnConnectionProfileManager {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionProfiles<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, mbninterface: Param0) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionProfiles)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, mbninterface: Param0, profilename: Param1) -> ::windows::core::Result<IMbnConnectionProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionProfile)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), profilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnConnectionProfile>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn CreateConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, xmlprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateConnectionProfile)(::core::mem::transmute_copy(self), xmlprofile.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnConnectionProfileManager> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfileManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionProfileManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfileManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionProfileManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnectionProfiles: usize,
    pub GetConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, profilename: ::windows::core::PCWSTR, connectionprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlprofile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileManagerEvents(::windows::core::IUnknown);
impl IMbnConnectionProfileManagerEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnConnectionProfileArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, newconnectionprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectionProfileArrival)(::core::mem::transmute_copy(self), newconnectionprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnConnectionProfileRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, oldconnectionprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectionProfileRemoval)(::core::mem::transmute_copy(self), oldconnectionprofile.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnConnectionProfileManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfileManagerEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnConnectionProfileManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfileManagerEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfileManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfileManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnConnectionProfileManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManagerEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnConnectionProfileArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnConnectionProfileRemoval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceService(::windows::core::IUnknown);
impl IMbnDeviceService {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn QuerySupportedCommands(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuerySupportedCommands)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OpenCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OpenCommandSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn CloseCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CloseCommandSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueryCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OpenDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OpenDataSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn CloseDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CloseDataSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WriteData)(::core::mem::transmute_copy(self), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InterfaceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceServiceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceServiceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCommandSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsCommandSessionOpen)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDataSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsDataSessionOpen)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IMbnDeviceService> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnDeviceService> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3bb9a71_dc70_4be9_a4da_7886ae8b191b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceService_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
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
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceServiceID: usize,
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
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSessionsStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfaceid: Param0, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSessionsStateChange)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(statechange)).ok()
    }
}
impl ::core::convert::From<IMbnDeviceServiceStateEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServiceStateEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnDeviceServiceStateEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServiceStateEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServiceStateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServiceStateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnDeviceServiceStateEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d3ff196_89ee_49d8_8b60_33ffddffc58d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServiceStateEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnSessionsStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnSessionsStateChange: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesContext(::windows::core::IUnknown);
impl IMbnDeviceServicesContext {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateDeviceServices(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumerateDeviceServices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, deviceserviceid: Param0) -> ::windows::core::Result<IMbnDeviceService> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceService)(::core::mem::transmute_copy(self), deviceserviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnDeviceService>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn MaxCommandSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxCommandSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn MaxDataSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxDataSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnDeviceServicesContext> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServicesContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnDeviceServicesContext> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServicesContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServicesContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServicesContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnDeviceServicesContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc5ac347_1592_4068_80bb_6a57580150d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesContext_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateDeviceServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateDeviceServices: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeviceService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeviceService: usize,
    pub MaxCommandSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT,
    pub MaxDataSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesEvents(::windows::core::IUnknown);
impl IMbnDeviceServicesEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuerySupportedCommandsComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQuerySupportedCommandsComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(commandidlist), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnOpenCommandSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOpenCommandSessionComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnCloseCommandSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCloseCommandSessionComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetCommandComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetCommandComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(responseid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQueryCommandComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQueryCommandComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(responseid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEventNotification)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(eventid), ::core::mem::transmute(deviceservicedata)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnOpenDataSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOpenDataSessionComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnCloseDataSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCloseDataSessionComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnWriteDataComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnWriteDataComplete)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReadData<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReadData)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(deviceservicedata)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInterfaceStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfaceid: Param0, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInterfaceStateChange)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(statechange)).ok()
    }
}
impl ::core::convert::From<IMbnDeviceServicesEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServicesEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnDeviceServicesEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServicesEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServicesEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServicesEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnDeviceServicesEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a900c19_6824_4e97_b76e_cf239d0ca642);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQuerySupportedCommandsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQuerySupportedCommandsComplete: usize,
    pub OnOpenCommandSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub OnCloseCommandSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetCommandComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQueryCommandComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQueryCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    pub OnOpenDataSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub OnCloseDataSessionComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub OnWriteDataComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnReadData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInterfaceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInterfaceStateChange: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesManager(::windows::core::IUnknown);
impl IMbnDeviceServicesManager {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceServicesContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, networkinterfaceid: Param0) -> ::windows::core::Result<IMbnDeviceServicesContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceServicesContext)(::core::mem::transmute_copy(self), networkinterfaceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnDeviceServicesContext>(result__)
    }
}
impl ::core::convert::From<IMbnDeviceServicesManager> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServicesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnDeviceServicesManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServicesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnDeviceServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20a26258_6811_4478_ac1d_13324e45e41c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeviceServicesContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeviceServicesContext: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterface(::windows::core::IUnknown);
impl IMbnInterface {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InterfaceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInterfaceCapability(&self) -> ::windows::core::Result<MBN_INTERFACE_CAPS> {
        let mut result__: ::core::mem::ManuallyDrop<MBN_INTERFACE_CAPS> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInterfaceCapability)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_INTERFACE_CAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetSubscriberInformation(&self) -> ::windows::core::Result<IMbnSubscriberInformation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriberInformation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnSubscriberInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetReadyState(&self) -> ::windows::core::Result<MBN_READY_STATE> {
        let mut result__: MBN_READY_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetReadyState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_READY_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn InEmergencyMode(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InEmergencyMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHomeProvider(&self) -> ::windows::core::Result<MBN_PROVIDER> {
        let mut result__: ::core::mem::ManuallyDrop<MBN_PROVIDER> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHomeProvider)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PROVIDER>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPreferredProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetPreferredProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(preferredproviders), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVisibleProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(age), ::core::mem::transmute(visibleproviders)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScanNetwork)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMbnConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnConnection>(result__)
    }
}
impl ::core::convert::From<IMbnInterface> for ::windows::core::IUnknown {
    fn from(value: IMbnInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnInterface> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2001_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterface_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInterfaceCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInterfaceCapability: usize,
    pub GetSubscriberInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriberinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT,
    pub InEmergencyMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emergencymode: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHomeProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHomeProvider: usize,
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
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterfaceEvents(::windows::core::IUnknown);
impl IMbnInterfaceEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnInterfaceCapabilityAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInterfaceCapabilityAvailable)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSubscriberInformationChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSubscriberInformationChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnReadyStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReadyStateChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnEmergencyModeChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEmergencyModeChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnHomeProviderAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnHomeProviderAvailable)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnPreferredProvidersChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPreferredProvidersChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSetPreferredProvidersComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetPreferredProvidersComplete)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnScanNetworkComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnScanNetworkComplete)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IMbnInterfaceEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnInterfaceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnInterfaceEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterfaceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterfaceEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterfaceEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnInterfaceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2002_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnInterfaceCapabilityAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnSubscriberInformationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnReadyStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnEmergencyModeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnHomeProviderAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnSetPreferredProvidersComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterfaceManager(::windows::core::IUnknown);
impl IMbnInterfaceManager {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, interfaceid: Param0) -> ::windows::core::Result<IMbnInterface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInterface)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnInterface>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInterfaces)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
impl ::core::convert::From<IMbnInterfaceManager> for ::windows::core::IUnknown {
    fn from(value: IMbnInterfaceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnInterfaceManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterfaceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterfaceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterfaceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnInterfaceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201b_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::windows::core::PCWSTR, mbninterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInterfaces: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnInterfaceManagerEvents(::windows::core::IUnknown);
impl IMbnInterfaceManagerEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnInterfaceArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInterfaceArrival)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnInterfaceRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, oldinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInterfaceRemoval)(::core::mem::transmute_copy(self), oldinterface.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnInterfaceManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnInterfaceManagerEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnInterfaceManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterfaceManagerEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterfaceManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterfaceManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnInterfaceManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201c_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManagerEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnInterfaceArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnInterfaceRemoval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnMultiCarrier(::windows::core::IUnknown);
impl IMbnMultiCarrier {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetHomeProvider)(::core::mem::transmute_copy(self), ::core::mem::transmute(homeprovider), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPreferredProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVisibleProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(age), ::core::mem::transmute(visibleproviders)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedCellularClasses(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSupportedCellularClasses)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetCurrentCellularClass(&self) -> ::windows::core::Result<MBN_CELLULAR_CLASS> {
        let mut result__: MBN_CELLULAR_CLASS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentCellularClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_CELLULAR_CLASS>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScanNetwork)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnMultiCarrier> for ::windows::core::IUnknown {
    fn from(value: IMbnMultiCarrier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnMultiCarrier> for ::windows::core::IUnknown {
    fn from(value: &IMbnMultiCarrier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnMultiCarrier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnMultiCarrier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnMultiCarrier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2020_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrier_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHomeProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHomeProvider: usize,
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
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSetHomeProviderComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetHomeProviderComplete)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnCurrentCellularClassChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCurrentCellularClassChange)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnPreferredProvidersChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPreferredProvidersChange)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnScanNetworkComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnScanNetworkComplete)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnInterfaceCapabilityChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInterfaceCapabilityChange)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnMultiCarrierEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnMultiCarrierEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnMultiCarrierEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnMultiCarrierEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnMultiCarrierEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnMultiCarrierEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnMultiCarrierEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_2021_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrierEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnSetHomeProviderComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnCurrentCellularClassChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnInterfaceCapabilityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPin(::windows::core::IUnknown);
impl IMbnPin {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn PinType(&self) -> ::windows::core::Result<MBN_PIN_TYPE> {
        let mut result__: MBN_PIN_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PinType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PIN_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn PinFormat(&self) -> ::windows::core::Result<MBN_PIN_FORMAT> {
        let mut result__: MBN_PIN_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PinFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PIN_FORMAT>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn PinLengthMin(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PinLengthMin)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn PinLengthMax(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PinLengthMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn PinMode(&self) -> ::windows::core::Result<MBN_PIN_MODE> {
        let mut result__: MBN_PIN_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PinMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PIN_MODE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Enable<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enable)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Disable<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Disable)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Enter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enter)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Change<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pin: Param0, newpin: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Change)(::core::mem::transmute_copy(self), pin.into_param().abi(), newpin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Unblock<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, puk: Param0, newpin: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Unblock)(::core::mem::transmute_copy(self), puk.into_param().abi(), newpin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetPinManager(&self) -> ::windows::core::Result<IMbnPinManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPinManager)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnPinManager>(result__)
    }
}
impl ::core::convert::From<IMbnPin> for ::windows::core::IUnknown {
    fn from(value: IMbnPin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnPin> for ::windows::core::IUnknown {
    fn from(value: &IMbnPin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnPin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2007_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPin_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
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
    pub GetPinManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPinEvents(::windows::core::IUnknown);
impl IMbnPinEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnEnableComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEnableComplete)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnDisableComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDisableComplete)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnEnterComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEnterComplete)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnChangeComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChangeComplete)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnUnblockComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUnblockComplete)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IMbnPinEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnPinEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnPinEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnPinEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPinEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPinEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnPinEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2008_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnEnableComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnDisableComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnEnterComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnChangeComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnUnblockComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPinManager(::windows::core::IUnknown);
impl IMbnPinManager {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPinList(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPinList)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetPin(&self, pintype: MBN_PIN_TYPE) -> ::windows::core::Result<IMbnPin> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPin)(::core::mem::transmute_copy(self), ::core::mem::transmute(pintype), ::core::mem::transmute(&mut result__)).from_abi::<IMbnPin>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetPinState(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPinState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnPinManager> for ::windows::core::IUnknown {
    fn from(value: IMbnPinManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnPinManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnPinManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnPinManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2005_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPinList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPinList: usize,
    pub GetPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPinState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnPinManagerEvents(::windows::core::IUnknown);
impl IMbnPinManagerEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnPinListAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnPinManager>>(&self, pinmanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPinListAvailable)(::core::mem::transmute_copy(self), pinmanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnGetPinStateComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPinManager>, Param1: ::windows::core::IntoParam<'a, MBN_PIN_INFO>>(&self, pinmanager: Param0, pininfo: Param1, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnGetPinStateComplete)(::core::mem::transmute_copy(self), pinmanager.into_param().abi(), pininfo.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IMbnPinManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnPinManagerEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnPinManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnPinManagerEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPinManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPinManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnPinManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2006_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManagerEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnPinListAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnGetPinStateComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnRadio(::windows::core::IUnknown);
impl IMbnRadio {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SoftwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__: MBN_RADIO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SoftwareRadioState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_RADIO>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn HardwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__: MBN_RADIO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HardwareRadioState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_RADIO>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetSoftwareRadioState)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiostate), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnRadio> for ::windows::core::IUnknown {
    fn from(value: IMbnRadio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnRadio> for ::windows::core::IUnknown {
    fn from(value: &IMbnRadio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRadio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRadio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnRadio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdccccab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadio_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SoftwareRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub HardwareRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub SetSoftwareRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnRadioEvents(::windows::core::IUnknown);
impl IMbnRadioEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnRadioStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRadio>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnRadioStateChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSetSoftwareRadioStateComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnRadio>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetSoftwareRadioStateComplete)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IMbnRadioEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnRadioEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnRadioEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnRadioEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRadioEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRadioEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnRadioEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadioEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnRadioStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnSetSoftwareRadioStateComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnRegistration(::windows::core::IUnknown);
impl IMbnRegistration {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetRegisterState(&self) -> ::windows::core::Result<MBN_REGISTER_STATE> {
        let mut result__: MBN_REGISTER_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegisterState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_REGISTER_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetRegisterMode(&self) -> ::windows::core::Result<MBN_REGISTER_MODE> {
        let mut result__: MBN_REGISTER_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegisterMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_REGISTER_MODE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProviderID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRoamingText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRoamingText)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetAvailableDataClasses(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAvailableDataClasses)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetCurrentDataClass(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentDataClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetRegistrationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegistrationNetworkError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetPacketAttachNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPacketAttachNetworkError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SetRegisterMode<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, registermode: MBN_REGISTER_MODE, providerid: Param1, dataclass: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetRegisterMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(registermode), providerid.into_param().abi(), ::core::mem::transmute(dataclass), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnRegistration> for ::windows::core::IUnknown {
    fn from(value: IMbnRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnRegistration> for ::windows::core::IUnknown {
    fn from(value: &IMbnRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2009_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistration_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetRegisterState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT,
    pub GetRegisterMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProviderID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProviderID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRoamingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roamingtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRoamingText: usize,
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
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnRegisterModeAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnRegisterModeAvailable)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnRegisterStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnRegisterStateChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnPacketServiceStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPacketServiceStateChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSetRegisterModeComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetRegisterModeComplete)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IMbnRegistrationEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnRegistrationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnRegistrationEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnRegistrationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRegistrationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRegistrationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnRegistrationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200a_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistrationEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnRegisterModeAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnRegisterStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnPacketServiceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnSetRegisterModeComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnServiceActivation(::windows::core::IUnknown);
impl IMbnServiceActivation {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Activate)(::core::mem::transmute_copy(self), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnServiceActivation> for ::windows::core::IUnknown {
    fn from(value: IMbnServiceActivation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnServiceActivation> for ::windows::core::IUnknown {
    fn from(value: &IMbnServiceActivation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnServiceActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnServiceActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnServiceActivation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2017_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activate: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnServiceActivationEvents(::windows::core::IUnknown);
impl IMbnServiceActivationEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivationComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnServiceActivation>>(&self, serviceactivation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivationComplete)(::core::mem::transmute_copy(self), serviceactivation.into_param().abi(), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(requestid), ::core::mem::transmute(status), ::core::mem::transmute(networkerror)).ok()
    }
}
impl ::core::convert::From<IMbnServiceActivationEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnServiceActivationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnServiceActivationEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnServiceActivationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnServiceActivationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnServiceActivationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnServiceActivationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2018_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivationEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceactivation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivationComplete: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSignal(::windows::core::IUnknown);
impl IMbnSignal {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetSignalStrength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignalStrength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetSignalError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignalError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnSignal> for ::windows::core::IUnknown {
    fn from(value: IMbnSignal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSignal> for ::windows::core::IUnknown {
    fn from(value: &IMbnSignal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSignal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2003_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignal_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSignalStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignalError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSignalEvents(::windows::core::IUnknown);
impl IMbnSignalEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSignalStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSignal>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSignalStateChange)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnSignalEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnSignalEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSignalEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnSignalEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSignalEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSignalEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSignalEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2004_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignalEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnSignalStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSms(::windows::core::IUnknown);
impl IMbnSms {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetSmsConfiguration(&self) -> ::windows::core::Result<IMbnSmsConfiguration> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSmsConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnSmsConfiguration>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SetSmsConfiguration<'a, Param0: ::windows::core::IntoParam<'a, IMbnSmsConfiguration>>(&self, smsconfiguration: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetSmsConfiguration)(::core::mem::transmute_copy(self), smsconfiguration.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SmsSendPdu<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pdudata: Param0, size: u8) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SmsSendPdu)(::core::mem::transmute_copy(self), pdudata.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdma<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, address: Param0, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SmsSendCdma)(::core::mem::transmute_copy(self), address.into_param().abi(), ::core::mem::transmute(encoding), ::core::mem::transmute(language), ::core::mem::transmute(sizeincharacters), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SmsSendCdmaPdu)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SmsRead)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsfilter), ::core::mem::transmute(smsformat), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SmsDelete)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsfilter), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn GetSmsStatus(&self) -> ::windows::core::Result<MBN_SMS_STATUS_INFO> {
        let mut result__: MBN_SMS_STATUS_INFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSmsStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_STATUS_INFO>(result__)
    }
}
impl ::core::convert::From<IMbnSms> for ::windows::core::IUnknown {
    fn from(value: IMbnSms) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSms> for ::windows::core::IUnknown {
    fn from(value: &IMbnSms) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSms {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSms {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSms {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2015_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSms_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSmsConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSmsConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsconfiguration: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
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
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceCenterAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ServiceCenterAddress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SetServiceCenterAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, scaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetServiceCenterAddress)(::core::mem::transmute_copy(self), scaddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn MaxMessageIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MaxMessageIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn CdmaShortMsgSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CdmaShortMsgSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SmsFormat(&self) -> ::windows::core::Result<MBN_SMS_FORMAT> {
        let mut result__: MBN_SMS_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SmsFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_FORMAT>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSmsFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsformat)).ok()
    }
}
impl ::core::convert::From<IMbnSmsConfiguration> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSmsConfiguration> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSmsConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2012_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsConfiguration_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceCenterAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceCenterAddress: usize,
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
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSmsConfigurationChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSmsConfigurationChange)(::core::mem::transmute_copy(self), sms.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSetSmsConfigurationComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetSmsConfigurationComplete)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSmsSendComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSmsSendComplete)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsReadComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSmsReadComplete)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(smsformat), ::core::mem::transmute(readmsgs), ::core::mem::transmute(moremsgs), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsNewClass0Message<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSmsNewClass0Message)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(smsformat), ::core::mem::transmute(readmsgs)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSmsDeleteComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSmsDeleteComplete)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn OnSmsStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSmsStatusChange)(::core::mem::transmute_copy(self), sms.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMbnSmsEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSmsEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSmsEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2016_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnSmsConfigurationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnSetSmsConfigurationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnSmsSendComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSmsReadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSmsReadComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSmsNewClass0Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSmsNewClass0Message: usize,
    pub OnSmsDeleteComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub OnSmsStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSmsReadMsgPdu(::windows::core::IUnknown);
impl IMbnSmsReadMsgPdu {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__: MBN_MSG_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_MSG_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PduData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PduData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Message)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
impl ::core::convert::From<IMbnSmsReadMsgPdu> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsReadMsgPdu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSmsReadMsgPdu> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsReadMsgPdu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsReadMsgPdu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsReadMsgPdu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSmsReadMsgPdu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2013_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgPdu_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PduData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdudata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PduData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnSmsReadMsgTextCdma(::windows::core::IUnknown);
impl IMbnSmsReadMsgTextCdma {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Index)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__: MBN_MSG_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_MSG_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Address)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Timestamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn EncodingID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_ENCODING> {
        let mut result__: MBN_SMS_CDMA_ENCODING = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EncodingID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_CDMA_ENCODING>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn LanguageID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_LANG> {
        let mut result__: MBN_SMS_CDMA_LANG = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LanguageID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_CDMA_LANG>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
    pub unsafe fn SizeInCharacters(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeInCharacters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Message)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
impl ::core::convert::From<IMbnSmsReadMsgTextCdma> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsReadMsgTextCdma) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSmsReadMsgTextCdma> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsReadMsgTextCdma) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsReadMsgTextCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsReadMsgTextCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSmsReadMsgTextCdma {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2014_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgTextCdma_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Address: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Timestamp: usize,
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
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriberID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubscriberID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimIccID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SimIccID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TelephoneNumbers(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TelephoneNumbers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
impl ::core::convert::From<IMbnSubscriberInformation> for ::windows::core::IUnknown {
    fn from(value: IMbnSubscriberInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnSubscriberInformation> for ::windows::core::IUnknown {
    fn from(value: &IMbnSubscriberInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSubscriberInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSubscriberInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnSubscriberInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x459ecc43_bcf5_11dc_a8a8_001321f1405f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSubscriberInformation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriberID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriberid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriberID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SimIccID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simiccid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimIccID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TelephoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TelephoneNumbers: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnVendorSpecificEvents(::windows::core::IUnknown);
impl IMbnVendorSpecificEvents {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<'a, Param0: ::windows::core::IntoParam<'a, IMbnVendorSpecificOperation>>(&self, vendoroperation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEventNotification)(::core::mem::transmute_copy(self), vendoroperation.into_param().abi(), ::core::mem::transmute(vendorspecificdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetVendorSpecificComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnVendorSpecificOperation>>(&self, vendoroperation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetVendorSpecificComplete)(::core::mem::transmute_copy(self), vendoroperation.into_param().abi(), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(requestid)).ok()
    }
}
impl ::core::convert::From<IMbnVendorSpecificEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnVendorSpecificEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnVendorSpecificEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnVendorSpecificEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnVendorSpecificEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnVendorSpecificEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnVendorSpecificEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201a_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetVendorSpecificComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetVendorSpecificComplete: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
pub struct IMbnVendorSpecificOperation(::windows::core::IUnknown);
impl IMbnVendorSpecificOperation {
    #[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetVendorSpecific)(::core::mem::transmute_copy(self), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMbnVendorSpecificOperation> for ::windows::core::IUnknown {
    fn from(value: IMbnVendorSpecificOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMbnVendorSpecificOperation> for ::windows::core::IUnknown {
    fn from(value: &IMbnVendorSpecificOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnVendorSpecificOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnVendorSpecificOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMbnVendorSpecificOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2019_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificOperation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetVendorSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetVendorSpecific: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_ACTIVATION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_ACTIVATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_ACTIVATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_AUTH_PROTOCOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_AUTH_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_AUTH_PROTOCOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_BAND_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_BAND_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_BAND_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_CELLULAR_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_CELLULAR_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CELLULAR_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_COMPRESSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_COMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_COMPRESSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_CONNECTION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_CONNECTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CONNECTION_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: super::super::Foundation::BSTR,
    pub userName: super::super::Foundation::BSTR,
    pub password: super::super::Foundation::BSTR,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_CONTEXT {
    fn clone(&self) -> Self {
        Self {
            contextID: self.contextID,
            contextType: self.contextType,
            accessString: self.accessString.clone(),
            userName: self.userName.clone(),
            password: self.password.clone(),
            compression: self.compression,
            authType: self.authType,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_CONTEXT").field("contextID", &self.contextID).field("contextType", &self.contextType).field("accessString", &self.accessString).field("userName", &self.userName).field("password", &self.password).field("compression", &self.compression).field("authType", &self.authType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.contextID == other.contextID && self.contextType == other.contextType && self.accessString == other.accessString && self.userName == other.userName && self.password == other.password && self.compression == other.compression && self.authType == other.authType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_CONTEXT_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_CONTEXT_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CONTEXT_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_CONTEXT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_CTRL_CAPS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_CTRL_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_CTRL_CAPS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_DATA_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_DATA_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_DATA_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: super::super::Foundation::BSTR,
    pub dataWriteSupported: i16,
    pub dataReadSupported: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_DEVICE_SERVICE {
    fn clone(&self) -> Self {
        Self { deviceServiceID: self.deviceServiceID.clone(), dataWriteSupported: self.dataWriteSupported, dataReadSupported: self.dataReadSupported }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_DEVICE_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_DEVICE_SERVICE").field("deviceServiceID", &self.deviceServiceID).field("dataWriteSupported", &self.dataWriteSupported).field("dataReadSupported", &self.dataReadSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_DEVICE_SERVICE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_DEVICE_SERVICES_INTERFACE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_DEVICE_SERVICE_SESSIONS_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_INTERFACE_CAPS {
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub voiceClass: MBN_VOICE_CLASS,
    pub dataClass: u32,
    pub customDataClass: super::super::Foundation::BSTR,
    pub gsmBandClass: u32,
    pub cdmaBandClass: u32,
    pub customBandClass: super::super::Foundation::BSTR,
    pub smsCaps: u32,
    pub controlCaps: u32,
    pub deviceID: super::super::Foundation::BSTR,
    pub manufacturer: super::super::Foundation::BSTR,
    pub model: super::super::Foundation::BSTR,
    pub firmwareInfo: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_INTERFACE_CAPS {
    fn clone(&self) -> Self {
        Self {
            cellularClass: self.cellularClass,
            voiceClass: self.voiceClass,
            dataClass: self.dataClass,
            customDataClass: self.customDataClass.clone(),
            gsmBandClass: self.gsmBandClass,
            cdmaBandClass: self.cdmaBandClass,
            customBandClass: self.customBandClass.clone(),
            smsCaps: self.smsCaps,
            controlCaps: self.controlCaps,
            deviceID: self.deviceID.clone(),
            manufacturer: self.manufacturer.clone(),
            model: self.model.clone(),
            firmwareInfo: self.firmwareInfo.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_INTERFACE_CAPS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_INTERFACE_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.cellularClass == other.cellularClass && self.voiceClass == other.voiceClass && self.dataClass == other.dataClass && self.customDataClass == other.customDataClass && self.gsmBandClass == other.gsmBandClass && self.cdmaBandClass == other.cdmaBandClass && self.customBandClass == other.customBandClass && self.smsCaps == other.smsCaps && self.controlCaps == other.controlCaps && self.deviceID == other.deviceID && self.manufacturer == other.manufacturer && self.model == other.model && self.firmwareInfo == other.firmwareInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_INTERFACE_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_INTERFACE_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_INTERFACE_CAPS_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_INTERFACE_CAPS_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_INTERFACE_CAPS_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_MSG_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_MSG_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_MSG_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_PIN_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_PIN_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_PIN_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_PIN_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_FORMAT").field(&self.0).finish()
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
unsafe impl ::windows::core::Abi for MBN_PIN_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MBN_PIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MBN_PIN_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MBN_PIN_INFO {}
impl ::core::default::Default for MBN_PIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_PIN_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_PIN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_PIN_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_PIN_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_PIN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_PIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PIN_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER {
    pub providerID: super::super::Foundation::BSTR,
    pub providerState: u32,
    pub providerName: super::super::Foundation::BSTR,
    pub dataClass: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_PROVIDER {
    fn clone(&self) -> Self {
        Self { providerID: self.providerID.clone(), providerState: self.providerState, providerName: self.providerName.clone(), dataClass: self.dataClass }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_PROVIDER").field("providerID", &self.providerID).field("providerState", &self.providerState).field("providerName", &self.providerName).field("dataClass", &self.dataClass).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_PROVIDER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.providerID == other.providerID && self.providerState == other.providerState && self.providerName == other.providerName && self.dataClass == other.dataClass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_PROVIDER2 {
    fn clone(&self) -> Self {
        Self { provider: self.provider.clone(), cellularClass: self.cellularClass, signalStrength: self.signalStrength, signalError: self.signalError }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_PROVIDER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MBN_PROVIDER2").field("provider", &self.provider).field("cellularClass", &self.cellularClass).field("signalStrength", &self.signalStrength).field("signalError", &self.signalError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_PROVIDER2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_PROVIDER2 {
    fn eq(&self, other: &Self) -> bool {
        self.provider == other.provider && self.cellularClass == other.cellularClass && self.signalStrength == other.signalStrength && self.signalError == other.signalError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_PROVIDER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_PROVIDER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_PROVIDER_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_PROVIDER_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PROVIDER_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_PROVIDER_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_PROVIDER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_PROVIDER_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_RADIO {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_RADIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_RADIO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_READY_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_READY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_READY_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_REGISTER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_REGISTER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_REGISTER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_REGISTER_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_REGISTER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_REGISTER_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_REGISTRATION_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_REGISTRATION_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_REGISTRATION_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_SIGNAL_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_SIGNAL_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SIGNAL_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_SMS_CAPS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_SMS_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_CAPS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_SMS_CDMA_ENCODING {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_SMS_CDMA_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_CDMA_ENCODING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_SMS_CDMA_LANG {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_SMS_CDMA_LANG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_CDMA_LANG").field(&self.0).finish()
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
unsafe impl ::windows::core::Abi for MBN_SMS_FILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MBN_SMS_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MBN_SMS_FILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MBN_SMS_FILTER {}
impl ::core::default::Default for MBN_SMS_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_SMS_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_SMS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_SMS_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_SMS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_SMS_STATUS_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_SMS_STATUS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_SMS_STATUS_FLAG").field(&self.0).finish()
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
unsafe impl ::windows::core::Abi for MBN_SMS_STATUS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MBN_SMS_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MBN_SMS_STATUS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MBN_SMS_STATUS_INFO {}
impl ::core::default::Default for MBN_SMS_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_VOICE_CALL_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_VOICE_CALL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_VOICE_CALL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MBN_VOICE_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MBN_VOICE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MBN_VOICE_CLASS").field(&self.0).finish()
    }
}
pub const MbnConnectionManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05c_4418_11dd_90ed_001c257ccff1);
pub const MbnConnectionProfileManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05a_4418_11dd_90ed_001c257ccff1);
pub const MbnDeviceServicesManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2269daa3_2a9f_4165_a501_ce00a6f7a75b);
pub const MbnInterfaceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05b_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WWAEXT_SMS_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WWAEXT_SMS_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WWAEXT_SMS_CONSTANTS").field(&self.0).finish()
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
unsafe impl ::windows::core::Abi for __DummyPinType__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for __DummyPinType__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<__DummyPinType__>()) == 0 }
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
unsafe impl ::windows::core::Abi for __mbnapi_ReferenceRemainingTypes__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for __mbnapi_ReferenceRemainingTypes__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<__mbnapi_ReferenceRemainingTypes__>()) == 0 }
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
