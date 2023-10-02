#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallBlockedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneCallBlockedTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4a690a2_e4c1_427f_864e_e470477ddb67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CallBlockedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallBlockedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallOriginDataRequestTriggerDetails(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneCallOriginDataRequestTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginDataRequestTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestId: usize,
    #[cfg(feature = "deprecated")]
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhoneNumber: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneIncomingCallDismissedTriggerDetails(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPhoneIncomingCallDismissedTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbad30276_83b6_5732_9c38_0c206546196a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallDismissedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LineId: usize,
    #[cfg(feature = "deprecated")]
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhoneNumber: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DismissalTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DismissalTime: usize,
    #[cfg(feature = "deprecated")]
    pub TextReplyMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TextReplyMessage: usize,
    #[cfg(feature = "deprecated")]
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneIncomingCallDismissedReason) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Reason: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneIncomingCallNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneIncomingCallNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneLineChangedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneLineChangedTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineChangedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineChangeKind) -> ::windows_core::HRESULT,
    pub HasLinePropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineproperty: PhoneLineProperties, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneNewVoicemailMessageTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneNewVoicemailMessageTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNewVoicemailMessageTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub OperatorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneCallBlockedTriggerDetails(::windows_core::IUnknown);
impl PhoneCallBlockedTriggerDetails {
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallBlockedReason(&self) -> ::windows_core::Result<PhoneCallBlockedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallBlockedReason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PhoneCallBlockedTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails;{a4a690a2-e4c1-427f-864e-e470477ddb67})");
}
unsafe impl ::windows_core::Interface for PhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallBlockedTriggerDetails {
    const IID: ::windows_core::GUID = <IPhoneCallBlockedTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallBlockedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneCallBlockedTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallBlockedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallBlockedTriggerDetails {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneCallOriginDataRequestTriggerDetails(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PhoneCallOriginDataRequestTriggerDetails {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn RequestId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PhoneCallOriginDataRequestTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails;{6e9b5b3f-c54b-4e82-4cc9-e329a4184592})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PhoneCallOriginDataRequestTriggerDetails {
    const IID: ::windows_core::GUID = <IPhoneCallOriginDataRequestTriggerDetails as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PhoneCallOriginDataRequestTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PhoneCallOriginDataRequestTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PhoneCallOriginDataRequestTriggerDetails {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PhoneCallOriginDataRequestTriggerDetails {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneIncomingCallDismissedTriggerDetails(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PhoneIncomingCallDismissedTriggerDetails {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DismissalTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DismissalTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn TextReplyMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextReplyMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Reason(&self) -> ::windows_core::Result<PhoneIncomingCallDismissedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PhoneIncomingCallDismissedTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails;{bad30276-83b6-5732-9c38-0c206546196a})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PhoneIncomingCallDismissedTriggerDetails {
    const IID: ::windows_core::GUID = <IPhoneIncomingCallDismissedTriggerDetails as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PhoneIncomingCallDismissedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PhoneIncomingCallDismissedTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PhoneIncomingCallDismissedTriggerDetails {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PhoneIncomingCallDismissedTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneIncomingCallNotificationTriggerDetails(::windows_core::IUnknown);
impl PhoneIncomingCallNotificationTriggerDetails {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PhoneIncomingCallNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails;{2b0e6044-9b32-5d42-8222-d2812e39fb21})");
}
unsafe impl ::windows_core::Interface for PhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneIncomingCallNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IPhoneIncomingCallNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneIncomingCallNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneIncomingCallNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneIncomingCallNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallNotificationTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneLineChangedTriggerDetails(::windows_core::IUnknown);
impl PhoneLineChangedTriggerDetails {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeType(&self) -> ::windows_core::Result<PhoneLineChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasLinePropertyChanged(&self, lineproperty: PhoneLineProperties) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasLinePropertyChanged)(::windows_core::Interface::as_raw(this), lineproperty, &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PhoneLineChangedTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails;{c6d321e7-d11d-40d8-b2b7-e40a01d66249})");
}
unsafe impl ::windows_core::Interface for PhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineChangedTriggerDetails {
    const IID: ::windows_core::GUID = <IPhoneLineChangedTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineChangedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneLineChangedTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineChangedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneLineChangedTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneNewVoicemailMessageTriggerDetails(::windows_core::IUnknown);
impl PhoneNewVoicemailMessageTriggerDetails {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VoicemailCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VoicemailCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OperatorMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperatorMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PhoneNewVoicemailMessageTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails;{13a8c01b-b831-48d3-8ba9-8d22a6580dcf})");
}
unsafe impl ::windows_core::Interface for PhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneNewVoicemailMessageTriggerDetails {
    const IID: ::windows_core::GUID = <IPhoneNewVoicemailMessageTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNewVoicemailMessageTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneNewVoicemailMessageTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneNewVoicemailMessageTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneNewVoicemailMessageTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for PhoneCallBlockedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallBlockedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallBlockedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallBlockedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallBlockedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
#[cfg(feature = "deprecated")]
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: Self = Self(0i32);
    pub const CallRejected: Self = Self(1i32);
    pub const TextReply: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PhoneIncomingCallDismissedReason {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PhoneIncomingCallDismissedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PhoneIncomingCallDismissedReason {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for PhoneIncomingCallDismissedReason {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneIncomingCallDismissedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallDismissedReason").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PhoneIncomingCallDismissedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for PhoneLineChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneLineChangeKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineChangeKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineChangeKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for PhoneLineProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneLineProperties {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineProperties").field(&self.0).finish()
    }
}
impl PhoneLineProperties {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PhoneLineProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PhoneLineProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PhoneLineProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PhoneLineProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PhoneLineProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for PhoneLineProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineProperties;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for PhoneTriggerType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneTriggerType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneTriggerType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneTriggerType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneTriggerType;i4)");
}
