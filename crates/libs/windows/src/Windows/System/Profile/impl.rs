#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsInfoStaticsImpl: Sized {
    fn VersionInfo(&mut self) -> ::windows::core::Result<AnalyticsVersionInfo>;
    fn DeviceForm(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnalyticsInfoStatics {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAnalyticsInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnalyticsInfoStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnalyticsInfoStatics, BASE_OFFSET>(),
            VersionInfo: VersionInfo::<Impl, IMPL_OFFSET>,
            DeviceForm: DeviceForm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnalyticsInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAnalyticsInfoStatics2Impl: Sized {
    fn GetSystemPropertiesAsync(&mut self, attributenames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAnalyticsInfoStatics2 {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsInfoStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAnalyticsInfoStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsInfoStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnalyticsInfoStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnalyticsInfoStatics2, BASE_OFFSET>(),
            GetSystemPropertiesAsync: GetSystemPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnalyticsInfoStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsVersionInfoImpl: Sized {
    fn DeviceFamily(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceFamilyVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnalyticsVersionInfo {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsVersionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAnalyticsVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsVersionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnalyticsVersionInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnalyticsVersionInfo, BASE_OFFSET>(),
            DeviceFamily: DeviceFamily::<Impl, IMPL_OFFSET>,
            DeviceFamilyVersion: DeviceFamilyVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnalyticsVersionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsVersionInfo2Impl: Sized {
    fn ProductName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnalyticsVersionInfo2 {
    const NAME: &'static str = "Windows.System.Profile.IAnalyticsVersionInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAnalyticsVersionInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnalyticsVersionInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnalyticsVersionInfo2Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAnalyticsVersionInfo2, BASE_OFFSET>(), ProductName: ProductName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnalyticsVersionInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppApplicabilityStaticsImpl: Sized {
    fn GetUnsupportedAppRequirements(&mut self, capabilities: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UnsupportedAppRequirement>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppApplicabilityStatics {
    const NAME: &'static str = "Windows.System.Profile.IAppApplicabilityStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppApplicabilityStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppApplicabilityStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppApplicabilityStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppApplicabilityStatics, BASE_OFFSET>(),
            GetUnsupportedAppRequirements: GetUnsupportedAppRequirements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppApplicabilityStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEducationSettingsStaticsImpl: Sized {
    fn IsEducationEnvironment(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEducationSettingsStatics {
    const NAME: &'static str = "Windows.System.Profile.IEducationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEducationSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEducationSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEducationSettingsStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEducationSettingsStatics, BASE_OFFSET>(),
            IsEducationEnvironment: IsEducationEnvironment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEducationSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHardwareIdentificationStaticsImpl: Sized {
    fn GetPackageSpecificToken(&mut self, nonce: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<HardwareToken>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHardwareIdentificationStatics {
    const NAME: &'static str = "Windows.System.Profile.IHardwareIdentificationStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHardwareIdentificationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareIdentificationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHardwareIdentificationStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHardwareIdentificationStatics, BASE_OFFSET>(),
            GetPackageSpecificToken: GetPackageSpecificToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHardwareIdentificationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHardwareTokenImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Signature(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Certificate(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHardwareToken {
    const NAME: &'static str = "Windows.System.Profile.IHardwareToken";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHardwareTokenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareTokenImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHardwareTokenVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHardwareToken, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Signature: Signature::<Impl, IMPL_OFFSET>,
            Certificate: Certificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHardwareToken as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownRetailInfoPropertiesStaticsImpl: Sized {
    fn RetailAccessCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayModelName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Price(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsFeatured(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormFactor(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ScreenSize(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Weight(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BatteryLifeDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProcessorDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Memory(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GraphicsDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrontCameraDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RearCameraDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasNfc(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasSdSlot(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasOpticalDrive(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOfficeInstalled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WindowsEdition(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownRetailInfoPropertiesStatics {
    const NAME: &'static str = "Windows.System.Profile.IKnownRetailInfoPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownRetailInfoPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownRetailInfoPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownRetailInfoPropertiesStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownRetailInfoPropertiesStatics, BASE_OFFSET>(),
            RetailAccessCode: RetailAccessCode::<Impl, IMPL_OFFSET>,
            ManufacturerName: ManufacturerName::<Impl, IMPL_OFFSET>,
            ModelName: ModelName::<Impl, IMPL_OFFSET>,
            DisplayModelName: DisplayModelName::<Impl, IMPL_OFFSET>,
            Price: Price::<Impl, IMPL_OFFSET>,
            IsFeatured: IsFeatured::<Impl, IMPL_OFFSET>,
            FormFactor: FormFactor::<Impl, IMPL_OFFSET>,
            ScreenSize: ScreenSize::<Impl, IMPL_OFFSET>,
            Weight: Weight::<Impl, IMPL_OFFSET>,
            DisplayDescription: DisplayDescription::<Impl, IMPL_OFFSET>,
            BatteryLifeDescription: BatteryLifeDescription::<Impl, IMPL_OFFSET>,
            ProcessorDescription: ProcessorDescription::<Impl, IMPL_OFFSET>,
            Memory: Memory::<Impl, IMPL_OFFSET>,
            StorageDescription: StorageDescription::<Impl, IMPL_OFFSET>,
            GraphicsDescription: GraphicsDescription::<Impl, IMPL_OFFSET>,
            FrontCameraDescription: FrontCameraDescription::<Impl, IMPL_OFFSET>,
            RearCameraDescription: RearCameraDescription::<Impl, IMPL_OFFSET>,
            HasNfc: HasNfc::<Impl, IMPL_OFFSET>,
            HasSdSlot: HasSdSlot::<Impl, IMPL_OFFSET>,
            HasOpticalDrive: HasOpticalDrive::<Impl, IMPL_OFFSET>,
            IsOfficeInstalled: IsOfficeInstalled::<Impl, IMPL_OFFSET>,
            WindowsEdition: WindowsEdition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownRetailInfoPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl: Sized {
    fn CollectionLevel(&mut self) -> ::windows::core::Result<PlatformDataCollectionLevel>;
    fn CollectionLevelChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCollectionLevelChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanCollectDiagnostics(&mut self, level: PlatformDataCollectionLevel) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    const NAME: &'static str = "Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlatformDiagnosticsAndUsageDataSettingsStatics, BASE_OFFSET>(),
            CollectionLevel: CollectionLevel::<Impl, IMPL_OFFSET>,
            CollectionLevelChanged: CollectionLevelChanged::<Impl, IMPL_OFFSET>,
            RemoveCollectionLevelChanged: RemoveCollectionLevelChanged::<Impl, IMPL_OFFSET>,
            CanCollectDiagnostics: CanCollectDiagnostics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlatformDiagnosticsAndUsageDataSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRetailInfoStaticsImpl: Sized {
    fn IsDemoModeEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRetailInfoStatics {
    const NAME: &'static str = "Windows.System.Profile.IRetailInfoStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRetailInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRetailInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRetailInfoStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRetailInfoStatics, BASE_OFFSET>(),
            IsDemoModeEnabled: IsDemoModeEnabled::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRetailInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedModeSettingsStaticsImpl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedModeSettingsStatics {
    const NAME: &'static str = "Windows.System.Profile.ISharedModeSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedModeSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedModeSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedModeSettingsStaticsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedModeSettingsStatics, BASE_OFFSET>(), IsEnabled: IsEnabled::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedModeSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedModeSettingsStatics2Impl: Sized {
    fn ShouldAvoidLocalStorage(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedModeSettingsStatics2 {
    const NAME: &'static str = "Windows.System.Profile.ISharedModeSettingsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedModeSettingsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedModeSettingsStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedModeSettingsStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedModeSettingsStatics2, BASE_OFFSET>(),
            ShouldAvoidLocalStorage: ShouldAvoidLocalStorage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedModeSettingsStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISystemIdentificationInfoImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Source(&mut self) -> ::windows::core::Result<SystemIdentificationSource>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemIdentificationInfo {
    const NAME: &'static str = "Windows.System.Profile.ISystemIdentificationInfo";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISystemIdentificationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemIdentificationInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemIdentificationInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemIdentificationInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemIdentificationInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemIdentificationStaticsImpl: Sized {
    fn GetSystemIdForPublisher(&mut self) -> ::windows::core::Result<SystemIdentificationInfo>;
    fn GetSystemIdForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<SystemIdentificationInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemIdentificationStatics {
    const NAME: &'static str = "Windows.System.Profile.ISystemIdentificationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemIdentificationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemIdentificationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemIdentificationStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemIdentificationStatics, BASE_OFFSET>(),
            GetSystemIdForPublisher: GetSystemIdForPublisher::<Impl, IMPL_OFFSET>,
            GetSystemIdForUser: GetSystemIdForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemIdentificationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemSetupInfoStaticsImpl: Sized {
    fn OutOfBoxExperienceState(&mut self) -> ::windows::core::Result<SystemOutOfBoxExperienceState>;
    fn OutOfBoxExperienceStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOutOfBoxExperienceStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemSetupInfoStatics {
    const NAME: &'static str = "Windows.System.Profile.ISystemSetupInfoStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemSetupInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemSetupInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemSetupInfoStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemSetupInfoStatics, BASE_OFFSET>(),
            OutOfBoxExperienceState: OutOfBoxExperienceState::<Impl, IMPL_OFFSET>,
            OutOfBoxExperienceStateChanged: OutOfBoxExperienceStateChanged::<Impl, IMPL_OFFSET>,
            RemoveOutOfBoxExperienceStateChanged: RemoveOutOfBoxExperienceStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemSetupInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnsupportedAppRequirementImpl: Sized {
    fn Requirement(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reasons(&mut self) -> ::windows::core::Result<UnsupportedAppRequirementReasons>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnsupportedAppRequirement {
    const NAME: &'static str = "Windows.System.Profile.IUnsupportedAppRequirement";
}
#[cfg(feature = "implement_exclusive")]
impl IUnsupportedAppRequirementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnsupportedAppRequirementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnsupportedAppRequirementVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnsupportedAppRequirement, BASE_OFFSET>(),
            Requirement: Requirement::<Impl, IMPL_OFFSET>,
            Reasons: Reasons::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnsupportedAppRequirement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWindowsIntegrityPolicyStaticsImpl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnabledForTrial(&mut self) -> ::windows::core::Result<bool>;
    fn CanDisable(&mut self) -> ::windows::core::Result<bool>;
    fn IsDisableSupported(&mut self) -> ::windows::core::Result<bool>;
    fn PolicyChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePolicyChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowsIntegrityPolicyStatics {
    const NAME: &'static str = "Windows.System.Profile.IWindowsIntegrityPolicyStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWindowsIntegrityPolicyStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsIntegrityPolicyStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsIntegrityPolicyStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowsIntegrityPolicyStatics, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            IsEnabledForTrial: IsEnabledForTrial::<Impl, IMPL_OFFSET>,
            CanDisable: CanDisable::<Impl, IMPL_OFFSET>,
            IsDisableSupported: IsDisableSupported::<Impl, IMPL_OFFSET>,
            PolicyChanged: PolicyChanged::<Impl, IMPL_OFFSET>,
            RemovePolicyChanged: RemovePolicyChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsIntegrityPolicyStatics as ::windows::core::Interface>::IID
    }
}
