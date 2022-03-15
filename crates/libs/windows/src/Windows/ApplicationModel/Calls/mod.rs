#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct CallAnswerEventArgs(::windows::core::IUnknown);
impl CallAnswerEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn AcceptedMedia(&self) -> ::windows::core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcceptedMedia)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallMedia>(result__)
        }
    }
}
impl ::core::clone::Clone for CallAnswerEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CallAnswerEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallAnswerEventArgs {}
impl ::core::fmt::Debug for CallAnswerEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallAnswerEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CallAnswerEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallAnswerEventArgs;{fd789617-2dd7-4c8c-b2bd-95d17a5bb733})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICallAnswerEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CallAnswerEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallAnswerEventArgs";
}
impl ::core::convert::From<CallAnswerEventArgs> for ::windows::core::IUnknown {
    fn from(value: CallAnswerEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallAnswerEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CallAnswerEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CallAnswerEventArgs> for ::windows::core::IInspectable {
    fn from(value: CallAnswerEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallAnswerEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CallAnswerEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CallAnswerEventArgs {}
unsafe impl ::core::marker::Sync for CallAnswerEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct CallRejectEventArgs(::windows::core::IUnknown);
impl CallRejectEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RejectReason(&self) -> ::windows::core::Result<VoipPhoneCallRejectReason> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallRejectReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RejectReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallRejectReason>(result__)
        }
    }
}
impl ::core::clone::Clone for CallRejectEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CallRejectEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallRejectEventArgs {}
impl ::core::fmt::Debug for CallRejectEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallRejectEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CallRejectEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallRejectEventArgs;{da47fad7-13d4-4d92-a1c2-b77811ee37ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICallRejectEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CallRejectEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallRejectEventArgs";
}
impl ::core::convert::From<CallRejectEventArgs> for ::windows::core::IUnknown {
    fn from(value: CallRejectEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallRejectEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CallRejectEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CallRejectEventArgs> for ::windows::core::IInspectable {
    fn from(value: CallRejectEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallRejectEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CallRejectEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CallRejectEventArgs {}
unsafe impl ::core::marker::Sync for CallRejectEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct CallStateChangeEventArgs(::windows::core::IUnknown);
impl CallStateChangeEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn State(&self) -> ::windows::core::Result<VoipPhoneCallState> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallState>(result__)
        }
    }
}
impl ::core::clone::Clone for CallStateChangeEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CallStateChangeEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallStateChangeEventArgs {}
impl ::core::fmt::Debug for CallStateChangeEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallStateChangeEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CallStateChangeEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallStateChangeEventArgs;{eab2349e-66f5-47f9-9fb5-459c5198c720})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICallStateChangeEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CallStateChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallStateChangeEventArgs";
}
impl ::core::convert::From<CallStateChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: CallStateChangeEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallStateChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CallStateChangeEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CallStateChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: CallStateChangeEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallStateChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CallStateChangeEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CallStateChangeEventArgs {}
unsafe impl ::core::marker::Sync for CallStateChangeEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: Self = Self(0i32);
    pub const Burst: Self = Self(1i32);
}
impl ::core::marker::Copy for CellularDtmfMode {}
impl ::core::clone::Clone for CellularDtmfMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CellularDtmfMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CellularDtmfMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CellularDtmfMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CellularDtmfMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CellularDtmfMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.CellularDtmfMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DtmfKey(pub i32);
impl DtmfKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
}
impl ::core::marker::Copy for DtmfKey {}
impl ::core::clone::Clone for DtmfKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DtmfKey {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DtmfKey {
    type Abi = Self;
}
impl ::core::fmt::Debug for DtmfKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtmfKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DtmfKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfKey;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: Self = Self(0i32);
    pub const DoNotPlay: Self = Self(1i32);
}
impl ::core::marker::Copy for DtmfToneAudioPlayback {}
impl ::core::clone::Clone for DtmfToneAudioPlayback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DtmfToneAudioPlayback {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DtmfToneAudioPlayback {
    type Abi = Self;
}
impl ::core::fmt::Debug for DtmfToneAudioPlayback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtmfToneAudioPlayback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DtmfToneAudioPlayback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfToneAudioPlayback;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallAnswerEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd789617_2dd7_4c8c_b2bd_95d17a5bb733);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallAnswerEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AcceptedMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallMedia) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallRejectEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda47fad7_13d4_4d92_a1c2_b77811ee37ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallRejectEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RejectReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallRejectReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallStateChangeEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeab2349e_66f5_47f9_9fb5_459c5198c720);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallStateChangeEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenCallEndCallDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd7ed0d_98ed_4041_9632_50ff812b773f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndCallDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenCallEndRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8190a363_6f27_46e9_aeb6_c0ae83e47dc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenCallUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILockScreenCallUI {
    type Vtable = ILockScreenCallUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc596fd8d_73c9_4a14_b021_ec1c50a3b727);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallUI_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Dismiss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CallTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCallTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMuteChangeEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8585e159_0c41_432c_814d_c5f1fdf530be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Muted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCall(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCall {
    type Vtable = IPhoneCall_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc14ed0f8_c17d_59d2_9628_66e545b6cd21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCall_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AudioDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsMutedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsMutedChanged: usize,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallStatus) -> ::windows::core::HRESULT,
    pub AudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallAudioDevice) -> ::windows::core::HRESULT,
    pub GetPhoneCallInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPhoneCallInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPhoneCallInfoAsync: usize,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndAsync: usize,
    pub SendDtmfKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendDtmfKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendDtmfKeyAsync: usize,
    pub AcceptIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AcceptIncomingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptIncomingAsync: usize,
    pub Hold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HoldAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldAsync: usize,
    pub ResumeFromHold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResumeFromHoldAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeFromHoldAsync: usize,
    pub Mute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MuteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteAsync: usize,
    pub Unmute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnmuteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnmuteAsync: usize,
    pub RejectIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RejectIncomingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RejectIncomingAsync: usize,
    pub ChangeAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioDevice, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ChangeAudioDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioDevice, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeAudioDeviceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallBlockingStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallBlockingStatics {
    type Vtable = IPhoneCallBlockingStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19646f84_2b79_26f1_a46f_694be043f313);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockingStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BlockUnknownNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBlockUnknownNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub BlockPrivateNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBlockPrivateNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetCallBlockingListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumberlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetCallBlockingListAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntry(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfab0e129_32a4_4b85_83d1_f90d8c23a857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntry_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsCallerIdBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCallerIdBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsEmergency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEmergency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsMissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsMissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRinging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRinging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsVoicemail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsVoicemail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Media: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryMedia) -> ::windows::core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryMedia) -> ::windows::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceIdKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistorySourceIdKind) -> ::windows::core::HRESULT,
    pub SetSourceIdKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistorySourceIdKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddress(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f159da_3955_4042_84e6_66eebf82e67f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddress_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RawAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRawAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RawAddressKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::HRESULT,
    pub SetRawAddressKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddressFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryAddressFactory {
    type Vtable = IPhoneCallHistoryEntryAddressFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb0fadba_c7f0_4bb6_9f6b_ba5d73209aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddressFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rawaddresskind: PhoneCallHistoryEntryRawAddressKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c5fe15c_8bed_40ca_b06e_c4ca8eae5c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryQueryOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesiredMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::HRESULT,
    pub SetDesiredMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ece4be_8d86_479f_8404_a9846920fee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd925c523_f55f_4353_9db4_0205a5265a55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerForUser_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryManagerStatics {
    type Vtable = IPhoneCallHistoryManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5a6da39_b31f_4f45_ac8e_1b08893c1b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryManagerStatics2 {
    type Vtable = IPhoneCallHistoryManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefd474f0_a2db_4188_9e92_bc3cfa6813cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f907db8_b40e_422b_8545_cb1910a61c52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryStore_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentryid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetEntryAsync: usize,
    pub GetEntryReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetEntryReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveEntryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteEntryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteEntriesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteEntriesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkEntryAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkEntryAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkEntriesAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkEntriesAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetUnseenCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUnseenCountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkAllAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAllAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSourcesUnseenCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSourcesUnseenCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkSourcesAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkSourcesAsSeenAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallInfo {
    type Vtable = IPhoneCallInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22b42577_3e4d_5dc6_89c2_469fe5ffc189);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IsHoldSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallDirection) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallManagerStatics {
    type Vtable = IPhoneCallManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60edac78_78a6_4872_a3ef_98325ec8b843);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ShowPhoneCallUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallManagerStatics2 {
    type Vtable = IPhoneCallManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7e3c8bc_2370_431c_98fd_43be5f03086d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CallStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CallStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCallStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCallStateChanged: usize,
    pub IsCallActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCallIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShowPhoneCallSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallStatics {
    type Vtable = IPhoneCallStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2218eeab_f60b_53e7_ba13_5aeafbc22957);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallStore {
    type Vtable = IPhoneCallStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f610748_18a6_4173_86d1_28be9dc62dba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStore_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub IsEmergencyPhoneNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsEmergencyPhoneNumberAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultLineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultLineAsync: usize,
    pub RequestLineWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02382786_b16a_4fdb_be3b_c4240e13ad0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsVideoCallingCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallVideoCapabilitiesManagerStatics {
    type Vtable = IPhoneCallVideoCapabilitiesManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3c64b56_f00b_4a1c_a0c6_ee1910749ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallsResult {
    type Vtable = IPhoneCallsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bfad365_57cf_57dd_986d_b057c91eac33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallsResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllActivePhoneCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllActivePhoneCalls: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneDialOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneDialOptions {
    type Vtable = IPhoneDialOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb639c4b8_f06f_36cb_a863_823742b5f2d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneDialOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPhone: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContactPhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContactPhone: usize,
    pub Media: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallMedia) -> ::windows::core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallMedia) -> ::windows::core::HRESULT,
    pub AudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneAudioRoutingEndpoint) -> ::windows::core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneAudioRoutingEndpoint) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLine(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLine {
    type Vtable = IPhoneLine_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27c66f30_6a69_34ca_a2ba_65302530c311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LineChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineChanged: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub NetworkState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneNetworkState) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Voicemail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NetworkName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CellularDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Transport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineTransport) -> ::windows::core::HRESULT,
    pub CanDial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportsTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub VideoCallingCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LineConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsImmediateDialNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsImmediateDialNumberAsync: usize,
    pub Dial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DialWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLine2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLine2 {
    type Vtable = IPhoneLine2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0167f56a_5344_5d64_8af3_a31a950e916a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnableTextReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TransportDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLine3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLine3 {
    type Vtable = IPhoneLine3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2e33cf7_2406_57f3_826a_e5a5f40d6fb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DialWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DialWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialWithResultAsync: usize,
    pub GetAllActivePhoneCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAllActivePhoneCallsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllActivePhoneCallsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineCellularDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x192601d5_147c_4769_b673_98a5ec8426cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineCellularDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SimState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneSimState) -> ::windows::core::HRESULT,
    pub SimSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsModemOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RegistrationRejectCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetNetworkOperatorDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: PhoneLineNetworkOperatorDisplayTextLocation, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe265862_f64f_4312_b2a8_4e257721aa95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsVideoCallingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDialResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe825a30a_5c7f_546f_b918_3ad2fe70fb34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDialResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DialCallStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    pub DialedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineStatics {
    type Vtable = IPhoneLineStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf38b5f23_ceb0_404f_bcf2_ba9f697d8adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineTransportDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8f889_cffa_59f4_97e4_74705b7dc490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Transport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineTransport) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    pub RegisterApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub RegisterAppForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    RegisterAppForUser: usize,
    pub UnregisterApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub UnregisterAppForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    UnregisterAppForUser: usize,
    pub IsRegistered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineTransportDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineTransportDevice2 {
    type Vtable = IPhoneLineTransportDevice2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64c885f2_ecf4_5761_8c04_3c248ce61690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AudioRoutingStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TransportDeviceAudioRoutingStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioRoutingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioRoutingStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioRoutingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioRoutingStatusChanged: usize,
    pub InBandRingingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InBandRingingEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InBandRingingEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInBandRingingEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInBandRingingEnabledChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineTransportDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineTransportDeviceStatics {
    type Vtable = IPhoneLineTransportDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f3121ac_d609_51a1_96f3_fb00d1819252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorForPhoneLineTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transport: PhoneLineTransport, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a45cd0a_6323_44e0_a6f6_9f21f64dc90a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LineAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineAdded: usize,
    #[cfg(feature = "Foundation")]
    pub LineRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub LineUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineWatcherStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineWatcherEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd07c753e_9e12_4a37_82b7_ad535dad6a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcherEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneVoicemail(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneVoicemail {
    type Vtable = IPhoneVoicemail_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9ce77f6_6e9f_3a8b_b727_6e0cf6998224);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneVoicemail_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneVoicemailType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DialVoicemailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialVoicemailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f118bcf_e8ef_4434_9c5f_a8d893fafe79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReserveCallResourcesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskentrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReserveCallResourcesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MuteStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mutechangehandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMuteStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMuteStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestNewIncomingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactimage: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brandingimage: ::windows::core::RawPtr, calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ringtone: ::windows::core::RawPtr, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewIncomingCall: usize,
    pub RequestNewOutgoingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyUnmuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestOutgoingUpgradeToVideoCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows::core::GUID, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestIncomingUpgradeToVideoCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactimage: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brandingimage: ::windows::core::RawPtr, calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ringtone: ::windows::core::RawPtr, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestIncomingUpgradeToVideoCall: usize,
    pub TerminateCellularCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CancelUpgrade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator2 {
    type Vtable = IVoipCallCoordinator2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb4a9f3_c704_4234_89ce_e88cc0d28fbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetupNewAcceptedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator3 {
    type Vtable = IVoipCallCoordinator3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338d0cbf_9b55_4021_87ca_e64b9bd666c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestNewAppInitiatedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestNewIncomingCallWithContactRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactimage: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brandingimage: ::windows::core::RawPtr, calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ringtone: ::windows::core::RawPtr, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, contactremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewIncomingCallWithContactRemoteId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator4 {
    type Vtable = IVoipCallCoordinator4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83737239_9311_468f_bb49_47e0dfb5d93e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReserveOneProcessCallResourcesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReserveOneProcessCallResourcesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipCallCoordinatorStatics {
    type Vtable = IVoipCallCoordinatorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5d1f2b_e04a_4d10_b31a_a55c922cc2fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinatorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipPhoneCall(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipPhoneCall {
    type Vtable = IVoipPhoneCall_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cf1f19a_7794_4a5a_8c68_ae87947a6990);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub EndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HoldRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResumeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResumeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accepthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RejectRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rejecthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RejectRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRejectRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRejectRequested: usize,
    pub NotifyCallHeld: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyCallActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyCallEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    pub CallMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallMedia) -> ::windows::core::HRESULT,
    pub SetCallMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VoipPhoneCallMedia) -> ::windows::core::HRESULT,
    pub NotifyCallReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipPhoneCall2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipPhoneCall2 {
    type Vtable = IVoipPhoneCall2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x741b46e1_245f_41f3_9399_3141d25b52e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryShowAppUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipPhoneCall3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoipPhoneCall3 {
    type Vtable = IVoipPhoneCall3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d891522_e258_4aa9_907a_1aa413c25523);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NotifyCallAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, media: VoipPhoneCallMedia) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct LockScreenCallEndCallDeferral(::windows::core::IUnknown);
impl LockScreenCallEndCallDeferral {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for LockScreenCallEndCallDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenCallEndCallDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallEndCallDeferral {}
impl ::core::fmt::Debug for LockScreenCallEndCallDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallEndCallDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallEndCallDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral;{2dd7ed0d-98ed-4041-9632-50ff812b773f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenCallEndCallDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenCallEndCallDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral";
}
impl ::core::convert::From<LockScreenCallEndCallDeferral> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallEndCallDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallEndCallDeferral> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallEndCallDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenCallEndCallDeferral> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallEndCallDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallEndCallDeferral> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallEndCallDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenCallEndCallDeferral {}
unsafe impl ::core::marker::Sync for LockScreenCallEndCallDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct LockScreenCallEndRequestedEventArgs(::windows::core::IUnknown);
impl LockScreenCallEndRequestedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<LockScreenCallEndCallDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LockScreenCallEndCallDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenCallEndRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenCallEndRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallEndRequestedEventArgs {}
impl ::core::fmt::Debug for LockScreenCallEndRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallEndRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallEndRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs;{8190a363-6f27-46e9-aeb6-c0ae83e47dc7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenCallEndRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenCallEndRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs";
}
impl ::core::convert::From<LockScreenCallEndRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallEndRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallEndRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallEndRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenCallEndRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallEndRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallEndRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallEndRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenCallEndRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallEndRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct LockScreenCallUI(::windows::core::IUnknown);
impl LockScreenCallUI {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Dismiss(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Dismiss)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEndRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn CallTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CallTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetCallTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCallTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LockScreenCallUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenCallUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallUI {}
impl ::core::fmt::Debug for LockScreenCallUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallUI;{c596fd8d-73c9-4a14-b021-ec1c50a3b727})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LockScreenCallUI {
    type Vtable = ILockScreenCallUI_Vtbl;
    const IID: ::windows::core::GUID = <ILockScreenCallUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenCallUI {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallUI";
}
impl ::core::convert::From<LockScreenCallUI> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallUI> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenCallUI> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallUI> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenCallUI {}
unsafe impl ::core::marker::Sync for LockScreenCallUI {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct MuteChangeEventArgs(::windows::core::IUnknown);
impl MuteChangeEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Muted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Muted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MuteChangeEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MuteChangeEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MuteChangeEventArgs {}
impl ::core::fmt::Debug for MuteChangeEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MuteChangeEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MuteChangeEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.MuteChangeEventArgs;{8585e159-0c41-432c-814d-c5f1fdf530be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMuteChangeEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MuteChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.MuteChangeEventArgs";
}
impl ::core::convert::From<MuteChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: MuteChangeEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MuteChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MuteChangeEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MuteChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: MuteChangeEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MuteChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MuteChangeEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MuteChangeEventArgs {}
unsafe impl ::core::marker::Sync for MuteChangeEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Bluetooth: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneAudioRoutingEndpoint {}
impl ::core::clone::Clone for PhoneAudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneAudioRoutingEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneAudioRoutingEndpoint {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneAudioRoutingEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneAudioRoutingEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneAudioRoutingEndpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneAudioRoutingEndpoint;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCall(::windows::core::IUnknown);
impl PhoneCall {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioDeviceChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioDeviceChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsMutedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMutedChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsMutedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsMutedChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn CallId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CallId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMuted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PhoneCallStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn AudioDevice(&self) -> ::windows::core::Result<PhoneCallAudioDevice> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallAudioDevice = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallAudioDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetPhoneCallInfo(&self) -> ::windows::core::Result<PhoneCallInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPhoneCallInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPhoneCallInfoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPhoneCallInfoAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn End(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).End)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SendDtmfKey(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendDtmfKey)(::core::mem::transmute_copy(this), key, dtmftoneaudioplayback, &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendDtmfKeyAsync(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendDtmfKeyAsync)(::core::mem::transmute_copy(this), key, dtmftoneaudioplayback, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn AcceptIncoming(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcceptIncoming)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceptIncomingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcceptIncomingAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Hold(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Hold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HoldAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn ResumeFromHold(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResumeFromHold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeFromHoldAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResumeFromHoldAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Mute(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MuteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Unmute(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unmute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnmuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnmuteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RejectIncoming(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RejectIncoming)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RejectIncomingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RejectIncomingAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn ChangeAudioDevice(&self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeAudioDevice)(::core::mem::transmute_copy(this), endpoint, &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChangeAudioDeviceAsync(&self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeAudioDeviceAsync)(::core::mem::transmute_copy(this), endpoint, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(callid: Param0) -> ::windows::core::Result<PhoneCall> {
        Self::IPhoneCallStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFromId)(::core::mem::transmute_copy(this), callid.into_param().abi(), &mut result__).from_abi::<PhoneCall>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallStatics<R, F: FnOnce(&IPhoneCallStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCall, IPhoneCallStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhoneCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCall {}
impl ::core::fmt::Debug for PhoneCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCall").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCall {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCall;{c14ed0f8-c17d-59d2-9628-66e545b6cd21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCall {
    type Vtable = IPhoneCall_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCall as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCall";
}
impl ::core::convert::From<PhoneCall> for ::windows::core::IUnknown {
    fn from(value: PhoneCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCall> for ::windows::core::IUnknown {
    fn from(value: &PhoneCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCall> for ::windows::core::IInspectable {
    fn from(value: PhoneCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCall> for ::windows::core::IInspectable {
    fn from(value: &PhoneCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCall {}
unsafe impl ::core::marker::Sync for PhoneCall {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: Self = Self(0i32);
    pub const LocalDevice: Self = Self(1i32);
    pub const RemoteDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallAudioDevice {}
impl ::core::clone::Clone for PhoneCallAudioDevice {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallAudioDevice {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallAudioDevice {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallAudioDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallAudioDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallAudioDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallAudioDevice;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallBlocking {}
impl PhoneCallBlocking {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn BlockUnknownNumbers() -> ::windows::core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BlockUnknownNumbers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetBlockUnknownNumbers(value: bool) -> ::windows::core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetBlockUnknownNumbers)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn BlockPrivateNumbers() -> ::windows::core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BlockPrivateNumbers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetBlockPrivateNumbers(value: bool) -> ::windows::core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetBlockPrivateNumbers)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetCallBlockingListAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(phonenumberlist: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetCallBlockingListAsync)(::core::mem::transmute_copy(this), phonenumberlist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallBlockingStatics<R, F: FnOnce(&IPhoneCallBlockingStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallBlocking, IPhoneCallBlockingStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallBlocking";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Outgoing: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallDirection {}
impl ::core::clone::Clone for PhoneCallDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntry(::windows::core::IUnknown);
impl PhoneCallHistoryEntry {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntry, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Address(&self) -> ::windows::core::Result<PhoneCallHistoryEntryAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryAddress>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntryAddress>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsCallerIdBlocked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCallerIdBlocked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsCallerIdBlocked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCallerIdBlocked)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsEmergency(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEmergency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsEmergency(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEmergency)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsIncoming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsIncoming)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsIncoming(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsIncoming)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsMissed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMissed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsMissed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsMissed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsRinging(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRinging)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsRinging(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRinging)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsSeen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSeen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSeen)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsSuppressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuppressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsSuppressed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSuppressed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsVoicemail(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVoicemail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetIsVoicemail(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsVoicemail)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Media(&self) -> ::windows::core::Result<PhoneCallHistoryEntryMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Media)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryMedia>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetMedia(&self, value: PhoneCallHistoryEntryMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMedia)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<PhoneCallHistoryEntryOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryOtherAppReadAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OtherAppReadAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetOtherAppReadAccess(&self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppReadAccess)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SourceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetSourceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSourceId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SourceIdKind(&self) -> ::windows::core::Result<PhoneCallHistorySourceIdKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistorySourceIdKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceIdKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistorySourceIdKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetSourceIdKind(&self, value: PhoneCallHistorySourceIdKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSourceIdKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PhoneCallHistoryEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntry {}
impl ::core::fmt::Debug for PhoneCallHistoryEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntry;{fab0e129-32a4-4b85-83d1-f90d8c23a857})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallHistoryEntry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntry";
}
impl ::core::convert::From<PhoneCallHistoryEntry> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntry> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallHistoryEntry> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntry> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntry {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntry {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryAddress(::windows::core::IUnknown);
impl PhoneCallHistoryEntryAddress {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntryAddress, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetContactId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContactId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RawAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetRawAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRawAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RawAddressKind(&self) -> ::windows::core::Result<PhoneCallHistoryEntryRawAddressKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryRawAddressKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawAddressKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryRawAddressKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetRawAddressKind(&self, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRawAddressKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(rawaddress: Param0, rawaddresskind: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<PhoneCallHistoryEntryAddress> {
        Self::IPhoneCallHistoryEntryAddressFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), rawaddress.into_param().abi(), rawaddresskind, &mut result__).from_abi::<PhoneCallHistoryEntryAddress>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryEntryAddressFactory<R, F: FnOnce(&IPhoneCallHistoryEntryAddressFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntryAddress, IPhoneCallHistoryEntryAddressFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhoneCallHistoryEntryAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntryAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntryAddress {}
impl ::core::fmt::Debug for PhoneCallHistoryEntryAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress;{30f159da-3955-4042-84e6-66eebf82e67f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallHistoryEntryAddress as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntryAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress";
}
impl ::core::convert::From<PhoneCallHistoryEntryAddress> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntryAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryAddress> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntryAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallHistoryEntryAddress> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntryAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryAddress> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntryAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryAddress {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryAddress {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryMedia {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryMedia {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryMedia").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryMedia;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryOtherAppReadAccess {}
impl ::core::clone::Clone for PhoneCallHistoryEntryOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryQueryDesiredMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryQueryDesiredMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryQueryDesiredMedia {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryQueryDesiredMedia {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryQueryDesiredMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryQueryDesiredMedia").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryQueryDesiredMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryDesiredMedia;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryOptions(::windows::core::IUnknown);
impl PhoneCallHistoryEntryQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntryQueryOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DesiredMedia(&self) -> ::windows::core::Result<PhoneCallHistoryEntryQueryDesiredMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryQueryDesiredMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredMedia)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryQueryDesiredMedia>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetDesiredMedia(&self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredMedia)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceIds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallHistoryEntryQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntryQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntryQueryOptions {}
impl ::core::fmt::Debug for PhoneCallHistoryEntryQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryQueryOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions;{9c5fe15c-8bed-40ca-b06e-c4ca8eae5c87})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallHistoryEntryQueryOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntryQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions";
}
impl ::core::convert::From<PhoneCallHistoryEntryQueryOptions> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntryQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryQueryOptions> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntryQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallHistoryEntryQueryOptions> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntryQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryQueryOptions> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntryQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryQueryOptions {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryRawAddressKind {}
impl ::core::clone::Clone for PhoneCallHistoryEntryRawAddressKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryRawAddressKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryRawAddressKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryRawAddressKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryRawAddressKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryRawAddressKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryRawAddressKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryReader(::windows::core::IUnknown);
impl PhoneCallHistoryEntryReader {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadBatchAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallHistoryEntryReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntryReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntryReader {}
impl ::core::fmt::Debug for PhoneCallHistoryEntryReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader;{61ece4be-8d86-479f-8404-a9846920fee6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallHistoryEntryReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntryReader {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader";
}
impl ::core::convert::From<PhoneCallHistoryEntryReader> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntryReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryReader> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntryReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallHistoryEntryReader> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntryReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryReader> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntryReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryReader {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryReader {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallHistoryManager {}
impl PhoneCallHistoryManager {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        Self::IPhoneCallHistoryManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<PhoneCallHistoryManagerForUser> {
        Self::IPhoneCallHistoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<PhoneCallHistoryManagerForUser>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryManagerStatics<R, F: FnOnce(&IPhoneCallHistoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryManagerStatics2<R, F: FnOnce(&IPhoneCallHistoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallHistoryManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManager";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryManagerForUser(::windows::core::IUnknown);
impl PhoneCallHistoryManagerForUser {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallHistoryManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryManagerForUser {}
impl ::core::fmt::Debug for PhoneCallHistoryManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser;{d925c523-f55f-4353-9db4-0205a5265a55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallHistoryManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallHistoryManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser";
}
impl ::core::convert::From<PhoneCallHistoryManagerForUser> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallHistoryManagerForUser> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryManagerForUser {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryManagerForUser {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: Self = Self(0i32);
    pub const PackageFamilyName: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistorySourceIdKind {}
impl ::core::clone::Clone for PhoneCallHistorySourceIdKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistorySourceIdKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistorySourceIdKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallHistorySourceIdKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistorySourceIdKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistorySourceIdKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistorySourceIdKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryStore(::windows::core::IUnknown);
impl PhoneCallHistoryStore {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, callhistoryentryid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEntryAsync)(::core::mem::transmute_copy(this), callhistoryentryid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetEntryReader(&self) -> ::windows::core::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEntryReader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryReader>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetEntryReaderWithOptions<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntryQueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEntryReaderWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<PhoneCallHistoryEntryReader>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveEntryAsync)(::core::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteEntryAsync)(::core::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeleteEntriesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>>(&self, callhistoryentries: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteEntriesAsync)(::core::mem::transmute_copy(this), callhistoryentries.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkEntryAsSeenAsync<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarkEntryAsSeenAsync)(::core::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkEntriesAsSeenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>>(&self, callhistoryentries: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarkEntriesAsSeenAsync)(::core::mem::transmute_copy(this), callhistoryentries.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetUnseenCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUnseenCountAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAllAsSeenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarkAllAsSeenAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSourcesUnseenCountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, sourceids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSourcesUnseenCountAsync)(::core::mem::transmute_copy(this), sourceids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkSourcesAsSeenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, sourceids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarkSourcesAsSeenAsync)(::core::mem::transmute_copy(this), sourceids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallHistoryStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryStore {}
impl ::core::fmt::Debug for PhoneCallHistoryStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryStore;{2f907db8-b40e-422b-8545-cb1910a61c52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallHistoryStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallHistoryStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryStore";
}
impl ::core::convert::From<PhoneCallHistoryStore> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryStore> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallHistoryStore> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallHistoryStore> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryStore {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryStore {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: Self = Self(0i32);
    pub const AllEntriesLimitedReadWrite: Self = Self(1i32);
    pub const AllEntriesReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallHistoryStoreAccessType {}
impl ::core::clone::Clone for PhoneCallHistoryStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallHistoryStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallInfo(::windows::core::IUnknown);
impl PhoneCallInfo {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsHoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHoldSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn CallDirection(&self) -> ::windows::core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CallDirection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallDirection>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallInfo {}
impl ::core::fmt::Debug for PhoneCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallInfo;{22b42577-3e4d-5dc6-89c2-469fe5ffc189})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallInfo {
    type Vtable = IPhoneCallInfo_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallInfo";
}
impl ::core::convert::From<PhoneCallInfo> for ::windows::core::IUnknown {
    fn from(value: PhoneCallInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallInfo> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallInfo> for ::windows::core::IInspectable {
    fn from(value: PhoneCallInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallInfo> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallInfo {}
unsafe impl ::core::marker::Sync for PhoneCallInfo {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallManager {}
impl PhoneCallManager {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn ShowPhoneCallUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phonenumber: Param0, displayname: Param1) -> ::windows::core::Result<()> {
        Self::IPhoneCallManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowPhoneCallUI)(::core::mem::transmute_copy(this), phonenumber.into_param().abi(), displayname.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CallStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CallStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCallStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCallStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsCallActive() -> ::windows::core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCallActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsCallIncoming() -> ::windows::core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCallIncoming)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn ShowPhoneCallSettingsUI() -> ::windows::core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).ShowPhoneCallSettingsUI)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallStore>> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallStore>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallManagerStatics<R, F: FnOnce(&IPhoneCallManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPhoneCallManagerStatics2<R, F: FnOnce(&IPhoneCallManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallManager";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: Self = Self(0i32);
    pub const AudioAndVideo: Self = Self(1i32);
    pub const AudioAndRealTimeText: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallMedia {}
impl ::core::clone::Clone for PhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallMedia {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallMedia {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallMedia").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallMedia;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneCallOperationStatus {}
impl ::core::clone::Clone for PhoneCallOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallStatus(pub i32);
impl PhoneCallStatus {
    pub const Lost: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Dialing: Self = Self(2i32);
    pub const Talking: Self = Self(3i32);
    pub const Held: Self = Self(4i32);
    pub const Ended: Self = Self(5i32);
}
impl ::core::marker::Copy for PhoneCallStatus {}
impl ::core::clone::Clone for PhoneCallStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallStore(::windows::core::IUnknown);
impl PhoneCallStore {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsEmergencyPhoneNumberAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEmergencyPhoneNumberAsync)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultLineAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::GUID>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultLineAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::GUID>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RequestLineWatcher(&self) -> ::windows::core::Result<PhoneLineWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestLineWatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineWatcher>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallStore {}
impl ::core::fmt::Debug for PhoneCallStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallStore;{5f610748-18a6-4173-86d1-28be9dc62dba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallStore {
    type Vtable = IPhoneCallStore_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallStore";
}
impl ::core::convert::From<PhoneCallStore> for ::windows::core::IUnknown {
    fn from(value: PhoneCallStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallStore> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallStore> for ::windows::core::IInspectable {
    fn from(value: PhoneCallStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallStore> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallStore {}
unsafe impl ::core::marker::Sync for PhoneCallStore {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallVideoCapabilities(::windows::core::IUnknown);
impl PhoneCallVideoCapabilities {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsVideoCallingCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVideoCallingCapable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallVideoCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallVideoCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallVideoCapabilities {}
impl ::core::fmt::Debug for PhoneCallVideoCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallVideoCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallVideoCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities;{02382786-b16a-4fdb-be3b-c4240e13ad0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallVideoCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallVideoCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities";
}
impl ::core::convert::From<PhoneCallVideoCapabilities> for ::windows::core::IUnknown {
    fn from(value: PhoneCallVideoCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallVideoCapabilities> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallVideoCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallVideoCapabilities> for ::windows::core::IInspectable {
    fn from(value: PhoneCallVideoCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallVideoCapabilities> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallVideoCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallVideoCapabilities {}
unsafe impl ::core::marker::Sync for PhoneCallVideoCapabilities {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallVideoCapabilitiesManager {}
impl PhoneCallVideoCapabilitiesManager {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phonenumber: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>> {
        Self::IPhoneCallVideoCapabilitiesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCapabilitiesAsync)(::core::mem::transmute_copy(this), phonenumber.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallVideoCapabilitiesManagerStatics<R, F: FnOnce(&IPhoneCallVideoCapabilitiesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallVideoCapabilitiesManager, IPhoneCallVideoCapabilitiesManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallVideoCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallsResult(::windows::core::IUnknown);
impl PhoneCallsResult {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn OperationStatus(&self) -> ::windows::core::Result<PhoneLineOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OperationStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllActivePhoneCalls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhoneCall>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllActivePhoneCalls)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhoneCall>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallsResult {}
impl ::core::fmt::Debug for PhoneCallsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallsResult;{1bfad365-57cf-57dd-986d-b057c91eac33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallsResult {
    type Vtable = IPhoneCallsResult_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallsResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallsResult";
}
impl ::core::convert::From<PhoneCallsResult> for ::windows::core::IUnknown {
    fn from(value: PhoneCallsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallsResult> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallsResult> for ::windows::core::IInspectable {
    fn from(value: PhoneCallsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallsResult> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallsResult {}
unsafe impl ::core::marker::Sync for PhoneCallsResult {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneDialOptions(::windows::core::IUnknown);
impl PhoneDialOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneDialOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Number)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNumber)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContact<'a, Param0: ::windows::core::IntoParam<'a, super::Contacts::Contact>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContact)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPhone(&self) -> ::windows::core::Result<super::Contacts::ContactPhone> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactPhone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPhone>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContactPhone<'a, Param0: ::windows::core::IntoParam<'a, super::Contacts::ContactPhone>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContactPhone)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Media(&self) -> ::windows::core::Result<PhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Media)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallMedia>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetMedia(&self, value: PhoneCallMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMedia)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn AudioEndpoint(&self) -> ::windows::core::Result<PhoneAudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__: PhoneAudioRoutingEndpoint = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioEndpoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneAudioRoutingEndpoint>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetAudioEndpoint(&self, value: PhoneAudioRoutingEndpoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioEndpoint)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for PhoneDialOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneDialOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneDialOptions {}
impl ::core::fmt::Debug for PhoneDialOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneDialOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneDialOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneDialOptions;{b639c4b8-f06f-36cb-a863-823742b5f2d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneDialOptions {
    type Vtable = IPhoneDialOptions_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneDialOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneDialOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneDialOptions";
}
impl ::core::convert::From<PhoneDialOptions> for ::windows::core::IUnknown {
    fn from(value: PhoneDialOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneDialOptions> for ::windows::core::IUnknown {
    fn from(value: &PhoneDialOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneDialOptions> for ::windows::core::IInspectable {
    fn from(value: PhoneDialOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneDialOptions> for ::windows::core::IInspectable {
    fn from(value: &PhoneDialOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneDialOptions {}
unsafe impl ::core::marker::Sync for PhoneDialOptions {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLine(::windows::core::IUnknown);
impl PhoneLine {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLine, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLineChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn DisplayColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NetworkState(&self) -> ::windows::core::Result<PhoneNetworkState> {
        let this = self;
        unsafe {
            let mut result__: PhoneNetworkState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneNetworkState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Voicemail(&self) -> ::windows::core::Result<PhoneVoicemail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Voicemail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneVoicemail>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NetworkName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NetworkName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn CellularDetails(&self) -> ::windows::core::Result<PhoneLineCellularDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CellularDetails)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineCellularDetails>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Transport(&self) -> ::windows::core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineTransport = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineTransport>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn CanDial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanDial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SupportsTile(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportsTile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn VideoCallingCapabilities(&self) -> ::windows::core::Result<PhoneCallVideoCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoCallingCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallVideoCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn LineConfiguration(&self) -> ::windows::core::Result<PhoneLineConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineConfiguration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsImmediateDialNumberAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsImmediateDialNumberAsync)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Dial<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Dial)(::core::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DialWithOptions<'a, Param0: ::windows::core::IntoParam<'a, PhoneDialOptions>>(&self, options: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DialWithOptions)(::core::mem::transmute_copy(this), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn EnableTextReply(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneLine2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableTextReply)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn TransportDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPhoneLine2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransportDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DialWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::core::Result<PhoneLineDialResult> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DialWithResult)(::core::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<PhoneLineDialResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DialWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DialWithResultAsync)(::core::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetAllActivePhoneCalls(&self) -> ::windows::core::Result<PhoneCallsResult> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAllActivePhoneCalls)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallsResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAllActivePhoneCallsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallsResult>> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAllActivePhoneCallsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallsResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(lineid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLine>> {
        Self::IPhoneLineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), lineid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneLine>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneLineStatics<R, F: FnOnce(&IPhoneLineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneLine, IPhoneLineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhoneLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLine {}
impl ::core::fmt::Debug for PhoneLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLine;{27c66f30-6a69-34ca-a2ba-65302530c311})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLine {
    type Vtable = IPhoneLine_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLine as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLine {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLine";
}
impl ::core::convert::From<PhoneLine> for ::windows::core::IUnknown {
    fn from(value: PhoneLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLine> for ::windows::core::IUnknown {
    fn from(value: &PhoneLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLine> for ::windows::core::IInspectable {
    fn from(value: PhoneLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLine> for ::windows::core::IInspectable {
    fn from(value: &PhoneLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLine {}
unsafe impl ::core::marker::Sync for PhoneLine {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineCellularDetails(::windows::core::IUnknown);
impl PhoneLineCellularDetails {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SimState(&self) -> ::windows::core::Result<PhoneSimState> {
        let this = self;
        unsafe {
            let mut result__: PhoneSimState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneSimState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SimSlotIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimSlotIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsModemOn(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsModemOn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RegistrationRejectCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegistrationRejectCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetNetworkOperatorDisplayText(&self, location: PhoneLineNetworkOperatorDisplayTextLocation) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNetworkOperatorDisplayText)(::core::mem::transmute_copy(this), location, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineCellularDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineCellularDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineCellularDetails {}
impl ::core::fmt::Debug for PhoneLineCellularDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineCellularDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineCellularDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineCellularDetails;{192601d5-147c-4769-b673-98a5ec8426cb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLineCellularDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineCellularDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineCellularDetails";
}
impl ::core::convert::From<PhoneLineCellularDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneLineCellularDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineCellularDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineCellularDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineCellularDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneLineCellularDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineCellularDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineCellularDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineCellularDetails {}
unsafe impl ::core::marker::Sync for PhoneLineCellularDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineConfiguration(::windows::core::IUnknown);
impl PhoneLineConfiguration {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsVideoCallingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVideoCallingEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineConfiguration {}
impl ::core::fmt::Debug for PhoneLineConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineConfiguration;{fe265862-f64f-4312-b2a8-4e257721aa95})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLineConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineConfiguration";
}
impl ::core::convert::From<PhoneLineConfiguration> for ::windows::core::IUnknown {
    fn from(value: PhoneLineConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineConfiguration> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineConfiguration> for ::windows::core::IInspectable {
    fn from(value: PhoneLineConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineConfiguration> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineConfiguration {}
unsafe impl ::core::marker::Sync for PhoneLineConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineDialResult(::windows::core::IUnknown);
impl PhoneLineDialResult {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DialCallStatus(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DialCallStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DialedCall(&self) -> ::windows::core::Result<PhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DialedCall)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCall>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineDialResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineDialResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineDialResult {}
impl ::core::fmt::Debug for PhoneLineDialResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineDialResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineDialResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineDialResult;{e825a30a-5c7f-546f-b918-3ad2fe70fb34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLineDialResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineDialResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineDialResult";
}
impl ::core::convert::From<PhoneLineDialResult> for ::windows::core::IUnknown {
    fn from(value: PhoneLineDialResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineDialResult> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineDialResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineDialResult> for ::windows::core::IInspectable {
    fn from(value: PhoneLineDialResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineDialResult> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineDialResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineDialResult {}
unsafe impl ::core::marker::Sync for PhoneLineDialResult {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Dialer: Self = Self(2i32);
    pub const InCallUI: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineNetworkOperatorDisplayTextLocation {}
impl ::core::clone::Clone for PhoneLineNetworkOperatorDisplayTextLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineNetworkOperatorDisplayTextLocation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineNetworkOperatorDisplayTextLocation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineNetworkOperatorDisplayTextLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineNetworkOperatorDisplayTextLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineNetworkOperatorDisplayTextLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineNetworkOperatorDisplayTextLocation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneLineOperationStatus {}
impl ::core::clone::Clone for PhoneLineOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: Self = Self(0i32);
    pub const VoipApp: Self = Self(1i32);
    pub const Bluetooth: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineTransport {}
impl ::core::clone::Clone for PhoneLineTransport {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineTransport {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineTransport {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineTransport;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineTransportDevice(::windows::core::IUnknown);
impl PhoneLineTransportDevice {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Transport(&self) -> ::windows::core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineTransport = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineTransport>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RegisterApp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterApp)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn RegisterAppForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterAppForUser)(::core::mem::transmute_copy(this), user.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn UnregisterApp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UnregisterApp)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn UnregisterAppForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UnregisterAppForUser)(::core::mem::transmute_copy(this), user.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn IsRegistered(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRegistered)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Connect(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Connect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn AudioRoutingStatus(&self) -> ::windows::core::Result<TransportDeviceAudioRoutingStatus> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: TransportDeviceAudioRoutingStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioRoutingStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TransportDeviceAudioRoutingStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioRoutingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioRoutingStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioRoutingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioRoutingStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn InBandRingingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InBandRingingEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InBandRingingEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InBandRingingEnabledChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInBandRingingEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInBandRingingEnabledChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<PhoneLineTransportDevice> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromId)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<PhoneLineTransportDevice>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetDeviceSelectorForPhoneLineTransport(transport: PhoneLineTransport) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorForPhoneLineTransport)(::core::mem::transmute_copy(this), transport, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneLineTransportDeviceStatics<R, F: FnOnce(&IPhoneLineTransportDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneLineTransportDevice, IPhoneLineTransportDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhoneLineTransportDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineTransportDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineTransportDevice {}
impl ::core::fmt::Debug for PhoneLineTransportDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineTransportDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineTransportDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineTransportDevice;{efa8f889-cffa-59f4-97e4-74705b7dc490})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLineTransportDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineTransportDevice {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineTransportDevice";
}
impl ::core::convert::From<PhoneLineTransportDevice> for ::windows::core::IUnknown {
    fn from(value: PhoneLineTransportDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineTransportDevice> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineTransportDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineTransportDevice> for ::windows::core::IInspectable {
    fn from(value: PhoneLineTransportDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineTransportDevice> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineTransportDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineTransportDevice {}
unsafe impl ::core::marker::Sync for PhoneLineTransportDevice {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineWatcher(::windows::core::IUnknown);
impl PhoneLineWatcher {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineAdded)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLineAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineRemoved)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLineRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLineUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PhoneLineWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineWatcherStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineWatcher {}
impl ::core::fmt::Debug for PhoneLineWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcher;{8a45cd0a-6323-44e0-a6f6-9f21f64dc90a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLineWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineWatcher {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcher";
}
impl ::core::convert::From<PhoneLineWatcher> for ::windows::core::IUnknown {
    fn from(value: PhoneLineWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineWatcher> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineWatcher> for ::windows::core::IInspectable {
    fn from(value: PhoneLineWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineWatcher> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineWatcher {}
unsafe impl ::core::marker::Sync for PhoneLineWatcher {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineWatcherEventArgs(::windows::core::IUnknown);
impl PhoneLineWatcherEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineWatcherEventArgs {}
impl ::core::fmt::Debug for PhoneLineWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineWatcherEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs;{d07c753e-9e12-4a37-82b7-ad535dad6a67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLineWatcherEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineWatcherEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs";
}
impl ::core::convert::From<PhoneLineWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhoneLineWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhoneLineWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineWatcherEventArgs {}
unsafe impl ::core::marker::Sync for PhoneLineWatcherEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineWatcherStatus {}
impl ::core::clone::Clone for PhoneLineWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneNetworkState(pub i32);
impl PhoneNetworkState {
    pub const Unknown: Self = Self(0i32);
    pub const NoSignal: Self = Self(1i32);
    pub const Deregistered: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const Searching: Self = Self(4i32);
    pub const Home: Self = Self(5i32);
    pub const RoamingInternational: Self = Self(6i32);
    pub const RoamingDomestic: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneNetworkState {}
impl ::core::clone::Clone for PhoneNetworkState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNetworkState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneNetworkState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneNetworkState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNetworkState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneNetworkState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneNetworkState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneSimState(pub i32);
impl PhoneSimState {
    pub const Unknown: Self = Self(0i32);
    pub const PinNotRequired: Self = Self(1i32);
    pub const PinUnlocked: Self = Self(2i32);
    pub const PinLocked: Self = Self(3i32);
    pub const PukLocked: Self = Self(4i32);
    pub const NotInserted: Self = Self(5i32);
    pub const Invalid: Self = Self(6i32);
    pub const Disabled: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneSimState {}
impl ::core::clone::Clone for PhoneSimState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneSimState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneSimState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneSimState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneSimState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneSimState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneSimState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneVoicemail(::windows::core::IUnknown);
impl PhoneVoicemail {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Number)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MessageCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<PhoneVoicemailType> {
        let this = self;
        unsafe {
            let mut result__: PhoneVoicemailType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneVoicemailType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DialVoicemailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DialVoicemailAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneVoicemail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneVoicemail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneVoicemail {}
impl ::core::fmt::Debug for PhoneVoicemail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneVoicemail").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneVoicemail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneVoicemail;{c9ce77f6-6e9f-3a8b-b727-6e0cf6998224})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneVoicemail {
    type Vtable = IPhoneVoicemail_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneVoicemail as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneVoicemail {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneVoicemail";
}
impl ::core::convert::From<PhoneVoicemail> for ::windows::core::IUnknown {
    fn from(value: PhoneVoicemail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneVoicemail> for ::windows::core::IUnknown {
    fn from(value: &PhoneVoicemail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneVoicemail> for ::windows::core::IInspectable {
    fn from(value: PhoneVoicemail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneVoicemail> for ::windows::core::IInspectable {
    fn from(value: &PhoneVoicemail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneVoicemail {}
unsafe impl ::core::marker::Sync for PhoneVoicemail {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: Self = Self(0i32);
    pub const Traditional: Self = Self(1i32);
    pub const Visual: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneVoicemailType {}
impl ::core::clone::Clone for PhoneVoicemailType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneVoicemailType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneVoicemailType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneVoicemailType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneVoicemailType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneVoicemailType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneVoicemailType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CanRouteToLocalDevice: Self = Self(1i32);
    pub const CannotRouteToLocalDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for TransportDeviceAudioRoutingStatus {}
impl ::core::clone::Clone for TransportDeviceAudioRoutingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TransportDeviceAudioRoutingStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TransportDeviceAudioRoutingStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for TransportDeviceAudioRoutingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransportDeviceAudioRoutingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TransportDeviceAudioRoutingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.TransportDeviceAudioRoutingStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipCallCoordinator(::windows::core::IUnknown);
impl VoipCallCoordinator {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReserveCallResourcesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, taskentrypoint: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReserveCallResourcesAsync)(::core::mem::transmute_copy(this), taskentrypoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MuteStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs>>>(&self, mutechangehandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MuteStateChanged)(::core::mem::transmute_copy(this), mutechangehandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMuteStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMuteStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestNewIncomingCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(
        &self,
        context: Param0,
        contactname: Param1,
        contactnumber: Param2,
        contactimage: Param3,
        servicename: Param4,
        brandingimage: Param5,
        calldetails: Param6,
        ringtone: Param7,
        media: VoipPhoneCallMedia,
        ringtimeout: Param9,
    ) -> ::windows::core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestNewIncomingCall)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), contactimage.into_param().abi(), servicename.into_param().abi(), brandingimage.into_param().abi(), calldetails.into_param().abi(), ringtone.into_param().abi(), media, ringtimeout.into_param().abi(), &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RequestNewOutgoingCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: Param0, contactname: Param1, servicename: Param2, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestNewOutgoingCall)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NotifyMuted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyMuted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NotifyUnmuted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyUnmuted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RequestOutgoingUpgradeToVideoCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, callupgradeguid: Param0, context: Param1, contactname: Param2, servicename: Param3) -> ::windows::core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestOutgoingUpgradeToVideoCall)(::core::mem::transmute_copy(this), callupgradeguid.into_param().abi(), context.into_param().abi(), contactname.into_param().abi(), servicename.into_param().abi(), &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestIncomingUpgradeToVideoCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(
        &self,
        context: Param0,
        contactname: Param1,
        contactnumber: Param2,
        contactimage: Param3,
        servicename: Param4,
        brandingimage: Param5,
        calldetails: Param6,
        ringtone: Param7,
        ringtimeout: Param8,
    ) -> ::windows::core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestIncomingUpgradeToVideoCall)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), contactimage.into_param().abi(), servicename.into_param().abi(), brandingimage.into_param().abi(), calldetails.into_param().abi(), ringtone.into_param().abi(), ringtimeout.into_param().abi(), &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn TerminateCellularCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, callupgradeguid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).TerminateCellularCall)(::core::mem::transmute_copy(this), callupgradeguid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn CancelUpgrade<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, callupgradeguid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CancelUpgrade)(::core::mem::transmute_copy(this), callupgradeguid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetupNewAcceptedCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: Param0, contactname: Param1, contactnumber: Param2, servicename: Param3, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall> {
        let this = &::windows::core::Interface::cast::<IVoipCallCoordinator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetupNewAcceptedCall)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn RequestNewAppInitiatedCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: Param0, contactname: Param1, contactnumber: Param2, servicename: Param3, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall> {
        let this = &::windows::core::Interface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestNewAppInitiatedCall)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestNewIncomingCallWithContactRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param10: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        context: Param0,
        contactname: Param1,
        contactnumber: Param2,
        contactimage: Param3,
        servicename: Param4,
        brandingimage: Param5,
        calldetails: Param6,
        ringtone: Param7,
        media: VoipPhoneCallMedia,
        ringtimeout: Param9,
        contactremoteid: Param10,
    ) -> ::windows::core::Result<VoipPhoneCall> {
        let this = &::windows::core::Interface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestNewIncomingCallWithContactRemoteId)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), contactimage.into_param().abi(), servicename.into_param().abi(), brandingimage.into_param().abi(), calldetails.into_param().abi(), ringtone.into_param().abi(), media, ringtimeout.into_param().abi(), contactremoteid.into_param().abi(), &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReserveOneProcessCallResourcesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = &::windows::core::Interface::cast::<IVoipCallCoordinator4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReserveOneProcessCallResourcesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<VoipCallCoordinator> {
        Self::IVoipCallCoordinatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipCallCoordinator>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoipCallCoordinatorStatics<R, F: FnOnce(&IVoipCallCoordinatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoipCallCoordinator, IVoipCallCoordinatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VoipCallCoordinator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoipCallCoordinator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoipCallCoordinator {}
impl ::core::fmt::Debug for VoipCallCoordinator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipCallCoordinator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoipCallCoordinator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipCallCoordinator;{4f118bcf-e8ef-4434-9c5f-a8d893fafe79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_Vtbl;
    const IID: ::windows::core::GUID = <IVoipCallCoordinator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VoipCallCoordinator {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipCallCoordinator";
}
impl ::core::convert::From<VoipCallCoordinator> for ::windows::core::IUnknown {
    fn from(value: VoipCallCoordinator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoipCallCoordinator> for ::windows::core::IUnknown {
    fn from(value: &VoipCallCoordinator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoipCallCoordinator> for ::windows::core::IInspectable {
    fn from(value: VoipCallCoordinator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoipCallCoordinator> for ::windows::core::IInspectable {
    fn from(value: &VoipCallCoordinator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoipCallCoordinator {}
unsafe impl ::core::marker::Sync for VoipCallCoordinator {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipPhoneCall(::windows::core::IUnknown);
impl VoipPhoneCall {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEndRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HoldRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHoldRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHoldRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResumeRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResumeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveResumeRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnswerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs>>>(&self, accepthandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AnswerRequested)(::core::mem::transmute_copy(this), accepthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAnswerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAnswerRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RejectRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs>>>(&self, rejecthandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RejectRequested)(::core::mem::transmute_copy(this), rejecthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRejectRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRejectRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NotifyCallHeld(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyCallHeld)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NotifyCallActive(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyCallActive)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NotifyCallEnded(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyCallEnded)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetContactName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContactName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn CallMedia(&self) -> ::windows::core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CallMedia)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallMedia>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn SetCallMedia(&self, value: VoipPhoneCallMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCallMedia)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NotifyCallReady(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyCallReady)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn TryShowAppUI(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVoipPhoneCall2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TryShowAppUI)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    pub fn NotifyCallAccepted(&self, media: VoipPhoneCallMedia) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVoipPhoneCall3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).NotifyCallAccepted)(::core::mem::transmute_copy(this), media).ok() }
    }
}
impl ::core::clone::Clone for VoipPhoneCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoipPhoneCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoipPhoneCall {}
impl ::core::fmt::Debug for VoipPhoneCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCall").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCall {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipPhoneCall;{6cf1f19a-7794-4a5a-8c68-ae87947a6990})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VoipPhoneCall {
    type Vtable = IVoipPhoneCall_Vtbl;
    const IID: ::windows::core::GUID = <IVoipPhoneCall as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VoipPhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipPhoneCall";
}
impl ::core::convert::From<VoipPhoneCall> for ::windows::core::IUnknown {
    fn from(value: VoipPhoneCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoipPhoneCall> for ::windows::core::IUnknown {
    fn from(value: &VoipPhoneCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoipPhoneCall> for ::windows::core::IInspectable {
    fn from(value: VoipPhoneCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoipPhoneCall> for ::windows::core::IInspectable {
    fn from(value: &VoipPhoneCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoipPhoneCall {}
unsafe impl ::core::marker::Sync for VoipPhoneCall {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
}
impl ::core::marker::Copy for VoipPhoneCallMedia {}
impl ::core::clone::Clone for VoipPhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallMedia {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallMedia {
    type Abi = Self;
}
impl ::core::fmt::Debug for VoipPhoneCallMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallMedia").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VoipPhoneCallMedia {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VoipPhoneCallMedia {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VoipPhoneCallMedia {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VoipPhoneCallMedia {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VoipPhoneCallMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallMedia;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const OtherIncomingCall: Self = Self(2i32);
    pub const EmergencyCallExists: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for VoipPhoneCallRejectReason {}
impl ::core::clone::Clone for VoipPhoneCallRejectReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallRejectReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallRejectReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for VoipPhoneCallRejectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallRejectReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallRejectReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallRejectReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: Self = Self(0i32);
    pub const ResourcesNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for VoipPhoneCallResourceReservationStatus {}
impl ::core::clone::Clone for VoipPhoneCallResourceReservationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallResourceReservationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallResourceReservationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for VoipPhoneCallResourceReservationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallResourceReservationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallResourceReservationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: Self = Self(0i32);
    pub const Held: Self = Self(1i32);
    pub const Active: Self = Self(2i32);
    pub const Incoming: Self = Self(3i32);
    pub const Outgoing: Self = Self(4i32);
}
impl ::core::marker::Copy for VoipPhoneCallState {}
impl ::core::clone::Clone for VoipPhoneCallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallState {
    type Abi = Self;
}
impl ::core::fmt::Debug for VoipPhoneCallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
