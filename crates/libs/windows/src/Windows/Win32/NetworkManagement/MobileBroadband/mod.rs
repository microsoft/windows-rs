#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IDummyMBNUCMExt(::windows::core::IUnknown);
impl IDummyMBNUCMExt {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDummyMBNUCMExt> for ::windows::core::IUnknown {
    fn from(value: IDummyMBNUCMExt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDummyMBNUCMExt> for ::windows::core::IUnknown {
    fn from(value: &IDummyMBNUCMExt) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDummyMBNUCMExt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDummyMBNUCMExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDummyMBNUCMExt {}
impl ::core::fmt::Debug for IDummyMBNUCMExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDummyMBNUCMExt").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDummyMBNUCMExt {
    type Vtable = IDummyMBNUCMExtVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_ffff_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyMBNUCMExtVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnection(::windows::core::IUnknown);
impl IMbnConnection {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectionID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(connectionmode), strprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(connectionstate), ::core::mem::transmute(profilename)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetVoiceCallState(&self) -> ::windows::core::Result<MBN_VOICE_CALL_STATE> {
        let mut result__: MBN_VOICE_CALL_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_VOICE_CALL_STATE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetActivationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnection {
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
    type Vtable = IMbnConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200d_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionContext(::windows::core::IUnknown);
impl IMbnConnectionContext {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProvisionedContexts(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProvisionedContext<'a, Param0: ::windows::core::IntoParam<'a, MBN_CONTEXT>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, provisionedcontexts: Param0, providerid: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), provisionedcontexts.into_param().abi(), providerid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionContext {
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
    type Vtable = IMbnConnectionContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200b_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionContextEvents(::windows::core::IUnknown);
impl IMbnConnectionContextEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnProvisionedContextListChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionContext>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSetProvisionedContextComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionContext>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionContextEvents {
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
    type Vtable = IMbnConnectionContextEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200c_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContextEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionEvents(::windows::core::IUnknown);
impl IMbnConnectionEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnConnectComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newconnection.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnDisconnectComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newconnection.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnConnectStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnVoiceCallStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionEvents {
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
    type Vtable = IMbnConnectionEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200e_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionManager(::windows::core::IUnknown);
impl IMbnConnectionManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, connectionid: Param0) -> ::windows::core::Result<IMbnConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), connectionid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnConnection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnections(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionManager {
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
    type Vtable = IMbnConnectionManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201d_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: super::super::Foundation::PWSTR, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionManagerEvents(::windows::core::IUnknown);
impl IMbnConnectionManagerEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnConnectionArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnConnectionRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, oldconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), oldconnection.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionManagerEvents {
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
    type Vtable = IMbnConnectionManagerEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201e_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManagerEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionProfile(::windows::core::IUnknown);
impl IMbnConnectionProfile {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProfileXmlData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), strprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionProfile {
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
    type Vtable = IMbnConnectionProfileVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2010_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiledata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileEvents(::windows::core::IUnknown);
impl IMbnConnectionProfileEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnProfileUpdate<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, newprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newprofile.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionProfileEvents {
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
    type Vtable = IMbnConnectionProfileEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2011_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileEventsVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileManager(::windows::core::IUnknown);
impl IMbnConnectionProfileManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionProfiles<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, mbninterface: Param0) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, mbninterface: Param0, profilename: Param1) -> ::windows::core::Result<IMbnConnectionProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), profilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnConnectionProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, xmlprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), xmlprofile.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionProfileManager {
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
    type Vtable = IMbnConnectionProfileManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, profilename: super::super::Foundation::PWSTR, connectionprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnConnectionProfileManagerEvents(::windows::core::IUnknown);
impl IMbnConnectionProfileManagerEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnConnectionProfileArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, newconnectionprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newconnectionprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnConnectionProfileRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, oldconnectionprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), oldconnectionprofile.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnConnectionProfileManagerEvents {
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
    type Vtable = IMbnConnectionProfileManagerEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManagerEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnDeviceService(::windows::core::IUnknown);
impl IMbnDeviceService {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn QuerySupportedCommands(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OpenCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn CloseCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OpenDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn CloseDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceServiceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCommandSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDataSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnDeviceService {
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
    type Vtable = IMbnDeviceServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3bb9a71_dc70_4be9_a4da_7886ae8b191b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnDeviceServiceStateEvents(::windows::core::IUnknown);
impl IMbnDeviceServiceStateEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSessionsStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfaceid: Param0, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(statechange)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnDeviceServiceStateEvents {
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
    type Vtable = IMbnDeviceServiceStateEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d3ff196_89ee_49d8_8b60_33ffddffc58d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServiceStateEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesContext(::windows::core::IUnknown);
impl IMbnDeviceServicesContext {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateDeviceServices(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, deviceserviceid: Param0) -> ::windows::core::Result<IMbnDeviceService> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), deviceserviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnDeviceService>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn MaxCommandSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn MaxDataSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnDeviceServicesContext {
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
    type Vtable = IMbnDeviceServicesContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc5ac347_1592_4068_80bb_6a57580150d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesEvents(::windows::core::IUnknown);
impl IMbnDeviceServicesEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuerySupportedCommandsComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(commandidlist), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnOpenCommandSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnCloseCommandSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetCommandComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(responseid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQueryCommandComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(responseid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(eventid), ::core::mem::transmute(deviceservicedata)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnOpenDataSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnCloseDataSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnWriteDataComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReadData<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(deviceservicedata)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInterfaceStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfaceid: Param0, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(statechange)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnDeviceServicesEvents {
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
    type Vtable = IMbnDeviceServicesEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a900c19_6824_4e97_b76e_cf239d0ca642);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnDeviceServicesManager(::windows::core::IUnknown);
impl IMbnDeviceServicesManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceServicesContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, networkinterfaceid: Param0) -> ::windows::core::Result<IMbnDeviceServicesContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), networkinterfaceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnDeviceServicesContext>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnDeviceServicesManager {
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
    type Vtable = IMbnDeviceServicesManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20a26258_6811_4478_ac1d_13324e45e41c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnInterface(::windows::core::IUnknown);
impl IMbnInterface {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInterfaceCapability(&self) -> ::windows::core::Result<MBN_INTERFACE_CAPS> {
        let mut result__: ::core::mem::ManuallyDrop<MBN_INTERFACE_CAPS> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_INTERFACE_CAPS>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetSubscriberInformation(&self) -> ::windows::core::Result<IMbnSubscriberInformation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnSubscriberInformation>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetReadyState(&self) -> ::windows::core::Result<MBN_READY_STATE> {
        let mut result__: MBN_READY_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_READY_STATE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn InEmergencyMode(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHomeProvider(&self) -> ::windows::core::Result<MBN_PROVIDER> {
        let mut result__: ::core::mem::ManuallyDrop<MBN_PROVIDER> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PROVIDER>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(preferredproviders), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(age), ::core::mem::transmute(visibleproviders)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMbnConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnConnection>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnInterface {
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
    type Vtable = IMbnInterfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2001_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriberinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emergencymode: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnInterfaceEvents(::windows::core::IUnknown);
impl IMbnInterfaceEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnInterfaceCapabilityAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSubscriberInformationChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnReadyStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnEmergencyModeChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnHomeProviderAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnPreferredProvidersChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSetPreferredProvidersComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnScanNetworkComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnInterfaceEvents {
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
    type Vtable = IMbnInterfaceEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2002_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnInterfaceManager(::windows::core::IUnknown);
impl IMbnInterfaceManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInterface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, interfaceid: Param0) -> ::windows::core::Result<IMbnInterface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMbnInterface>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnInterfaceManager {
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
    type Vtable = IMbnInterfaceManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201b_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: super::super::Foundation::PWSTR, mbninterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnInterfaceManagerEvents(::windows::core::IUnknown);
impl IMbnInterfaceManagerEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnInterfaceArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnInterfaceRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, oldinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), oldinterface.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnInterfaceManagerEvents {
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
    type Vtable = IMbnInterfaceManagerEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201c_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManagerEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnMultiCarrier(::windows::core::IUnknown);
impl IMbnMultiCarrier {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(homeprovider), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(age), ::core::mem::transmute(visibleproviders)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedCellularClasses(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetCurrentCellularClass(&self) -> ::windows::core::Result<MBN_CELLULAR_CLASS> {
        let mut result__: MBN_CELLULAR_CLASS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_CELLULAR_CLASS>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnMultiCarrier {
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
    type Vtable = IMbnMultiCarrierVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2020_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrierVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnMultiCarrierEvents(::windows::core::IUnknown);
impl IMbnMultiCarrierEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSetHomeProviderComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnCurrentCellularClassChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnPreferredProvidersChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnScanNetworkComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnInterfaceCapabilityChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnMultiCarrierEvents {
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
    type Vtable = IMbnMultiCarrierEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_2021_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrierEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnPin(::windows::core::IUnknown);
impl IMbnPin {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn PinType(&self) -> ::windows::core::Result<MBN_PIN_TYPE> {
        let mut result__: MBN_PIN_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PIN_TYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn PinFormat(&self) -> ::windows::core::Result<MBN_PIN_FORMAT> {
        let mut result__: MBN_PIN_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PIN_FORMAT>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn PinLengthMin(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn PinLengthMax(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn PinMode(&self) -> ::windows::core::Result<MBN_PIN_MODE> {
        let mut result__: MBN_PIN_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_PIN_MODE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Disable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Change<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0, newpin: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pin.into_param().abi(), newpin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unblock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, puk: Param0, newpin: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), puk.into_param().abi(), newpin.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetPinManager(&self) -> ::windows::core::Result<IMbnPinManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnPinManager>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnPin {
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
    type Vtable = IMbnPinVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2007_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puk: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnPinEvents(::windows::core::IUnknown);
impl IMbnPinEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnEnableComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnDisableComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnEnterComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnChangeComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnUnblockComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnPinEvents {
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
    type Vtable = IMbnPinEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2008_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnPinManager(::windows::core::IUnknown);
impl IMbnPinManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPinList(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetPin(&self, pintype: MBN_PIN_TYPE) -> ::windows::core::Result<IMbnPin> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pintype), ::core::mem::transmute(&mut result__)).from_abi::<IMbnPin>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetPinState(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnPinManager {
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
    type Vtable = IMbnPinManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2005_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnPinManagerEvents(::windows::core::IUnknown);
impl IMbnPinManagerEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnPinListAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnPinManager>>(&self, pinmanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pinmanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnGetPinStateComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPinManager>, Param1: ::windows::core::IntoParam<'a, MBN_PIN_INFO>>(&self, pinmanager: Param0, pininfo: Param1, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pinmanager.into_param().abi(), pininfo.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnPinManagerEvents {
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
    type Vtable = IMbnPinManagerEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2006_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManagerEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnRadio(::windows::core::IUnknown);
impl IMbnRadio {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SoftwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__: MBN_RADIO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_RADIO>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn HardwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__: MBN_RADIO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_RADIO>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiostate), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnRadio {
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
    type Vtable = IMbnRadioVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdccccab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadioVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnRadioEvents(::windows::core::IUnknown);
impl IMbnRadioEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnRadioStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRadio>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSetSoftwareRadioStateComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnRadio>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnRadioEvents {
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
    type Vtable = IMbnRadioEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_201f_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadioEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnRegistration(::windows::core::IUnknown);
impl IMbnRegistration {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetRegisterState(&self) -> ::windows::core::Result<MBN_REGISTER_STATE> {
        let mut result__: MBN_REGISTER_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_REGISTER_STATE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetRegisterMode(&self) -> ::windows::core::Result<MBN_REGISTER_MODE> {
        let mut result__: MBN_REGISTER_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_REGISTER_MODE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProviderID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRoamingText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetAvailableDataClasses(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetCurrentDataClass(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetRegistrationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetPacketAttachNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRegisterMode<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, registermode: MBN_REGISTER_MODE, providerid: Param1, dataclass: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(registermode), providerid.into_param().abi(), ::core::mem::transmute(dataclass), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnRegistration {
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
    type Vtable = IMbnRegistrationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2009_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistrationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roamingtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: super::super::Foundation::PWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnRegistrationEvents(::windows::core::IUnknown);
impl IMbnRegistrationEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnRegisterModeAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnRegisterStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnPacketServiceStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSetRegisterModeComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnRegistrationEvents {
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
    type Vtable = IMbnRegistrationEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200a_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistrationEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnServiceActivation(::windows::core::IUnknown);
impl IMbnServiceActivation {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnServiceActivation {
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
    type Vtable = IMbnServiceActivationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2017_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnServiceActivationEvents(::windows::core::IUnknown);
impl IMbnServiceActivationEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivationComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnServiceActivation>>(&self, serviceactivation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), serviceactivation.into_param().abi(), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(requestid), ::core::mem::transmute(status), ::core::mem::transmute(networkerror)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnServiceActivationEvents {
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
    type Vtable = IMbnServiceActivationEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2018_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivationEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceactivation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSignal(::windows::core::IUnknown);
impl IMbnSignal {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetSignalStrength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetSignalError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSignal {
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
    type Vtable = IMbnSignalVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2003_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignalVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSignalEvents(::windows::core::IUnknown);
impl IMbnSignalEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSignalStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSignal>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSignalEvents {
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
    type Vtable = IMbnSignalEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2004_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignalEventsVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSms(::windows::core::IUnknown);
impl IMbnSms {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetSmsConfiguration(&self) -> ::windows::core::Result<IMbnSmsConfiguration> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMbnSmsConfiguration>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SetSmsConfiguration<'a, Param0: ::windows::core::IntoParam<'a, IMbnSmsConfiguration>>(&self, smsconfiguration: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), smsconfiguration.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmsSendPdu<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pdudata: Param0, size: u8) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdudata.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SmsSendCdma<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, address: Param0, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), address.into_param().abi(), ::core::mem::transmute(encoding), ::core::mem::transmute(language), ::core::mem::transmute(sizeincharacters), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsfilter), ::core::mem::transmute(smsformat), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsfilter), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn GetSmsStatus(&self) -> ::windows::core::Result<MBN_SMS_STATUS_INFO> {
        let mut result__: MBN_SMS_STATUS_INFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_STATUS_INFO>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSms {
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
    type Vtable = IMbnSmsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2015_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsconfiguration: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdudata: super::super::Foundation::PWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSmsConfiguration(::windows::core::IUnknown);
impl IMbnSmsConfiguration {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceCenterAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceCenterAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, scaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), scaddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn MaxMessageIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn CdmaShortMsgSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SmsFormat(&self) -> ::windows::core::Result<MBN_SMS_FORMAT> {
        let mut result__: MBN_SMS_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_FORMAT>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsformat)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSmsConfiguration {
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
    type Vtable = IMbnSmsConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2012_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSmsEvents(::windows::core::IUnknown);
impl IMbnSmsEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSmsConfigurationChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sms.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSetSmsConfigurationComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSmsSendComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsReadComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(smsformat), ::core::mem::transmute(readmsgs), ::core::mem::transmute(moremsgs), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsNewClass0Message<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(smsformat), ::core::mem::transmute(readmsgs)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSmsDeleteComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn OnSmsStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), sms.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSmsEvents {
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
    type Vtable = IMbnSmsEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2016_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSmsReadMsgPdu(::windows::core::IUnknown);
impl IMbnSmsReadMsgPdu {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__: MBN_MSG_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_MSG_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PduData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSmsReadMsgPdu {
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
    type Vtable = IMbnSmsReadMsgPduVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2013_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgPduVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdudata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSmsReadMsgTextCdma(::windows::core::IUnknown);
impl IMbnSmsReadMsgTextCdma {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__: MBN_MSG_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_MSG_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn EncodingID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_ENCODING> {
        let mut result__: MBN_SMS_CDMA_ENCODING = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_CDMA_ENCODING>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn LanguageID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_LANG> {
        let mut result__: MBN_SMS_CDMA_LANG = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<MBN_SMS_CDMA_LANG>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
    pub unsafe fn SizeInCharacters(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSmsReadMsgTextCdma {
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
    type Vtable = IMbnSmsReadMsgTextCdmaVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2014_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgTextCdmaVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnSubscriberInformation(::windows::core::IUnknown);
impl IMbnSubscriberInformation {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriberID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimIccID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TelephoneNumbers(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnSubscriberInformation {
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
    type Vtable = IMbnSubscriberInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x459ecc43_bcf5_11dc_a8a8_001321f1405f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSubscriberInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriberid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simiccid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnVendorSpecificEvents(::windows::core::IUnknown);
impl IMbnVendorSpecificEvents {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<'a, Param0: ::windows::core::IntoParam<'a, IMbnVendorSpecificOperation>>(&self, vendoroperation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), vendoroperation.into_param().abi(), ::core::mem::transmute(vendorspecificdata)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetVendorSpecificComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnVendorSpecificOperation>>(&self, vendoroperation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), vendoroperation.into_param().abi(), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(requestid)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnVendorSpecificEvents {
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
    type Vtable = IMbnVendorSpecificEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201a_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
#[repr(transparent)]
pub struct IMbnVendorSpecificOperation(::windows::core::IUnknown);
impl IMbnVendorSpecificOperation {
    #[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMbnVendorSpecificOperation {
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
    type Vtable = IMbnVendorSpecificOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2019_4bbb_aaee_338e368af6fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_ACTIVATION_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ACTIVATION_STATE_NONE: MBN_ACTIVATION_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ACTIVATION_STATE_ACTIVATED: MBN_ACTIVATION_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ACTIVATION_STATE_ACTIVATING: MBN_ACTIVATION_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATED: MBN_ACTIVATION_STATE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATING: MBN_ACTIVATION_STATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_AUTH_PROTOCOL = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_AUTH_PROTOCOL_NONE: MBN_AUTH_PROTOCOL = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_AUTH_PROTOCOL_PAP: MBN_AUTH_PROTOCOL = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_AUTH_PROTOCOL_CHAP: MBN_AUTH_PROTOCOL = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: MBN_AUTH_PROTOCOL = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_BAND_CLASS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_NONE: MBN_BAND_CLASS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_0: MBN_BAND_CLASS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_I: MBN_BAND_CLASS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_II: MBN_BAND_CLASS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_III: MBN_BAND_CLASS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_IV: MBN_BAND_CLASS = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_V: MBN_BAND_CLASS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_VI: MBN_BAND_CLASS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_VII: MBN_BAND_CLASS = 128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_VIII: MBN_BAND_CLASS = 256i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_IX: MBN_BAND_CLASS = 512i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_X: MBN_BAND_CLASS = 1024i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_XI: MBN_BAND_CLASS = 2048i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_XII: MBN_BAND_CLASS = 4096i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_XIII: MBN_BAND_CLASS = 8192i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_XIV: MBN_BAND_CLASS = 16384i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_XV: MBN_BAND_CLASS = 32768i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_XVI: MBN_BAND_CLASS = 65536i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_XVII: MBN_BAND_CLASS = 131072i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_BAND_CLASS_CUSTOM: MBN_BAND_CLASS = -2147483648i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_CELLULAR_CLASS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CELLULAR_CLASS_NONE: MBN_CELLULAR_CLASS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CELLULAR_CLASS_GSM: MBN_CELLULAR_CLASS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CELLULAR_CLASS_CDMA: MBN_CELLULAR_CLASS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_COMPRESSION = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_COMPRESSION_NONE: MBN_COMPRESSION = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_COMPRESSION_ENABLE: MBN_COMPRESSION = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_CONNECTION_MODE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONNECTION_MODE_PROFILE: MBN_CONNECTION_MODE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONNECTION_MODE_TMP_PROFILE: MBN_CONNECTION_MODE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_CONTEXT_CONSTANTS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ACCESSSTRING_LEN: MBN_CONTEXT_CONSTANTS = 100i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_USERNAME_LEN: MBN_CONTEXT_CONSTANTS = 255i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PASSWORD_LEN: MBN_CONTEXT_CONSTANTS = 255i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_ID_APPEND: MBN_CONTEXT_CONSTANTS = -1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_CONTEXT_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_TYPE_NONE: MBN_CONTEXT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_TYPE_INTERNET: MBN_CONTEXT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_TYPE_VPN: MBN_CONTEXT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_TYPE_VOICE: MBN_CONTEXT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: MBN_CONTEXT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_TYPE_CUSTOM: MBN_CONTEXT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CONTEXT_TYPE_PURCHASE: MBN_CONTEXT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_CTRL_CAPS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_NONE: MBN_CTRL_CAPS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_REG_MANUAL: MBN_CTRL_CAPS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: MBN_CTRL_CAPS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: MBN_CTRL_CAPS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: MBN_CTRL_CAPS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: MBN_CTRL_CAPS = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: MBN_CTRL_CAPS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_USSD: MBN_CTRL_CAPS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CTRL_CAPS_MULTI_MODE: MBN_CTRL_CAPS = 128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_DATA_CLASS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_NONE: MBN_DATA_CLASS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_GPRS: MBN_DATA_CLASS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_EDGE: MBN_DATA_CLASS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_UMTS: MBN_DATA_CLASS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_HSDPA: MBN_DATA_CLASS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_HSUPA: MBN_DATA_CLASS = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_LTE: MBN_DATA_CLASS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_5G_NSA: MBN_DATA_CLASS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_5G_SA: MBN_DATA_CLASS = 128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_1XRTT: MBN_DATA_CLASS = 65536i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_1XEVDO: MBN_DATA_CLASS = 131072i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_1XEVDO_REVA: MBN_DATA_CLASS = 262144i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_1XEVDV: MBN_DATA_CLASS = 524288i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_3XRTT: MBN_DATA_CLASS = 1048576i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_1XEVDO_REVB: MBN_DATA_CLASS = 2097152i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_UMB: MBN_DATA_CLASS = 4194304i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DATA_CLASS_CUSTOM: MBN_DATA_CLASS = -2147483648i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_DEVICE_SERVICES_INTERFACE_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_DEVICE_SERVICE_SESSIONS_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: MBN_DEVICE_SERVICE_SESSIONS_STATE = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_INTERFACE_CAPS_CONSTANTS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_DEVICEID_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 18i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_MANUFACTURER_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_MODEL_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_FIRMWARE_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_MSG_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_MSG_STATUS_NEW: MBN_MSG_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_MSG_STATUS_OLD: MBN_MSG_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_MSG_STATUS_DRAFT: MBN_MSG_STATUS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_MSG_STATUS_SENT: MBN_MSG_STATUS = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_PIN_CONSTANTS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: MBN_PIN_CONSTANTS = -1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_LENGTH_UNKNOWN: MBN_PIN_CONSTANTS = -1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_PIN_FORMAT = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_FORMAT_NONE: MBN_PIN_FORMAT = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_FORMAT_NUMERIC: MBN_PIN_FORMAT = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_FORMAT_ALPHANUMERIC: MBN_PIN_FORMAT = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_PIN_MODE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_MODE_ENABLED: MBN_PIN_MODE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_MODE_DISABLED: MBN_PIN_MODE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_PIN_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_STATE_NONE: MBN_PIN_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_STATE_ENTER: MBN_PIN_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_STATE_UNBLOCK: MBN_PIN_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_PIN_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_NONE: MBN_PIN_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_CUSTOM: MBN_PIN_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_PIN1: MBN_PIN_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_PIN2: MBN_PIN_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_DEVICE_SIM_PIN: MBN_PIN_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN: MBN_PIN_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_NETWORK_PIN: MBN_PIN_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_NETWORK_SUBSET_PIN: MBN_PIN_TYPE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_SVC_PROVIDER_PIN: MBN_PIN_TYPE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_CORPORATE_PIN: MBN_PIN_TYPE = 9i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PIN_TYPE_SUBSIDY_LOCK: MBN_PIN_TYPE = 10i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_PROVIDER_CONSTANTS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDERNAME_LEN: MBN_PROVIDER_CONSTANTS = 20i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDERID_LEN: MBN_PROVIDER_CONSTANTS = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_PROVIDER_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDER_STATE_NONE: MBN_PROVIDER_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDER_STATE_HOME: MBN_PROVIDER_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDER_STATE_FORBIDDEN: MBN_PROVIDER_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDER_STATE_PREFERRED: MBN_PROVIDER_STATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDER_STATE_VISIBLE: MBN_PROVIDER_STATE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDER_STATE_REGISTERED: MBN_PROVIDER_STATE = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: MBN_PROVIDER_STATE = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_RADIO = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_RADIO_OFF: MBN_RADIO = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_RADIO_ON: MBN_RADIO = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_READY_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_OFF: MBN_READY_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_INITIALIZED: MBN_READY_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_SIM_NOT_INSERTED: MBN_READY_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_BAD_SIM: MBN_READY_STATE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_FAILURE: MBN_READY_STATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_NOT_ACTIVATED: MBN_READY_STATE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_DEVICE_LOCKED: MBN_READY_STATE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_DEVICE_BLOCKED: MBN_READY_STATE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_READY_STATE_NO_ESIM_PROFILE: MBN_READY_STATE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_REGISTER_MODE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_MODE_NONE: MBN_REGISTER_MODE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_MODE_AUTOMATIC: MBN_REGISTER_MODE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_MODE_MANUAL: MBN_REGISTER_MODE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_REGISTER_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_STATE_NONE: MBN_REGISTER_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_STATE_DEREGISTERED: MBN_REGISTER_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_STATE_SEARCHING: MBN_REGISTER_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_STATE_HOME: MBN_REGISTER_STATE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_STATE_ROAMING: MBN_REGISTER_STATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_STATE_PARTNER: MBN_REGISTER_STATE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_REGISTER_STATE_DENIED: MBN_REGISTER_STATE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_REGISTRATION_CONSTANTS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ROAMTEXT_LEN: MBN_REGISTRATION_CONSTANTS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CDMA_DEFAULT_PROVIDER_ID: MBN_REGISTRATION_CONSTANTS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_SIGNAL_CONSTANTS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_RSSI_DEFAULT: MBN_SIGNAL_CONSTANTS = -1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_RSSI_DISABLE: MBN_SIGNAL_CONSTANTS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_RSSI_UNKNOWN: MBN_SIGNAL_CONSTANTS = 99i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_ERROR_RATE_UNKNOWN: MBN_SIGNAL_CONSTANTS = 99i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_SMS_CAPS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CAPS_NONE: MBN_SMS_CAPS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CAPS_PDU_RECEIVE: MBN_SMS_CAPS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CAPS_PDU_SEND: MBN_SMS_CAPS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CAPS_TEXT_RECEIVE: MBN_SMS_CAPS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CAPS_TEXT_SEND: MBN_SMS_CAPS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_SMS_CDMA_ENCODING = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_OCTET: MBN_SMS_CDMA_ENCODING = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_EPM: MBN_SMS_CDMA_ENCODING = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_7BIT_ASCII: MBN_SMS_CDMA_ENCODING = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_IA5: MBN_SMS_CDMA_ENCODING = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_UNICODE: MBN_SMS_CDMA_ENCODING = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_SHIFT_JIS: MBN_SMS_CDMA_ENCODING = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_KOREAN: MBN_SMS_CDMA_ENCODING = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN_HEBREW: MBN_SMS_CDMA_ENCODING = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN: MBN_SMS_CDMA_ENCODING = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_ENCODING_GSM_7BIT: MBN_SMS_CDMA_ENCODING = 9i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_SMS_CDMA_LANG = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_NONE: MBN_SMS_CDMA_LANG = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_ENGLISH: MBN_SMS_CDMA_LANG = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_FRENCH: MBN_SMS_CDMA_LANG = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_SPANISH: MBN_SMS_CDMA_LANG = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_JAPANESE: MBN_SMS_CDMA_LANG = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_KOREAN: MBN_SMS_CDMA_LANG = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_CHINESE: MBN_SMS_CDMA_LANG = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_CDMA_LANG_HEBREW: MBN_SMS_CDMA_LANG = 7i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_SMS_FLAG = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_ALL: MBN_SMS_FLAG = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_INDEX: MBN_SMS_FLAG = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_NEW: MBN_SMS_FLAG = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_OLD: MBN_SMS_FLAG = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_SENT: MBN_SMS_FLAG = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_DRAFT: MBN_SMS_FLAG = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_SMS_FORMAT = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FORMAT_NONE: MBN_SMS_FORMAT = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FORMAT_PDU: MBN_SMS_FORMAT = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FORMAT_TEXT: MBN_SMS_FORMAT = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_SMS_STATUS_FLAG = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_NONE: MBN_SMS_STATUS_FLAG = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: MBN_SMS_STATUS_FLAG = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_SMS_FLAG_NEW_MESSAGE: MBN_SMS_STATUS_FLAG = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_VOICE_CALL_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_VOICE_CALL_STATE_NONE: MBN_VOICE_CALL_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: MBN_VOICE_CALL_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_VOICE_CALL_STATE_HANGUP: MBN_VOICE_CALL_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type MBN_VOICE_CLASS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_VOICE_CLASS_NONE: MBN_VOICE_CLASS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_VOICE_CLASS_NO_VOICE: MBN_VOICE_CLASS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: MBN_VOICE_CLASS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: MBN_VOICE_CLASS = 3i32;
pub const MbnConnectionManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05c_4418_11dd_90ed_001c257ccff1);
pub const MbnConnectionProfileManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05a_4418_11dd_90ed_001c257ccff1);
pub const MbnDeviceServicesManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2269daa3_2a9f_4165_a501_ce00a6f7a75b);
pub const MbnInterfaceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05b_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub type WWAEXT_SMS_CONSTANTS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_MESSAGE_INDEX_NONE: WWAEXT_SMS_CONSTANTS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN: WWAEXT_SMS_CONSTANTS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_MAX: WWAEXT_SMS_CONSTANTS = 160i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_MobileBroadband'*"]
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
