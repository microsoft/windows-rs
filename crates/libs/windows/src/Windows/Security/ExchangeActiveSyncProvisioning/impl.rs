#[cfg(feature = "implement_exclusive")]
pub trait IEasClientDeviceInformationImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OperatingSystem(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemManufacturer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemProductName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemSku(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasClientDeviceInformation {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasClientDeviceInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IEasClientDeviceInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasClientDeviceInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasClientDeviceInformationVtbl {
        unsafe extern "system" fn Id<Impl: IEasClientDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperatingSystem<Impl: IEasClientDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperatingSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IEasClientDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemManufacturer<Impl: IEasClientDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemProductName<Impl: IEasClientDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemProductName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemSku<Impl: IEasClientDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemSku() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasClientDeviceInformation, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            OperatingSystem: OperatingSystem::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SystemManufacturer: SystemManufacturer::<Impl, IMPL_OFFSET>,
            SystemProductName: SystemProductName::<Impl, IMPL_OFFSET>,
            SystemSku: SystemSku::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasClientDeviceInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasClientDeviceInformation2Impl: Sized + IEasClientDeviceInformationImpl {
    fn SystemHardwareVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemFirmwareVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasClientDeviceInformation2 {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasClientDeviceInformation2";
}
#[cfg(feature = "implement_exclusive")]
impl IEasClientDeviceInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasClientDeviceInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasClientDeviceInformation2Vtbl {
        unsafe extern "system" fn SystemHardwareVersion<Impl: IEasClientDeviceInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemHardwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemFirmwareVersion<Impl: IEasClientDeviceInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemFirmwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasClientDeviceInformation2, BASE_OFFSET>(),
            SystemHardwareVersion: SystemHardwareVersion::<Impl, IMPL_OFFSET>,
            SystemFirmwareVersion: SystemFirmwareVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasClientDeviceInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEasClientSecurityPolicyImpl: Sized {
    fn RequireEncryption(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequireEncryption(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MinPasswordLength(&mut self) -> ::windows::core::Result<u8>;
    fn SetMinPasswordLength(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn DisallowConvenienceLogon(&mut self) -> ::windows::core::Result<bool>;
    fn SetDisallowConvenienceLogon(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MinPasswordComplexCharacters(&mut self) -> ::windows::core::Result<u8>;
    fn SetMinPasswordComplexCharacters(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn PasswordExpiration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPasswordExpiration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PasswordHistory(&mut self) -> ::windows::core::Result<u32>;
    fn SetPasswordHistory(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn MaxPasswordFailedAttempts(&mut self) -> ::windows::core::Result<u8>;
    fn SetMaxPasswordFailedAttempts(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn MaxInactivityTimeLock(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetMaxInactivityTimeLock(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CheckCompliance(&mut self) -> ::windows::core::Result<EasComplianceResults>;
    fn ApplyAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EasComplianceResults>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEasClientSecurityPolicy {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasClientSecurityPolicy";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEasClientSecurityPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasClientSecurityPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasClientSecurityPolicyVtbl {
        unsafe extern "system" fn RequireEncryption<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequireEncryption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireEncryption<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequireEncryption(value).into()
        }
        unsafe extern "system" fn MinPasswordLength<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPasswordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordLength<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPasswordLength(value).into()
        }
        unsafe extern "system" fn DisallowConvenienceLogon<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisallowConvenienceLogon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisallowConvenienceLogon<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisallowConvenienceLogon(value).into()
        }
        unsafe extern "system" fn MinPasswordComplexCharacters<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPasswordComplexCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordComplexCharacters<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPasswordComplexCharacters(value).into()
        }
        unsafe extern "system" fn PasswordExpiration<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordExpiration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordExpiration<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordExpiration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PasswordHistory<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordHistory<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordHistory(value).into()
        }
        unsafe extern "system" fn MaxPasswordFailedAttempts<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPasswordFailedAttempts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPasswordFailedAttempts<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPasswordFailedAttempts(value).into()
        }
        unsafe extern "system" fn MaxInactivityTimeLock<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxInactivityTimeLock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxInactivityTimeLock<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxInactivityTimeLock(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckCompliance<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCompliance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyAsync<Impl: IEasClientSecurityPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasClientSecurityPolicy, BASE_OFFSET>(),
            RequireEncryption: RequireEncryption::<Impl, IMPL_OFFSET>,
            SetRequireEncryption: SetRequireEncryption::<Impl, IMPL_OFFSET>,
            MinPasswordLength: MinPasswordLength::<Impl, IMPL_OFFSET>,
            SetMinPasswordLength: SetMinPasswordLength::<Impl, IMPL_OFFSET>,
            DisallowConvenienceLogon: DisallowConvenienceLogon::<Impl, IMPL_OFFSET>,
            SetDisallowConvenienceLogon: SetDisallowConvenienceLogon::<Impl, IMPL_OFFSET>,
            MinPasswordComplexCharacters: MinPasswordComplexCharacters::<Impl, IMPL_OFFSET>,
            SetMinPasswordComplexCharacters: SetMinPasswordComplexCharacters::<Impl, IMPL_OFFSET>,
            PasswordExpiration: PasswordExpiration::<Impl, IMPL_OFFSET>,
            SetPasswordExpiration: SetPasswordExpiration::<Impl, IMPL_OFFSET>,
            PasswordHistory: PasswordHistory::<Impl, IMPL_OFFSET>,
            SetPasswordHistory: SetPasswordHistory::<Impl, IMPL_OFFSET>,
            MaxPasswordFailedAttempts: MaxPasswordFailedAttempts::<Impl, IMPL_OFFSET>,
            SetMaxPasswordFailedAttempts: SetMaxPasswordFailedAttempts::<Impl, IMPL_OFFSET>,
            MaxInactivityTimeLock: MaxInactivityTimeLock::<Impl, IMPL_OFFSET>,
            SetMaxInactivityTimeLock: SetMaxInactivityTimeLock::<Impl, IMPL_OFFSET>,
            CheckCompliance: CheckCompliance::<Impl, IMPL_OFFSET>,
            ApplyAsync: ApplyAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasClientSecurityPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasComplianceResultsImpl: Sized {
    fn Compliant(&mut self) -> ::windows::core::Result<bool>;
    fn RequireEncryptionResult(&mut self) -> ::windows::core::Result<EasRequireEncryptionResult>;
    fn MinPasswordLengthResult(&mut self) -> ::windows::core::Result<EasMinPasswordLengthResult>;
    fn DisallowConvenienceLogonResult(&mut self) -> ::windows::core::Result<EasDisallowConvenienceLogonResult>;
    fn MinPasswordComplexCharactersResult(&mut self) -> ::windows::core::Result<EasMinPasswordComplexCharactersResult>;
    fn PasswordExpirationResult(&mut self) -> ::windows::core::Result<EasPasswordExpirationResult>;
    fn PasswordHistoryResult(&mut self) -> ::windows::core::Result<EasPasswordHistoryResult>;
    fn MaxPasswordFailedAttemptsResult(&mut self) -> ::windows::core::Result<EasMaxPasswordFailedAttemptsResult>;
    fn MaxInactivityTimeLockResult(&mut self) -> ::windows::core::Result<EasMaxInactivityTimeLockResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasComplianceResults {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasComplianceResults";
}
#[cfg(feature = "implement_exclusive")]
impl IEasComplianceResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasComplianceResultsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasComplianceResultsVtbl {
        unsafe extern "system" fn Compliant<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compliant() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequireEncryptionResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasRequireEncryptionResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequireEncryptionResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordLengthResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordLengthResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPasswordLengthResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisallowConvenienceLogonResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasDisallowConvenienceLogonResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisallowConvenienceLogonResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordComplexCharactersResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordComplexCharactersResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPasswordComplexCharactersResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordExpirationResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordExpirationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordExpirationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordHistoryResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordHistoryResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordHistoryResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPasswordFailedAttemptsResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasMaxPasswordFailedAttemptsResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPasswordFailedAttemptsResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxInactivityTimeLockResult<Impl: IEasComplianceResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasMaxInactivityTimeLockResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxInactivityTimeLockResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasComplianceResults, BASE_OFFSET>(),
            Compliant: Compliant::<Impl, IMPL_OFFSET>,
            RequireEncryptionResult: RequireEncryptionResult::<Impl, IMPL_OFFSET>,
            MinPasswordLengthResult: MinPasswordLengthResult::<Impl, IMPL_OFFSET>,
            DisallowConvenienceLogonResult: DisallowConvenienceLogonResult::<Impl, IMPL_OFFSET>,
            MinPasswordComplexCharactersResult: MinPasswordComplexCharactersResult::<Impl, IMPL_OFFSET>,
            PasswordExpirationResult: PasswordExpirationResult::<Impl, IMPL_OFFSET>,
            PasswordHistoryResult: PasswordHistoryResult::<Impl, IMPL_OFFSET>,
            MaxPasswordFailedAttemptsResult: MaxPasswordFailedAttemptsResult::<Impl, IMPL_OFFSET>,
            MaxInactivityTimeLockResult: MaxInactivityTimeLockResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasComplianceResults as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasComplianceResults2Impl: Sized + IEasComplianceResultsImpl {
    fn EncryptionProviderType(&mut self) -> ::windows::core::Result<EasEncryptionProviderType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasComplianceResults2 {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasComplianceResults2";
}
#[cfg(feature = "implement_exclusive")]
impl IEasComplianceResults2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasComplianceResults2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasComplianceResults2Vtbl {
        unsafe extern "system" fn EncryptionProviderType<Impl: IEasComplianceResults2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasEncryptionProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasComplianceResults2, BASE_OFFSET>(),
            EncryptionProviderType: EncryptionProviderType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasComplianceResults2 as ::windows::core::Interface>::IID
    }
}
