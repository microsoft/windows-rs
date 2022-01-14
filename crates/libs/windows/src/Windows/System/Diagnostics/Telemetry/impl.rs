#[cfg(feature = "implement_exclusive")]
pub trait IPlatformTelemetryClientStatics_Impl: Sized {
    fn Register(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<PlatformTelemetryRegistrationResult>;
    fn RegisterWithSettings(&mut self, id: &::windows::core::HSTRING, settings: &::core::option::Option<PlatformTelemetryRegistrationSettings>) -> ::windows::core::Result<PlatformTelemetryRegistrationResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlatformTelemetryClientStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.IPlatformTelemetryClientStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlatformTelemetryClientStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlatformTelemetryClientStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlatformTelemetryClientStatics_Vtbl {
        unsafe extern "system" fn Register<Impl: IPlatformTelemetryClientStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterWithSettings<Impl: IPlatformTelemetryClientStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWithSettings(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&settings as *const <PlatformTelemetryRegistrationSettings as ::windows::core::Abi>::Abi as *const <PlatformTelemetryRegistrationSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlatformTelemetryClientStatics, BASE_OFFSET>(),
            Register: Register::<Impl, IMPL_OFFSET>,
            RegisterWithSettings: RegisterWithSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlatformTelemetryClientStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformTelemetryRegistrationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PlatformTelemetryRegistrationStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlatformTelemetryRegistrationResult {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPlatformTelemetryRegistrationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlatformTelemetryRegistrationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlatformTelemetryRegistrationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IPlatformTelemetryRegistrationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlatformTelemetryRegistrationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlatformTelemetryRegistrationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlatformTelemetryRegistrationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformTelemetryRegistrationSettings_Impl: Sized {
    fn StorageSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetStorageSize(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn UploadQuotaSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetUploadQuotaSize(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlatformTelemetryRegistrationSettings {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IPlatformTelemetryRegistrationSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlatformTelemetryRegistrationSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlatformTelemetryRegistrationSettings_Vtbl {
        unsafe extern "system" fn StorageSize<Impl: IPlatformTelemetryRegistrationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageSize<Impl: IPlatformTelemetryRegistrationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStorageSize(value).into()
        }
        unsafe extern "system" fn UploadQuotaSize<Impl: IPlatformTelemetryRegistrationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UploadQuotaSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUploadQuotaSize<Impl: IPlatformTelemetryRegistrationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUploadQuotaSize(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlatformTelemetryRegistrationSettings, BASE_OFFSET>(),
            StorageSize: StorageSize::<Impl, IMPL_OFFSET>,
            SetStorageSize: SetStorageSize::<Impl, IMPL_OFFSET>,
            UploadQuotaSize: UploadQuotaSize::<Impl, IMPL_OFFSET>,
            SetUploadQuotaSize: SetUploadQuotaSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlatformTelemetryRegistrationSettings as ::windows::core::Interface>::IID
    }
}
