#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct CallsBackgroundContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallBlockedTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a690a2_e4c1_427f_864e_e470477ddb67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockedTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneCallBlockedReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallOriginDataRequestTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginDataRequestTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneIncomingCallDismissedTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad30276_83b6_5732_9c38_0c206546196a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallDismissedTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneIncomingCallDismissedReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneIncomingCallNotificationTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineChangedTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineChangedTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhoneLineChangeKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lineproperty: PhoneLineProperties, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneNewVoicemailMessageTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNewVoicemailMessageTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallBlockedReason(pub i32);
impl PhoneCallBlockedReason {
    pub const InCallBlockingList: PhoneCallBlockedReason = PhoneCallBlockedReason(0i32);
    pub const PrivateNumber: PhoneCallBlockedReason = PhoneCallBlockedReason(1i32);
    pub const UnknownNumber: PhoneCallBlockedReason = PhoneCallBlockedReason(2i32);
}
impl ::core::convert::From<i32> for PhoneCallBlockedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallBlockedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneCallBlockedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason;i4)");
}
impl ::windows::core::DefaultType for PhoneCallBlockedReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallBlockedTriggerDetails(pub ::windows::core::IInspectable);
impl PhoneCallBlockedTriggerDetails {
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn CallBlockedReason(&self) -> ::windows::core::Result<PhoneCallBlockedReason> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallBlockedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallBlockedReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallBlockedTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails;{a4a690a2-e4c1-427f-864e-e470477ddb67})");
}
unsafe impl ::windows::core::Interface for PhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a690a2_e4c1_427f_864e_e470477ddb67);
}
impl ::windows::core::RuntimeName for PhoneCallBlockedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
}
impl ::core::convert::From<PhoneCallBlockedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneCallBlockedTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallBlockedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallBlockedTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallBlockedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneCallBlockedTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallBlockedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallBlockedTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallBlockedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallBlockedTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallOriginDataRequestTriggerDetails(pub ::windows::core::IInspectable);
impl PhoneCallOriginDataRequestTriggerDetails {
    pub fn RequestId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallOriginDataRequestTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails;{6e9b5b3f-c54b-4e82-4cc9-e329a4184592})");
}
unsafe impl ::windows::core::Interface for PhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
}
impl ::windows::core::RuntimeName for PhoneCallOriginDataRequestTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
}
impl ::core::convert::From<PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneCallOriginDataRequestTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallOriginDataRequestTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneCallOriginDataRequestTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallOriginDataRequestTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneCallOriginDataRequestTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallOriginDataRequestTriggerDetails {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(0i32);
    pub const CallRejected: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(1i32);
    pub const TextReply: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(2i32);
    pub const ConnectionLost: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(3i32);
}
impl ::core::convert::From<i32> for PhoneIncomingCallDismissedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneIncomingCallDismissedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneIncomingCallDismissedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason;i4)");
}
impl ::windows::core::DefaultType for PhoneIncomingCallDismissedReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneIncomingCallDismissedTriggerDetails(pub ::windows::core::IInspectable);
impl PhoneIncomingCallDismissedTriggerDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DismissalTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn TextReplyMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<PhoneIncomingCallDismissedReason> {
        let this = self;
        unsafe {
            let mut result__: PhoneIncomingCallDismissedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneIncomingCallDismissedReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneIncomingCallDismissedTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails;{bad30276-83b6-5732-9c38-0c206546196a})");
}
unsafe impl ::windows::core::Interface for PhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad30276_83b6_5732_9c38_0c206546196a);
}
impl ::windows::core::RuntimeName for PhoneIncomingCallDismissedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
}
impl ::core::convert::From<PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneIncomingCallDismissedTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneIncomingCallDismissedTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneIncomingCallDismissedTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneIncomingCallDismissedTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneIncomingCallDismissedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallDismissedTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneIncomingCallNotificationTriggerDetails(pub ::windows::core::IInspectable);
impl PhoneIncomingCallNotificationTriggerDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn CallId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneIncomingCallNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails;{2b0e6044-9b32-5d42-8222-d2812e39fb21})");
}
unsafe impl ::windows::core::Interface for PhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
}
impl ::windows::core::RuntimeName for PhoneIncomingCallNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
}
impl ::core::convert::From<PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneIncomingCallNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneIncomingCallNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneIncomingCallNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneIncomingCallNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneIncomingCallNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallNotificationTriggerDetails {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineChangeKind(pub i32);
impl PhoneLineChangeKind {
    pub const Added: PhoneLineChangeKind = PhoneLineChangeKind(0i32);
    pub const Removed: PhoneLineChangeKind = PhoneLineChangeKind(1i32);
    pub const PropertiesChanged: PhoneLineChangeKind = PhoneLineChangeKind(2i32);
}
impl ::core::convert::From<i32> for PhoneLineChangeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineChangeKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneLineChangeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind;i4)");
}
impl ::windows::core::DefaultType for PhoneLineChangeKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineChangedTriggerDetails(pub ::windows::core::IInspectable);
impl PhoneLineChangedTriggerDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ChangeType(&self) -> ::windows::core::Result<PhoneLineChangeKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineChangeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineChangeKind>(result__)
        }
    }
    pub fn HasLinePropertyChanged(&self, lineproperty: PhoneLineProperties) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), lineproperty, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineChangedTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails;{c6d321e7-d11d-40d8-b2b7-e40a01d66249})");
}
unsafe impl ::windows::core::Interface for PhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
}
impl ::windows::core::RuntimeName for PhoneLineChangedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
}
impl ::core::convert::From<PhoneLineChangedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneLineChangedTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineChangedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineChangedTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineChangedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneLineChangedTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineChangedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineChangedTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneLineChangedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneLineChangedTriggerDetails {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineProperties(pub u32);
impl PhoneLineProperties {
    pub const None: PhoneLineProperties = PhoneLineProperties(0u32);
    pub const BrandingOptions: PhoneLineProperties = PhoneLineProperties(1u32);
    pub const CanDial: PhoneLineProperties = PhoneLineProperties(2u32);
    pub const CellularDetails: PhoneLineProperties = PhoneLineProperties(4u32);
    pub const DisplayColor: PhoneLineProperties = PhoneLineProperties(8u32);
    pub const DisplayName: PhoneLineProperties = PhoneLineProperties(16u32);
    pub const NetworkName: PhoneLineProperties = PhoneLineProperties(32u32);
    pub const NetworkState: PhoneLineProperties = PhoneLineProperties(64u32);
    pub const Transport: PhoneLineProperties = PhoneLineProperties(128u32);
    pub const Voicemail: PhoneLineProperties = PhoneLineProperties(256u32);
}
impl ::core::convert::From<u32> for PhoneLineProperties {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineProperties {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneLineProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineProperties;u4)");
}
impl ::windows::core::DefaultType for PhoneLineProperties {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PhoneLineProperties {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PhoneLineProperties {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PhoneLineProperties {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PhoneLineProperties {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PhoneLineProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneNewVoicemailMessageTriggerDetails(pub ::windows::core::IInspectable);
impl PhoneNewVoicemailMessageTriggerDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn VoicemailCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn OperatorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneNewVoicemailMessageTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails;{13a8c01b-b831-48d3-8ba9-8d22a6580dcf})");
}
unsafe impl ::windows::core::Interface for PhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
}
impl ::windows::core::RuntimeName for PhoneNewVoicemailMessageTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
}
impl ::core::convert::From<PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneNewVoicemailMessageTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneNewVoicemailMessageTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneNewVoicemailMessageTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneNewVoicemailMessageTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneNewVoicemailMessageTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneNewVoicemailMessageTriggerDetails {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneTriggerType(pub i32);
impl PhoneTriggerType {
    pub const NewVoicemailMessage: PhoneTriggerType = PhoneTriggerType(0i32);
    pub const CallHistoryChanged: PhoneTriggerType = PhoneTriggerType(1i32);
    pub const LineChanged: PhoneTriggerType = PhoneTriggerType(2i32);
    pub const AirplaneModeDisabledForEmergencyCall: PhoneTriggerType = PhoneTriggerType(3i32);
    pub const CallOriginDataRequest: PhoneTriggerType = PhoneTriggerType(4i32);
    pub const CallBlocked: PhoneTriggerType = PhoneTriggerType(5i32);
    pub const IncomingCallDismissed: PhoneTriggerType = PhoneTriggerType(6i32);
    pub const IncomingCallNotification: PhoneTriggerType = PhoneTriggerType(7i32);
}
impl ::core::convert::From<i32> for PhoneTriggerType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhoneTriggerType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhoneTriggerType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneTriggerType;i4)");
}
impl ::windows::core::DefaultType for PhoneTriggerType {
    type DefaultType = Self;
}
