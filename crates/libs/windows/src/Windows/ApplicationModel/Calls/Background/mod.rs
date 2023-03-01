#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallBlockedTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallBlockedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneCallBlockedTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a690a2_e4c1_427f_864e_e470477ddb67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockedTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CallBlockedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallBlockedReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPhoneCallOriginDataRequestTriggerDetails(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IPhoneCallOriginDataRequestTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IPhoneCallOriginDataRequestTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginDataRequestTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestId: usize,
    #[cfg(feature = "deprecated")]
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhoneNumber: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPhoneIncomingCallDismissedTriggerDetails(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IPhoneIncomingCallDismissedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IPhoneIncomingCallDismissedTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad30276_83b6_5732_9c38_0c206546196a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallDismissedTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LineId: usize,
    #[cfg(feature = "deprecated")]
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhoneNumber: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DismissalTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DismissalTime: usize,
    #[cfg(feature = "deprecated")]
    pub TextReplyMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TextReplyMessage: usize,
    #[cfg(feature = "deprecated")]
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneIncomingCallDismissedReason) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Reason: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneIncomingCallNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneIncomingCallNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneIncomingCallNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineChangedTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneLineChangedTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineChangedTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineChangeKind) -> ::windows::core::HRESULT,
    pub HasLinePropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineproperty: PhoneLineProperties, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNewVoicemailMessageTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneNewVoicemailMessageTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneNewVoicemailMessageTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNewVoicemailMessageTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub OperatorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneCallBlockedTriggerDetails(::windows::core::IUnknown);
impl PhoneCallBlockedTriggerDetails {
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallBlockedReason(&self) -> ::windows::core::Result<PhoneCallBlockedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneCallBlockedReason>();
            (::windows::core::Interface::vtable(this).CallBlockedReason)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallBlockedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallBlockedTriggerDetails {}
impl ::core::fmt::Debug for PhoneCallBlockedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallBlockedTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneCallBlockedTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails;{a4a690a2-e4c1-427f-864e-e470477ddb67})");
}
impl ::core::clone::Clone for PhoneCallBlockedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneCallBlockedTriggerDetails {
    const IID: ::windows::core::GUID = <IPhoneCallBlockedTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallBlockedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
}
::windows::imp::interface_hierarchy!(PhoneCallBlockedTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallBlockedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallBlockedTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PhoneCallOriginDataRequestTriggerDetails(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PhoneCallOriginDataRequestTriggerDetails {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RequestId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RequestId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PhoneCallOriginDataRequestTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PhoneCallOriginDataRequestTriggerDetails {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneCallOriginDataRequestTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallOriginDataRequestTriggerDetails").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for PhoneCallOriginDataRequestTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails;{6e9b5b3f-c54b-4e82-4cc9-e329a4184592})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PhoneCallOriginDataRequestTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for PhoneCallOriginDataRequestTriggerDetails {
    const IID: ::windows::core::GUID = <IPhoneCallOriginDataRequestTriggerDetails as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PhoneCallOriginDataRequestTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(PhoneCallOriginDataRequestTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PhoneCallOriginDataRequestTriggerDetails {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PhoneCallOriginDataRequestTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedTriggerDetails(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PhoneIncomingCallDismissedTriggerDetails {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DismissalTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).DismissalTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TextReplyMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TextReplyMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Reason(&self) -> ::windows::core::Result<PhoneIncomingCallDismissedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneIncomingCallDismissedReason>();
            (::windows::core::Interface::vtable(this).Reason)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PhoneIncomingCallDismissedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PhoneIncomingCallDismissedTriggerDetails {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneIncomingCallDismissedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallDismissedTriggerDetails").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for PhoneIncomingCallDismissedTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails;{bad30276-83b6-5732-9c38-0c206546196a})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PhoneIncomingCallDismissedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for PhoneIncomingCallDismissedTriggerDetails {
    const IID: ::windows::core::GUID = <IPhoneIncomingCallDismissedTriggerDetails as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PhoneIncomingCallDismissedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(PhoneIncomingCallDismissedTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PhoneIncomingCallDismissedTriggerDetails {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PhoneIncomingCallDismissedTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneIncomingCallNotificationTriggerDetails(::windows::core::IUnknown);
impl PhoneIncomingCallNotificationTriggerDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneIncomingCallNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneIncomingCallNotificationTriggerDetails {}
impl ::core::fmt::Debug for PhoneIncomingCallNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneIncomingCallNotificationTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails;{2b0e6044-9b32-5d42-8222-d2812e39fb21})");
}
impl ::core::clone::Clone for PhoneIncomingCallNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneIncomingCallNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IPhoneIncomingCallNotificationTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneIncomingCallNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
}
::windows::imp::interface_hierarchy!(PhoneIncomingCallNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PhoneIncomingCallNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneLineChangedTriggerDetails(::windows::core::IUnknown);
impl PhoneLineChangedTriggerDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeType(&self) -> ::windows::core::Result<PhoneLineChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneLineChangeKind>();
            (::windows::core::Interface::vtable(this).ChangeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasLinePropertyChanged(&self, lineproperty: PhoneLineProperties) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasLinePropertyChanged)(::windows::core::Interface::as_raw(this), lineproperty, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneLineChangedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineChangedTriggerDetails {}
impl ::core::fmt::Debug for PhoneLineChangedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineChangedTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneLineChangedTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails;{c6d321e7-d11d-40d8-b2b7-e40a01d66249})");
}
impl ::core::clone::Clone for PhoneLineChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneLineChangedTriggerDetails {
    const IID: ::windows::core::GUID = <IPhoneLineChangedTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineChangedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
}
::windows::imp::interface_hierarchy!(PhoneLineChangedTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineChangedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneLineChangedTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneNewVoicemailMessageTriggerDetails(::windows::core::IUnknown);
impl PhoneNewVoicemailMessageTriggerDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VoicemailCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).VoicemailCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OperatorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OperatorMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneNewVoicemailMessageTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNewVoicemailMessageTriggerDetails {}
impl ::core::fmt::Debug for PhoneNewVoicemailMessageTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNewVoicemailMessageTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneNewVoicemailMessageTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails;{13a8c01b-b831-48d3-8ba9-8d22a6580dcf})");
}
impl ::core::clone::Clone for PhoneNewVoicemailMessageTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneNewVoicemailMessageTriggerDetails {
    const IID: ::windows::core::GUID = <IPhoneNewVoicemailMessageTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneNewVoicemailMessageTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
}
::windows::imp::interface_hierarchy!(PhoneNewVoicemailMessageTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PhoneNewVoicemailMessageTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneNewVoicemailMessageTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
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
impl ::windows::core::TypeKind for PhoneCallBlockedReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallBlockedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallBlockedReason").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneCallBlockedReason {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`, `\"deprecated\"`*"]
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
impl ::windows::core::TypeKind for PhoneIncomingCallDismissedReason {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneIncomingCallDismissedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallDismissedReason").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for PhoneIncomingCallDismissedReason {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
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
impl ::windows::core::TypeKind for PhoneLineChangeKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineChangeKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneLineChangeKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
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
impl ::windows::core::TypeKind for PhoneLineProperties {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::RuntimeType for PhoneLineProperties {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineProperties;u4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
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
impl ::windows::core::TypeKind for PhoneTriggerType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PhoneTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneTriggerType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneTriggerType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneTriggerType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
