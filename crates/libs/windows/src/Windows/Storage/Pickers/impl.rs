#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileOpenPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker";
}
#[cfg(feature = "implement_exclusive")]
impl IFileOpenPickerVtbl {
    pub const fn new<Impl: IFileOpenPickerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPickerVtbl {
        unsafe extern "system" fn ViewMode<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewMode<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetViewMode(value).into()
        }
        unsafe extern "system" fn SettingsIdentifier<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SettingsIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestedStartLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitButtonText<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeFilter<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileTypeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleFileAsync<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PickSingleFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickMultipleFilesAsync<Impl: IFileOpenPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IFileOpenPicker>,
            base.5,
            ViewMode::<Impl, OFFSET>,
            SetViewMode::<Impl, OFFSET>,
            SettingsIdentifier::<Impl, OFFSET>,
            SetSettingsIdentifier::<Impl, OFFSET>,
            SuggestedStartLocation::<Impl, OFFSET>,
            SetSuggestedStartLocation::<Impl, OFFSET>,
            CommitButtonText::<Impl, OFFSET>,
            SetCommitButtonText::<Impl, OFFSET>,
            FileTypeFilter::<Impl, OFFSET>,
            PickSingleFileAsync::<Impl, OFFSET>,
            PickMultipleFilesAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSingleFileAndContinue(&self) -> ::windows::core::Result<()>;
    fn PickMultipleFilesAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileOpenPicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker2";
}
#[cfg(feature = "implement_exclusive")]
impl IFileOpenPicker2Vtbl {
    pub const fn new<Impl: IFileOpenPicker2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPicker2Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFileOpenPicker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleFileAndContinue<Impl: IFileOpenPicker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PickSingleFileAndContinue().into()
        }
        unsafe extern "system" fn PickMultipleFilesAndContinue<Impl: IFileOpenPicker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PickMultipleFilesAndContinue().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPicker2>, base.5, ContinuationData::<Impl, OFFSET>, PickSingleFileAndContinue::<Impl, OFFSET>, PickMultipleFilesAndContinue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileOpenPicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker3";
}
#[cfg(feature = "implement_exclusive")]
impl IFileOpenPicker3Vtbl {
    pub const fn new<Impl: IFileOpenPicker3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPicker3Vtbl {
        unsafe extern "system" fn User<Impl: IFileOpenPicker3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPicker3>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerStaticsImpl: Sized {
    fn ResumePickSingleFileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileOpenPickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFileOpenPickerStaticsVtbl {
    pub const fn new<Impl: IFileOpenPickerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPickerStaticsVtbl {
        unsafe extern "system" fn ResumePickSingleFileAsync<Impl: IFileOpenPickerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResumePickSingleFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPickerStatics>, base.5, ResumePickSingleFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerStatics2Impl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileOpenPicker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileOpenPickerStatics2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFileOpenPickerStatics2Vtbl {
    pub const fn new<Impl: IFileOpenPickerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPickerStatics2Vtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFileOpenPickerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPickerStatics2>, base.5, CreateForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileOpenPickerWithOperationIdImpl: Sized {
    fn PickSingleFileAsync(&self, pickeroperationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileOpenPickerWithOperationId {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerWithOperationId";
}
#[cfg(feature = "implement_exclusive")]
impl IFileOpenPickerWithOperationIdVtbl {
    pub const fn new<Impl: IFileOpenPickerWithOperationIdImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileOpenPickerWithOperationIdVtbl {
        unsafe extern "system" fn PickSingleFileAsync<Impl: IFileOpenPickerWithOperationIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pickeroperationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PickSingleFileAsync(&*(&pickeroperationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileOpenPickerWithOperationId>, base.5, PickSingleFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileSavePicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker";
}
#[cfg(feature = "implement_exclusive")]
impl IFileSavePickerVtbl {
    pub const fn new<Impl: IFileSavePickerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePickerVtbl {
        unsafe extern "system" fn SettingsIdentifier<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SettingsIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestedStartLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitButtonText<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeChoices<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileTypeChoices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultFileExtension<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultFileExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultFileExtension<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultFileExtension(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedSaveFile<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestedSaveFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedSaveFile<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuggestedSaveFile(&*(&value as *const <super::StorageFile as ::windows::core::Abi>::Abi as *const <super::StorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedFileName<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestedFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedFileName<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuggestedFileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PickSaveFileAsync<Impl: IFileSavePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IFileSavePicker>,
            base.5,
            SettingsIdentifier::<Impl, OFFSET>,
            SetSettingsIdentifier::<Impl, OFFSET>,
            SuggestedStartLocation::<Impl, OFFSET>,
            SetSuggestedStartLocation::<Impl, OFFSET>,
            CommitButtonText::<Impl, OFFSET>,
            SetCommitButtonText::<Impl, OFFSET>,
            FileTypeChoices::<Impl, OFFSET>,
            DefaultFileExtension::<Impl, OFFSET>,
            SetDefaultFileExtension::<Impl, OFFSET>,
            SuggestedSaveFile::<Impl, OFFSET>,
            SetSuggestedSaveFile::<Impl, OFFSET>,
            SuggestedFileName::<Impl, OFFSET>,
            SetSuggestedFileName::<Impl, OFFSET>,
            PickSaveFileAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSaveFileAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileSavePicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker2";
}
#[cfg(feature = "implement_exclusive")]
impl IFileSavePicker2Vtbl {
    pub const fn new<Impl: IFileSavePicker2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePicker2Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFileSavePicker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSaveFileAndContinue<Impl: IFileSavePicker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PickSaveFileAndContinue().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileSavePicker2>, base.5, ContinuationData::<Impl, OFFSET>, PickSaveFileAndContinue::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFileSavePicker3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePicker3Vtbl {
        unsafe extern "system" fn EnterpriseId<Impl: IFileSavePicker3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnterpriseId<Impl: IFileSavePicker3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnterpriseId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileSavePicker3>, base.5, EnterpriseId::<Impl, OFFSET>, SetEnterpriseId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePicker4Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileSavePicker4 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker4";
}
#[cfg(feature = "implement_exclusive")]
impl IFileSavePicker4Vtbl {
    pub const fn new<Impl: IFileSavePicker4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePicker4Vtbl {
        unsafe extern "system" fn User<Impl: IFileSavePicker4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileSavePicker4>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileSavePicker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileSavePickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePickerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFileSavePickerStaticsVtbl {
    pub const fn new<Impl: IFileSavePickerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileSavePickerStaticsVtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFileSavePickerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileSavePickerStatics>, base.5, CreateForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFolderPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker";
}
#[cfg(feature = "implement_exclusive")]
impl IFolderPickerVtbl {
    pub const fn new<Impl: IFolderPickerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFolderPickerVtbl {
        unsafe extern "system" fn ViewMode<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewMode<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetViewMode(value).into()
        }
        unsafe extern "system" fn SettingsIdentifier<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SettingsIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestedStartLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitButtonText<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeFilter<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileTypeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleFolderAsync<Impl: IFolderPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PickSingleFolderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFolderPicker>, base.5, ViewMode::<Impl, OFFSET>, SetViewMode::<Impl, OFFSET>, SettingsIdentifier::<Impl, OFFSET>, SetSettingsIdentifier::<Impl, OFFSET>, SuggestedStartLocation::<Impl, OFFSET>, SetSuggestedStartLocation::<Impl, OFFSET>, CommitButtonText::<Impl, OFFSET>, SetCommitButtonText::<Impl, OFFSET>, FileTypeFilter::<Impl, OFFSET>, PickSingleFolderAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderPicker2Impl: Sized {
    fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickFolderAndContinue(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFolderPicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker2";
}
#[cfg(feature = "implement_exclusive")]
impl IFolderPicker2Vtbl {
    pub const fn new<Impl: IFolderPicker2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFolderPicker2Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFolderPicker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContinuationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickFolderAndContinue<Impl: IFolderPicker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PickFolderAndContinue().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFolderPicker2>, base.5, ContinuationData::<Impl, OFFSET>, PickFolderAndContinue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFolderPicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker3";
}
#[cfg(feature = "implement_exclusive")]
impl IFolderPicker3Vtbl {
    pub const fn new<Impl: IFolderPicker3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFolderPicker3Vtbl {
        unsafe extern "system" fn User<Impl: IFolderPicker3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFolderPicker3>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderPickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FolderPicker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFolderPickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPickerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFolderPickerStaticsVtbl {
    pub const fn new<Impl: IFolderPickerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFolderPickerStaticsVtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFolderPickerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFolderPickerStatics>, base.5, CreateForUser::<Impl, OFFSET>)
    }
}
