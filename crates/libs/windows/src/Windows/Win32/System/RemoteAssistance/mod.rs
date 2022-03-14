#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DRendezvousSessionEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DRendezvousSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DRendezvousSessionEvents> for ::windows::core::IUnknown {
    fn from(value: DRendezvousSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DRendezvousSessionEvents> for ::windows::core::IUnknown {
    fn from(value: &DRendezvousSessionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DRendezvousSessionEvents> for super::Com::IDispatch {
    fn from(value: DRendezvousSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DRendezvousSessionEvents> for super::Com::IDispatch {
    fn from(value: &DRendezvousSessionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DRendezvousSessionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DRendezvousSessionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DRendezvousSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DRendezvousSessionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRendezvousSessionEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DRendezvousSessionEvents {
    type Vtable = DRendezvousSessionEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa19cf8_64c4_4f53_ae60_635b3806eca6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DRendezvousSessionEvents_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
pub struct IRendezvousApplication(::windows::core::IUnknown);
impl IRendezvousApplication {
    #[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
    pub unsafe fn SetRendezvousSession<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, prendezvoussession: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRendezvousSession)(::core::mem::transmute_copy(self), prendezvoussession.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRendezvousApplication> for ::windows::core::IUnknown {
    fn from(value: IRendezvousApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRendezvousApplication> for ::windows::core::IUnknown {
    fn from(value: &IRendezvousApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRendezvousApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRendezvousApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRendezvousApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRendezvousApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRendezvousApplication {}
impl ::core::fmt::Debug for IRendezvousApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRendezvousApplication").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRendezvousApplication {
    type Vtable = IRendezvousApplication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f4d070b_a275_49fb_b10d_8ec26387b50d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousApplication_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetRendezvousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
pub struct IRendezvousSession(::windows::core::IUnknown);
impl IRendezvousSession {
    #[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
    pub unsafe fn State(&self) -> ::windows::core::Result<RENDEZVOUS_SESSION_STATE> {
        let mut result__: RENDEZVOUS_SESSION_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).State)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<RENDEZVOUS_SESSION_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RemoteUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Flags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendContextData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendContextData)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Terminate<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hr: ::windows::core::HRESULT, bstrappdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), bstrappdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRendezvousSession> for ::windows::core::IUnknown {
    fn from(value: IRendezvousSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRendezvousSession> for ::windows::core::IUnknown {
    fn from(value: &IRendezvousSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRendezvousSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRendezvousSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRendezvousSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRendezvousSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRendezvousSession {}
impl ::core::fmt::Debug for IRendezvousSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRendezvousSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRendezvousSession {
    type Vtable = IRendezvousSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ba4b1dd_8b0c_48b7_9e7c_2f25857c8df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousSession_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteUser: usize,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendContextData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendContextData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Terminate: usize,
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RENDEZVOUS_SESSION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_NONE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_INVITEE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_ORIGINAL_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_REMOTE_LEGACYSESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_REMOTE_WIN7SESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(16i32);
impl ::core::marker::Copy for RENDEZVOUS_SESSION_FLAGS {}
impl ::core::clone::Clone for RENDEZVOUS_SESSION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RENDEZVOUS_SESSION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RENDEZVOUS_SESSION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RENDEZVOUS_SESSION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RENDEZVOUS_SESSION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_UNKNOWN: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_READY: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_INVITATION: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_ACCEPTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_CONNECTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_CANCELLED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_DECLINED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_TERMINATED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(7i32);
impl ::core::marker::Copy for RENDEZVOUS_SESSION_STATE {}
impl ::core::clone::Clone for RENDEZVOUS_SESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RENDEZVOUS_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RENDEZVOUS_SESSION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RENDEZVOUS_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_STATE").field(&self.0).finish()
    }
}
pub const RendezvousApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b7e019a_b5de_47fa_8966_9082f82fb192);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
