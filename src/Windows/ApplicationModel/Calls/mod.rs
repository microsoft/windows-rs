#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CallAnswerEventArgs(pub ::windows::core::IInspectable);
impl CallAnswerEventArgs {
    pub fn AcceptedMedia(&self) -> ::windows::core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallMedia>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CallAnswerEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallAnswerEventArgs;{fd789617-2dd7-4c8c-b2bd-95d17a5bb733})");
}
unsafe impl ::windows::core::Interface for CallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd789617_2dd7_4c8c_b2bd_95d17a5bb733);
}
impl ::windows::core::RuntimeName for CallAnswerEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallAnswerEventArgs";
}
impl ::core::convert::From<CallAnswerEventArgs> for ::windows::core::IUnknown {
    fn from(value: CallAnswerEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CallAnswerEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CallAnswerEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CallAnswerEventArgs> for ::windows::core::IInspectable {
    fn from(value: CallAnswerEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CallAnswerEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CallAnswerEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CallAnswerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CallAnswerEventArgs {}
unsafe impl ::core::marker::Sync for CallAnswerEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CallRejectEventArgs(pub ::windows::core::IInspectable);
impl CallRejectEventArgs {
    pub fn RejectReason(&self) -> ::windows::core::Result<VoipPhoneCallRejectReason> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallRejectReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallRejectReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CallRejectEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallRejectEventArgs;{da47fad7-13d4-4d92-a1c2-b77811ee37ec})");
}
unsafe impl ::windows::core::Interface for CallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda47fad7_13d4_4d92_a1c2_b77811ee37ec);
}
impl ::windows::core::RuntimeName for CallRejectEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallRejectEventArgs";
}
impl ::core::convert::From<CallRejectEventArgs> for ::windows::core::IUnknown {
    fn from(value: CallRejectEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CallRejectEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CallRejectEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CallRejectEventArgs> for ::windows::core::IInspectable {
    fn from(value: CallRejectEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CallRejectEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CallRejectEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CallRejectEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CallRejectEventArgs {}
unsafe impl ::core::marker::Sync for CallRejectEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CallStateChangeEventArgs(pub ::windows::core::IInspectable);
impl CallStateChangeEventArgs {
    pub fn State(&self) -> ::windows::core::Result<VoipPhoneCallState> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CallStateChangeEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallStateChangeEventArgs;{eab2349e-66f5-47f9-9fb5-459c5198c720})");
}
unsafe impl ::windows::core::Interface for CallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeab2349e_66f5_47f9_9fb5_459c5198c720);
}
impl ::windows::core::RuntimeName for CallStateChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallStateChangeEventArgs";
}
impl ::core::convert::From<CallStateChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: CallStateChangeEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CallStateChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CallStateChangeEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CallStateChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: CallStateChangeEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CallStateChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CallStateChangeEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CallStateChangeEventArgs {}
unsafe impl ::core::marker::Sync for CallStateChangeEventArgs {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct CallsPhoneContract(pub u8);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct CallsVoipContract(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: CellularDtmfMode = CellularDtmfMode(0i32);
    pub const Burst: CellularDtmfMode = CellularDtmfMode(1i32);
}
impl ::core::convert::From<i32> for CellularDtmfMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CellularDtmfMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CellularDtmfMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.CellularDtmfMode;i4)");
}
impl ::windows::core::DefaultType for CellularDtmfMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DtmfKey(pub i32);
impl DtmfKey {
    pub const D0: DtmfKey = DtmfKey(0i32);
    pub const D1: DtmfKey = DtmfKey(1i32);
    pub const D2: DtmfKey = DtmfKey(2i32);
    pub const D3: DtmfKey = DtmfKey(3i32);
    pub const D4: DtmfKey = DtmfKey(4i32);
    pub const D5: DtmfKey = DtmfKey(5i32);
    pub const D6: DtmfKey = DtmfKey(6i32);
    pub const D7: DtmfKey = DtmfKey(7i32);
    pub const D8: DtmfKey = DtmfKey(8i32);
    pub const D9: DtmfKey = DtmfKey(9i32);
    pub const Star: DtmfKey = DtmfKey(10i32);
    pub const Pound: DtmfKey = DtmfKey(11i32);
}
impl ::core::convert::From<i32> for DtmfKey {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DtmfKey {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DtmfKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfKey;i4)");
}
impl ::windows::core::DefaultType for DtmfKey {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: DtmfToneAudioPlayback = DtmfToneAudioPlayback(0i32);
    pub const DoNotPlay: DtmfToneAudioPlayback = DtmfToneAudioPlayback(1i32);
}
impl ::core::convert::From<i32> for DtmfToneAudioPlayback {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DtmfToneAudioPlayback {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DtmfToneAudioPlayback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfToneAudioPlayback;i4)");
}
impl ::windows::core::DefaultType for DtmfToneAudioPlayback {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICallAnswerEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd789617_2dd7_4c8c_b2bd_95d17a5bb733);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallAnswerEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VoipPhoneCallMedia) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICallRejectEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda47fad7_13d4_4d92_a1c2_b77811ee37ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallRejectEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VoipPhoneCallRejectReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICallStateChangeEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeab2349e_66f5_47f9_9fb5_459c5198c720);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallStateChangeEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VoipPhoneCallState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILockScreenCallEndCallDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd7ed0d_98ed_4041_9632_50ff812b773f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndCallDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILockScreenCallEndRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8190a363_6f27_46e9_aeb6_c0ae83e47dc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILockScreenCallUI(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILockScreenCallUI {
    type Vtable = ILockScreenCallUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc596fd8d_73c9_4a14_b021_ec1c50a3b727);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallUI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMuteChangeEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8585e159_0c41_432c_814d_c5f1fdf530be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCall(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCall {
    type Vtable = IPhoneCall_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc14ed0f8_c17d_59d2_9628_66e545b6cd21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCall_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallAudioDevice) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endpoint: PhoneCallAudioDevice, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endpoint: PhoneCallAudioDevice, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallBlockingStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallBlockingStatics {
    type Vtable = IPhoneCallBlockingStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19646f84_2b79_26f1_a46f_694be043f313);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockingStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phonenumberlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfab0e129_32a4_4b85_83d1_f90d8c23a857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallHistoryEntryMedia) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhoneCallHistoryEntryMedia) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallHistorySourceIdKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhoneCallHistorySourceIdKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddress(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f159da_3955_4042_84e6_66eebf82e67f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddressFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryAddressFactory {
    type Vtable = IPhoneCallHistoryEntryAddressFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb0fadba_c7f0_4bb6_9f6b_ba5d73209aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddressFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rawaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rawaddresskind: PhoneCallHistoryEntryRawAddressKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryQueryOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c5fe15c_8bed_40ca_b06e_c4ca8eae5c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryQueryOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ece4be_8d86_479f_8404_a9846920fee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerForUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd925c523_f55f_4353_9db4_0205a5265a55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryManagerStatics {
    type Vtable = IPhoneCallHistoryManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5a6da39_b31f_4f45_ac8e_1b08893c1b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryManagerStatics2 {
    type Vtable = IPhoneCallHistoryManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefd474f0_a2db_4188_9e92_bc3cfa6813cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f907db8_b40e_422b_8545_cb1910a61c52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callhistoryentryid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callhistoryentries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callhistoryentries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallInfo {
    type Vtable = IPhoneCallInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22b42577_3e4d_5dc6_89c2_469fe5ffc189);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallDirection) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallManagerStatics {
    type Vtable = IPhoneCallManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60edac78_78a6_4872_a3ef_98325ec8b843);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallManagerStatics2 {
    type Vtable = IPhoneCallManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7e3c8bc_2370_431c_98fd_43be5f03086d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallStatics {
    type Vtable = IPhoneCallStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2218eeab_f60b_53e7_ba13_5aeafbc22957);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallStore {
    type Vtable = IPhoneCallStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f610748_18a6_4173_86d1_28be9dc62dba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02382786_b16a_4fdb_be3b_c4240e13ad0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallVideoCapabilitiesManagerStatics {
    type Vtable = IPhoneCallVideoCapabilitiesManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3c64b56_f00b_4a1c_a0c6_ee1910749ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallsResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallsResult {
    type Vtable = IPhoneCallsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bfad365_57cf_57dd_986d_b057c91eac33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneLineOperationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneDialOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneDialOptions {
    type Vtable = IPhoneDialOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb639c4b8_f06f_36cb_a863_823742b5f2d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneDialOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallMedia) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhoneCallMedia) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneAudioRoutingEndpoint) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhoneAudioRoutingEndpoint) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLine(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLine {
    type Vtable = IPhoneLine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27c66f30_6a69_34ca_a2ba_65302530c311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneNetworkState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneLineTransport) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLine2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLine2 {
    type Vtable = IPhoneLine2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0167f56a_5344_5d64_8af3_a31a950e916a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLine3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLine3 {
    type Vtable = IPhoneLine3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2e33cf7_2406_57f3_826a_e5a5f40d6fb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineCellularDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x192601d5_147c_4769_b673_98a5ec8426cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineCellularDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneSimState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, location: PhoneLineNetworkOperatorDisplayTextLocation, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe265862_f64f_4312_b2a8_4e257721aa95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineDialResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe825a30a_5c7f_546f_b918_3ad2fe70fb34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDialResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineStatics {
    type Vtable = IPhoneLineStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf38b5f23_ceb0_404f_bcf2_ba9f697d8adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lineid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8f889_cffa_59f4_97e4_74705b7dc490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneLineTransport) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineTransportDevice2 {
    type Vtable = IPhoneLineTransportDevice2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64c885f2_ecf4_5761_8c04_3c248ce61690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TransportDeviceAudioRoutingStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineTransportDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineTransportDeviceStatics {
    type Vtable = IPhoneLineTransportDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f3121ac_d609_51a1_96f3_fb00d1819252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transport: PhoneLineTransport, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a45cd0a_6323_44e0_a6f6_9f21f64dc90a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneLineWatcherStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineWatcherEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd07c753e_9e12_4a37_82b7_ad535dad6a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcherEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneVoicemail(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneVoicemail {
    type Vtable = IPhoneVoicemail_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9ce77f6_6e9f_3a8b_b727_6e0cf6998224);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneVoicemail_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneVoicemailType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f118bcf_e8ef_4434_9c5f_a8d893fafe79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, taskentrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mutechangehandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactimage: ::windows::core::RawPtr,
        servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        brandingimage: ::windows::core::RawPtr,
        calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ringtone: ::windows::core::RawPtr,
        media: VoipPhoneCallMedia,
        ringtimeout: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callupgradeguid: ::windows::core::GUID, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactimage: ::windows::core::RawPtr,
        servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        brandingimage: ::windows::core::RawPtr,
        calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ringtone: ::windows::core::RawPtr,
        ringtimeout: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callupgradeguid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callupgradeguid: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator2 {
    type Vtable = IVoipCallCoordinator2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb4a9f3_c704_4234_89ce_e88cc0d28fbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator3 {
    type Vtable = IVoipCallCoordinator3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338d0cbf_9b55_4021_87ca_e64b9bd666c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        contactimage: ::windows::core::RawPtr,
        servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        brandingimage: ::windows::core::RawPtr,
        calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ringtone: ::windows::core::RawPtr,
        media: VoipPhoneCallMedia,
        ringtimeout: super::super::Foundation::TimeSpan,
        contactremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipCallCoordinator4 {
    type Vtable = IVoipCallCoordinator4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83737239_9311_468f_bb49_47e0dfb5d93e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinatorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipCallCoordinatorStatics {
    type Vtable = IVoipCallCoordinatorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5d1f2b_e04a_4d10_b31a_a55c922cc2fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipPhoneCall(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipPhoneCall {
    type Vtable = IVoipPhoneCall_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cf1f19a_7794_4a5a_8c68_ae87947a6990);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accepthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rejecthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VoipPhoneCallMedia) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VoipPhoneCallMedia) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipPhoneCall2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipPhoneCall2 {
    type Vtable = IVoipPhoneCall2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x741b46e1_245f_41f3_9399_3141d25b52e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipPhoneCall3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoipPhoneCall3 {
    type Vtable = IVoipPhoneCall3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d891522_e258_4aa9_907a_1aa413c25523);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, media: VoipPhoneCallMedia) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct LockScreenCallContract(pub u8);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenCallEndCallDeferral(pub ::windows::core::IInspectable);
impl LockScreenCallEndCallDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallEndCallDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral;{2dd7ed0d-98ed-4041-9632-50ff812b773f})");
}
unsafe impl ::windows::core::Interface for LockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd7ed0d_98ed_4041_9632_50ff812b773f);
}
impl ::windows::core::RuntimeName for LockScreenCallEndCallDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral";
}
impl ::core::convert::From<LockScreenCallEndCallDeferral> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallEndCallDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenCallEndCallDeferral> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallEndCallDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenCallEndCallDeferral> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallEndCallDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenCallEndCallDeferral> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallEndCallDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LockScreenCallEndCallDeferral {}
unsafe impl ::core::marker::Sync for LockScreenCallEndCallDeferral {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenCallEndRequestedEventArgs(pub ::windows::core::IInspectable);
impl LockScreenCallEndRequestedEventArgs {
    pub fn GetDeferral(&self) -> ::windows::core::Result<LockScreenCallEndCallDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LockScreenCallEndCallDeferral>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallEndRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs;{8190a363-6f27-46e9-aeb6-c0ae83e47dc7})");
}
unsafe impl ::windows::core::Interface for LockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8190a363_6f27_46e9_aeb6_c0ae83e47dc7);
}
impl ::windows::core::RuntimeName for LockScreenCallEndRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs";
}
impl ::core::convert::From<LockScreenCallEndRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallEndRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenCallEndRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallEndRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenCallEndRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallEndRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenCallEndRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallEndRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LockScreenCallEndRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallEndRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenCallUI(pub ::windows::core::IInspectable);
impl LockScreenCallUI {
    pub fn Dismiss(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CallTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCallTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallUI;{c596fd8d-73c9-4a14-b021-ec1c50a3b727})");
}
unsafe impl ::windows::core::Interface for LockScreenCallUI {
    type Vtable = ILockScreenCallUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc596fd8d_73c9_4a14_b021_ec1c50a3b727);
}
impl ::windows::core::RuntimeName for LockScreenCallUI {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallUI";
}
impl ::core::convert::From<LockScreenCallUI> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallUI) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenCallUI> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallUI) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenCallUI> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallUI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenCallUI> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenCallUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LockScreenCallUI {}
unsafe impl ::core::marker::Sync for LockScreenCallUI {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MuteChangeEventArgs(pub ::windows::core::IInspectable);
impl MuteChangeEventArgs {
    pub fn Muted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MuteChangeEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.MuteChangeEventArgs;{8585e159-0c41-432c-814d-c5f1fdf530be})");
}
unsafe impl ::windows::core::Interface for MuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8585e159_0c41_432c_814d_c5f1fdf530be);
}
impl ::windows::core::RuntimeName for MuteChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.MuteChangeEventArgs";
}
impl ::core::convert::From<MuteChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: MuteChangeEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MuteChangeEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MuteChangeEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MuteChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: MuteChangeEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MuteChangeEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MuteChangeEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MuteChangeEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MuteChangeEventArgs {}
unsafe impl ::core::marker::Sync for MuteChangeEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(0i32);
    pub const Bluetooth: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(1i32);
    pub const Speakerphone: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(2i32);
}
impl ::core::convert::From<i32> for PhoneAudioRoutingEndpoint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneAudioRoutingEndpoint {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneAudioRoutingEndpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneAudioRoutingEndpoint;i4)");
}
impl ::windows::core::DefaultType for PhoneAudioRoutingEndpoint {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCall(pub ::windows::core::IInspectable);
impl PhoneCall {
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AudioDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsMutedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsMutedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CallId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<PhoneCallStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallStatus>(result__)
        }
    }
    pub fn AudioDevice(&self) -> ::windows::core::Result<PhoneCallAudioDevice> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallAudioDevice = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallAudioDevice>(result__)
        }
    }
    pub fn GetPhoneCallInfo(&self) -> ::windows::core::Result<PhoneCallInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPhoneCallInfoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallInfo>>(result__)
        }
    }
    pub fn End(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn SendDtmfKey(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), key, dtmftoneaudioplayback, &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SendDtmfKeyAsync(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), key, dtmftoneaudioplayback, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn AcceptIncoming(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AcceptIncomingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn Hold(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HoldAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn ResumeFromHold(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResumeFromHoldAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn Mute(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn Unmute(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnmuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn RejectIncoming(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RejectIncomingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn ChangeAudioDevice(&self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), endpoint, &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChangeAudioDeviceAsync(&self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), endpoint, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    pub fn GetFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(callid: Param0) -> ::windows::core::Result<PhoneCall> {
        Self::IPhoneCallStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), callid.into_param().abi(), &mut result__).from_abi::<PhoneCall>(result__)
        })
    }
    pub fn IPhoneCallStatics<R, F: FnOnce(&IPhoneCallStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCall, IPhoneCallStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCall {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCall;{c14ed0f8-c17d-59d2-9628-66e545b6cd21})");
}
unsafe impl ::windows::core::Interface for PhoneCall {
    type Vtable = IPhoneCall_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc14ed0f8_c17d_59d2_9628_66e545b6cd21);
}
impl ::windows::core::RuntimeName for PhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCall";
}
impl ::core::convert::From<PhoneCall> for ::windows::core::IUnknown {
    fn from(value: PhoneCall) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCall> for ::windows::core::IUnknown {
    fn from(value: &PhoneCall) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCall> for ::windows::core::IInspectable {
    fn from(value: PhoneCall) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCall> for ::windows::core::IInspectable {
    fn from(value: &PhoneCall) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCall {}
unsafe impl ::core::marker::Sync for PhoneCall {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: PhoneCallAudioDevice = PhoneCallAudioDevice(0i32);
    pub const LocalDevice: PhoneCallAudioDevice = PhoneCallAudioDevice(1i32);
    pub const RemoteDevice: PhoneCallAudioDevice = PhoneCallAudioDevice(2i32);
}
impl ::core::convert::From<i32> for PhoneCallAudioDevice {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallAudioDevice {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallAudioDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallAudioDevice;i4)");
}
impl ::windows::core::DefaultType for PhoneCallAudioDevice {
    type DefaultType = Self;
}
pub struct PhoneCallBlocking {}
impl PhoneCallBlocking {
    pub fn BlockUnknownNumbers() -> ::windows::core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetBlockUnknownNumbers(value: bool) -> ::windows::core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn BlockPrivateNumbers() -> ::windows::core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetBlockPrivateNumbers(value: bool) -> ::windows::core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetCallBlockingListAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(phonenumberlist: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), phonenumberlist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IPhoneCallBlockingStatics<R, F: FnOnce(&IPhoneCallBlockingStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallBlocking, IPhoneCallBlockingStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallBlocking";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: PhoneCallDirection = PhoneCallDirection(0i32);
    pub const Incoming: PhoneCallDirection = PhoneCallDirection(1i32);
    pub const Outgoing: PhoneCallDirection = PhoneCallDirection(2i32);
}
impl ::core::convert::From<i32> for PhoneCallDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallDirection;i4)");
}
impl ::windows::core::DefaultType for PhoneCallDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallHistoryEntry(pub ::windows::core::IInspectable);
impl PhoneCallHistoryEntry {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntry, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Address(&self) -> ::windows::core::Result<PhoneCallHistoryEntryAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryAddress>(result__)
        }
    }
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntryAddress>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsCallerIdBlocked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCallerIdBlocked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsEmergency(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEmergency(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsIncoming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsIncoming(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsMissed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMissed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsRinging(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRinging(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsSeen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsSuppressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSuppressed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsVoicemail(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVoicemail(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Media(&self) -> ::windows::core::Result<PhoneCallHistoryEntryMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryMedia>(result__)
        }
    }
    pub fn SetMedia(&self, value: PhoneCallHistoryEntryMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<PhoneCallHistoryEntryOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryOtherAppReadAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryOtherAppReadAccess>(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SourceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSourceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SourceIdKind(&self) -> ::windows::core::Result<PhoneCallHistorySourceIdKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistorySourceIdKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistorySourceIdKind>(result__)
        }
    }
    pub fn SetSourceIdKind(&self, value: PhoneCallHistorySourceIdKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntry;{fab0e129-32a4-4b85-83d1-f90d8c23a857})");
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfab0e129_32a4_4b85_83d1_f90d8c23a857);
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntry";
}
impl ::core::convert::From<PhoneCallHistoryEntry> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntry> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallHistoryEntry> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntry> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntry {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntry {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallHistoryEntryAddress(pub ::windows::core::IInspectable);
impl PhoneCallHistoryEntryAddress {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntryAddress, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContactId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RawAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRawAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RawAddressKind(&self) -> ::windows::core::Result<PhoneCallHistoryEntryRawAddressKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryRawAddressKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryRawAddressKind>(result__)
        }
    }
    pub fn SetRawAddressKind(&self, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(rawaddress: Param0, rawaddresskind: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<PhoneCallHistoryEntryAddress> {
        Self::IPhoneCallHistoryEntryAddressFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), rawaddress.into_param().abi(), rawaddresskind, &mut result__).from_abi::<PhoneCallHistoryEntryAddress>(result__)
        })
    }
    pub fn IPhoneCallHistoryEntryAddressFactory<R, F: FnOnce(&IPhoneCallHistoryEntryAddressFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntryAddress, IPhoneCallHistoryEntryAddressFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress;{30f159da-3955-4042-84e6-66eebf82e67f})");
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f159da_3955_4042_84e6_66eebf82e67f);
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntryAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress";
}
impl ::core::convert::From<PhoneCallHistoryEntryAddress> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntryAddress) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryAddress> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntryAddress) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallHistoryEntryAddress> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntryAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryAddress> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntryAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryAddress {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryAddress {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: PhoneCallHistoryEntryMedia = PhoneCallHistoryEntryMedia(0i32);
    pub const Video: PhoneCallHistoryEntryMedia = PhoneCallHistoryEntryMedia(1i32);
}
impl ::core::convert::From<i32> for PhoneCallHistoryEntryMedia {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryMedia {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryMedia;i4)");
}
impl ::windows::core::DefaultType for PhoneCallHistoryEntryMedia {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: PhoneCallHistoryEntryOtherAppReadAccess = PhoneCallHistoryEntryOtherAppReadAccess(0i32);
    pub const SystemOnly: PhoneCallHistoryEntryOtherAppReadAccess = PhoneCallHistoryEntryOtherAppReadAccess(1i32);
}
impl ::core::convert::From<i32> for PhoneCallHistoryEntryOtherAppReadAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryOtherAppReadAccess {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryOtherAppReadAccess;i4)");
}
impl ::windows::core::DefaultType for PhoneCallHistoryEntryOtherAppReadAccess {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(0u32);
    pub const Audio: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(1u32);
    pub const Video: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(2u32);
    pub const All: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(4294967295u32);
}
impl ::core::convert::From<u32> for PhoneCallHistoryEntryQueryDesiredMedia {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryQueryDesiredMedia {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryQueryDesiredMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryDesiredMedia;u4)");
}
impl ::windows::core::DefaultType for PhoneCallHistoryEntryQueryDesiredMedia {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallHistoryEntryQueryOptions(pub ::windows::core::IInspectable);
impl PhoneCallHistoryEntryQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryEntryQueryOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DesiredMedia(&self) -> ::windows::core::Result<PhoneCallHistoryEntryQueryDesiredMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryQueryDesiredMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryQueryDesiredMedia>(result__)
        }
    }
    pub fn SetDesiredMedia(&self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions;{9c5fe15c-8bed-40ca-b06e-c4ca8eae5c87})");
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c5fe15c_8bed_40ca_b06e_c4ca8eae5c87);
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntryQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions";
}
impl ::core::convert::From<PhoneCallHistoryEntryQueryOptions> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryQueryOptions> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallHistoryEntryQueryOptions> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryQueryOptions> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryQueryOptions {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryQueryOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: PhoneCallHistoryEntryRawAddressKind = PhoneCallHistoryEntryRawAddressKind(0i32);
    pub const Custom: PhoneCallHistoryEntryRawAddressKind = PhoneCallHistoryEntryRawAddressKind(1i32);
}
impl ::core::convert::From<i32> for PhoneCallHistoryEntryRawAddressKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryEntryRawAddressKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryRawAddressKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryRawAddressKind;i4)");
}
impl ::windows::core::DefaultType for PhoneCallHistoryEntryRawAddressKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallHistoryEntryReader(pub ::windows::core::IInspectable);
impl PhoneCallHistoryEntryReader {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryEntryReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader;{61ece4be-8d86-479f-8404-a9846920fee6})");
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ece4be_8d86_479f_8404_a9846920fee6);
}
impl ::windows::core::RuntimeName for PhoneCallHistoryEntryReader {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader";
}
impl ::core::convert::From<PhoneCallHistoryEntryReader> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryEntryReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryReader> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryEntryReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallHistoryEntryReader> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryEntryReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallHistoryEntryReader> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryEntryReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryReader {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryReader {}
pub struct PhoneCallHistoryManager {}
impl PhoneCallHistoryManager {
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        Self::IPhoneCallHistoryManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<PhoneCallHistoryManagerForUser> {
        Self::IPhoneCallHistoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<PhoneCallHistoryManagerForUser>(result__)
        })
    }
    pub fn IPhoneCallHistoryManagerStatics<R, F: FnOnce(&IPhoneCallHistoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPhoneCallHistoryManagerStatics2<R, F: FnOnce(&IPhoneCallHistoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallHistoryManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallHistoryManagerForUser(pub ::windows::core::IInspectable);
impl PhoneCallHistoryManagerForUser {
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser;{d925c523-f55f-4353-9db4-0205a5265a55})");
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd925c523_f55f_4353_9db4_0205a5265a55);
}
impl ::windows::core::RuntimeName for PhoneCallHistoryManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser";
}
impl ::core::convert::From<PhoneCallHistoryManagerForUser> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryManagerForUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallHistoryManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryManagerForUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallHistoryManagerForUser> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryManagerForUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallHistoryManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryManagerForUser {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryManagerForUser {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: PhoneCallHistorySourceIdKind = PhoneCallHistorySourceIdKind(0i32);
    pub const PackageFamilyName: PhoneCallHistorySourceIdKind = PhoneCallHistorySourceIdKind(1i32);
}
impl ::core::convert::From<i32> for PhoneCallHistorySourceIdKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistorySourceIdKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistorySourceIdKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistorySourceIdKind;i4)");
}
impl ::windows::core::DefaultType for PhoneCallHistorySourceIdKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallHistoryStore(pub ::windows::core::IInspectable);
impl PhoneCallHistoryStore {
    #[cfg(feature = "Foundation")]
    pub fn GetEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, callhistoryentryid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), callhistoryentryid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>>(result__)
        }
    }
    pub fn GetEntryReader(&self) -> ::windows::core::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryReader>(result__)
        }
    }
    pub fn GetEntryReaderWithOptions<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntryQueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<PhoneCallHistoryEntryReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn DeleteEntriesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>>(&self, callhistoryentries: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), callhistoryentries.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MarkEntryAsSeenAsync<'a, Param0: ::windows::core::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn MarkEntriesAsSeenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>>(&self, callhistoryentries: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), callhistoryentries.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetUnseenCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MarkAllAsSeenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSourcesUnseenCountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, sourceids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), sourceids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn MarkSourcesAsSeenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, sourceids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), sourceids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryStore;{2f907db8-b40e-422b-8545-cb1910a61c52})");
}
unsafe impl ::windows::core::Interface for PhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f907db8_b40e_422b_8545_cb1910a61c52);
}
impl ::windows::core::RuntimeName for PhoneCallHistoryStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryStore";
}
impl ::core::convert::From<PhoneCallHistoryStore> for ::windows::core::IUnknown {
    fn from(value: PhoneCallHistoryStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallHistoryStore> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallHistoryStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallHistoryStore> for ::windows::core::IInspectable {
    fn from(value: PhoneCallHistoryStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallHistoryStore> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallHistoryStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallHistoryStore {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryStore {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(0i32);
    pub const AllEntriesLimitedReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(1i32);
    pub const AllEntriesReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(2i32);
}
impl ::core::convert::From<i32> for PhoneCallHistoryStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallHistoryStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallHistoryStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryStoreAccessType;i4)");
}
impl ::windows::core::DefaultType for PhoneCallHistoryStoreAccessType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallInfo(pub ::windows::core::IInspectable);
impl PhoneCallInfo {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn IsHoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CallDirection(&self) -> ::windows::core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallDirection>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallInfo;{22b42577-3e4d-5dc6-89c2-469fe5ffc189})");
}
unsafe impl ::windows::core::Interface for PhoneCallInfo {
    type Vtable = IPhoneCallInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22b42577_3e4d_5dc6_89c2_469fe5ffc189);
}
impl ::windows::core::RuntimeName for PhoneCallInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallInfo";
}
impl ::core::convert::From<PhoneCallInfo> for ::windows::core::IUnknown {
    fn from(value: PhoneCallInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallInfo> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallInfo> for ::windows::core::IInspectable {
    fn from(value: PhoneCallInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallInfo> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallInfo {}
unsafe impl ::core::marker::Sync for PhoneCallInfo {}
pub struct PhoneCallManager {}
impl PhoneCallManager {
    pub fn ShowPhoneCallUI<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phonenumber: Param0, displayname: Param1) -> ::windows::core::Result<()> {
        Self::IPhoneCallManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), phonenumber.into_param().abi(), displayname.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn CallStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCallStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IsCallActive() -> ::windows::core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsCallIncoming() -> ::windows::core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ShowPhoneCallSettingsUI() -> ::windows::core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallStore>> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallStore>>(result__)
        })
    }
    pub fn IPhoneCallManagerStatics<R, F: FnOnce(&IPhoneCallManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPhoneCallManagerStatics2<R, F: FnOnce(&IPhoneCallManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallManager";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: PhoneCallMedia = PhoneCallMedia(0i32);
    pub const AudioAndVideo: PhoneCallMedia = PhoneCallMedia(1i32);
    pub const AudioAndRealTimeText: PhoneCallMedia = PhoneCallMedia(2i32);
}
impl ::core::convert::From<i32> for PhoneCallMedia {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallMedia {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallMedia;i4)");
}
impl ::windows::core::DefaultType for PhoneCallMedia {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: PhoneCallOperationStatus = PhoneCallOperationStatus(0i32);
    pub const OtherFailure: PhoneCallOperationStatus = PhoneCallOperationStatus(1i32);
    pub const TimedOut: PhoneCallOperationStatus = PhoneCallOperationStatus(2i32);
    pub const ConnectionLost: PhoneCallOperationStatus = PhoneCallOperationStatus(3i32);
    pub const InvalidCallState: PhoneCallOperationStatus = PhoneCallOperationStatus(4i32);
}
impl ::core::convert::From<i32> for PhoneCallOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallOperationStatus;i4)");
}
impl ::windows::core::DefaultType for PhoneCallOperationStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallStatus(pub i32);
impl PhoneCallStatus {
    pub const Lost: PhoneCallStatus = PhoneCallStatus(0i32);
    pub const Incoming: PhoneCallStatus = PhoneCallStatus(1i32);
    pub const Dialing: PhoneCallStatus = PhoneCallStatus(2i32);
    pub const Talking: PhoneCallStatus = PhoneCallStatus(3i32);
    pub const Held: PhoneCallStatus = PhoneCallStatus(4i32);
    pub const Ended: PhoneCallStatus = PhoneCallStatus(5i32);
}
impl ::core::convert::From<i32> for PhoneCallStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallStatus;i4)");
}
impl ::windows::core::DefaultType for PhoneCallStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallStore(pub ::windows::core::IInspectable);
impl PhoneCallStore {
    #[cfg(feature = "Foundation")]
    pub fn IsEmergencyPhoneNumberAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultLineAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::GUID>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::GUID>>(result__)
        }
    }
    pub fn RequestLineWatcher(&self) -> ::windows::core::Result<PhoneLineWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineWatcher>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallStore;{5f610748-18a6-4173-86d1-28be9dc62dba})");
}
unsafe impl ::windows::core::Interface for PhoneCallStore {
    type Vtable = IPhoneCallStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f610748_18a6_4173_86d1_28be9dc62dba);
}
impl ::windows::core::RuntimeName for PhoneCallStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallStore";
}
impl ::core::convert::From<PhoneCallStore> for ::windows::core::IUnknown {
    fn from(value: PhoneCallStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallStore> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallStore> for ::windows::core::IInspectable {
    fn from(value: PhoneCallStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallStore> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallStore {}
unsafe impl ::core::marker::Sync for PhoneCallStore {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallVideoCapabilities(pub ::windows::core::IInspectable);
impl PhoneCallVideoCapabilities {
    pub fn IsVideoCallingCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallVideoCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities;{02382786-b16a-4fdb-be3b-c4240e13ad0d})");
}
unsafe impl ::windows::core::Interface for PhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02382786_b16a_4fdb_be3b_c4240e13ad0d);
}
impl ::windows::core::RuntimeName for PhoneCallVideoCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities";
}
impl ::core::convert::From<PhoneCallVideoCapabilities> for ::windows::core::IUnknown {
    fn from(value: PhoneCallVideoCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallVideoCapabilities> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallVideoCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallVideoCapabilities> for ::windows::core::IInspectable {
    fn from(value: PhoneCallVideoCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallVideoCapabilities> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallVideoCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallVideoCapabilities {}
unsafe impl ::core::marker::Sync for PhoneCallVideoCapabilities {}
pub struct PhoneCallVideoCapabilitiesManager {}
impl PhoneCallVideoCapabilitiesManager {
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phonenumber: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>> {
        Self::IPhoneCallVideoCapabilitiesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), phonenumber.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>>(result__)
        })
    }
    pub fn IPhoneCallVideoCapabilitiesManagerStatics<R, F: FnOnce(&IPhoneCallVideoCapabilitiesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneCallVideoCapabilitiesManager, IPhoneCallVideoCapabilitiesManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhoneCallVideoCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallsResult(pub ::windows::core::IInspectable);
impl PhoneCallsResult {
    pub fn OperationStatus(&self) -> ::windows::core::Result<PhoneLineOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllActivePhoneCalls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhoneCall>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhoneCall>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallsResult;{1bfad365-57cf-57dd-986d-b057c91eac33})");
}
unsafe impl ::windows::core::Interface for PhoneCallsResult {
    type Vtable = IPhoneCallsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bfad365_57cf_57dd_986d_b057c91eac33);
}
impl ::windows::core::RuntimeName for PhoneCallsResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallsResult";
}
impl ::core::convert::From<PhoneCallsResult> for ::windows::core::IUnknown {
    fn from(value: PhoneCallsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallsResult> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallsResult> for ::windows::core::IInspectable {
    fn from(value: PhoneCallsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallsResult> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallsResult {}
unsafe impl ::core::marker::Sync for PhoneCallsResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneDialOptions(pub ::windows::core::IInspectable);
impl PhoneDialOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneDialOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContact<'a, Param0: ::windows::core::IntoParam<'a, super::Contacts::Contact>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPhone(&self) -> ::windows::core::Result<super::Contacts::ContactPhone> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPhone>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContactPhone<'a, Param0: ::windows::core::IntoParam<'a, super::Contacts::ContactPhone>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Media(&self) -> ::windows::core::Result<PhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallMedia>(result__)
        }
    }
    pub fn SetMedia(&self, value: PhoneCallMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AudioEndpoint(&self) -> ::windows::core::Result<PhoneAudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__: PhoneAudioRoutingEndpoint = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneAudioRoutingEndpoint>(result__)
        }
    }
    pub fn SetAudioEndpoint(&self, value: PhoneAudioRoutingEndpoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneDialOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneDialOptions;{b639c4b8-f06f-36cb-a863-823742b5f2d4})");
}
unsafe impl ::windows::core::Interface for PhoneDialOptions {
    type Vtable = IPhoneDialOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb639c4b8_f06f_36cb_a863_823742b5f2d4);
}
impl ::windows::core::RuntimeName for PhoneDialOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneDialOptions";
}
impl ::core::convert::From<PhoneDialOptions> for ::windows::core::IUnknown {
    fn from(value: PhoneDialOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneDialOptions> for ::windows::core::IUnknown {
    fn from(value: &PhoneDialOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneDialOptions> for ::windows::core::IInspectable {
    fn from(value: PhoneDialOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneDialOptions> for ::windows::core::IInspectable {
    fn from(value: &PhoneDialOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneDialOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneDialOptions {}
unsafe impl ::core::marker::Sync for PhoneDialOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLine(pub ::windows::core::IInspectable);
impl PhoneLine {
    #[cfg(feature = "Foundation")]
    pub fn LineChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLine, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn DisplayColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    pub fn NetworkState(&self) -> ::windows::core::Result<PhoneNetworkState> {
        let this = self;
        unsafe {
            let mut result__: PhoneNetworkState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneNetworkState>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Voicemail(&self) -> ::windows::core::Result<PhoneVoicemail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneVoicemail>(result__)
        }
    }
    pub fn NetworkName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CellularDetails(&self) -> ::windows::core::Result<PhoneLineCellularDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineCellularDetails>(result__)
        }
    }
    pub fn Transport(&self) -> ::windows::core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineTransport = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineTransport>(result__)
        }
    }
    pub fn CanDial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SupportsTile(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn VideoCallingCapabilities(&self) -> ::windows::core::Result<PhoneCallVideoCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallVideoCapabilities>(result__)
        }
    }
    pub fn LineConfiguration(&self) -> ::windows::core::Result<PhoneLineConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineConfiguration>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsImmediateDialNumberAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Dial<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    pub fn DialWithOptions<'a, Param0: ::windows::core::IntoParam<'a, PhoneDialOptions>>(&self, options: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), options.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(lineid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLine>> {
        Self::IPhoneLineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lineid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneLine>>(result__)
        })
    }
    pub fn EnableTextReply(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneLine2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TransportDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPhoneLine2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DialWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::core::Result<PhoneLineDialResult> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<PhoneLineDialResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DialWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>>(result__)
        }
    }
    pub fn GetAllActivePhoneCalls(&self) -> ::windows::core::Result<PhoneCallsResult> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallsResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetAllActivePhoneCallsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallsResult>> {
        let this = &::windows::core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallsResult>>(result__)
        }
    }
    pub fn IPhoneLineStatics<R, F: FnOnce(&IPhoneLineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneLine, IPhoneLineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLine;{27c66f30-6a69-34ca-a2ba-65302530c311})");
}
unsafe impl ::windows::core::Interface for PhoneLine {
    type Vtable = IPhoneLine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27c66f30_6a69_34ca_a2ba_65302530c311);
}
impl ::windows::core::RuntimeName for PhoneLine {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLine";
}
impl ::core::convert::From<PhoneLine> for ::windows::core::IUnknown {
    fn from(value: PhoneLine) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLine> for ::windows::core::IUnknown {
    fn from(value: &PhoneLine) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLine> for ::windows::core::IInspectable {
    fn from(value: PhoneLine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLine> for ::windows::core::IInspectable {
    fn from(value: &PhoneLine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLine {}
unsafe impl ::core::marker::Sync for PhoneLine {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineCellularDetails(pub ::windows::core::IInspectable);
impl PhoneLineCellularDetails {
    pub fn SimState(&self) -> ::windows::core::Result<PhoneSimState> {
        let this = self;
        unsafe {
            let mut result__: PhoneSimState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneSimState>(result__)
        }
    }
    pub fn SimSlotIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn IsModemOn(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn RegistrationRejectCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn GetNetworkOperatorDisplayText(&self, location: PhoneLineNetworkOperatorDisplayTextLocation) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), location, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineCellularDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineCellularDetails;{192601d5-147c-4769-b673-98a5ec8426cb})");
}
unsafe impl ::windows::core::Interface for PhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x192601d5_147c_4769_b673_98a5ec8426cb);
}
impl ::windows::core::RuntimeName for PhoneLineCellularDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineCellularDetails";
}
impl ::core::convert::From<PhoneLineCellularDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneLineCellularDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineCellularDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineCellularDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineCellularDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneLineCellularDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineCellularDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineCellularDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLineCellularDetails {}
unsafe impl ::core::marker::Sync for PhoneLineCellularDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineConfiguration(pub ::windows::core::IInspectable);
impl PhoneLineConfiguration {
    pub fn IsVideoCallingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineConfiguration;{fe265862-f64f-4312-b2a8-4e257721aa95})");
}
unsafe impl ::windows::core::Interface for PhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe265862_f64f_4312_b2a8_4e257721aa95);
}
impl ::windows::core::RuntimeName for PhoneLineConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineConfiguration";
}
impl ::core::convert::From<PhoneLineConfiguration> for ::windows::core::IUnknown {
    fn from(value: PhoneLineConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineConfiguration> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineConfiguration> for ::windows::core::IInspectable {
    fn from(value: PhoneLineConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineConfiguration> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLineConfiguration {}
unsafe impl ::core::marker::Sync for PhoneLineConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineDialResult(pub ::windows::core::IInspectable);
impl PhoneLineDialResult {
    pub fn DialCallStatus(&self) -> ::windows::core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    pub fn DialedCall(&self) -> ::windows::core::Result<PhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCall>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineDialResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineDialResult;{e825a30a-5c7f-546f-b918-3ad2fe70fb34})");
}
unsafe impl ::windows::core::Interface for PhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe825a30a_5c7f_546f_b918_3ad2fe70fb34);
}
impl ::windows::core::RuntimeName for PhoneLineDialResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineDialResult";
}
impl ::core::convert::From<PhoneLineDialResult> for ::windows::core::IUnknown {
    fn from(value: PhoneLineDialResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineDialResult> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineDialResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineDialResult> for ::windows::core::IInspectable {
    fn from(value: PhoneLineDialResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineDialResult> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineDialResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineDialResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLineDialResult {}
unsafe impl ::core::marker::Sync for PhoneLineDialResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(0i32);
    pub const Tile: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(1i32);
    pub const Dialer: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(2i32);
    pub const InCallUI: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(3i32);
}
impl ::core::convert::From<i32> for PhoneLineNetworkOperatorDisplayTextLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineNetworkOperatorDisplayTextLocation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneLineNetworkOperatorDisplayTextLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineNetworkOperatorDisplayTextLocation;i4)");
}
impl ::windows::core::DefaultType for PhoneLineNetworkOperatorDisplayTextLocation {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: PhoneLineOperationStatus = PhoneLineOperationStatus(0i32);
    pub const OtherFailure: PhoneLineOperationStatus = PhoneLineOperationStatus(1i32);
    pub const TimedOut: PhoneLineOperationStatus = PhoneLineOperationStatus(2i32);
    pub const ConnectionLost: PhoneLineOperationStatus = PhoneLineOperationStatus(3i32);
    pub const InvalidCallState: PhoneLineOperationStatus = PhoneLineOperationStatus(4i32);
}
impl ::core::convert::From<i32> for PhoneLineOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneLineOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineOperationStatus;i4)");
}
impl ::windows::core::DefaultType for PhoneLineOperationStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: PhoneLineTransport = PhoneLineTransport(0i32);
    pub const VoipApp: PhoneLineTransport = PhoneLineTransport(1i32);
    pub const Bluetooth: PhoneLineTransport = PhoneLineTransport(2i32);
}
impl ::core::convert::From<i32> for PhoneLineTransport {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineTransport {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneLineTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineTransport;i4)");
}
impl ::windows::core::DefaultType for PhoneLineTransport {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineTransportDevice(pub ::windows::core::IInspectable);
impl PhoneLineTransportDevice {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Transport(&self) -> ::windows::core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineTransport = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineTransport>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    pub fn RegisterApp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn RegisterAppForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), user.into_param().abi()).ok() }
    }
    pub fn UnregisterApp(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UnregisterAppForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), user.into_param().abi()).ok() }
    }
    pub fn IsRegistered(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Connect(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<PhoneLineTransportDevice> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<PhoneLineTransportDevice>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorForPhoneLineTransport(transport: PhoneLineTransport) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), transport, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AudioRoutingStatus(&self) -> ::windows::core::Result<TransportDeviceAudioRoutingStatus> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: TransportDeviceAudioRoutingStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TransportDeviceAudioRoutingStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AudioRoutingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioRoutingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn InBandRingingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InBandRingingEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInBandRingingEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IPhoneLineTransportDeviceStatics<R, F: FnOnce(&IPhoneLineTransportDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhoneLineTransportDevice, IPhoneLineTransportDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineTransportDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineTransportDevice;{efa8f889-cffa-59f4-97e4-74705b7dc490})");
}
unsafe impl ::windows::core::Interface for PhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8f889_cffa_59f4_97e4_74705b7dc490);
}
impl ::windows::core::RuntimeName for PhoneLineTransportDevice {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineTransportDevice";
}
impl ::core::convert::From<PhoneLineTransportDevice> for ::windows::core::IUnknown {
    fn from(value: PhoneLineTransportDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineTransportDevice> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineTransportDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineTransportDevice> for ::windows::core::IInspectable {
    fn from(value: PhoneLineTransportDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineTransportDevice> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineTransportDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLineTransportDevice {}
unsafe impl ::core::marker::Sync for PhoneLineTransportDevice {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineWatcher(pub ::windows::core::IInspectable);
impl PhoneLineWatcher {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LineAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LineRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LineUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<PhoneLineWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineWatcherStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcher;{8a45cd0a-6323-44e0-a6f6-9f21f64dc90a})");
}
unsafe impl ::windows::core::Interface for PhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a45cd0a_6323_44e0_a6f6_9f21f64dc90a);
}
impl ::windows::core::RuntimeName for PhoneLineWatcher {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcher";
}
impl ::core::convert::From<PhoneLineWatcher> for ::windows::core::IUnknown {
    fn from(value: PhoneLineWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineWatcher> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineWatcher> for ::windows::core::IInspectable {
    fn from(value: PhoneLineWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineWatcher> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLineWatcher {}
unsafe impl ::core::marker::Sync for PhoneLineWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineWatcherEventArgs(pub ::windows::core::IInspectable);
impl PhoneLineWatcherEventArgs {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineWatcherEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs;{d07c753e-9e12-4a37-82b7-ad535dad6a67})");
}
unsafe impl ::windows::core::Interface for PhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd07c753e_9e12_4a37_82b7_ad535dad6a67);
}
impl ::windows::core::RuntimeName for PhoneLineWatcherEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs";
}
impl ::core::convert::From<PhoneLineWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhoneLineWatcherEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineWatcherEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineWatcherEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhoneLineWatcherEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineWatcherEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineWatcherEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLineWatcherEventArgs {}
unsafe impl ::core::marker::Sync for PhoneLineWatcherEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: PhoneLineWatcherStatus = PhoneLineWatcherStatus(0i32);
    pub const Started: PhoneLineWatcherStatus = PhoneLineWatcherStatus(1i32);
    pub const EnumerationCompleted: PhoneLineWatcherStatus = PhoneLineWatcherStatus(2i32);
    pub const Stopped: PhoneLineWatcherStatus = PhoneLineWatcherStatus(3i32);
}
impl ::core::convert::From<i32> for PhoneLineWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneLineWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for PhoneLineWatcherStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneNetworkState(pub i32);
impl PhoneNetworkState {
    pub const Unknown: PhoneNetworkState = PhoneNetworkState(0i32);
    pub const NoSignal: PhoneNetworkState = PhoneNetworkState(1i32);
    pub const Deregistered: PhoneNetworkState = PhoneNetworkState(2i32);
    pub const Denied: PhoneNetworkState = PhoneNetworkState(3i32);
    pub const Searching: PhoneNetworkState = PhoneNetworkState(4i32);
    pub const Home: PhoneNetworkState = PhoneNetworkState(5i32);
    pub const RoamingInternational: PhoneNetworkState = PhoneNetworkState(6i32);
    pub const RoamingDomestic: PhoneNetworkState = PhoneNetworkState(7i32);
}
impl ::core::convert::From<i32> for PhoneNetworkState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneNetworkState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneNetworkState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneNetworkState;i4)");
}
impl ::windows::core::DefaultType for PhoneNetworkState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneSimState(pub i32);
impl PhoneSimState {
    pub const Unknown: PhoneSimState = PhoneSimState(0i32);
    pub const PinNotRequired: PhoneSimState = PhoneSimState(1i32);
    pub const PinUnlocked: PhoneSimState = PhoneSimState(2i32);
    pub const PinLocked: PhoneSimState = PhoneSimState(3i32);
    pub const PukLocked: PhoneSimState = PhoneSimState(4i32);
    pub const NotInserted: PhoneSimState = PhoneSimState(5i32);
    pub const Invalid: PhoneSimState = PhoneSimState(6i32);
    pub const Disabled: PhoneSimState = PhoneSimState(7i32);
}
impl ::core::convert::From<i32> for PhoneSimState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneSimState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneSimState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneSimState;i4)");
}
impl ::windows::core::DefaultType for PhoneSimState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneVoicemail(pub ::windows::core::IInspectable);
impl PhoneVoicemail {
    pub fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PhoneVoicemailType> {
        let this = self;
        unsafe {
            let mut result__: PhoneVoicemailType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneVoicemailType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DialVoicemailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneVoicemail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneVoicemail;{c9ce77f6-6e9f-3a8b-b727-6e0cf6998224})");
}
unsafe impl ::windows::core::Interface for PhoneVoicemail {
    type Vtable = IPhoneVoicemail_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9ce77f6_6e9f_3a8b_b727_6e0cf6998224);
}
impl ::windows::core::RuntimeName for PhoneVoicemail {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneVoicemail";
}
impl ::core::convert::From<PhoneVoicemail> for ::windows::core::IUnknown {
    fn from(value: PhoneVoicemail) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneVoicemail> for ::windows::core::IUnknown {
    fn from(value: &PhoneVoicemail) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneVoicemail> for ::windows::core::IInspectable {
    fn from(value: PhoneVoicemail) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneVoicemail> for ::windows::core::IInspectable {
    fn from(value: &PhoneVoicemail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneVoicemail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneVoicemail {}
unsafe impl ::core::marker::Sync for PhoneVoicemail {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: PhoneVoicemailType = PhoneVoicemailType(0i32);
    pub const Traditional: PhoneVoicemailType = PhoneVoicemailType(1i32);
    pub const Visual: PhoneVoicemailType = PhoneVoicemailType(2i32);
}
impl ::core::convert::From<i32> for PhoneVoicemailType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneVoicemailType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneVoicemailType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneVoicemailType;i4)");
}
impl ::windows::core::DefaultType for PhoneVoicemailType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(0i32);
    pub const CanRouteToLocalDevice: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(1i32);
    pub const CannotRouteToLocalDevice: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(2i32);
}
impl ::core::convert::From<i32> for TransportDeviceAudioRoutingStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TransportDeviceAudioRoutingStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TransportDeviceAudioRoutingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.TransportDeviceAudioRoutingStatus;i4)");
}
impl ::windows::core::DefaultType for TransportDeviceAudioRoutingStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoipCallCoordinator(pub ::windows::core::IInspectable);
impl VoipCallCoordinator {
    #[cfg(feature = "Foundation")]
    pub fn ReserveCallResourcesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, taskentrypoint: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), taskentrypoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MuteStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs>>>(&self, mutechangehandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mutechangehandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMuteStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestNewIncomingCall<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param9: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
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
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                context.into_param().abi(),
                contactname.into_param().abi(),
                contactnumber.into_param().abi(),
                contactimage.into_param().abi(),
                servicename.into_param().abi(),
                brandingimage.into_param().abi(),
                calldetails.into_param().abi(),
                ringtone.into_param().abi(),
                media,
                ringtimeout.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoipPhoneCall>(result__)
        }
    }
    pub fn RequestNewOutgoingCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: Param0, contactname: Param1, servicename: Param2, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    pub fn NotifyMuted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn NotifyUnmuted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn RequestOutgoingUpgradeToVideoCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, callupgradeguid: Param0, context: Param1, contactname: Param2, servicename: Param3) -> ::windows::core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), callupgradeguid.into_param().abi(), context.into_param().abi(), contactname.into_param().abi(), servicename.into_param().abi(), &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestIncomingUpgradeToVideoCall<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param8: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
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
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                context.into_param().abi(),
                contactname.into_param().abi(),
                contactnumber.into_param().abi(),
                contactimage.into_param().abi(),
                servicename.into_param().abi(),
                brandingimage.into_param().abi(),
                calldetails.into_param().abi(),
                ringtone.into_param().abi(),
                ringtimeout.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoipPhoneCall>(result__)
        }
    }
    pub fn TerminateCellularCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, callupgradeguid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), callupgradeguid.into_param().abi()).ok() }
    }
    pub fn CancelUpgrade<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, callupgradeguid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), callupgradeguid.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<VoipCallCoordinator> {
        Self::IVoipCallCoordinatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipCallCoordinator>(result__)
        })
    }
    pub fn SetupNewAcceptedCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: Param0, contactname: Param1, contactnumber: Param2, servicename: Param3, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall> {
        let this = &::windows::core::Interface::cast::<IVoipCallCoordinator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    pub fn RequestNewAppInitiatedCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, context: Param0, contactname: Param1, contactnumber: Param2, servicename: Param3, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall> {
        let this = &::windows::core::Interface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestNewIncomingCallWithContactRemoteId<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param5: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param7: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>,
        Param9: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>,
        Param10: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                context.into_param().abi(),
                contactname.into_param().abi(),
                contactnumber.into_param().abi(),
                contactimage.into_param().abi(),
                servicename.into_param().abi(),
                brandingimage.into_param().abi(),
                calldetails.into_param().abi(),
                ringtone.into_param().abi(),
                media,
                ringtimeout.into_param().abi(),
                contactremoteid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReserveOneProcessCallResourcesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = &::windows::core::Interface::cast::<IVoipCallCoordinator4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>(result__)
        }
    }
    pub fn IVoipCallCoordinatorStatics<R, F: FnOnce(&IVoipCallCoordinatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoipCallCoordinator, IVoipCallCoordinatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VoipCallCoordinator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipCallCoordinator;{4f118bcf-e8ef-4434-9c5f-a8d893fafe79})");
}
unsafe impl ::windows::core::Interface for VoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f118bcf_e8ef_4434_9c5f_a8d893fafe79);
}
impl ::windows::core::RuntimeName for VoipCallCoordinator {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipCallCoordinator";
}
impl ::core::convert::From<VoipCallCoordinator> for ::windows::core::IUnknown {
    fn from(value: VoipCallCoordinator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoipCallCoordinator> for ::windows::core::IUnknown {
    fn from(value: &VoipCallCoordinator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoipCallCoordinator> for ::windows::core::IInspectable {
    fn from(value: VoipCallCoordinator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoipCallCoordinator> for ::windows::core::IInspectable {
    fn from(value: &VoipCallCoordinator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoipCallCoordinator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoipCallCoordinator {}
unsafe impl ::core::marker::Sync for VoipCallCoordinator {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoipPhoneCall(pub ::windows::core::IInspectable);
impl VoipPhoneCall {
    #[cfg(feature = "Foundation")]
    pub fn EndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEndRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HoldRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHoldRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResumeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveResumeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AnswerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs>>>(&self, accepthandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), accepthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAnswerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RejectRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs>>>(&self, rejecthandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), rejecthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRejectRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn NotifyCallHeld(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn NotifyCallActive(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn NotifyCallEnded(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContactName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CallMedia(&self) -> ::windows::core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallMedia = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallMedia>(result__)
        }
    }
    pub fn SetCallMedia(&self, value: VoipPhoneCallMedia) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NotifyCallReady(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn TryShowAppUI(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVoipPhoneCall2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn NotifyCallAccepted(&self, media: VoipPhoneCallMedia) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVoipPhoneCall3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), media).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCall {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipPhoneCall;{6cf1f19a-7794-4a5a-8c68-ae87947a6990})");
}
unsafe impl ::windows::core::Interface for VoipPhoneCall {
    type Vtable = IVoipPhoneCall_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cf1f19a_7794_4a5a_8c68_ae87947a6990);
}
impl ::windows::core::RuntimeName for VoipPhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipPhoneCall";
}
impl ::core::convert::From<VoipPhoneCall> for ::windows::core::IUnknown {
    fn from(value: VoipPhoneCall) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoipPhoneCall> for ::windows::core::IUnknown {
    fn from(value: &VoipPhoneCall) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoipPhoneCall> for ::windows::core::IInspectable {
    fn from(value: VoipPhoneCall) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoipPhoneCall> for ::windows::core::IInspectable {
    fn from(value: &VoipPhoneCall) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoipPhoneCall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoipPhoneCall {}
unsafe impl ::core::marker::Sync for VoipPhoneCall {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: VoipPhoneCallMedia = VoipPhoneCallMedia(0u32);
    pub const Audio: VoipPhoneCallMedia = VoipPhoneCallMedia(1u32);
    pub const Video: VoipPhoneCallMedia = VoipPhoneCallMedia(2u32);
}
impl ::core::convert::From<u32> for VoipPhoneCallMedia {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallMedia {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallMedia {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallMedia;u4)");
}
impl ::windows::core::DefaultType for VoipPhoneCallMedia {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for VoipPhoneCallMedia {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for VoipPhoneCallMedia {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for VoipPhoneCallMedia {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for VoipPhoneCallMedia {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for VoipPhoneCallMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(0i32);
    pub const TimedOut: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(1i32);
    pub const OtherIncomingCall: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(2i32);
    pub const EmergencyCallExists: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(3i32);
    pub const InvalidCallState: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(4i32);
}
impl ::core::convert::From<i32> for VoipPhoneCallRejectReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallRejectReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallRejectReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallRejectReason;i4)");
}
impl ::windows::core::DefaultType for VoipPhoneCallRejectReason {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: VoipPhoneCallResourceReservationStatus = VoipPhoneCallResourceReservationStatus(0i32);
    pub const ResourcesNotAvailable: VoipPhoneCallResourceReservationStatus = VoipPhoneCallResourceReservationStatus(1i32);
}
impl ::core::convert::From<i32> for VoipPhoneCallResourceReservationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallResourceReservationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallResourceReservationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus;i4)");
}
impl ::windows::core::DefaultType for VoipPhoneCallResourceReservationStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: VoipPhoneCallState = VoipPhoneCallState(0i32);
    pub const Held: VoipPhoneCallState = VoipPhoneCallState(1i32);
    pub const Active: VoipPhoneCallState = VoipPhoneCallState(2i32);
    pub const Incoming: VoipPhoneCallState = VoipPhoneCallState(3i32);
    pub const Outgoing: VoipPhoneCallState = VoipPhoneCallState(4i32);
}
impl ::core::convert::From<i32> for VoipPhoneCallState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VoipPhoneCallState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VoipPhoneCallState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallState;i4)");
}
impl ::windows::core::DefaultType for VoipPhoneCallState {
    type DefaultType = Self;
}
