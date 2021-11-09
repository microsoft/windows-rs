#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DRendezvousSessionEvents(pub ::windows::runtime::IUnknown);
impl DRendezvousSessionEvents {}
unsafe impl ::windows::runtime::Interface for DRendezvousSessionEvents {
    type Vtable = DRendezvousSessionEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1067556088, 25796, 20307, [174, 96, 99, 91, 56, 6, 236, 166]);
}
impl ::core::convert::From<DRendezvousSessionEvents> for ::windows::runtime::IUnknown {
    fn from(value: DRendezvousSessionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DRendezvousSessionEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DRendezvousSessionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IDispatch> for &DRendezvousSessionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DRendezvousSessionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRendezvousApplication(pub ::windows::runtime::IUnknown);
impl IRendezvousApplication {
    #[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
    pub unsafe fn SetRendezvousSession<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, prendezvoussession: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), prendezvoussession.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRendezvousApplication {
    type Vtable = IRendezvousApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1330448139, 41589, 18939, [177, 13, 142, 194, 99, 135, 181, 13]);
}
impl ::core::convert::From<IRendezvousApplication> for ::windows::runtime::IUnknown {
    fn from(value: IRendezvousApplication) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRendezvousApplication> for ::windows::runtime::IUnknown {
    fn from(value: &IRendezvousApplication) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRendezvousApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRendezvousApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prendezvoussession: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRendezvousSession(pub ::windows::runtime::IUnknown);
impl IRendezvousSession {
    #[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
    pub unsafe fn State(&self) -> ::windows::runtime::Result<RENDEZVOUS_SESSION_STATE> {
        let mut result__: <RENDEZVOUS_SESSION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RENDEZVOUS_SESSION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteAssistance`, `Win32_Foundation`*"]
    pub unsafe fn RemoteUser(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
    pub unsafe fn Flags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteAssistance`, `Win32_Foundation`*"]
    pub unsafe fn SendContextData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_RemoteAssistance`, `Win32_Foundation`*"]
    pub unsafe fn Terminate<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hr: ::windows::runtime::HRESULT, bstrappdata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr), bstrappdata.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRendezvousSession {
    type Vtable = IRendezvousSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2611261917, 35596, 18615, [158, 124, 47, 37, 133, 124, 141, 245]);
}
impl ::core::convert::From<IRendezvousSession> for ::windows::runtime::IUnknown {
    fn from(value: IRendezvousSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRendezvousSession> for ::windows::runtime::IUnknown {
    fn from(value: &IRendezvousSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRendezvousSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRendezvousSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RENDEZVOUS_SESSION_FLAGS(pub i32);
pub const RSF_NONE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(0i32);
pub const RSF_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(1i32);
pub const RSF_INVITEE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(2i32);
pub const RSF_ORIGINAL_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(4i32);
pub const RSF_REMOTE_LEGACYSESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(8i32);
pub const RSF_REMOTE_WIN7SESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(16i32);
impl ::core::convert::From<i32> for RENDEZVOUS_SESSION_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RENDEZVOUS_SESSION_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RENDEZVOUS_SESSION_STATE(pub i32);
pub const RSS_UNKNOWN: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(0i32);
pub const RSS_READY: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(1i32);
pub const RSS_INVITATION: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(2i32);
pub const RSS_ACCEPTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(3i32);
pub const RSS_CONNECTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(4i32);
pub const RSS_CANCELLED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(5i32);
pub const RSS_DECLINED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(6i32);
pub const RSS_TERMINATED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(7i32);
impl ::core::convert::From<i32> for RENDEZVOUS_SESSION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RENDEZVOUS_SESSION_STATE {
    type Abi = Self;
}
pub const RendezvousApplication: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(192807322, 46558, 18426, [137, 102, 144, 130, 248, 47, 177, 146]);
