#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileOpenPickerImpl: Sized {
    fn ViewMode(&self) -> ::windows::core::Result<PickerViewMode>;
    fn SetViewMode(&self, value: PickerViewMode) -> ::windows::core::Result<()>;
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn PickMultipleFilesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileOpenPickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerVtbl {
        unsafe extern "system" fn ViewMode<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewMode<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewMode(value).into()
        }
        unsafe extern "system" fn SettingsIdentifier<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SettingsIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedStartLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitButtonText<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeFilter<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileTypeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleFileAsync<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickMultipleFilesAsync<Impl: IFileOpenPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickMultipleFilesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFileOpenPicker>,
            ::windows::core::GetTrustLevel,
            ViewMode::<Impl, IMPL_OFFSET>,
            SetViewMode::<Impl, IMPL_OFFSET>,
            SettingsIdentifier::<Impl, IMPL_OFFSET>,
            SetSettingsIdentifier::<Impl, IMPL_OFFSET>,
            SuggestedStartLocation::<Impl, IMPL_OFFSET>,
            SetSuggestedStartLocation::<Impl, IMPL_OFFSET>,
            CommitButtonText::<Impl, IMPL_OFFSET>,
            SetCommitButtonText::<Impl, IMPL_OFFSET>,
            FileTypeFilter::<Impl, IMPL_OFFSET>,
            PickSingleFileAsync::<Impl, IMPL_OFFSET>,
            PickMultipleFilesAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileOpenPicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSingleFileAndContinue(&self) -> ::windows::core::Result<()>;
    fn PickMultipleFilesAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileOpenPicker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPicker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPicker2Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFileOpenPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleFileAndContinue<Impl: IFileOpenPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickSingleFileAndContinue().into()
        }
        unsafe extern "system" fn PickMultipleFilesAndContinue<Impl: IFileOpenPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickMultipleFilesAndContinue().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPicker2>, ::windows::core::GetTrustLevel, ContinuationData::<Impl, IMPL_OFFSET>, PickSingleFileAndContinue::<Impl, IMPL_OFFSET>, PickMultipleFilesAndContinue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPicker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileOpenPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileOpenPicker3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPicker3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPicker3Vtbl {
        unsafe extern "system" fn User<Impl: IFileOpenPicker3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPicker3>, ::windows::core::GetTrustLevel, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPicker3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFileOpenPickerStaticsImpl: Sized {
    fn ResumePickSingleFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFileOpenPickerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerStaticsVtbl {
        unsafe extern "system" fn ResumePickSingleFileAsync<Impl: IFileOpenPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumePickSingleFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPickerStatics>, ::windows::core::GetTrustLevel, ResumePickSingleFileAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileOpenPickerStatics2Impl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileOpenPicker>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPickerStatics2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerStatics2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileOpenPickerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerStatics2Vtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFileOpenPickerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPickerStatics2>, ::windows::core::GetTrustLevel, CreateForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFileOpenPickerWithOperationIdImpl: Sized {
    fn PickSingleFileAsync(&self, pickeroperationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPickerWithOperationId {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerWithOperationId";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFileOpenPickerWithOperationIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerWithOperationIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerWithOperationIdVtbl {
        unsafe extern "system" fn PickSingleFileAsync<Impl: IFileOpenPickerWithOperationIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pickeroperationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleFileAsync(&*(&pickeroperationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileOpenPickerWithOperationId>, ::windows::core::GetTrustLevel, PickSingleFileAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerWithOperationId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileSavePickerImpl: Sized {
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeChoices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn DefaultFileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultFileExtension(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedSaveFile(&self) -> ::windows::core::Result<super::StorageFile>;
    fn SetSuggestedSaveFile(&self, value: &::core::option::Option<super::StorageFile>) -> ::windows::core::Result<()>;
    fn SuggestedFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSuggestedFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PickSaveFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileSavePickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePickerVtbl {
        unsafe extern "system" fn SettingsIdentifier<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SettingsIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedStartLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitButtonText<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeChoices<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileTypeChoices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultFileExtension<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultFileExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultFileExtension<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultFileExtension(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedSaveFile<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedSaveFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedSaveFile<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedSaveFile(&*(&value as *const <super::StorageFile as ::windows::core::Abi>::Abi as *const <super::StorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedFileName<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedFileName<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedFileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PickSaveFileAsync<Impl: IFileSavePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSaveFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFileSavePicker>,
            ::windows::core::GetTrustLevel,
            SettingsIdentifier::<Impl, IMPL_OFFSET>,
            SetSettingsIdentifier::<Impl, IMPL_OFFSET>,
            SuggestedStartLocation::<Impl, IMPL_OFFSET>,
            SetSuggestedStartLocation::<Impl, IMPL_OFFSET>,
            CommitButtonText::<Impl, IMPL_OFFSET>,
            SetCommitButtonText::<Impl, IMPL_OFFSET>,
            FileTypeChoices::<Impl, IMPL_OFFSET>,
            DefaultFileExtension::<Impl, IMPL_OFFSET>,
            SetDefaultFileExtension::<Impl, IMPL_OFFSET>,
            SuggestedSaveFile::<Impl, IMPL_OFFSET>,
            SetSuggestedSaveFile::<Impl, IMPL_OFFSET>,
            SuggestedFileName::<Impl, IMPL_OFFSET>,
            SetSuggestedFileName::<Impl, IMPL_OFFSET>,
            PickSaveFileAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileSavePicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSaveFileAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileSavePicker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePicker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePicker2Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFileSavePicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSaveFileAndContinue<Impl: IFileSavePicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickSaveFileAndContinue().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSavePicker2>, ::windows::core::GetTrustLevel, ContinuationData::<Impl, IMPL_OFFSET>, PickSaveFileAndContinue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePicker3Impl: Sized {
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileSavePicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker3";
}
#[cfg(feature = "implement_exclusive")]
impl IFileSavePicker3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePicker3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePicker3Vtbl {
        unsafe extern "system" fn EnterpriseId<Impl: IFileSavePicker3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnterpriseId<Impl: IFileSavePicker3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSavePicker3>, ::windows::core::GetTrustLevel, EnterpriseId::<Impl, IMPL_OFFSET>, SetEnterpriseId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileSavePicker4Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePicker4 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker4";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileSavePicker4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePicker4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePicker4Vtbl {
        unsafe extern "system" fn User<Impl: IFileSavePicker4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSavePicker4>, ::windows::core::GetTrustLevel, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileSavePickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileSavePicker>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePickerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileSavePickerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePickerStaticsVtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFileSavePickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSavePickerStatics>, ::windows::core::GetTrustLevel, CreateForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFolderPickerImpl: Sized {
    fn ViewMode(&self) -> ::windows::core::Result<PickerViewMode>;
    fn SetViewMode(&self, value: PickerViewMode) -> ::windows::core::Result<()>;
    fn SettingsIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleFolderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFolderPickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPickerVtbl {
        unsafe extern "system" fn ViewMode<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewMode<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewMode(value).into()
        }
        unsafe extern "system" fn SettingsIdentifier<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SettingsIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedStartLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitButtonText<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeFilter<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileTypeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleFolderAsync<Impl: IFolderPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleFolderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFolderPicker>,
            ::windows::core::GetTrustLevel,
            ViewMode::<Impl, IMPL_OFFSET>,
            SetViewMode::<Impl, IMPL_OFFSET>,
            SettingsIdentifier::<Impl, IMPL_OFFSET>,
            SetSettingsIdentifier::<Impl, IMPL_OFFSET>,
            SuggestedStartLocation::<Impl, IMPL_OFFSET>,
            SetSuggestedStartLocation::<Impl, IMPL_OFFSET>,
            CommitButtonText::<Impl, IMPL_OFFSET>,
            SetCommitButtonText::<Impl, IMPL_OFFSET>,
            FileTypeFilter::<Impl, IMPL_OFFSET>,
            PickSingleFolderAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFolderPicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickFolderAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFolderPicker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPicker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPicker2Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFolderPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickFolderAndContinue<Impl: IFolderPicker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickFolderAndContinue().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFolderPicker2>, ::windows::core::GetTrustLevel, ContinuationData::<Impl, IMPL_OFFSET>, PickFolderAndContinue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPicker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFolderPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFolderPicker3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPicker3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPicker3Vtbl {
        unsafe extern "system" fn User<Impl: IFolderPicker3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFolderPicker3>, ::windows::core::GetTrustLevel, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPicker3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFolderPickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FolderPicker>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPickerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFolderPickerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPickerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPickerStaticsVtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFolderPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFolderPickerStatics>, ::windows::core::GetTrustLevel, CreateForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPickerStatics as ::windows::core::Interface>::IID
    }
}
