#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "Storage_Streams", feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaActionableInsights_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
    fn IsAvailableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowInsightsForImageAsync(&mut self, imagestream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsForImageWithOptionsAsync(&mut self, imagestream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, options: &::core::option::Option<CortanaActionableInsightsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsForTextAsync(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsForTextWithOptionsAsync(&mut self, text: &::windows::core::HSTRING, options: &::core::option::Option<CortanaActionableInsightsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsAsync(&mut self, datapackage: &::core::option::Option<super::super::ApplicationModel::DataTransfer::DataPackage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsWithOptionsAsync(&mut self, datapackage: &::core::option::Option<super::super::ApplicationModel::DataTransfer::DataPackage>, options: &::core::option::Option<CortanaActionableInsightsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "Storage_Streams", feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaActionableInsights {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaActionableInsights";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "Storage_Streams", feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaActionableInsights_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaActionableInsights_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaActionableInsights_Vtbl {
        unsafe extern "system" fn User<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailableAsync<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowInsightsForImageAsync<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagestream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowInsightsForImageAsync(&*(&imagestream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowInsightsForImageWithOptionsAsync<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagestream: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowInsightsForImageWithOptionsAsync(&*(&imagestream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <CortanaActionableInsightsOptions as ::windows::core::Abi>::Abi as *const <CortanaActionableInsightsOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowInsightsForTextAsync<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowInsightsForTextAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowInsightsForTextWithOptionsAsync<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowInsightsForTextWithOptionsAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <CortanaActionableInsightsOptions as ::windows::core::Abi>::Abi as *const <CortanaActionableInsightsOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowInsightsAsync<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datapackage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowInsightsAsync(&*(&datapackage as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowInsightsWithOptionsAsync<Impl: ICortanaActionableInsights_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datapackage: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowInsightsWithOptionsAsync(&*(&datapackage as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <CortanaActionableInsightsOptions as ::windows::core::Abi>::Abi as *const <CortanaActionableInsightsOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaActionableInsights, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            IsAvailableAsync: IsAvailableAsync::<Impl, IMPL_OFFSET>,
            ShowInsightsForImageAsync: ShowInsightsForImageAsync::<Impl, IMPL_OFFSET>,
            ShowInsightsForImageWithOptionsAsync: ShowInsightsForImageWithOptionsAsync::<Impl, IMPL_OFFSET>,
            ShowInsightsForTextAsync: ShowInsightsForTextAsync::<Impl, IMPL_OFFSET>,
            ShowInsightsForTextWithOptionsAsync: ShowInsightsForTextWithOptionsAsync::<Impl, IMPL_OFFSET>,
            ShowInsightsAsync: ShowInsightsAsync::<Impl, IMPL_OFFSET>,
            ShowInsightsWithOptionsAsync: ShowInsightsWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaActionableInsights as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaActionableInsightsOptions_Impl: Sized {
    fn ContentSourceWebLink(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceWebLink(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SurroundingText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSurroundingText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaActionableInsightsOptions {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaActionableInsightsOptions";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaActionableInsightsOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaActionableInsightsOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaActionableInsightsOptions_Vtbl {
        unsafe extern "system" fn ContentSourceWebLink<Impl: ICortanaActionableInsightsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSourceWebLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentSourceWebLink<Impl: ICortanaActionableInsightsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentSourceWebLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SurroundingText<Impl: ICortanaActionableInsightsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SurroundingText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSurroundingText<Impl: ICortanaActionableInsightsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurroundingText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaActionableInsightsOptions, BASE_OFFSET>(),
            ContentSourceWebLink: ContentSourceWebLink::<Impl, IMPL_OFFSET>,
            SetContentSourceWebLink: SetContentSourceWebLink::<Impl, IMPL_OFFSET>,
            SurroundingText: SurroundingText::<Impl, IMPL_OFFSET>,
            SetSurroundingText: SetSurroundingText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaActionableInsightsOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaActionableInsightsStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<CortanaActionableInsights>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<CortanaActionableInsights>;
}
#[cfg(all(feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaActionableInsightsStatics {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaActionableInsightsStatics";
}
#[cfg(all(feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaActionableInsightsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaActionableInsightsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaActionableInsightsStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ICortanaActionableInsightsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: ICortanaActionableInsightsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaActionableInsightsStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaActionableInsightsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaPermissionsManager_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn ArePermissionsGrantedAsync(&mut self, permissions: &::core::option::Option<super::super::Foundation::Collections::IIterable<CortanaPermission>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GrantPermissionsAsync(&mut self, permissions: &::core::option::Option<super::super::Foundation::Collections::IIterable<CortanaPermission>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>;
    fn RevokePermissionsAsync(&mut self, permissions: &::core::option::Option<super::super::Foundation::Collections::IIterable<CortanaPermission>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaPermissionsManager {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaPermissionsManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaPermissionsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaPermissionsManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaPermissionsManager_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: ICortanaPermissionsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArePermissionsGrantedAsync<Impl: ICortanaPermissionsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, permissions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArePermissionsGrantedAsync(&*(&permissions as *const <super::super::Foundation::Collections::IIterable<CortanaPermission> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<CortanaPermission> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrantPermissionsAsync<Impl: ICortanaPermissionsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, permissions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrantPermissionsAsync(&*(&permissions as *const <super::super::Foundation::Collections::IIterable<CortanaPermission> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<CortanaPermission> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokePermissionsAsync<Impl: ICortanaPermissionsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, permissions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevokePermissionsAsync(&*(&permissions as *const <super::super::Foundation::Collections::IIterable<CortanaPermission> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<CortanaPermission> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaPermissionsManager, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            ArePermissionsGrantedAsync: ArePermissionsGrantedAsync::<Impl, IMPL_OFFSET>,
            GrantPermissionsAsync: GrantPermissionsAsync::<Impl, IMPL_OFFSET>,
            RevokePermissionsAsync: RevokePermissionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaPermissionsManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaPermissionsManagerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<CortanaPermissionsManager>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaPermissionsManagerStatics {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaPermissionsManagerStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaPermissionsManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaPermissionsManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaPermissionsManagerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ICortanaPermissionsManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaPermissionsManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaPermissionsManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaSettings_Impl: Sized {
    fn HasUserConsentToVoiceActivation(&mut self) -> ::windows::core::Result<bool>;
    fn IsVoiceActivationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsVoiceActivationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaSettings {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaSettings";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaSettings_Vtbl {
        unsafe extern "system" fn HasUserConsentToVoiceActivation<Impl: ICortanaSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasUserConsentToVoiceActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVoiceActivationEnabled<Impl: ICortanaSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVoiceActivationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsVoiceActivationEnabled<Impl: ICortanaSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVoiceActivationEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaSettings, BASE_OFFSET>(),
            HasUserConsentToVoiceActivation: HasUserConsentToVoiceActivation::<Impl, IMPL_OFFSET>,
            IsVoiceActivationEnabled: IsVoiceActivationEnabled::<Impl, IMPL_OFFSET>,
            SetIsVoiceActivationEnabled: SetIsVoiceActivationEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaSettingsStatics_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn GetDefault(&mut self) -> ::windows::core::Result<CortanaSettings>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaSettingsStatics {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaSettingsStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaSettingsStatics_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: ICortanaSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Impl: ICortanaSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaSettingsStatics, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaSettingsStatics as ::windows::core::Interface>::IID
    }
}
