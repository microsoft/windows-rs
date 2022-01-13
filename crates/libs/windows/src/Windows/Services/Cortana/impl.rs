#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "Storage_Streams", feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaActionableInsightsImpl: Sized {
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
impl ICortanaActionableInsightsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaActionableInsightsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaActionableInsightsVtbl {
        unsafe extern "system" fn User<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAvailableAsync<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowInsightsForImageAsync<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagestream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowInsightsForImageWithOptionsAsync<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagestream: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowInsightsForTextAsync<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowInsightsForTextWithOptionsAsync<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowInsightsAsync<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datapackage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowInsightsWithOptionsAsync<Impl: ICortanaActionableInsightsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datapackage: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICortanaActionableInsightsOptionsImpl: Sized {
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
impl ICortanaActionableInsightsOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaActionableInsightsOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaActionableInsightsOptionsVtbl {
        unsafe extern "system" fn ContentSourceWebLink<Impl: ICortanaActionableInsightsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentSourceWebLink<Impl: ICortanaActionableInsightsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentSourceWebLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SurroundingText<Impl: ICortanaActionableInsightsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSurroundingText<Impl: ICortanaActionableInsightsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ICortanaActionableInsightsStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<CortanaActionableInsights>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<CortanaActionableInsights>;
}
#[cfg(all(feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaActionableInsightsStatics {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaActionableInsightsStatics";
}
#[cfg(all(feature = "System", feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaActionableInsightsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaActionableInsightsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaActionableInsightsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ICortanaActionableInsightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: ICortanaActionableInsightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICortanaPermissionsManagerImpl: Sized {
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
impl ICortanaPermissionsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaPermissionsManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaPermissionsManagerVtbl {
        unsafe extern "system" fn IsSupported<Impl: ICortanaPermissionsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ArePermissionsGrantedAsync<Impl: ICortanaPermissionsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, permissions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GrantPermissionsAsync<Impl: ICortanaPermissionsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, permissions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RevokePermissionsAsync<Impl: ICortanaPermissionsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, permissions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICortanaPermissionsManagerStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<CortanaPermissionsManager>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaPermissionsManagerStatics {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaPermissionsManagerStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaPermissionsManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaPermissionsManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaPermissionsManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ICortanaPermissionsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICortanaSettingsImpl: Sized {
    fn HasUserConsentToVoiceActivation(&mut self) -> ::windows::core::Result<bool>;
    fn IsVoiceActivationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsVoiceActivationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaSettings {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaSettings";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaSettingsVtbl {
        unsafe extern "system" fn HasUserConsentToVoiceActivation<Impl: ICortanaSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVoiceActivationEnabled<Impl: ICortanaSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsVoiceActivationEnabled<Impl: ICortanaSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ICortanaSettingsStaticsImpl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn GetDefault(&mut self) -> ::windows::core::Result<CortanaSettings>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaSettingsStatics {
    const NAME: &'static str = "Windows.Services.Cortana.ICortanaSettingsStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ICortanaSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaSettingsStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: ICortanaSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefault<Impl: ICortanaSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
