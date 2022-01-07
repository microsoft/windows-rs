#[cfg(feature = "implement_exclusive")]
pub trait IEasClientDeviceInformationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OperatingSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemSku(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasClientDeviceInformation {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasClientDeviceInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IEasClientDeviceInformationVtbl {
    pub const fn new<Impl: IEasClientDeviceInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasClientDeviceInformationVtbl {
        unsafe extern "system" fn Id<Impl: IEasClientDeviceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperatingSystem<Impl: IEasClientDeviceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OperatingSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IEasClientDeviceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemManufacturer<Impl: IEasClientDeviceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemProductName<Impl: IEasClientDeviceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemProductName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemSku<Impl: IEasClientDeviceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemSku() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasClientDeviceInformation>, base.5, Id::<Impl, OFFSET>, OperatingSystem::<Impl, OFFSET>, FriendlyName::<Impl, OFFSET>, SystemManufacturer::<Impl, OFFSET>, SystemProductName::<Impl, OFFSET>, SystemSku::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasClientDeviceInformation2Impl: Sized + IEasClientDeviceInformationImpl {
    fn SystemHardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemFirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasClientDeviceInformation2 {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasClientDeviceInformation2";
}
#[cfg(feature = "implement_exclusive")]
impl IEasClientDeviceInformation2Vtbl {
    pub const fn new<Impl: IEasClientDeviceInformation2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasClientDeviceInformation2Vtbl {
        unsafe extern "system" fn SystemHardwareVersion<Impl: IEasClientDeviceInformation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemHardwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemFirmwareVersion<Impl: IEasClientDeviceInformation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemFirmwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasClientDeviceInformation2>, base.5, SystemHardwareVersion::<Impl, OFFSET>, SystemFirmwareVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasClientSecurityPolicyImpl: Sized {
    fn RequireEncryption(&self) -> ::windows::core::Result<bool>;
    fn SetRequireEncryption(&self, value: bool) -> ::windows::core::Result<()>;
    fn MinPasswordLength(&self) -> ::windows::core::Result<u8>;
    fn SetMinPasswordLength(&self, value: u8) -> ::windows::core::Result<()>;
    fn DisallowConvenienceLogon(&self) -> ::windows::core::Result<bool>;
    fn SetDisallowConvenienceLogon(&self, value: bool) -> ::windows::core::Result<()>;
    fn MinPasswordComplexCharacters(&self) -> ::windows::core::Result<u8>;
    fn SetMinPasswordComplexCharacters(&self, value: u8) -> ::windows::core::Result<()>;
    fn PasswordExpiration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPasswordExpiration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PasswordHistory(&self) -> ::windows::core::Result<u32>;
    fn SetPasswordHistory(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxPasswordFailedAttempts(&self) -> ::windows::core::Result<u8>;
    fn SetMaxPasswordFailedAttempts(&self, value: u8) -> ::windows::core::Result<()>;
    fn MaxInactivityTimeLock(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetMaxInactivityTimeLock(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CheckCompliance(&self) -> ::windows::core::Result<EasComplianceResults>;
    fn ApplyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EasComplianceResults>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasClientSecurityPolicy {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasClientSecurityPolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IEasClientSecurityPolicyVtbl {
    pub const fn new<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasClientSecurityPolicyVtbl {
        unsafe extern "system" fn RequireEncryption<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequireEncryption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireEncryption<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRequireEncryption(value).into()
        }
        unsafe extern "system" fn MinPasswordLength<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinPasswordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordLength<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinPasswordLength(value).into()
        }
        unsafe extern "system" fn DisallowConvenienceLogon<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisallowConvenienceLogon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisallowConvenienceLogon<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisallowConvenienceLogon(value).into()
        }
        unsafe extern "system" fn MinPasswordComplexCharacters<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinPasswordComplexCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordComplexCharacters<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinPasswordComplexCharacters(value).into()
        }
        unsafe extern "system" fn PasswordExpiration<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PasswordExpiration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordExpiration<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPasswordExpiration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PasswordHistory<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PasswordHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordHistory<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPasswordHistory(value).into()
        }
        unsafe extern "system" fn MaxPasswordFailedAttempts<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPasswordFailedAttempts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPasswordFailedAttempts<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxPasswordFailedAttempts(value).into()
        }
        unsafe extern "system" fn MaxInactivityTimeLock<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxInactivityTimeLock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxInactivityTimeLock<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxInactivityTimeLock(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckCompliance<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckCompliance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyAsync<Impl: IEasClientSecurityPolicyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplyAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IEasClientSecurityPolicy>,
            base.5,
            RequireEncryption::<Impl, OFFSET>,
            SetRequireEncryption::<Impl, OFFSET>,
            MinPasswordLength::<Impl, OFFSET>,
            SetMinPasswordLength::<Impl, OFFSET>,
            DisallowConvenienceLogon::<Impl, OFFSET>,
            SetDisallowConvenienceLogon::<Impl, OFFSET>,
            MinPasswordComplexCharacters::<Impl, OFFSET>,
            SetMinPasswordComplexCharacters::<Impl, OFFSET>,
            PasswordExpiration::<Impl, OFFSET>,
            SetPasswordExpiration::<Impl, OFFSET>,
            PasswordHistory::<Impl, OFFSET>,
            SetPasswordHistory::<Impl, OFFSET>,
            MaxPasswordFailedAttempts::<Impl, OFFSET>,
            SetMaxPasswordFailedAttempts::<Impl, OFFSET>,
            MaxInactivityTimeLock::<Impl, OFFSET>,
            SetMaxInactivityTimeLock::<Impl, OFFSET>,
            CheckCompliance::<Impl, OFFSET>,
            ApplyAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasComplianceResultsImpl: Sized {
    fn Compliant(&self) -> ::windows::core::Result<bool>;
    fn RequireEncryptionResult(&self) -> ::windows::core::Result<EasRequireEncryptionResult>;
    fn MinPasswordLengthResult(&self) -> ::windows::core::Result<EasMinPasswordLengthResult>;
    fn DisallowConvenienceLogonResult(&self) -> ::windows::core::Result<EasDisallowConvenienceLogonResult>;
    fn MinPasswordComplexCharactersResult(&self) -> ::windows::core::Result<EasMinPasswordComplexCharactersResult>;
    fn PasswordExpirationResult(&self) -> ::windows::core::Result<EasPasswordExpirationResult>;
    fn PasswordHistoryResult(&self) -> ::windows::core::Result<EasPasswordHistoryResult>;
    fn MaxPasswordFailedAttemptsResult(&self) -> ::windows::core::Result<EasMaxPasswordFailedAttemptsResult>;
    fn MaxInactivityTimeLockResult(&self) -> ::windows::core::Result<EasMaxInactivityTimeLockResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasComplianceResults {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasComplianceResults";
}
#[cfg(feature = "implement_exclusive")]
impl IEasComplianceResultsVtbl {
    pub const fn new<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasComplianceResultsVtbl {
        unsafe extern "system" fn Compliant<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compliant() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequireEncryptionResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasRequireEncryptionResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequireEncryptionResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordLengthResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordLengthResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinPasswordLengthResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisallowConvenienceLogonResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasDisallowConvenienceLogonResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisallowConvenienceLogonResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordComplexCharactersResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordComplexCharactersResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinPasswordComplexCharactersResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordExpirationResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordExpirationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PasswordExpirationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordHistoryResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordHistoryResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PasswordHistoryResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPasswordFailedAttemptsResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasMaxPasswordFailedAttemptsResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPasswordFailedAttemptsResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxInactivityTimeLockResult<Impl: IEasComplianceResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasMaxInactivityTimeLockResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxInactivityTimeLockResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IEasComplianceResults>,
            base.5,
            Compliant::<Impl, OFFSET>,
            RequireEncryptionResult::<Impl, OFFSET>,
            MinPasswordLengthResult::<Impl, OFFSET>,
            DisallowConvenienceLogonResult::<Impl, OFFSET>,
            MinPasswordComplexCharactersResult::<Impl, OFFSET>,
            PasswordExpirationResult::<Impl, OFFSET>,
            PasswordHistoryResult::<Impl, OFFSET>,
            MaxPasswordFailedAttemptsResult::<Impl, OFFSET>,
            MaxInactivityTimeLockResult::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasComplianceResults2Impl: Sized + IEasComplianceResultsImpl {
    fn EncryptionProviderType(&self) -> ::windows::core::Result<EasEncryptionProviderType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasComplianceResults2 {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.IEasComplianceResults2";
}
#[cfg(feature = "implement_exclusive")]
impl IEasComplianceResults2Vtbl {
    pub const fn new<Impl: IEasComplianceResults2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasComplianceResults2Vtbl {
        unsafe extern "system" fn EncryptionProviderType<Impl: IEasComplianceResults2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasEncryptionProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncryptionProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasComplianceResults2>, base.5, EncryptionProviderType::<Impl, OFFSET>)
    }
}
