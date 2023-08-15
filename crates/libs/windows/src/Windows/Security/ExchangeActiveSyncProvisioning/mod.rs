#[doc(hidden)]
#[repr(transparent)]
pub struct IEasClientDeviceInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_Vtbl;
}
impl ::core::clone::Clone for IEasClientDeviceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEasClientDeviceInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54dfd981_1968_4ca3_b958_e595d16505eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasClientDeviceInformation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasClientDeviceInformation2 {
    type Vtable = IEasClientDeviceInformation2_Vtbl;
}
impl ::core::clone::Clone for IEasClientDeviceInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEasClientDeviceInformation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffb35923_bb26_4d6a_81bc_165aee0ad754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SystemHardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemFirmwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasClientSecurityPolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_Vtbl;
}
impl ::core::clone::Clone for IEasClientSecurityPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEasClientSecurityPolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45b72362_dfba_4a9b_aced_6fe2adcb6420);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientSecurityPolicy_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequireEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequireEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetMinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub DisallowConvenienceLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDisallowConvenienceLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MinPasswordComplexCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetMinPasswordComplexCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PasswordExpiration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PasswordExpiration: usize,
    #[cfg(feature = "Foundation")]
    pub SetPasswordExpiration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPasswordExpiration: usize,
    pub PasswordHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetPasswordHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MaxPasswordFailedAttempts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetMaxPasswordFailedAttempts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxInactivityTimeLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxInactivityTimeLock: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxInactivityTimeLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxInactivityTimeLock: usize,
    pub CheckCompliance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasComplianceResults(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasComplianceResults {
    type Vtable = IEasComplianceResults_Vtbl;
}
impl ::core::clone::Clone for IEasComplianceResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEasComplianceResults {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x463c299c_7f19_4c66_b403_cb45dd57a2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Compliant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RequireEncryptionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasRequireEncryptionResult) -> ::windows_core::HRESULT,
    pub MinPasswordLengthResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordLengthResult) -> ::windows_core::HRESULT,
    pub DisallowConvenienceLogonResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasDisallowConvenienceLogonResult) -> ::windows_core::HRESULT,
    pub MinPasswordComplexCharactersResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordComplexCharactersResult) -> ::windows_core::HRESULT,
    pub PasswordExpirationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordExpirationResult) -> ::windows_core::HRESULT,
    pub PasswordHistoryResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordHistoryResult) -> ::windows_core::HRESULT,
    pub MaxPasswordFailedAttemptsResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMaxPasswordFailedAttemptsResult) -> ::windows_core::HRESULT,
    pub MaxInactivityTimeLockResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMaxInactivityTimeLockResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasComplianceResults2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasComplianceResults2 {
    type Vtable = IEasComplianceResults2_Vtbl;
}
impl ::core::clone::Clone for IEasComplianceResults2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEasComplianceResults2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fbe60c9_1aa8_47f5_88bb_cb3ef0bffb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EncryptionProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasEncryptionProviderType) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasClientDeviceInformation(::windows_core::IUnknown);
impl EasClientDeviceInformation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<EasClientDeviceInformation, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OperatingSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperatingSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemManufacturer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemManufacturer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemProductName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemProductName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemSku(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemSku)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemHardwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemHardwareVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemFirmwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemFirmwareVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EasClientDeviceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasClientDeviceInformation {}
impl ::core::fmt::Debug for EasClientDeviceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasClientDeviceInformation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasClientDeviceInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation;{54dfd981-1968-4ca3-b958-e595d16505eb})");
}
impl ::core::clone::Clone for EasClientDeviceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EasClientDeviceInformation {
    const IID: ::windows_core::GUID = <IEasClientDeviceInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EasClientDeviceInformation {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation";
}
::windows_core::imp::interface_hierarchy!(EasClientDeviceInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasClientSecurityPolicy(::windows_core::IUnknown);
impl EasClientSecurityPolicy {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<EasClientSecurityPolicy, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequireEncryption(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequireEncryption)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequireEncryption(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequireEncryption)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinPasswordLength(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinPasswordLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMinPasswordLength(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinPasswordLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisallowConvenienceLogon(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisallowConvenienceLogon)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisallowConvenienceLogon(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisallowConvenienceLogon)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinPasswordComplexCharacters(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinPasswordComplexCharacters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMinPasswordComplexCharacters(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinPasswordComplexCharacters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PasswordExpiration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PasswordExpiration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPasswordExpiration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPasswordExpiration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PasswordHistory(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PasswordHistory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPasswordHistory(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPasswordHistory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxPasswordFailedAttempts(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPasswordFailedAttempts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxPasswordFailedAttempts(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPasswordFailedAttempts)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxInactivityTimeLock(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxInactivityTimeLock)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxInactivityTimeLock(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxInactivityTimeLock)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CheckCompliance(&self) -> ::windows_core::Result<EasComplianceResults> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckCompliance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplyAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EasComplianceResults>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplyAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EasClientSecurityPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasClientSecurityPolicy {}
impl ::core::fmt::Debug for EasClientSecurityPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasClientSecurityPolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasClientSecurityPolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy;{45b72362-dfba-4a9b-aced-6fe2adcb6420})");
}
impl ::core::clone::Clone for EasClientSecurityPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EasClientSecurityPolicy {
    const IID: ::windows_core::GUID = <IEasClientSecurityPolicy as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EasClientSecurityPolicy {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy";
}
::windows_core::imp::interface_hierarchy!(EasClientSecurityPolicy, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasComplianceResults(::windows_core::IUnknown);
impl EasComplianceResults {
    pub fn Compliant(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compliant)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequireEncryptionResult(&self) -> ::windows_core::Result<EasRequireEncryptionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequireEncryptionResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinPasswordLengthResult(&self) -> ::windows_core::Result<EasMinPasswordLengthResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinPasswordLengthResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisallowConvenienceLogonResult(&self) -> ::windows_core::Result<EasDisallowConvenienceLogonResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisallowConvenienceLogonResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinPasswordComplexCharactersResult(&self) -> ::windows_core::Result<EasMinPasswordComplexCharactersResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinPasswordComplexCharactersResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PasswordExpirationResult(&self) -> ::windows_core::Result<EasPasswordExpirationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PasswordExpirationResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PasswordHistoryResult(&self) -> ::windows_core::Result<EasPasswordHistoryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PasswordHistoryResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxPasswordFailedAttemptsResult(&self) -> ::windows_core::Result<EasMaxPasswordFailedAttemptsResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPasswordFailedAttemptsResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxInactivityTimeLockResult(&self) -> ::windows_core::Result<EasMaxInactivityTimeLockResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxInactivityTimeLockResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EncryptionProviderType(&self) -> ::windows_core::Result<EasEncryptionProviderType> {
        let this = &::windows_core::ComInterface::cast::<IEasComplianceResults2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncryptionProviderType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EasComplianceResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasComplianceResults {}
impl ::core::fmt::Debug for EasComplianceResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasComplianceResults").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasComplianceResults {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults;{463c299c-7f19-4c66-b403-cb45dd57a2b3})");
}
impl ::core::clone::Clone for EasComplianceResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EasComplianceResults {
    type Vtable = IEasComplianceResults_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EasComplianceResults {
    const IID: ::windows_core::GUID = <IEasComplianceResults as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EasComplianceResults {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults";
}
::windows_core::imp::interface_hierarchy!(EasComplianceResults, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasDisallowConvenienceLogonResult(pub i32);
impl EasDisallowConvenienceLogonResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
}
impl ::core::marker::Copy for EasDisallowConvenienceLogonResult {}
impl ::core::clone::Clone for EasDisallowConvenienceLogonResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasDisallowConvenienceLogonResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasDisallowConvenienceLogonResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasDisallowConvenienceLogonResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasDisallowConvenienceLogonResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasDisallowConvenienceLogonResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasDisallowConvenienceLogonResult;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasEncryptionProviderType(pub i32);
impl EasEncryptionProviderType {
    pub const NotEvaluated: Self = Self(0i32);
    pub const WindowsEncryption: Self = Self(1i32);
    pub const OtherEncryption: Self = Self(2i32);
}
impl ::core::marker::Copy for EasEncryptionProviderType {}
impl ::core::clone::Clone for EasEncryptionProviderType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasEncryptionProviderType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasEncryptionProviderType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasEncryptionProviderType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasEncryptionProviderType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasEncryptionProviderType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasEncryptionProviderType;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasMaxInactivityTimeLockResult(pub i32);
impl EasMaxInactivityTimeLockResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxInactivityTimeLockResult {}
impl ::core::clone::Clone for EasMaxInactivityTimeLockResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMaxInactivityTimeLockResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasMaxInactivityTimeLockResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasMaxInactivityTimeLockResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMaxInactivityTimeLockResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasMaxInactivityTimeLockResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxInactivityTimeLockResult;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasMaxPasswordFailedAttemptsResult(pub i32);
impl EasMaxPasswordFailedAttemptsResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxPasswordFailedAttemptsResult {}
impl ::core::clone::Clone for EasMaxPasswordFailedAttemptsResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMaxPasswordFailedAttemptsResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasMaxPasswordFailedAttemptsResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasMaxPasswordFailedAttemptsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMaxPasswordFailedAttemptsResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasMaxPasswordFailedAttemptsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxPasswordFailedAttemptsResult;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasMinPasswordComplexCharactersResult(pub i32);
impl EasMinPasswordComplexCharactersResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordComplexCharactersResult {}
impl ::core::clone::Clone for EasMinPasswordComplexCharactersResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMinPasswordComplexCharactersResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasMinPasswordComplexCharactersResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasMinPasswordComplexCharactersResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMinPasswordComplexCharactersResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasMinPasswordComplexCharactersResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordComplexCharactersResult;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasMinPasswordLengthResult(pub i32);
impl EasMinPasswordLengthResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordLengthResult {}
impl ::core::clone::Clone for EasMinPasswordLengthResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMinPasswordLengthResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasMinPasswordLengthResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasMinPasswordLengthResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMinPasswordLengthResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasMinPasswordLengthResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordLengthResult;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasPasswordExpirationResult(pub i32);
impl EasPasswordExpirationResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedExpirationIncompatible: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const UserCannotChangePassword: Self = Self(6i32);
    pub const AdminsCannotChangePassword: Self = Self(7i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(8i32);
}
impl ::core::marker::Copy for EasPasswordExpirationResult {}
impl ::core::clone::Clone for EasPasswordExpirationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasPasswordExpirationResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasPasswordExpirationResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasPasswordExpirationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasPasswordExpirationResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasPasswordExpirationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordExpirationResult;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasPasswordHistoryResult(pub i32);
impl EasPasswordHistoryResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasPasswordHistoryResult {}
impl ::core::clone::Clone for EasPasswordHistoryResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasPasswordHistoryResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasPasswordHistoryResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasPasswordHistoryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasPasswordHistoryResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasPasswordHistoryResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordHistoryResult;i4)");
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EasRequireEncryptionResult(pub i32);
impl EasRequireEncryptionResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const NotProvisionedOnAllVolumes: Self = Self(3i32);
    pub const DeFixedDataNotSupported: Self = Self(4i32);
    pub const FixedDataNotSupported: Self = Self(4i32);
    pub const DeHardwareNotCompliant: Self = Self(5i32);
    pub const HardwareNotCompliant: Self = Self(5i32);
    pub const DeWinReNotConfigured: Self = Self(6i32);
    pub const LockNotConfigured: Self = Self(6i32);
    pub const DeProtectionSuspended: Self = Self(7i32);
    pub const ProtectionSuspended: Self = Self(7i32);
    pub const DeOsVolumeNotProtected: Self = Self(8i32);
    pub const OsVolumeNotProtected: Self = Self(8i32);
    pub const DeProtectionNotYetEnabled: Self = Self(9i32);
    pub const ProtectionNotYetEnabled: Self = Self(9i32);
    pub const NoFeatureLicense: Self = Self(10i32);
    pub const OsNotProtected: Self = Self(11i32);
    pub const UnexpectedFailure: Self = Self(12i32);
}
impl ::core::marker::Copy for EasRequireEncryptionResult {}
impl ::core::clone::Clone for EasRequireEncryptionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasRequireEncryptionResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EasRequireEncryptionResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EasRequireEncryptionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasRequireEncryptionResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EasRequireEncryptionResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasRequireEncryptionResult;i4)");
}
