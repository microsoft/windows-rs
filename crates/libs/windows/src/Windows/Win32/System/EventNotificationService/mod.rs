#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_AOL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensLogon(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon {
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Logon<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Logon)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Logoff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Logoff)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartShell<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartShell)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayLock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayLock)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayUnlock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayUnlock)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartScreenSaver<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartScreenSaver)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StopScreenSaver<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopScreenSaver)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensLogon> for ::windows::core::IUnknown {
    fn from(value: ISensLogon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensLogon> for ::windows::core::IUnknown {
    fn from(value: &ISensLogon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensLogon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensLogon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensLogon> for super::Com::IDispatch {
    fn from(value: ISensLogon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensLogon> for super::Com::IDispatch {
    fn from(value: &ISensLogon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISensLogon {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ISensLogon {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensLogon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensLogon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensLogon {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensLogon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensLogon").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensLogon {
    type Vtable = ISensLogon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab3_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensLogon_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Logon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Logoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logoff: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartShell: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartShell: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayLock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUnlock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartScreenSaver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartScreenSaver: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StopScreenSaver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StopScreenSaver: usize,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensLogon2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon2 {
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Logon<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Logon)(::core::mem::transmute_copy(self), bstrusername.into_param().abi(), ::core::mem::transmute(dwsessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Logoff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Logoff)(::core::mem::transmute_copy(self), bstrusername.into_param().abi(), ::core::mem::transmute(dwsessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionDisconnect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SessionDisconnect)(::core::mem::transmute_copy(self), bstrusername.into_param().abi(), ::core::mem::transmute(dwsessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionReconnect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SessionReconnect)(::core::mem::transmute_copy(self), bstrusername.into_param().abi(), ::core::mem::transmute(dwsessionid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostShell<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PostShell)(::core::mem::transmute_copy(self), bstrusername.into_param().abi(), ::core::mem::transmute(dwsessionid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensLogon2> for ::windows::core::IUnknown {
    fn from(value: ISensLogon2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensLogon2> for ::windows::core::IUnknown {
    fn from(value: &ISensLogon2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensLogon2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensLogon2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensLogon2> for super::Com::IDispatch {
    fn from(value: ISensLogon2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensLogon2> for super::Com::IDispatch {
    fn from(value: &ISensLogon2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISensLogon2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ISensLogon2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensLogon2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensLogon2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensLogon2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensLogon2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensLogon2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensLogon2 {
    type Vtable = ISensLogon2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab4_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensLogon2_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Logon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Logoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logoff: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionDisconnect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionReconnect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostShell: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostShell: usize,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensNetwork(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensNetwork {
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectionMade<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnection: Param0, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionMade)(::core::mem::transmute_copy(self), bstrconnection.into_param().abi(), ::core::mem::transmute(ultype), ::core::mem::transmute(lpqocinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectionMadeNoQOCInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnection: Param0, ultype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionMadeNoQOCInfo)(::core::mem::transmute_copy(self), bstrconnection.into_param().abi(), ::core::mem::transmute(ultype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectionLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnection: Param0, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionLost)(::core::mem::transmute_copy(self), bstrconnection.into_param().abi(), ::core::mem::transmute(ultype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestinationReachable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdestination: Param0, bstrconnection: Param1, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DestinationReachable)(::core::mem::transmute_copy(self), bstrdestination.into_param().abi(), bstrconnection.into_param().abi(), ::core::mem::transmute(ultype), ::core::mem::transmute(lpqocinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestinationReachableNoQOCInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdestination: Param0, bstrconnection: Param1, ultype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DestinationReachableNoQOCInfo)(::core::mem::transmute_copy(self), bstrdestination.into_param().abi(), bstrconnection.into_param().abi(), ::core::mem::transmute(ultype)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensNetwork> for ::windows::core::IUnknown {
    fn from(value: ISensNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensNetwork> for ::windows::core::IUnknown {
    fn from(value: &ISensNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensNetwork> for super::Com::IDispatch {
    fn from(value: ISensNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensNetwork> for super::Com::IDispatch {
    fn from(value: &ISensNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISensNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ISensNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensNetwork").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensNetwork {
    type Vtable = ISensNetwork_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab1_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensNetwork_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionMade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionMade: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionMadeNoQOCInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionMadeNoQOCInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionLost: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestinationReachable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestinationReachable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestinationReachableNoQOCInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestinationReachableNoQOCInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensOnNow(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensOnNow {
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
    pub unsafe fn OnACPower(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnACPower)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
    pub unsafe fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBatteryPower)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbatterylifepercent)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
    pub unsafe fn BatteryLow(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BatteryLow)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbatterylifepercent)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensOnNow> for ::windows::core::IUnknown {
    fn from(value: ISensOnNow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensOnNow> for ::windows::core::IUnknown {
    fn from(value: &ISensOnNow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISensOnNow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISensOnNow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISensOnNow> for super::Com::IDispatch {
    fn from(value: ISensOnNow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISensOnNow> for super::Com::IDispatch {
    fn from(value: &ISensOnNow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISensOnNow {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a ISensOnNow {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensOnNow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensOnNow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensOnNow {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensOnNow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensOnNow").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensOnNow {
    type Vtable = ISensOnNow_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab2_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensOnNow_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub OnACPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnBatteryPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT,
    pub BatteryLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDestinationReachableA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpszdestination: Param0, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDestinationReachableA(lpszdestination: ::windows::core::PCSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsDestinationReachableA(lpszdestination.into_param().abi(), ::core::mem::transmute(lpqocinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDestinationReachableW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszdestination: Param0, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDestinationReachableW(lpszdestination: ::windows::core::PCWSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsDestinationReachableW(lpszdestination.into_param().abi(), ::core::mem::transmute(lpqocinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNetworkAlive(lpdwflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsNetworkAlive(lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsNetworkAlive(::core::mem::transmute(lpdwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
impl ::core::marker::Copy for QOCINFO {}
impl ::core::clone::Clone for QOCINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOCINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOCINFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwInSpeed", &self.dwInSpeed).field("dwOutSpeed", &self.dwOutSpeed).finish()
    }
}
unsafe impl ::windows::core::Abi for QOCINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOCINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOCINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOCINFO {}
impl ::core::default::Default for QOCINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SENS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597cafe_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_LOGON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978630_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_LOGON2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978650_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_NETWORK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978620_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_ONNOW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978640_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_PUBLISHER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fee1bd6_5b9b_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_LCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3938ab0_5b9d_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_WININET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3938ab5_5b9d_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SENS_CONNECTION_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_LAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_WAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(1u32);
impl ::core::marker::Copy for SENS_CONNECTION_TYPE {}
impl ::core::clone::Clone for SENS_CONNECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SENS_CONNECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SENS_CONNECTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SENS_CONNECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SENS_CONNECTION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub struct SENS_QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOutSpeed: u32,
    pub dwInSpeed: u32,
}
impl ::core::marker::Copy for SENS_QOCINFO {}
impl ::core::clone::Clone for SENS_QOCINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SENS_QOCINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SENS_QOCINFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwOutSpeed", &self.dwOutSpeed).field("dwInSpeed", &self.dwInSpeed).finish()
    }
}
unsafe impl ::windows::core::Abi for SENS_QOCINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SENS_QOCINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SENS_QOCINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SENS_QOCINFO {}
impl ::core::default::Default for SENS_QOCINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
