#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DRendezvousSessionEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DRendezvousSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(DRendezvousSessionEvents, ::windows::core::IUnknown, super::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DRendezvousSessionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for DRendezvousSessionEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa19cf8_64c4_4f53_ae60_635b3806eca6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DRendezvousSessionEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
pub struct IRendezvousApplication(::windows::core::IUnknown);
impl IRendezvousApplication {
    pub unsafe fn SetRendezvousSession<P0>(&self, prendezvoussession: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetRendezvousSession)(::windows::core::Interface::as_raw(self), prendezvoussession.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRendezvousApplication, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IRendezvousApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRendezvousApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f4d070b_a275_49fb_b10d_8ec26387b50d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousApplication_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetRendezvousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
pub struct IRendezvousSession(::windows::core::IUnknown);
impl IRendezvousSession {
    pub unsafe fn State(&self) -> ::windows::core::Result<RENDEZVOUS_SESSION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RENDEZVOUS_SESSION_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoteUser(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).RemoteUser)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Flags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendContextData<P0>(&self, bstrdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SendContextData)(::windows::core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Terminate<P0>(&self, hr: ::windows::core::HRESULT, bstrappdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self), hr, bstrappdata.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRendezvousSession, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IRendezvousSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRendezvousSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ba4b1dd_8b0c_48b7_9e7c_2f25857c8df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousSession_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::core::HRESULT,
    pub RemoteUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT,
    pub SendContextData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, bstrappdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RendezvousApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b7e019a_b5de_47fa_8966_9082f82fb192);
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for RENDEZVOUS_SESSION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RENDEZVOUS_SESSION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for RENDEZVOUS_SESSION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RENDEZVOUS_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
