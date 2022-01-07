#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsInfoStaticsImpl: Sized {
    fn VersionInfo(&self) -> ::windows::core::Result<AnalyticsVersionInfo>;
    fn DeviceForm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnalyticsInfoStatics {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAnalyticsInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsInfoStaticsImpl, const OFFSET: isize>() -> IAnalyticsInfoStaticsVtbl {
        unsafe extern "system" fn VersionInfo<Impl: IAnalyticsInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceForm<Impl: IAnalyticsInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceForm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnalyticsInfoStatics>, ::windows::core::GetTrustLevel, VersionInfo::<Impl, OFFSET>, DeviceForm::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsInfoStatics2Impl: Sized {
    fn GetSystemPropertiesAsync(&self, attributenames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnalyticsInfoStatics2 {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAnalyticsInfoStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsInfoStatics2Impl, const OFFSET: isize>() -> IAnalyticsInfoStatics2Vtbl {
        unsafe extern "system" fn GetSystemPropertiesAsync<Impl: IAnalyticsInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemPropertiesAsync(&*(&attributenames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnalyticsInfoStatics2>, ::windows::core::GetTrustLevel, GetSystemPropertiesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsVersionInfoImpl: Sized {
    fn DeviceFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceFamilyVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnalyticsVersionInfo {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsVersionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAnalyticsVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsVersionInfoImpl, const OFFSET: isize>() -> IAnalyticsVersionInfoVtbl {
        unsafe extern "system" fn DeviceFamily<Impl: IAnalyticsVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceFamilyVersion<Impl: IAnalyticsVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceFamilyVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnalyticsVersionInfo>, ::windows::core::GetTrustLevel, DeviceFamily::<Impl, OFFSET>, DeviceFamilyVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsVersionInfo2Impl: Sized {
    fn ProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnalyticsVersionInfo2 {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsVersionInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAnalyticsVersionInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsVersionInfo2Impl, const OFFSET: isize>() -> IAnalyticsVersionInfo2Vtbl {
        unsafe extern "system" fn ProductName<Impl: IAnalyticsVersionInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnalyticsVersionInfo2>, ::windows::core::GetTrustLevel, ProductName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppApplicabilityStaticsImpl: Sized {
    fn GetUnsupportedAppRequirements(&self, capabilities: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UnsupportedAppRequirement>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppApplicabilityStatics {
    const NAME: &'static str = "Windows.System.Profile.IAppApplicabilityStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppApplicabilityStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppApplicabilityStaticsImpl, const OFFSET: isize>() -> IAppApplicabilityStaticsVtbl {
        unsafe extern "system" fn GetUnsupportedAppRequirements<Impl: IAppApplicabilityStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnsupportedAppRequirements(&*(&capabilities as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppApplicabilityStatics>, ::windows::core::GetTrustLevel, GetUnsupportedAppRequirements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEducationSettingsStaticsImpl: Sized {
    fn IsEducationEnvironment(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEducationSettingsStatics {
    const NAME: &'static str = "Windows.System.Profile.IEducationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEducationSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEducationSettingsStaticsImpl, const OFFSET: isize>() -> IEducationSettingsStaticsVtbl {
        unsafe extern "system" fn IsEducationEnvironment<Impl: IEducationSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEducationEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEducationSettingsStatics>, ::windows::core::GetTrustLevel, IsEducationEnvironment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareIdentificationStaticsImpl: Sized {
    fn GetPackageSpecificToken(&self, nonce: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<HardwareToken>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHardwareIdentificationStatics {
    const NAME: &'static str = "Windows.System.Profile.IHardwareIdentificationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHardwareIdentificationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareIdentificationStaticsImpl, const OFFSET: isize>() -> IHardwareIdentificationStaticsVtbl {
        unsafe extern "system" fn GetPackageSpecificToken<Impl: IHardwareIdentificationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonce: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageSpecificToken(&*(&nonce as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHardwareIdentificationStatics>, ::windows::core::GetTrustLevel, GetPackageSpecificToken::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareTokenImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Signature(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Certificate(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHardwareToken {
    const NAME: &'static str = "Windows.System.Profile.IHardwareToken";
}
#[cfg(feature = "implement_exclusive")]
impl IHardwareTokenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareTokenImpl, const OFFSET: isize>() -> IHardwareTokenVtbl {
        unsafe extern "system" fn Id<Impl: IHardwareTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Signature<Impl: IHardwareTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Certificate<Impl: IHardwareTokenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHardwareToken>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Signature::<Impl, OFFSET>, Certificate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownRetailInfoPropertiesStaticsImpl: Sized {
    fn RetailAccessCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Price(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsFeatured(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormFactor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ScreenSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Weight(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BatteryLifeDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProcessorDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Memory(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GraphicsDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrontCameraDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RearCameraDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasNfc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasSdSlot(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasOpticalDrive(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOfficeInstalled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WindowsEdition(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownRetailInfoPropertiesStatics {
    const NAME: &'static str = "Windows.System.Profile.IKnownRetailInfoPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownRetailInfoPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>() -> IKnownRetailInfoPropertiesStaticsVtbl {
        unsafe extern "system" fn RetailAccessCode<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetailAccessCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayModelName<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Price<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Price() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFeatured<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFeatured() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormFactor<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScreenSize<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayDescription<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatteryLifeDescription<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatteryLifeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessorDescription<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessorDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Memory<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Memory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageDescription<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GraphicsDescription<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraphicsDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrontCameraDescription<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrontCameraDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RearCameraDescription<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RearCameraDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNfc<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNfc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasSdSlot<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasSdSlot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasOpticalDrive<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasOpticalDrive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOfficeInstalled<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOfficeInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowsEdition<Impl: IKnownRetailInfoPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowsEdition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IKnownRetailInfoPropertiesStatics>,
            ::windows::core::GetTrustLevel,
            RetailAccessCode::<Impl, OFFSET>,
            ManufacturerName::<Impl, OFFSET>,
            ModelName::<Impl, OFFSET>,
            DisplayModelName::<Impl, OFFSET>,
            Price::<Impl, OFFSET>,
            IsFeatured::<Impl, OFFSET>,
            FormFactor::<Impl, OFFSET>,
            ScreenSize::<Impl, OFFSET>,
            Weight::<Impl, OFFSET>,
            DisplayDescription::<Impl, OFFSET>,
            BatteryLifeDescription::<Impl, OFFSET>,
            ProcessorDescription::<Impl, OFFSET>,
            Memory::<Impl, OFFSET>,
            StorageDescription::<Impl, OFFSET>,
            GraphicsDescription::<Impl, OFFSET>,
            FrontCameraDescription::<Impl, OFFSET>,
            RearCameraDescription::<Impl, OFFSET>,
            HasNfc::<Impl, OFFSET>,
            HasSdSlot::<Impl, OFFSET>,
            HasOpticalDrive::<Impl, OFFSET>,
            IsOfficeInstalled::<Impl, OFFSET>,
            WindowsEdition::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl: Sized {
    fn CollectionLevel(&self) -> ::windows::core::Result<PlatformDataCollectionLevel>;
    fn CollectionLevelChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCollectionLevelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanCollectDiagnostics(&self, level: PlatformDataCollectionLevel) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    const NAME: &'static str = "Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl, const OFFSET: isize>() -> IPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl {
        unsafe extern "system" fn CollectionLevel<Impl: IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlatformDataCollectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionLevelChanged<Impl: IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionLevelChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCollectionLevelChanged<Impl: IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCollectionLevelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanCollectDiagnostics<Impl: IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: PlatformDataCollectionLevel, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCollectDiagnostics(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlatformDiagnosticsAndUsageDataSettingsStatics>, ::windows::core::GetTrustLevel, CollectionLevel::<Impl, OFFSET>, CollectionLevelChanged::<Impl, OFFSET>, RemoveCollectionLevelChanged::<Impl, OFFSET>, CanCollectDiagnostics::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRetailInfoStaticsImpl: Sized {
    fn IsDemoModeEnabled(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRetailInfoStatics {
    const NAME: &'static str = "Windows.System.Profile.IRetailInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRetailInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRetailInfoStaticsImpl, const OFFSET: isize>() -> IRetailInfoStaticsVtbl {
        unsafe extern "system" fn IsDemoModeEnabled<Impl: IRetailInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDemoModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IRetailInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRetailInfoStatics>, ::windows::core::GetTrustLevel, IsDemoModeEnabled::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedModeSettingsStaticsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedModeSettingsStatics {
    const NAME: &'static str = "Windows.System.Profile.ISharedModeSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedModeSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedModeSettingsStaticsImpl, const OFFSET: isize>() -> ISharedModeSettingsStaticsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: ISharedModeSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISharedModeSettingsStatics>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedModeSettingsStatics2Impl: Sized {
    fn ShouldAvoidLocalStorage(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedModeSettingsStatics2 {
    const NAME: &'static str = "Windows.System.Profile.ISharedModeSettingsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedModeSettingsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedModeSettingsStatics2Impl, const OFFSET: isize>() -> ISharedModeSettingsStatics2Vtbl {
        unsafe extern "system" fn ShouldAvoidLocalStorage<Impl: ISharedModeSettingsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldAvoidLocalStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISharedModeSettingsStatics2>, ::windows::core::GetTrustLevel, ShouldAvoidLocalStorage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemIdentificationInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Source(&self) -> ::windows::core::Result<SystemIdentificationSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemIdentificationInfo {
    const NAME: &'static str = "Windows.System.Profile.ISystemIdentificationInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemIdentificationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemIdentificationInfoImpl, const OFFSET: isize>() -> ISystemIdentificationInfoVtbl {
        unsafe extern "system" fn Id<Impl: ISystemIdentificationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Source<Impl: ISystemIdentificationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemIdentificationSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemIdentificationInfo>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Source::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemIdentificationStaticsImpl: Sized {
    fn GetSystemIdForPublisher(&self) -> ::windows::core::Result<SystemIdentificationInfo>;
    fn GetSystemIdForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<SystemIdentificationInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemIdentificationStatics {
    const NAME: &'static str = "Windows.System.Profile.ISystemIdentificationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemIdentificationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemIdentificationStaticsImpl, const OFFSET: isize>() -> ISystemIdentificationStaticsVtbl {
        unsafe extern "system" fn GetSystemIdForPublisher<Impl: ISystemIdentificationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemIdForPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemIdForUser<Impl: ISystemIdentificationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemIdForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemIdentificationStatics>, ::windows::core::GetTrustLevel, GetSystemIdForPublisher::<Impl, OFFSET>, GetSystemIdForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSetupInfoStaticsImpl: Sized {
    fn OutOfBoxExperienceState(&self) -> ::windows::core::Result<SystemOutOfBoxExperienceState>;
    fn OutOfBoxExperienceStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOutOfBoxExperienceStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemSetupInfoStatics {
    const NAME: &'static str = "Windows.System.Profile.ISystemSetupInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemSetupInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemSetupInfoStaticsImpl, const OFFSET: isize>() -> ISystemSetupInfoStaticsVtbl {
        unsafe extern "system" fn OutOfBoxExperienceState<Impl: ISystemSetupInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemOutOfBoxExperienceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutOfBoxExperienceState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutOfBoxExperienceStateChanged<Impl: ISystemSetupInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutOfBoxExperienceStateChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOutOfBoxExperienceStateChanged<Impl: ISystemSetupInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOutOfBoxExperienceStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemSetupInfoStatics>, ::windows::core::GetTrustLevel, OutOfBoxExperienceState::<Impl, OFFSET>, OutOfBoxExperienceStateChanged::<Impl, OFFSET>, RemoveOutOfBoxExperienceStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnsupportedAppRequirementImpl: Sized {
    fn Requirement(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reasons(&self) -> ::windows::core::Result<UnsupportedAppRequirementReasons>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnsupportedAppRequirement {
    const NAME: &'static str = "Windows.System.Profile.IUnsupportedAppRequirement";
}
#[cfg(feature = "implement_exclusive")]
impl IUnsupportedAppRequirementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnsupportedAppRequirementImpl, const OFFSET: isize>() -> IUnsupportedAppRequirementVtbl {
        unsafe extern "system" fn Requirement<Impl: IUnsupportedAppRequirementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Requirement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reasons<Impl: IUnsupportedAppRequirementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnsupportedAppRequirementReasons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reasons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUnsupportedAppRequirement>, ::windows::core::GetTrustLevel, Requirement::<Impl, OFFSET>, Reasons::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowsIntegrityPolicyStaticsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsEnabledForTrial(&self) -> ::windows::core::Result<bool>;
    fn CanDisable(&self) -> ::windows::core::Result<bool>;
    fn IsDisableSupported(&self) -> ::windows::core::Result<bool>;
    fn PolicyChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePolicyChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowsIntegrityPolicyStatics {
    const NAME: &'static str = "Windows.System.Profile.IWindowsIntegrityPolicyStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowsIntegrityPolicyStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsIntegrityPolicyStaticsImpl, const OFFSET: isize>() -> IWindowsIntegrityPolicyStaticsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IWindowsIntegrityPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledForTrial<Impl: IWindowsIntegrityPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledForTrial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDisable<Impl: IWindowsIntegrityPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDisable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisableSupported<Impl: IWindowsIntegrityPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisableSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyChanged<Impl: IWindowsIntegrityPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePolicyChanged<Impl: IWindowsIntegrityPolicyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePolicyChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsIntegrityPolicyStatics>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, OFFSET>, IsEnabledForTrial::<Impl, OFFSET>, CanDisable::<Impl, OFFSET>, IsDisableSupported::<Impl, OFFSET>, PolicyChanged::<Impl, OFFSET>, RemovePolicyChanged::<Impl, OFFSET>)
    }
}
