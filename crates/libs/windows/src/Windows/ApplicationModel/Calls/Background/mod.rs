#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallBlockedTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a690a2_e4c1_427f_864e_e470477ddb67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockedTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallBlockedReason) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOriginDataRequestTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginDataRequestTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneIncomingCallDismissedTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad30276_83b6_5732_9c38_0c206546196a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallDismissedTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneIncomingCallDismissedReason) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneIncomingCallNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallNotificationTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineChangedTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineChangedTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineChangeKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineproperty: PhoneLineProperties, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNewVoicemailMessageTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNewVoicemailMessageTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneCallBlockedReason(pub i32);
impl PhoneCallBlockedReason {
    pub const InCallBlockingList: Self = Self(0i32);
    pub const PrivateNumber: Self = Self(1i32);
    pub const UnknownNumber: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallBlockedReason {}
impl ::core::clone::Clone for PhoneCallBlockedReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PhoneCallBlockedReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PhoneCallBlockedReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallBlockedReason {}
unsafe impl ::windows::core::RuntimeType for PhoneCallBlockedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason;i4)");
}
impl ::windows::core::DefaultType for PhoneCallBlockedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneCallBlockedTriggerDetails(::windows::core::IUnknown);
impl PhoneCallBlockedTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn CallBlockedReason(&self) -> ::windows::core::Result<PhoneCallBlockedReason> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallBlockedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallBlockedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallBlockedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallBlockedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallBlockedTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for PhoneCallBlockedTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails;{a4a690a2-e4c1-427f-864e-e470477ddb67})");
}
unsafe impl ::windows::core::Interface for PhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a690a2_e4c1_427f_864e_e470477ddb67);
}
impl ::windows::core::RuntimeName for PhoneCallBlockedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
}
impl ::core::convert::From<PhoneCallBlockedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneCallBlockedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallBlockedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallBlockedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallBlockedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneCallBlockedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallBlockedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallBlockedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallBlockedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallBlockedTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneCallOriginDataRequestTriggerDetails(::windows::core::IUnknown);
impl PhoneCallOriginDataRequestTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn RequestId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallOriginDataRequestTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallOriginDataRequestTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallOriginDataRequestTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for PhoneCallOriginDataRequestTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails;{6e9b5b3f-c54b-4e82-4cc9-e329a4184592})");
}
unsafe impl ::windows::core::Interface for PhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
}
impl ::windows::core::RuntimeName for PhoneCallOriginDataRequestTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
}
impl ::core::convert::From<PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneCallOriginDataRequestTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallOriginDataRequestTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneCallOriginDataRequestTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallOriginDataRequestTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallOriginDataRequestTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallOriginDataRequestTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallOriginDataRequestTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: Self = Self(0i32);
    pub const CallRejected: Self = Self(1i32);
    pub const TextReply: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneIncomingCallDismissedReason {}
impl ::core::clone::Clone for PhoneIncomingCallDismissedReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PhoneIncomingCallDismissedReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PhoneIncomingCallDismissedReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneIncomingCallDismissedReason {}
unsafe impl ::windows::core::RuntimeType for PhoneIncomingCallDismissedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason;i4)");
}
impl ::windows::core::DefaultType for PhoneIncomingCallDismissedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedTriggerDetails(::windows::core::IUnknown);
impl PhoneIncomingCallDismissedTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DismissalTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn TextReplyMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn Reason(&self) -> ::windows::core::Result<PhoneIncomingCallDismissedReason> {
        let this = self;
        unsafe {
            let mut result__: PhoneIncomingCallDismissedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneIncomingCallDismissedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneIncomingCallDismissedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneIncomingCallDismissedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneIncomingCallDismissedTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for PhoneIncomingCallDismissedTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails;{bad30276-83b6-5732-9c38-0c206546196a})");
}
unsafe impl ::windows::core::Interface for PhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad30276_83b6_5732_9c38_0c206546196a);
}
impl ::windows::core::RuntimeName for PhoneIncomingCallDismissedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
}
impl ::core::convert::From<PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneIncomingCallDismissedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneIncomingCallDismissedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneIncomingCallDismissedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallDismissedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneIncomingCallDismissedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneIncomingCallDismissedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallDismissedTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneIncomingCallNotificationTriggerDetails(::windows::core::IUnknown);
impl PhoneIncomingCallNotificationTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn CallId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneIncomingCallNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneIncomingCallNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneIncomingCallNotificationTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for PhoneIncomingCallNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails;{2b0e6044-9b32-5d42-8222-d2812e39fb21})");
}
unsafe impl ::windows::core::Interface for PhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
}
impl ::windows::core::RuntimeName for PhoneIncomingCallNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
}
impl ::core::convert::From<PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneIncomingCallNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneIncomingCallNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneIncomingCallNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneIncomingCallNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneIncomingCallNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallNotificationTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneLineChangeKind(pub i32);
impl PhoneLineChangeKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const PropertiesChanged: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineChangeKind {}
impl ::core::clone::Clone for PhoneLineChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PhoneLineChangeKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PhoneLineChangeKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineChangeKind {}
unsafe impl ::windows::core::RuntimeType for PhoneLineChangeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind;i4)");
}
impl ::windows::core::DefaultType for PhoneLineChangeKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneLineChangedTriggerDetails(::windows::core::IUnknown);
impl PhoneLineChangedTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn ChangeType(&self) -> ::windows::core::Result<PhoneLineChangeKind> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineChangeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineChangeKind>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn HasLinePropertyChanged(&self, lineproperty: PhoneLineProperties) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), lineproperty, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineChangedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineChangedTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for PhoneLineChangedTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails;{c6d321e7-d11d-40d8-b2b7-e40a01d66249})");
}
unsafe impl ::windows::core::Interface for PhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
}
impl ::windows::core::RuntimeName for PhoneLineChangedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
}
impl ::core::convert::From<PhoneLineChangedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneLineChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineChangedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineChangedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneLineChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineChangedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineChangedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneLineChangedTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneLineProperties(pub u32);
impl PhoneLineProperties {
    pub const None: Self = Self(0u32);
    pub const BrandingOptions: Self = Self(1u32);
    pub const CanDial: Self = Self(2u32);
    pub const CellularDetails: Self = Self(4u32);
    pub const DisplayColor: Self = Self(8u32);
    pub const DisplayName: Self = Self(16u32);
    pub const NetworkName: Self = Self(32u32);
    pub const NetworkState: Self = Self(64u32);
    pub const Transport: Self = Self(128u32);
    pub const Voicemail: Self = Self(256u32);
}
impl ::core::marker::Copy for PhoneLineProperties {}
impl ::core::clone::Clone for PhoneLineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PhoneLineProperties {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PhoneLineProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineProperties {}
unsafe impl ::windows::core::RuntimeType for PhoneLineProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineProperties;u4)");
}
impl ::windows::core::DefaultType for PhoneLineProperties {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneNewVoicemailMessageTriggerDetails(::windows::core::IUnknown);
impl PhoneNewVoicemailMessageTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn VoicemailCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
    pub fn OperatorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneNewVoicemailMessageTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneNewVoicemailMessageTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNewVoicemailMessageTriggerDetails {}
unsafe impl ::windows::core::RuntimeType for PhoneNewVoicemailMessageTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails;{13a8c01b-b831-48d3-8ba9-8d22a6580dcf})");
}
unsafe impl ::windows::core::Interface for PhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
}
impl ::windows::core::RuntimeName for PhoneNewVoicemailMessageTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
}
impl ::core::convert::From<PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneNewVoicemailMessageTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneNewVoicemailMessageTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneNewVoicemailMessageTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNewVoicemailMessageTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneNewVoicemailMessageTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneNewVoicemailMessageTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneNewVoicemailMessageTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Calls_Background'*"]
#[repr(transparent)]
pub struct PhoneTriggerType(pub i32);
impl PhoneTriggerType {
    pub const NewVoicemailMessage: Self = Self(0i32);
    pub const CallHistoryChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const AirplaneModeDisabledForEmergencyCall: Self = Self(3i32);
    pub const CallOriginDataRequest: Self = Self(4i32);
    pub const CallBlocked: Self = Self(5i32);
    pub const IncomingCallDismissed: Self = Self(6i32);
    pub const IncomingCallNotification: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneTriggerType {}
impl ::core::clone::Clone for PhoneTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PhoneTriggerType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PhoneTriggerType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneTriggerType {}
unsafe impl ::windows::core::RuntimeType for PhoneTriggerType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneTriggerType;i4)");
}
impl ::windows::core::DefaultType for PhoneTriggerType {
    type DefaultType = Self;
}
