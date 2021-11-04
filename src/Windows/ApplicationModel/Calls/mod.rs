#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CallAnswerEventArgs(pub ::windows::runtime::IInspectable);
impl CallAnswerEventArgs {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn AcceptedMedia(&self) -> ::windows::runtime::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallMedia = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallMedia>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CallAnswerEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallAnswerEventArgs;{fd789617-2dd7-4c8c-b2bd-95d17a5bb733})");
}
unsafe impl ::windows::runtime::Interface for CallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4252538391, 11735, 19596, [178, 189, 149, 209, 122, 91, 183, 51]);
}
impl ::windows::runtime::RuntimeName for CallAnswerEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallAnswerEventArgs";
}
impl ::std::convert::From<CallAnswerEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CallAnswerEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CallAnswerEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CallAnswerEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CallAnswerEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CallAnswerEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CallAnswerEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CallAnswerEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CallAnswerEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CallAnswerEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CallAnswerEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CallAnswerEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CallAnswerEventArgs {}
unsafe impl ::std::marker::Sync for CallAnswerEventArgs {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CallRejectEventArgs(pub ::windows::runtime::IInspectable);
impl CallRejectEventArgs {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RejectReason(&self) -> ::windows::runtime::Result<VoipPhoneCallRejectReason> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallRejectReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallRejectReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CallRejectEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallRejectEventArgs;{da47fad7-13d4-4d92-a1c2-b77811ee37ec})");
}
unsafe impl ::windows::runtime::Interface for CallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3662150359, 5076, 19858, [161, 194, 183, 120, 17, 238, 55, 236]);
}
impl ::windows::runtime::RuntimeName for CallRejectEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallRejectEventArgs";
}
impl ::std::convert::From<CallRejectEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CallRejectEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CallRejectEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CallRejectEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CallRejectEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CallRejectEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CallRejectEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CallRejectEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CallRejectEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CallRejectEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CallRejectEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CallRejectEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CallRejectEventArgs {}
unsafe impl ::std::marker::Sync for CallRejectEventArgs {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CallStateChangeEventArgs(pub ::windows::runtime::IInspectable);
impl CallStateChangeEventArgs {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn State(&self) -> ::windows::runtime::Result<VoipPhoneCallState> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CallStateChangeEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallStateChangeEventArgs;{eab2349e-66f5-47f9-9fb5-459c5198c720})");
}
unsafe impl ::windows::runtime::Interface for CallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3937547422, 26357, 18425, [159, 181, 69, 156, 81, 152, 199, 32]);
}
impl ::windows::runtime::RuntimeName for CallStateChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallStateChangeEventArgs";
}
impl ::std::convert::From<CallStateChangeEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CallStateChangeEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CallStateChangeEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CallStateChangeEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CallStateChangeEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CallStateChangeEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CallStateChangeEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CallStateChangeEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CallStateChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CallStateChangeEventArgs {}
unsafe impl ::std::marker::Sync for CallStateChangeEventArgs {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct CallsPhoneContract(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct CallsVoipContract(pub u8);
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: CellularDtmfMode = CellularDtmfMode(0i32);
    pub const Burst: CellularDtmfMode = CellularDtmfMode(1i32);
}
impl ::std::convert::From<i32> for CellularDtmfMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CellularDtmfMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CellularDtmfMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.CellularDtmfMode;i4)");
}
impl ::windows::runtime::DefaultType for CellularDtmfMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DtmfKey {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DtmfKey {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DtmfKey {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfKey;i4)");
}
impl ::windows::runtime::DefaultType for DtmfKey {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: DtmfToneAudioPlayback = DtmfToneAudioPlayback(0i32);
    pub const DoNotPlay: DtmfToneAudioPlayback = DtmfToneAudioPlayback(1i32);
}
impl ::std::convert::From<i32> for DtmfToneAudioPlayback {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DtmfToneAudioPlayback {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DtmfToneAudioPlayback {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfToneAudioPlayback;i4)");
}
impl ::windows::runtime::DefaultType for DtmfToneAudioPlayback {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICallAnswerEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4252538391, 11735, 19596, [178, 189, 149, 209, 122, 91, 183, 51]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallAnswerEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VoipPhoneCallMedia) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICallRejectEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3662150359, 5076, 19858, [161, 194, 183, 120, 17, 238, 55, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallRejectEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VoipPhoneCallRejectReason) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICallStateChangeEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3937547422, 26357, 18425, [159, 181, 69, 156, 81, 152, 199, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallStateChangeEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VoipPhoneCallState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILockScreenCallEndCallDeferral(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(769125645, 39149, 16449, [150, 50, 80, 255, 129, 43, 119, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndCallDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILockScreenCallEndRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2173739875, 28455, 18153, [174, 182, 192, 174, 131, 228, 125, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILockScreenCallUI(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILockScreenCallUI {
    type Vtable = ILockScreenCallUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3315006861, 29641, 18964, [176, 33, 236, 28, 80, 163, 183, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMuteChangeEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2240143705, 3137, 17196, [129, 77, 197, 241, 253, 245, 48, 190]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCall(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCall {
    type Vtable = IPhoneCall_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3243167992, 49533, 22994, [150, 40, 102, 229, 69, 182, 205, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCall_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallAudioDevice) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, endpoint: PhoneCallAudioDevice, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, endpoint: PhoneCallAudioDevice, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallBlockingStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallBlockingStatics {
    type Vtable = IPhoneCallBlockingStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(426012548, 11129, 9969, [164, 111, 105, 75, 224, 67, 243, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockingStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonenumberlist: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntry(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4205895977, 12964, 19333, [131, 209, 249, 13, 140, 35, 168, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallHistoryEntryMedia) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneCallHistoryEntryMedia) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallHistorySourceIdKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneCallHistorySourceIdKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddress(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(821123546, 14677, 16450, [132, 230, 102, 238, 191, 130, 230, 127]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallHistoryEntryRawAddressKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddressFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryEntryAddressFactory {
    type Vtable = IPhoneCallHistoryEntryAddressFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4212108730, 51184, 19382, [159, 107, 186, 93, 115, 32, 154, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddressFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rawaddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, rawaddresskind: PhoneCallHistoryEntryRawAddressKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryQueryOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2623529308, 35821, 16586, [176, 110, 196, 202, 142, 174, 92, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryQueryOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryReader(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1642915006, 36230, 18335, [132, 4, 169, 132, 105, 32, 254, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerForUser(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3643131171, 62815, 17235, [157, 180, 2, 5, 165, 38, 90, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryManagerStatics {
    type Vtable = IPhoneCallHistoryManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4121352761, 45855, 20293, [172, 142, 27, 8, 137, 60, 27, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryManagerStatics2 {
    type Vtable = IPhoneCallHistoryManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023678192, 41691, 16776, [158, 146, 188, 60, 250, 104, 19, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallHistoryStore(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(797998520, 46094, 16939, [133, 69, 203, 25, 16, 166, 28, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callhistoryentryid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, queryoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callhistoryentry: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callhistoryentry: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callhistoryentries: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callhistoryentry: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callhistoryentries: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallInfo {
    type Vtable = IPhoneCallInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582231415, 15949, 24006, [137, 194, 70, 159, 229, 255, 193, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallDirection) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallManagerStatics {
    type Vtable = IPhoneCallManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1626188920, 30886, 18546, [163, 239, 152, 50, 94, 200, 184, 67]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonenumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallManagerStatics2 {
    type Vtable = IPhoneCallManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3353594044, 9072, 17180, [152, 253, 67, 190, 95, 3, 8, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallStatics {
    type Vtable = IPhoneCallStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(572059307, 62987, 21479, [186, 19, 90, 234, 251, 194, 41, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallStore(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallStore {
    type Vtable = IPhoneCallStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1600194376, 6310, 16755, [134, 209, 40, 190, 157, 198, 45, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilities(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37234566, 45418, 20443, [190, 59, 196, 36, 14, 19, 173, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallVideoCapabilitiesManagerStatics {
    type Vtable = IPhoneCallVideoCapabilitiesManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4089858902, 61451, 18972, [160, 198, 238, 25, 16, 116, 156, 231]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonenumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallsResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallsResult {
    type Vtable = IPhoneCallsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(469422949, 22479, 22493, [152, 109, 176, 87, 201, 30, 172, 51]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneLineOperationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneDialOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneDialOptions {
    type Vtable = IPhoneDialOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3057239224, 61551, 14027, [168, 99, 130, 55, 66, 181, 242, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneDialOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallMedia) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneCallMedia) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneAudioRoutingEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneAudioRoutingEndpoint) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLine(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLine {
    type Vtable = IPhoneLine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(667316016, 27241, 13514, [162, 186, 101, 48, 37, 48, 195, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneNetworkState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneLineTransport) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLine2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLine2 {
    type Vtable = IPhoneLine2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(23590250, 21316, 23908, [138, 243, 163, 26, 149, 14, 145, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLine3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLine3 {
    type Vtable = IPhoneLine3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3806543095, 9222, 22515, [130, 106, 229, 165, 244, 13, 111, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineCellularDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(421921237, 5244, 18281, [182, 115, 152, 165, 236, 132, 38, 203]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineCellularDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneSimState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, location: PhoneLineNetworkOperatorDisplayTextLocation, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineConfiguration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4263925858, 63055, 17170, [178, 168, 78, 37, 119, 33, 170, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineDialResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3894780682, 23679, 21615, [185, 24, 58, 210, 254, 112, 251, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDialResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallOperationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineStatics {
    type Vtable = IPhoneLineStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4085997347, 52912, 16463, [188, 242, 186, 159, 105, 125, 138, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lineid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020828297, 53242, 23028, [151, 228, 116, 112, 91, 125, 196, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneLineTransport) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineTransportDevice2 {
    type Vtable = IPhoneLineTransportDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1690863090, 60660, 22369, [140, 4, 60, 36, 140, 230, 22, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TransportDeviceAudioRoutingStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineTransportDeviceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineTransportDeviceStatics {
    type Vtable = IPhoneLineTransportDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(254878124, 54793, 20897, [150, 243, 251, 0, 209, 129, 146, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transport: PhoneLineTransport, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2319830282, 25379, 17632, [166, 246, 159, 33, 246, 77, 201, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneLineWatcherStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineWatcherEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3497817406, 40466, 18999, [130, 183, 173, 83, 93, 173, 106, 103]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcherEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneVoicemail(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneVoicemail {
    type Vtable = IPhoneVoicemail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3385751542, 28319, 14987, [183, 39, 110, 12, 246, 153, 130, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneVoicemail_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneVoicemailType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1326549967, 59631, 17460, [156, 95, 168, 216, 147, 250, 254, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskentrypoint: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mutechangehandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactnumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactimage: ::windows::runtime::RawPtr,
        servicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        brandingimage: ::windows::runtime::RawPtr,
        calldetails: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        ringtone: ::windows::runtime::RawPtr,
        media: VoipPhoneCallMedia,
        ringtimeout: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contactname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, servicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callupgradeguid: ::windows::runtime::GUID, context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contactname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, servicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactnumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactimage: ::windows::runtime::RawPtr,
        servicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        brandingimage: ::windows::runtime::RawPtr,
        calldetails: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        ringtone: ::windows::runtime::RawPtr,
        ringtimeout: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callupgradeguid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callupgradeguid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipCallCoordinator2 {
    type Vtable = IVoipCallCoordinator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3199511027, 50948, 16948, [137, 206, 232, 140, 192, 210, 143, 190]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contactname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contactnumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, servicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipCallCoordinator3 {
    type Vtable = IVoipCallCoordinator3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(864881855, 39765, 16417, [135, 202, 230, 75, 155, 214, 102, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contactname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contactnumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, servicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        context: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactnumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contactimage: ::windows::runtime::RawPtr,
        servicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        brandingimage: ::windows::runtime::RawPtr,
        calldetails: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        ringtone: ::windows::runtime::RawPtr,
        media: VoipPhoneCallMedia,
        ringtimeout: super::super::Foundation::TimeSpan,
        contactremoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinator4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipCallCoordinator4 {
    type Vtable = IVoipCallCoordinator4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2205381177, 37649, 18063, [187, 73, 71, 224, 223, 181, 217, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipCallCoordinatorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipCallCoordinatorStatics {
    type Vtable = IVoipCallCoordinatorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2136809259, 57418, 19728, [179, 26, 165, 92, 146, 44, 194, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipPhoneCall(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipPhoneCall {
    type Vtable = IVoipPhoneCall_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1827795354, 30612, 19034, [140, 104, 174, 135, 148, 122, 105, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accepthandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rejecthandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VoipPhoneCallMedia) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: VoipPhoneCallMedia) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipPhoneCall2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipPhoneCall2 {
    type Vtable = IVoipPhoneCall2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1947944673, 9311, 16883, [147, 153, 49, 65, 210, 91, 82, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoipPhoneCall3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoipPhoneCall3 {
    type Vtable = IVoipPhoneCall3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(227087650, 57944, 19113, [144, 122, 26, 164, 19, 194, 85, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, media: VoipPhoneCallMedia) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct LockScreenCallContract(pub u8);
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LockScreenCallEndCallDeferral(pub ::windows::runtime::IInspectable);
impl LockScreenCallEndCallDeferral {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LockScreenCallEndCallDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral;{2dd7ed0d-98ed-4041-9632-50ff812b773f})");
}
unsafe impl ::windows::runtime::Interface for LockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(769125645, 39149, 16449, [150, 50, 80, 255, 129, 43, 119, 63]);
}
impl ::windows::runtime::RuntimeName for LockScreenCallEndCallDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral";
}
impl ::std::convert::From<LockScreenCallEndCallDeferral> for ::windows::runtime::IUnknown {
    fn from(value: LockScreenCallEndCallDeferral) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LockScreenCallEndCallDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &LockScreenCallEndCallDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LockScreenCallEndCallDeferral> for ::windows::runtime::IInspectable {
    fn from(value: LockScreenCallEndCallDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LockScreenCallEndCallDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &LockScreenCallEndCallDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LockScreenCallEndCallDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LockScreenCallEndCallDeferral {}
unsafe impl ::std::marker::Sync for LockScreenCallEndCallDeferral {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LockScreenCallEndRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl LockScreenCallEndRequestedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<LockScreenCallEndCallDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LockScreenCallEndCallDeferral>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LockScreenCallEndRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs;{8190a363-6f27-46e9-aeb6-c0ae83e47dc7})");
}
unsafe impl ::windows::runtime::Interface for LockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2173739875, 28455, 18153, [174, 182, 192, 174, 131, 228, 125, 199]);
}
impl ::windows::runtime::RuntimeName for LockScreenCallEndRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs";
}
impl ::std::convert::From<LockScreenCallEndRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LockScreenCallEndRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LockScreenCallEndRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LockScreenCallEndRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LockScreenCallEndRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LockScreenCallEndRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LockScreenCallEndRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LockScreenCallEndRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LockScreenCallEndRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LockScreenCallEndRequestedEventArgs {}
unsafe impl ::std::marker::Sync for LockScreenCallEndRequestedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LockScreenCallUI(pub ::windows::runtime::IInspectable);
impl LockScreenCallUI {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Dismiss(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn EndRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveEndRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn CallTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetCallTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LockScreenCallUI {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallUI;{c596fd8d-73c9-4a14-b021-ec1c50a3b727})");
}
unsafe impl ::windows::runtime::Interface for LockScreenCallUI {
    type Vtable = ILockScreenCallUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3315006861, 29641, 18964, [176, 33, 236, 28, 80, 163, 183, 39]);
}
impl ::windows::runtime::RuntimeName for LockScreenCallUI {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallUI";
}
impl ::std::convert::From<LockScreenCallUI> for ::windows::runtime::IUnknown {
    fn from(value: LockScreenCallUI) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LockScreenCallUI> for ::windows::runtime::IUnknown {
    fn from(value: &LockScreenCallUI) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LockScreenCallUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LockScreenCallUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LockScreenCallUI> for ::windows::runtime::IInspectable {
    fn from(value: LockScreenCallUI) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LockScreenCallUI> for ::windows::runtime::IInspectable {
    fn from(value: &LockScreenCallUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LockScreenCallUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LockScreenCallUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LockScreenCallUI {}
unsafe impl ::std::marker::Sync for LockScreenCallUI {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MuteChangeEventArgs(pub ::windows::runtime::IInspectable);
impl MuteChangeEventArgs {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Muted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MuteChangeEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.MuteChangeEventArgs;{8585e159-0c41-432c-814d-c5f1fdf530be})");
}
unsafe impl ::windows::runtime::Interface for MuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2240143705, 3137, 17196, [129, 77, 197, 241, 253, 245, 48, 190]);
}
impl ::windows::runtime::RuntimeName for MuteChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.MuteChangeEventArgs";
}
impl ::std::convert::From<MuteChangeEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MuteChangeEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&MuteChangeEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MuteChangeEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MuteChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MuteChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<MuteChangeEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MuteChangeEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MuteChangeEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MuteChangeEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MuteChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MuteChangeEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MuteChangeEventArgs {}
unsafe impl ::std::marker::Sync for MuteChangeEventArgs {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(0i32);
    pub const Bluetooth: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(1i32);
    pub const Speakerphone: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(2i32);
}
impl ::std::convert::From<i32> for PhoneAudioRoutingEndpoint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneAudioRoutingEndpoint {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneAudioRoutingEndpoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneAudioRoutingEndpoint;i4)");
}
impl ::windows::runtime::DefaultType for PhoneAudioRoutingEndpoint {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCall(pub ::windows::runtime::IInspectable);
impl PhoneCall {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn AudioDeviceChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveAudioDeviceChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn IsMutedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveIsMutedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn CallId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsMuted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PhoneCallStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn AudioDevice(&self) -> ::windows::runtime::Result<PhoneCallAudioDevice> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallAudioDevice = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallAudioDevice>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetPhoneCallInfo(&self) -> ::windows::runtime::Result<PhoneCallInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn GetPhoneCallInfoAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallInfo>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn End(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn EndAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SendDtmfKey(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), key, dtmftoneaudioplayback, &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn SendDtmfKeyAsync(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), key, dtmftoneaudioplayback, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn AcceptIncoming(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn AcceptIncomingAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Hold(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn HoldAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn ResumeFromHold(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn ResumeFromHoldAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Mute(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn MuteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Unmute(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn UnmuteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RejectIncoming(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RejectIncomingAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn ChangeAudioDevice(&self, endpoint: PhoneCallAudioDevice) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), endpoint, &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn ChangeAudioDeviceAsync(&self, endpoint: PhoneCallAudioDevice) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), endpoint, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetFromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(callid: Param0) -> ::windows::runtime::Result<PhoneCall> {
        Self::IPhoneCallStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), callid.into_param().abi(), &mut result__).from_abi::<PhoneCall>(result__)
        })
    }
    pub fn IPhoneCallStatics<R, F: FnOnce(&IPhoneCallStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCall, IPhoneCallStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCall {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCall;{c14ed0f8-c17d-59d2-9628-66e545b6cd21})");
}
unsafe impl ::windows::runtime::Interface for PhoneCall {
    type Vtable = IPhoneCall_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3243167992, 49533, 22994, [150, 40, 102, 229, 69, 182, 205, 33]);
}
impl ::windows::runtime::RuntimeName for PhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCall";
}
impl ::std::convert::From<PhoneCall> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCall) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCall> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCall) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCall> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCall) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCall> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCall) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCall {}
unsafe impl ::std::marker::Sync for PhoneCall {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: PhoneCallAudioDevice = PhoneCallAudioDevice(0i32);
    pub const LocalDevice: PhoneCallAudioDevice = PhoneCallAudioDevice(1i32);
    pub const RemoteDevice: PhoneCallAudioDevice = PhoneCallAudioDevice(2i32);
}
impl ::std::convert::From<i32> for PhoneCallAudioDevice {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallAudioDevice {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallAudioDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallAudioDevice;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallAudioDevice {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
pub struct PhoneCallBlocking {}
impl PhoneCallBlocking {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn BlockUnknownNumbers() -> ::windows::runtime::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetBlockUnknownNumbers(value: bool) -> ::windows::runtime::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn BlockPrivateNumbers() -> ::windows::runtime::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetBlockPrivateNumbers(value: bool) -> ::windows::runtime::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetCallBlockingListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(phonenumberlist: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), phonenumberlist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IPhoneCallBlockingStatics<R, F: FnOnce(&IPhoneCallBlockingStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallBlocking, IPhoneCallBlockingStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PhoneCallBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallBlocking";
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: PhoneCallDirection = PhoneCallDirection(0i32);
    pub const Incoming: PhoneCallDirection = PhoneCallDirection(1i32);
    pub const Outgoing: PhoneCallDirection = PhoneCallDirection(2i32);
}
impl ::std::convert::From<i32> for PhoneCallDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallDirection {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallDirection;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallHistoryEntry(pub ::windows::runtime::IInspectable);
impl PhoneCallHistoryEntry {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallHistoryEntry, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Address(&self) -> ::windows::runtime::Result<PhoneCallHistoryEntryAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryAddress>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetAddress<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneCallHistoryEntryAddress>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn SetDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsCallerIdBlocked(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsCallerIdBlocked(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsEmergency(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsEmergency(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsIncoming(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsIncoming(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsMissed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsMissed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsRinging(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsRinging(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsSeen(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsSeen(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsSuppressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsSuppressed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsVoicemail(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetIsVoicemail(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Media(&self) -> ::windows::runtime::Result<PhoneCallHistoryEntryMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryMedia = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryMedia>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetMedia(&self, value: PhoneCallHistoryEntryMedia) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::runtime::Result<PhoneCallHistoryEntryOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryOtherAppReadAccess = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetOtherAppReadAccess(&self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SourceDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SourceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetSourceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SourceIdKind(&self) -> ::windows::runtime::Result<PhoneCallHistorySourceIdKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistorySourceIdKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistorySourceIdKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetSourceIdKind(&self, value: PhoneCallHistorySourceIdKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn SetStartTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntry;{fab0e129-32a4-4b85-83d1-f90d8c23a857})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4205895977, 12964, 19333, [131, 209, 249, 13, 140, 35, 168, 87]);
}
impl ::windows::runtime::RuntimeName for PhoneCallHistoryEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntry";
}
impl ::std::convert::From<PhoneCallHistoryEntry> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallHistoryEntry) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntry> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallHistoryEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallHistoryEntry> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallHistoryEntry) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntry> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallHistoryEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallHistoryEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallHistoryEntry {}
unsafe impl ::std::marker::Sync for PhoneCallHistoryEntry {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallHistoryEntryAddress(pub ::windows::runtime::IInspectable);
impl PhoneCallHistoryEntryAddress {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallHistoryEntryAddress, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn ContactId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetContactId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RawAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetRawAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RawAddressKind(&self) -> ::windows::runtime::Result<PhoneCallHistoryEntryRawAddressKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryRawAddressKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryRawAddressKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetRawAddressKind(&self, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(rawaddress: Param0, rawaddresskind: PhoneCallHistoryEntryRawAddressKind) -> ::windows::runtime::Result<PhoneCallHistoryEntryAddress> {
        Self::IPhoneCallHistoryEntryAddressFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), rawaddress.into_param().abi(), rawaddresskind, &mut result__).from_abi::<PhoneCallHistoryEntryAddress>(result__)
        })
    }
    pub fn IPhoneCallHistoryEntryAddressFactory<R, F: FnOnce(&IPhoneCallHistoryEntryAddressFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallHistoryEntryAddress, IPhoneCallHistoryEntryAddressFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntryAddress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress;{30f159da-3955-4042-84e6-66eebf82e67f})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(821123546, 14677, 16450, [132, 230, 102, 238, 191, 130, 230, 127]);
}
impl ::windows::runtime::RuntimeName for PhoneCallHistoryEntryAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress";
}
impl ::std::convert::From<PhoneCallHistoryEntryAddress> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallHistoryEntryAddress) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntryAddress> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallHistoryEntryAddress) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallHistoryEntryAddress> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallHistoryEntryAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntryAddress> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallHistoryEntryAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallHistoryEntryAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallHistoryEntryAddress {}
unsafe impl ::std::marker::Sync for PhoneCallHistoryEntryAddress {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: PhoneCallHistoryEntryMedia = PhoneCallHistoryEntryMedia(0i32);
    pub const Video: PhoneCallHistoryEntryMedia = PhoneCallHistoryEntryMedia(1i32);
}
impl ::std::convert::From<i32> for PhoneCallHistoryEntryMedia {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallHistoryEntryMedia {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntryMedia {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryMedia;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallHistoryEntryMedia {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: PhoneCallHistoryEntryOtherAppReadAccess = PhoneCallHistoryEntryOtherAppReadAccess(0i32);
    pub const SystemOnly: PhoneCallHistoryEntryOtherAppReadAccess = PhoneCallHistoryEntryOtherAppReadAccess(1i32);
}
impl ::std::convert::From<i32> for PhoneCallHistoryEntryOtherAppReadAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallHistoryEntryOtherAppReadAccess {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntryOtherAppReadAccess {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryOtherAppReadAccess;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallHistoryEntryOtherAppReadAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(0u32);
    pub const Audio: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(1u32);
    pub const Video: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(2u32);
    pub const All: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(4294967295u32);
}
impl ::std::convert::From<u32> for PhoneCallHistoryEntryQueryDesiredMedia {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallHistoryEntryQueryDesiredMedia {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntryQueryDesiredMedia {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryDesiredMedia;u4)");
}
impl ::windows::runtime::DefaultType for PhoneCallHistoryEntryQueryDesiredMedia {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallHistoryEntryQueryOptions(pub ::windows::runtime::IInspectable);
impl PhoneCallHistoryEntryQueryOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallHistoryEntryQueryOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DesiredMedia(&self) -> ::windows::runtime::Result<PhoneCallHistoryEntryQueryDesiredMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallHistoryEntryQueryDesiredMedia = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryQueryDesiredMedia>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetDesiredMedia(&self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation_Collections`*"]
    pub fn SourceIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntryQueryOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions;{9c5fe15c-8bed-40ca-b06e-c4ca8eae5c87})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2623529308, 35821, 16586, [176, 110, 196, 202, 142, 174, 92, 135]);
}
impl ::windows::runtime::RuntimeName for PhoneCallHistoryEntryQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions";
}
impl ::std::convert::From<PhoneCallHistoryEntryQueryOptions> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntryQueryOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallHistoryEntryQueryOptions> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntryQueryOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallHistoryEntryQueryOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallHistoryEntryQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallHistoryEntryQueryOptions {}
unsafe impl ::std::marker::Sync for PhoneCallHistoryEntryQueryOptions {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: PhoneCallHistoryEntryRawAddressKind = PhoneCallHistoryEntryRawAddressKind(0i32);
    pub const Custom: PhoneCallHistoryEntryRawAddressKind = PhoneCallHistoryEntryRawAddressKind(1i32);
}
impl ::std::convert::From<i32> for PhoneCallHistoryEntryRawAddressKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallHistoryEntryRawAddressKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntryRawAddressKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryRawAddressKind;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallHistoryEntryRawAddressKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallHistoryEntryReader(pub ::windows::runtime::IInspectable);
impl PhoneCallHistoryEntryReader {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryEntryReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader;{61ece4be-8d86-479f-8404-a9846920fee6})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1642915006, 36230, 18335, [132, 4, 169, 132, 105, 32, 254, 230]);
}
impl ::windows::runtime::RuntimeName for PhoneCallHistoryEntryReader {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader";
}
impl ::std::convert::From<PhoneCallHistoryEntryReader> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallHistoryEntryReader) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntryReader> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallHistoryEntryReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallHistoryEntryReader> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallHistoryEntryReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallHistoryEntryReader> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallHistoryEntryReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallHistoryEntryReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallHistoryEntryReader {}
unsafe impl ::std::marker::Sync for PhoneCallHistoryEntryReader {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
pub struct PhoneCallHistoryManager {}
impl PhoneCallHistoryManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RequestStoreAsync(accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        Self::IPhoneCallHistoryManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<PhoneCallHistoryManagerForUser> {
        Self::IPhoneCallHistoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<PhoneCallHistoryManagerForUser>(result__)
        })
    }
    pub fn IPhoneCallHistoryManagerStatics<R, F: FnOnce(&IPhoneCallHistoryManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPhoneCallHistoryManagerStatics2<R, F: FnOnce(&IPhoneCallHistoryManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PhoneCallHistoryManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManager";
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallHistoryManagerForUser(pub ::windows::runtime::IInspectable);
impl PhoneCallHistoryManagerForUser {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RequestStoreAsync(&self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryManagerForUser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser;{d925c523-f55f-4353-9db4-0205a5265a55})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3643131171, 62815, 17235, [157, 180, 2, 5, 165, 38, 90, 85]);
}
impl ::windows::runtime::RuntimeName for PhoneCallHistoryManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser";
}
impl ::std::convert::From<PhoneCallHistoryManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallHistoryManagerForUser) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallHistoryManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallHistoryManagerForUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallHistoryManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallHistoryManagerForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallHistoryManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallHistoryManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallHistoryManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallHistoryManagerForUser {}
unsafe impl ::std::marker::Sync for PhoneCallHistoryManagerForUser {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: PhoneCallHistorySourceIdKind = PhoneCallHistorySourceIdKind(0i32);
    pub const PackageFamilyName: PhoneCallHistorySourceIdKind = PhoneCallHistorySourceIdKind(1i32);
}
impl ::std::convert::From<i32> for PhoneCallHistorySourceIdKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallHistorySourceIdKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistorySourceIdKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistorySourceIdKind;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallHistorySourceIdKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallHistoryStore(pub ::windows::runtime::IInspectable);
impl PhoneCallHistoryStore {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn GetEntryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, callhistoryentryid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), callhistoryentryid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetEntryReader(&self) -> ::windows::runtime::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallHistoryEntryReader>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetEntryReaderWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneCallHistoryEntryQueryOptions>>(&self, queryoptions: Param0) -> ::windows::runtime::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<PhoneCallHistoryEntryReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn SaveEntryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn DeleteEntryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`, `Foundation_Collections`*"]
    pub fn DeleteEntriesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>>(&self, callhistoryentries: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), callhistoryentries.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn MarkEntryAsSeenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneCallHistoryEntry>>(&self, callhistoryentry: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), callhistoryentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`, `Foundation_Collections`*"]
    pub fn MarkEntriesAsSeenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>>(&self, callhistoryentries: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), callhistoryentries.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn GetUnseenCountAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn MarkAllAsSeenAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSourcesUnseenCountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, sourceids: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), sourceids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`, `Foundation_Collections`*"]
    pub fn MarkSourcesAsSeenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, sourceids: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), sourceids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryStore;{2f907db8-b40e-422b-8545-cb1910a61c52})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(797998520, 46094, 16939, [133, 69, 203, 25, 16, 166, 28, 82]);
}
impl ::windows::runtime::RuntimeName for PhoneCallHistoryStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryStore";
}
impl ::std::convert::From<PhoneCallHistoryStore> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallHistoryStore) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallHistoryStore> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallHistoryStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallHistoryStore> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallHistoryStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallHistoryStore> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallHistoryStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallHistoryStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallHistoryStore {}
unsafe impl ::std::marker::Sync for PhoneCallHistoryStore {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(0i32);
    pub const AllEntriesLimitedReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(1i32);
    pub const AllEntriesReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(2i32);
}
impl ::std::convert::From<i32> for PhoneCallHistoryStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallHistoryStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallHistoryStoreAccessType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryStoreAccessType;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallHistoryStoreAccessType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallInfo(pub ::windows::runtime::IInspectable);
impl PhoneCallInfo {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn LineId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsHoldSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn PhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn CallDirection(&self) -> ::windows::runtime::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallDirection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallDirection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallInfo;{22b42577-3e4d-5dc6-89c2-469fe5ffc189})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallInfo {
    type Vtable = IPhoneCallInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582231415, 15949, 24006, [137, 194, 70, 159, 229, 255, 193, 137]);
}
impl ::windows::runtime::RuntimeName for PhoneCallInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallInfo";
}
impl ::std::convert::From<PhoneCallInfo> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallInfo> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallInfo {}
unsafe impl ::std::marker::Sync for PhoneCallInfo {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
pub struct PhoneCallManager {}
impl PhoneCallManager {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn ShowPhoneCallUI<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phonenumber: Param0, displayname: Param1) -> ::windows::runtime::Result<()> {
        Self::IPhoneCallManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), phonenumber.into_param().abi(), displayname.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn CallStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveCallStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsCallActive() -> ::windows::runtime::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsCallIncoming() -> ::windows::runtime::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn ShowPhoneCallSettingsUI() -> ::windows::runtime::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RequestStoreAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallStore>> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallStore>>(result__)
        })
    }
    pub fn IPhoneCallManagerStatics<R, F: FnOnce(&IPhoneCallManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPhoneCallManagerStatics2<R, F: FnOnce(&IPhoneCallManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PhoneCallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallManager";
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: PhoneCallMedia = PhoneCallMedia(0i32);
    pub const AudioAndVideo: PhoneCallMedia = PhoneCallMedia(1i32);
    pub const AudioAndRealTimeText: PhoneCallMedia = PhoneCallMedia(2i32);
}
impl ::std::convert::From<i32> for PhoneCallMedia {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallMedia {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallMedia {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallMedia;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallMedia {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: PhoneCallOperationStatus = PhoneCallOperationStatus(0i32);
    pub const OtherFailure: PhoneCallOperationStatus = PhoneCallOperationStatus(1i32);
    pub const TimedOut: PhoneCallOperationStatus = PhoneCallOperationStatus(2i32);
    pub const ConnectionLost: PhoneCallOperationStatus = PhoneCallOperationStatus(3i32);
    pub const InvalidCallState: PhoneCallOperationStatus = PhoneCallOperationStatus(4i32);
}
impl ::std::convert::From<i32> for PhoneCallOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallOperationStatus;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallOperationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PhoneCallStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallStatus;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallStore(pub ::windows::runtime::IInspectable);
impl PhoneCallStore {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn IsEmergencyPhoneNumberAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn GetDefaultLineAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::GUID>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::GUID>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RequestLineWatcher(&self) -> ::windows::runtime::Result<PhoneLineWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineWatcher>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallStore;{5f610748-18a6-4173-86d1-28be9dc62dba})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallStore {
    type Vtable = IPhoneCallStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1600194376, 6310, 16755, [134, 209, 40, 190, 157, 198, 45, 186]);
}
impl ::windows::runtime::RuntimeName for PhoneCallStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallStore";
}
impl ::std::convert::From<PhoneCallStore> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallStore) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallStore> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallStore> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallStore> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallStore {}
unsafe impl ::std::marker::Sync for PhoneCallStore {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallVideoCapabilities(pub ::windows::runtime::IInspectable);
impl PhoneCallVideoCapabilities {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsVideoCallingCapable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallVideoCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities;{02382786-b16a-4fdb-be3b-c4240e13ad0d})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37234566, 45418, 20443, [190, 59, 196, 36, 14, 19, 173, 13]);
}
impl ::windows::runtime::RuntimeName for PhoneCallVideoCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities";
}
impl ::std::convert::From<PhoneCallVideoCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallVideoCapabilities) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallVideoCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallVideoCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallVideoCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallVideoCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallVideoCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallVideoCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallVideoCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallVideoCapabilities {}
unsafe impl ::std::marker::Sync for PhoneCallVideoCapabilities {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
pub struct PhoneCallVideoCapabilitiesManager {}
impl PhoneCallVideoCapabilitiesManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn GetCapabilitiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phonenumber: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>> {
        Self::IPhoneCallVideoCapabilitiesManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), phonenumber.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>>(result__)
        })
    }
    pub fn IPhoneCallVideoCapabilitiesManagerStatics<R, F: FnOnce(&IPhoneCallVideoCapabilitiesManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneCallVideoCapabilitiesManager, IPhoneCallVideoCapabilitiesManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PhoneCallVideoCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager";
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallsResult(pub ::windows::runtime::IInspectable);
impl PhoneCallsResult {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn OperationStatus(&self) -> ::windows::runtime::Result<PhoneLineOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation_Collections`*"]
    pub fn AllActivePhoneCalls(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PhoneCall>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhoneCall>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallsResult;{1bfad365-57cf-57dd-986d-b057c91eac33})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallsResult {
    type Vtable = IPhoneCallsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(469422949, 22479, 22493, [152, 109, 176, 87, 201, 30, 172, 51]);
}
impl ::windows::runtime::RuntimeName for PhoneCallsResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallsResult";
}
impl ::std::convert::From<PhoneCallsResult> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallsResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneCallsResult> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneCallsResult> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallsResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallsResult> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneCallsResult {}
unsafe impl ::std::marker::Sync for PhoneCallsResult {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneDialOptions(pub ::windows::runtime::IInspectable);
impl PhoneDialOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneDialOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Number(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetNumber<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `ApplicationModel_Contacts`*"]
    pub fn SetContact<'a, Param0: ::windows::runtime::IntoParam<'a, super::Contacts::Contact>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `ApplicationModel_Contacts`*"]
    pub fn ContactPhone(&self) -> ::windows::runtime::Result<super::Contacts::ContactPhone> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPhone>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `ApplicationModel_Contacts`*"]
    pub fn SetContactPhone<'a, Param0: ::windows::runtime::IntoParam<'a, super::Contacts::ContactPhone>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Media(&self) -> ::windows::runtime::Result<PhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallMedia = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallMedia>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetMedia(&self, value: PhoneCallMedia) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn AudioEndpoint(&self) -> ::windows::runtime::Result<PhoneAudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__: PhoneAudioRoutingEndpoint = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneAudioRoutingEndpoint>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetAudioEndpoint(&self, value: PhoneAudioRoutingEndpoint) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneDialOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneDialOptions;{b639c4b8-f06f-36cb-a863-823742b5f2d4})");
}
unsafe impl ::windows::runtime::Interface for PhoneDialOptions {
    type Vtable = IPhoneDialOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3057239224, 61551, 14027, [168, 99, 130, 55, 66, 181, 242, 212]);
}
impl ::windows::runtime::RuntimeName for PhoneDialOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneDialOptions";
}
impl ::std::convert::From<PhoneDialOptions> for ::windows::runtime::IUnknown {
    fn from(value: PhoneDialOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneDialOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneDialOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneDialOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneDialOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneDialOptions> for ::windows::runtime::IInspectable {
    fn from(value: PhoneDialOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneDialOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneDialOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneDialOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneDialOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneDialOptions {}
unsafe impl ::std::marker::Sync for PhoneDialOptions {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLine(pub ::windows::runtime::IInspectable);
impl PhoneLine {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn LineChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLine, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveLineChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `UI`*"]
    pub fn DisplayColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NetworkState(&self) -> ::windows::runtime::Result<PhoneNetworkState> {
        let this = self;
        unsafe {
            let mut result__: PhoneNetworkState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneNetworkState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Voicemail(&self) -> ::windows::runtime::Result<PhoneVoicemail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneVoicemail>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NetworkName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn CellularDetails(&self) -> ::windows::runtime::Result<PhoneLineCellularDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineCellularDetails>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Transport(&self) -> ::windows::runtime::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineTransport = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineTransport>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn CanDial(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SupportsTile(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn VideoCallingCapabilities(&self) -> ::windows::runtime::Result<PhoneCallVideoCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallVideoCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn LineConfiguration(&self) -> ::windows::runtime::Result<PhoneLineConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineConfiguration>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn IsImmediateDialNumberAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Dial<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DialWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneDialOptions>>(&self, options: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), options.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(lineid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneLine>> {
        Self::IPhoneLineStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), lineid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneLine>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn EnableTextReply(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLine2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn TransportDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLine2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DialWithResult<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::runtime::Result<PhoneLineDialResult> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<PhoneLineDialResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn DialWithResultAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0, displayname: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), number.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetAllActivePhoneCalls(&self) -> ::windows::runtime::Result<PhoneCallsResult> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallsResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn GetAllActivePhoneCallsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PhoneCallsResult>> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhoneCallsResult>>(result__)
        }
    }
    pub fn IPhoneLineStatics<R, F: FnOnce(&IPhoneLineStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneLine, IPhoneLineStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLine {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLine;{27c66f30-6a69-34ca-a2ba-65302530c311})");
}
unsafe impl ::windows::runtime::Interface for PhoneLine {
    type Vtable = IPhoneLine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(667316016, 27241, 13514, [162, 186, 101, 48, 37, 48, 195, 17]);
}
impl ::windows::runtime::RuntimeName for PhoneLine {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLine";
}
impl ::std::convert::From<PhoneLine> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLine) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneLine> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLine) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneLine> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLine) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLine> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneLine {}
unsafe impl ::std::marker::Sync for PhoneLine {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLineCellularDetails(pub ::windows::runtime::IInspectable);
impl PhoneLineCellularDetails {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SimState(&self) -> ::windows::runtime::Result<PhoneSimState> {
        let this = self;
        unsafe {
            let mut result__: PhoneSimState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneSimState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SimSlotIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsModemOn(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RegistrationRejectCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetNetworkOperatorDisplayText(&self, location: PhoneLineNetworkOperatorDisplayTextLocation) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), location, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineCellularDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineCellularDetails;{192601d5-147c-4769-b673-98a5ec8426cb})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(421921237, 5244, 18281, [182, 115, 152, 165, 236, 132, 38, 203]);
}
impl ::windows::runtime::RuntimeName for PhoneLineCellularDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineCellularDetails";
}
impl ::std::convert::From<PhoneLineCellularDetails> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineCellularDetails) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneLineCellularDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineCellularDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneLineCellularDetails> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineCellularDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLineCellularDetails> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLineCellularDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLineCellularDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneLineCellularDetails {}
unsafe impl ::std::marker::Sync for PhoneLineCellularDetails {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLineConfiguration(pub ::windows::runtime::IInspectable);
impl PhoneLineConfiguration {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsVideoCallingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation_Collections`*"]
    pub fn ExtendedProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineConfiguration;{fe265862-f64f-4312-b2a8-4e257721aa95})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4263925858, 63055, 17170, [178, 168, 78, 37, 119, 33, 170, 149]);
}
impl ::windows::runtime::RuntimeName for PhoneLineConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineConfiguration";
}
impl ::std::convert::From<PhoneLineConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineConfiguration) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneLineConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLineConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneLineConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLineConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLineConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLineConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLineConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneLineConfiguration {}
unsafe impl ::std::marker::Sync for PhoneLineConfiguration {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLineDialResult(pub ::windows::runtime::IInspectable);
impl PhoneLineDialResult {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DialCallStatus(&self) -> ::windows::runtime::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallOperationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DialedCall(&self) -> ::windows::runtime::Result<PhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCall>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineDialResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineDialResult;{e825a30a-5c7f-546f-b918-3ad2fe70fb34})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3894780682, 23679, 21615, [185, 24, 58, 210, 254, 112, 251, 52]);
}
impl ::windows::runtime::RuntimeName for PhoneLineDialResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineDialResult";
}
impl ::std::convert::From<PhoneLineDialResult> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineDialResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneLineDialResult> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineDialResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineDialResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLineDialResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneLineDialResult> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineDialResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLineDialResult> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLineDialResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLineDialResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLineDialResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneLineDialResult {}
unsafe impl ::std::marker::Sync for PhoneLineDialResult {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(0i32);
    pub const Tile: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(1i32);
    pub const Dialer: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(2i32);
    pub const InCallUI: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(3i32);
}
impl ::std::convert::From<i32> for PhoneLineNetworkOperatorDisplayTextLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneLineNetworkOperatorDisplayTextLocation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineNetworkOperatorDisplayTextLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineNetworkOperatorDisplayTextLocation;i4)");
}
impl ::windows::runtime::DefaultType for PhoneLineNetworkOperatorDisplayTextLocation {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: PhoneLineOperationStatus = PhoneLineOperationStatus(0i32);
    pub const OtherFailure: PhoneLineOperationStatus = PhoneLineOperationStatus(1i32);
    pub const TimedOut: PhoneLineOperationStatus = PhoneLineOperationStatus(2i32);
    pub const ConnectionLost: PhoneLineOperationStatus = PhoneLineOperationStatus(3i32);
    pub const InvalidCallState: PhoneLineOperationStatus = PhoneLineOperationStatus(4i32);
}
impl ::std::convert::From<i32> for PhoneLineOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneLineOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineOperationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineOperationStatus;i4)");
}
impl ::windows::runtime::DefaultType for PhoneLineOperationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: PhoneLineTransport = PhoneLineTransport(0i32);
    pub const VoipApp: PhoneLineTransport = PhoneLineTransport(1i32);
    pub const Bluetooth: PhoneLineTransport = PhoneLineTransport(2i32);
}
impl ::std::convert::From<i32> for PhoneLineTransport {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneLineTransport {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineTransport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineTransport;i4)");
}
impl ::windows::runtime::DefaultType for PhoneLineTransport {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLineTransportDevice(pub ::windows::runtime::IInspectable);
impl PhoneLineTransportDevice {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Transport(&self) -> ::windows::runtime::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineTransport = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineTransport>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Devices_Enumeration`, `Foundation`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RegisterApp(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `System`*"]
    pub fn RegisterAppForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), user.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn UnregisterApp(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `System`*"]
    pub fn UnregisterAppForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), user.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn IsRegistered(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Connect(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn ConnectAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn FromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(id: Param0) -> ::windows::runtime::Result<PhoneLineTransportDevice> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<PhoneLineTransportDevice>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetDeviceSelectorForPhoneLineTransport(transport: PhoneLineTransport) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), transport, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn AudioRoutingStatus(&self) -> ::windows::runtime::Result<TransportDeviceAudioRoutingStatus> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: TransportDeviceAudioRoutingStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TransportDeviceAudioRoutingStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn AudioRoutingStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveAudioRoutingStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn InBandRingingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn InBandRingingEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveInBandRingingEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IPhoneLineTransportDeviceStatics<R, F: FnOnce(&IPhoneLineTransportDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneLineTransportDevice, IPhoneLineTransportDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineTransportDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineTransportDevice;{efa8f889-cffa-59f4-97e4-74705b7dc490})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020828297, 53242, 23028, [151, 228, 116, 112, 91, 125, 196, 144]);
}
impl ::windows::runtime::RuntimeName for PhoneLineTransportDevice {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineTransportDevice";
}
impl ::std::convert::From<PhoneLineTransportDevice> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineTransportDevice) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneLineTransportDevice> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineTransportDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneLineTransportDevice> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineTransportDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLineTransportDevice> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLineTransportDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLineTransportDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneLineTransportDevice {}
unsafe impl ::std::marker::Sync for PhoneLineTransportDevice {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLineWatcher(pub ::windows::runtime::IInspectable);
impl PhoneLineWatcher {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn LineAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveLineAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn LineRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveLineRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn LineUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveLineUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PhoneLineWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineWatcherStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcher;{8a45cd0a-6323-44e0-a6f6-9f21f64dc90a})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2319830282, 25379, 17632, [166, 246, 159, 33, 246, 77, 201, 10]);
}
impl ::windows::runtime::RuntimeName for PhoneLineWatcher {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcher";
}
impl ::std::convert::From<PhoneLineWatcher> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineWatcher) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneLineWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLineWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneLineWatcher> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLineWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLineWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLineWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLineWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneLineWatcher {}
unsafe impl ::std::marker::Sync for PhoneLineWatcher {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLineWatcherEventArgs(pub ::windows::runtime::IInspectable);
impl PhoneLineWatcherEventArgs {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn LineId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineWatcherEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs;{d07c753e-9e12-4a37-82b7-ad535dad6a67})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3497817406, 40466, 18999, [130, 183, 173, 83, 93, 173, 106, 103]);
}
impl ::windows::runtime::RuntimeName for PhoneLineWatcherEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs";
}
impl ::std::convert::From<PhoneLineWatcherEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineWatcherEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneLineWatcherEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineWatcherEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneLineWatcherEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineWatcherEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLineWatcherEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLineWatcherEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLineWatcherEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneLineWatcherEventArgs {}
unsafe impl ::std::marker::Sync for PhoneLineWatcherEventArgs {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: PhoneLineWatcherStatus = PhoneLineWatcherStatus(0i32);
    pub const Started: PhoneLineWatcherStatus = PhoneLineWatcherStatus(1i32);
    pub const EnumerationCompleted: PhoneLineWatcherStatus = PhoneLineWatcherStatus(2i32);
    pub const Stopped: PhoneLineWatcherStatus = PhoneLineWatcherStatus(3i32);
}
impl ::std::convert::From<i32> for PhoneLineWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneLineWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for PhoneLineWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PhoneNetworkState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneNetworkState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNetworkState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneNetworkState;i4)");
}
impl ::windows::runtime::DefaultType for PhoneNetworkState {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PhoneSimState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneSimState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneSimState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneSimState;i4)");
}
impl ::windows::runtime::DefaultType for PhoneSimState {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneVoicemail(pub ::windows::runtime::IInspectable);
impl PhoneVoicemail {
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Number(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn MessageCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PhoneVoicemailType> {
        let this = self;
        unsafe {
            let mut result__: PhoneVoicemailType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneVoicemailType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn DialVoicemailAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneVoicemail {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneVoicemail;{c9ce77f6-6e9f-3a8b-b727-6e0cf6998224})");
}
unsafe impl ::windows::runtime::Interface for PhoneVoicemail {
    type Vtable = IPhoneVoicemail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3385751542, 28319, 14987, [183, 39, 110, 12, 246, 153, 130, 36]);
}
impl ::windows::runtime::RuntimeName for PhoneVoicemail {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneVoicemail";
}
impl ::std::convert::From<PhoneVoicemail> for ::windows::runtime::IUnknown {
    fn from(value: PhoneVoicemail) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PhoneVoicemail> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneVoicemail) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneVoicemail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneVoicemail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PhoneVoicemail> for ::windows::runtime::IInspectable {
    fn from(value: PhoneVoicemail) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneVoicemail> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneVoicemail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneVoicemail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneVoicemail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PhoneVoicemail {}
unsafe impl ::std::marker::Sync for PhoneVoicemail {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: PhoneVoicemailType = PhoneVoicemailType(0i32);
    pub const Traditional: PhoneVoicemailType = PhoneVoicemailType(1i32);
    pub const Visual: PhoneVoicemailType = PhoneVoicemailType(2i32);
}
impl ::std::convert::From<i32> for PhoneVoicemailType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneVoicemailType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneVoicemailType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneVoicemailType;i4)");
}
impl ::windows::runtime::DefaultType for PhoneVoicemailType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(0i32);
    pub const CanRouteToLocalDevice: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(1i32);
    pub const CannotRouteToLocalDevice: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(2i32);
}
impl ::std::convert::From<i32> for TransportDeviceAudioRoutingStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TransportDeviceAudioRoutingStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TransportDeviceAudioRoutingStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.TransportDeviceAudioRoutingStatus;i4)");
}
impl ::windows::runtime::DefaultType for TransportDeviceAudioRoutingStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VoipCallCoordinator(pub ::windows::runtime::IInspectable);
impl VoipCallCoordinator {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn ReserveCallResourcesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, taskentrypoint: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), taskentrypoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn MuteStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs>>>(&self, mutechangehandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mutechangehandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveMuteStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RequestNewIncomingCall<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
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
    ) -> ::windows::runtime::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
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
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RequestNewOutgoingCall<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, context: Param0, contactname: Param1, servicename: Param2, media: VoipPhoneCallMedia) -> ::windows::runtime::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NotifyMuted(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NotifyUnmuted(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RequestOutgoingUpgradeToVideoCall<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, callupgradeguid: Param0, context: Param1, contactname: Param2, servicename: Param3) -> ::windows::runtime::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), callupgradeguid.into_param().abi(), context.into_param().abi(), contactname.into_param().abi(), servicename.into_param().abi(), &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RequestIncomingUpgradeToVideoCall<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
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
    ) -> ::windows::runtime::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
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
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn TerminateCellularCall<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, callupgradeguid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), callupgradeguid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn CancelUpgrade<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, callupgradeguid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), callupgradeguid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<VoipCallCoordinator> {
        Self::IVoipCallCoordinatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VoipCallCoordinator>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetupNewAcceptedCall<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, context: Param0, contactname: Param1, contactnumber: Param2, servicename: Param3, media: VoipPhoneCallMedia) -> ::windows::runtime::Result<VoipPhoneCall> {
        let this = &::windows::runtime::Interface::cast::<IVoipCallCoordinator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn RequestNewAppInitiatedCall<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, context: Param0, contactname: Param1, contactnumber: Param2, servicename: Param3, media: VoipPhoneCallMedia) -> ::windows::runtime::Result<VoipPhoneCall> {
        let this = &::windows::runtime::Interface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), context.into_param().abi(), contactname.into_param().abi(), contactnumber.into_param().abi(), servicename.into_param().abi(), media, &mut result__).from_abi::<VoipPhoneCall>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RequestNewIncomingCallWithContactRemoteId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
        Param10: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
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
    ) -> ::windows::runtime::Result<VoipPhoneCall> {
        let this = &::windows::runtime::Interface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
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
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn ReserveOneProcessCallResourcesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = &::windows::runtime::Interface::cast::<IVoipCallCoordinator4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>(result__)
        }
    }
    pub fn IVoipCallCoordinatorStatics<R, F: FnOnce(&IVoipCallCoordinatorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VoipCallCoordinator, IVoipCallCoordinatorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoipCallCoordinator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipCallCoordinator;{4f118bcf-e8ef-4434-9c5f-a8d893fafe79})");
}
unsafe impl ::windows::runtime::Interface for VoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1326549967, 59631, 17460, [156, 95, 168, 216, 147, 250, 254, 121]);
}
impl ::windows::runtime::RuntimeName for VoipCallCoordinator {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipCallCoordinator";
}
impl ::std::convert::From<VoipCallCoordinator> for ::windows::runtime::IUnknown {
    fn from(value: VoipCallCoordinator) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&VoipCallCoordinator> for ::windows::runtime::IUnknown {
    fn from(value: &VoipCallCoordinator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VoipCallCoordinator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VoipCallCoordinator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<VoipCallCoordinator> for ::windows::runtime::IInspectable {
    fn from(value: VoipCallCoordinator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoipCallCoordinator> for ::windows::runtime::IInspectable {
    fn from(value: &VoipCallCoordinator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VoipCallCoordinator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VoipCallCoordinator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoipCallCoordinator {}
unsafe impl ::std::marker::Sync for VoipCallCoordinator {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VoipPhoneCall(pub ::windows::runtime::IInspectable);
impl VoipPhoneCall {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn EndRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveEndRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn HoldRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveHoldRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn ResumeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveResumeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn AnswerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs>>>(&self, accepthandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), accepthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveAnswerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RejectRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs>>>(&self, rejecthandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), rejecthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn RemoveRejectRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NotifyCallHeld(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NotifyCallActive(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NotifyCallEnded(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn ContactName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetContactName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Calls`, `Foundation`*"]
    pub fn SetStartTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn CallMedia(&self) -> ::windows::runtime::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__: VoipPhoneCallMedia = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VoipPhoneCallMedia>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn SetCallMedia(&self, value: VoipPhoneCallMedia) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NotifyCallReady(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn TryShowAppUI(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVoipPhoneCall2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Calls`*"]
    pub fn NotifyCallAccepted(&self, media: VoipPhoneCallMedia) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVoipPhoneCall3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), media).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoipPhoneCall {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipPhoneCall;{6cf1f19a-7794-4a5a-8c68-ae87947a6990})");
}
unsafe impl ::windows::runtime::Interface for VoipPhoneCall {
    type Vtable = IVoipPhoneCall_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1827795354, 30612, 19034, [140, 104, 174, 135, 148, 122, 105, 144]);
}
impl ::windows::runtime::RuntimeName for VoipPhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipPhoneCall";
}
impl ::std::convert::From<VoipPhoneCall> for ::windows::runtime::IUnknown {
    fn from(value: VoipPhoneCall) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&VoipPhoneCall> for ::windows::runtime::IUnknown {
    fn from(value: &VoipPhoneCall) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VoipPhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VoipPhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<VoipPhoneCall> for ::windows::runtime::IInspectable {
    fn from(value: VoipPhoneCall) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoipPhoneCall> for ::windows::runtime::IInspectable {
    fn from(value: &VoipPhoneCall) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VoipPhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VoipPhoneCall {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoipPhoneCall {}
unsafe impl ::std::marker::Sync for VoipPhoneCall {}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: VoipPhoneCallMedia = VoipPhoneCallMedia(0u32);
    pub const Audio: VoipPhoneCallMedia = VoipPhoneCallMedia(1u32);
    pub const Video: VoipPhoneCallMedia = VoipPhoneCallMedia(2u32);
}
impl ::std::convert::From<u32> for VoipPhoneCallMedia {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VoipPhoneCallMedia {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VoipPhoneCallMedia {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallMedia;u4)");
}
impl ::windows::runtime::DefaultType for VoipPhoneCallMedia {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VoipPhoneCallMedia {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VoipPhoneCallMedia {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VoipPhoneCallMedia {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VoipPhoneCallMedia {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VoipPhoneCallMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(0i32);
    pub const TimedOut: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(1i32);
    pub const OtherIncomingCall: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(2i32);
    pub const EmergencyCallExists: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(3i32);
    pub const InvalidCallState: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(4i32);
}
impl ::std::convert::From<i32> for VoipPhoneCallRejectReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VoipPhoneCallRejectReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VoipPhoneCallRejectReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallRejectReason;i4)");
}
impl ::windows::runtime::DefaultType for VoipPhoneCallRejectReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: VoipPhoneCallResourceReservationStatus = VoipPhoneCallResourceReservationStatus(0i32);
    pub const ResourcesNotAvailable: VoipPhoneCallResourceReservationStatus = VoipPhoneCallResourceReservationStatus(1i32);
}
impl ::std::convert::From<i32> for VoipPhoneCallResourceReservationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VoipPhoneCallResourceReservationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VoipPhoneCallResourceReservationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus;i4)");
}
impl ::windows::runtime::DefaultType for VoipPhoneCallResourceReservationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Calls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: VoipPhoneCallState = VoipPhoneCallState(0i32);
    pub const Held: VoipPhoneCallState = VoipPhoneCallState(1i32);
    pub const Active: VoipPhoneCallState = VoipPhoneCallState(2i32);
    pub const Incoming: VoipPhoneCallState = VoipPhoneCallState(3i32);
    pub const Outgoing: VoipPhoneCallState = VoipPhoneCallState(4i32);
}
impl ::std::convert::From<i32> for VoipPhoneCallState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VoipPhoneCallState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VoipPhoneCallState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallState;i4)");
}
impl ::windows::runtime::DefaultType for VoipPhoneCallState {
    type DefaultType = Self;
}
