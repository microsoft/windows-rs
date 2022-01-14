#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileOpenPicker_Impl: Sized {
    fn ViewMode(&mut self) -> ::windows::core::Result<PickerViewMode>;
    fn SetViewMode(&mut self, value: PickerViewMode) -> ::windows::core::Result<()>;
    fn SettingsIdentifier(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&mut self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&mut self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeFilter(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleFileAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn PickMultipleFilesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileOpenPicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPicker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPicker_Vtbl {
        unsafe extern "system" fn ViewMode<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewMode<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewMode(value).into()
        }
        unsafe extern "system" fn SettingsIdentifier<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommitButtonText<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeFilter<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickSingleFileAsync<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickMultipleFilesAsync<Impl: IFileOpenPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPicker, BASE_OFFSET>(),
            ViewMode: ViewMode::<Impl, IMPL_OFFSET>,
            SetViewMode: SetViewMode::<Impl, IMPL_OFFSET>,
            SettingsIdentifier: SettingsIdentifier::<Impl, IMPL_OFFSET>,
            SetSettingsIdentifier: SetSettingsIdentifier::<Impl, IMPL_OFFSET>,
            SuggestedStartLocation: SuggestedStartLocation::<Impl, IMPL_OFFSET>,
            SetSuggestedStartLocation: SetSuggestedStartLocation::<Impl, IMPL_OFFSET>,
            CommitButtonText: CommitButtonText::<Impl, IMPL_OFFSET>,
            SetCommitButtonText: SetCommitButtonText::<Impl, IMPL_OFFSET>,
            FileTypeFilter: FileTypeFilter::<Impl, IMPL_OFFSET>,
            PickSingleFileAsync: PickSingleFileAsync::<Impl, IMPL_OFFSET>,
            PickMultipleFilesAsync: PickMultipleFilesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileOpenPicker2_Impl: Sized {
    fn ContinuationData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSingleFileAndContinue(&mut self) -> ::windows::core::Result<()>;
    fn PickMultipleFilesAndContinue(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileOpenPicker2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPicker2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPicker2_Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFileOpenPicker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickSingleFileAndContinue<Impl: IFileOpenPicker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickSingleFileAndContinue().into()
        }
        unsafe extern "system" fn PickMultipleFilesAndContinue<Impl: IFileOpenPicker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickMultipleFilesAndContinue().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPicker2, BASE_OFFSET>(),
            ContinuationData: ContinuationData::<Impl, IMPL_OFFSET>,
            PickSingleFileAndContinue: PickSingleFileAndContinue::<Impl, IMPL_OFFSET>,
            PickMultipleFilesAndContinue: PickMultipleFilesAndContinue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPicker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileOpenPicker3_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPicker3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileOpenPicker3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPicker3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPicker3_Vtbl {
        unsafe extern "system" fn User<Impl: IFileOpenPicker3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPicker3, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPicker3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFileOpenPickerStatics_Impl: Sized {
    fn ResumePickSingleFileAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFileOpenPickerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerStatics_Vtbl {
        unsafe extern "system" fn ResumePickSingleFileAsync<Impl: IFileOpenPickerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPickerStatics, BASE_OFFSET>(),
            ResumePickSingleFileAsync: ResumePickSingleFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileOpenPickerStatics2_Impl: Sized {
    fn CreateForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileOpenPicker>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPickerStatics2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerStatics2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileOpenPickerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerStatics2_Vtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFileOpenPickerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPickerStatics2, BASE_OFFSET>(),
            CreateForUser: CreateForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFileOpenPickerWithOperationId_Impl: Sized {
    fn PickSingleFileAsync(&mut self, pickeroperationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileOpenPickerWithOperationId {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileOpenPickerWithOperationId";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFileOpenPickerWithOperationId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileOpenPickerWithOperationId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileOpenPickerWithOperationId_Vtbl {
        unsafe extern "system" fn PickSingleFileAsync<Impl: IFileOpenPickerWithOperationId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pickeroperationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileOpenPickerWithOperationId, BASE_OFFSET>(),
            PickSingleFileAsync: PickSingleFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileOpenPickerWithOperationId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileSavePicker_Impl: Sized {
    fn SettingsIdentifier(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&mut self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&mut self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeChoices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn DefaultFileExtension(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultFileExtension(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedSaveFile(&mut self) -> ::windows::core::Result<super::StorageFile>;
    fn SetSuggestedSaveFile(&mut self, value: &::core::option::Option<super::StorageFile>) -> ::windows::core::Result<()>;
    fn SuggestedFileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSuggestedFileName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PickSaveFileAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileSavePicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePicker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePicker_Vtbl {
        unsafe extern "system" fn SettingsIdentifier<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommitButtonText<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeChoices<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DefaultFileExtension<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDefaultFileExtension<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultFileExtension(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedSaveFile<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuggestedSaveFile<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedSaveFile(&*(&value as *const <super::StorageFile as ::windows::core::Abi>::Abi as *const <super::StorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedFileName<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuggestedFileName<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedFileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PickSaveFileAsync<Impl: IFileSavePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePicker, BASE_OFFSET>(),
            SettingsIdentifier: SettingsIdentifier::<Impl, IMPL_OFFSET>,
            SetSettingsIdentifier: SetSettingsIdentifier::<Impl, IMPL_OFFSET>,
            SuggestedStartLocation: SuggestedStartLocation::<Impl, IMPL_OFFSET>,
            SetSuggestedStartLocation: SetSuggestedStartLocation::<Impl, IMPL_OFFSET>,
            CommitButtonText: CommitButtonText::<Impl, IMPL_OFFSET>,
            SetCommitButtonText: SetCommitButtonText::<Impl, IMPL_OFFSET>,
            FileTypeChoices: FileTypeChoices::<Impl, IMPL_OFFSET>,
            DefaultFileExtension: DefaultFileExtension::<Impl, IMPL_OFFSET>,
            SetDefaultFileExtension: SetDefaultFileExtension::<Impl, IMPL_OFFSET>,
            SuggestedSaveFile: SuggestedSaveFile::<Impl, IMPL_OFFSET>,
            SetSuggestedSaveFile: SetSuggestedSaveFile::<Impl, IMPL_OFFSET>,
            SuggestedFileName: SuggestedFileName::<Impl, IMPL_OFFSET>,
            SetSuggestedFileName: SetSuggestedFileName::<Impl, IMPL_OFFSET>,
            PickSaveFileAsync: PickSaveFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFileSavePicker2_Impl: Sized {
    fn ContinuationData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickSaveFileAndContinue(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFileSavePicker2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePicker2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePicker2_Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFileSavePicker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickSaveFileAndContinue<Impl: IFileSavePicker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickSaveFileAndContinue().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePicker2, BASE_OFFSET>(),
            ContinuationData: ContinuationData::<Impl, IMPL_OFFSET>,
            PickSaveFileAndContinue: PickSaveFileAndContinue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileSavePicker3_Impl: Sized {
    fn EnterpriseId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileSavePicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker3";
}
#[cfg(feature = "implement_exclusive")]
impl IFileSavePicker3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePicker3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePicker3_Vtbl {
        unsafe extern "system" fn EnterpriseId<Impl: IFileSavePicker3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnterpriseId<Impl: IFileSavePicker3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePicker3, BASE_OFFSET>(),
            EnterpriseId: EnterpriseId::<Impl, IMPL_OFFSET>,
            SetEnterpriseId: SetEnterpriseId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileSavePicker4_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePicker4 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePicker4";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileSavePicker4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePicker4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePicker4_Vtbl {
        unsafe extern "system" fn User<Impl: IFileSavePicker4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePicker4, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePicker4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFileSavePickerStatics_Impl: Sized {
    fn CreateForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FileSavePicker>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileSavePickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFileSavePickerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFileSavePickerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSavePickerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSavePickerStatics_Vtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFileSavePickerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileSavePickerStatics, BASE_OFFSET>(),
            CreateForUser: CreateForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSavePickerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFolderPicker_Impl: Sized {
    fn ViewMode(&mut self) -> ::windows::core::Result<PickerViewMode>;
    fn SetViewMode(&mut self, value: PickerViewMode) -> ::windows::core::Result<()>;
    fn SettingsIdentifier(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSettingsIdentifier(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SuggestedStartLocation(&mut self) -> ::windows::core::Result<PickerLocationId>;
    fn SetSuggestedStartLocation(&mut self, value: PickerLocationId) -> ::windows::core::Result<()>;
    fn CommitButtonText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileTypeFilter(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleFolderAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFolderPicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPicker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPicker_Vtbl {
        unsafe extern "system" fn ViewMode<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewMode<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewMode(value).into()
        }
        unsafe extern "system" fn SettingsIdentifier<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSettingsIdentifier<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettingsIdentifier(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedStartLocation<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuggestedStartLocation<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedStartLocation(value).into()
        }
        unsafe extern "system" fn CommitButtonText<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommitButtonText<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitButtonText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypeFilter<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickSingleFolderAsync<Impl: IFolderPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFolderPicker, BASE_OFFSET>(),
            ViewMode: ViewMode::<Impl, IMPL_OFFSET>,
            SetViewMode: SetViewMode::<Impl, IMPL_OFFSET>,
            SettingsIdentifier: SettingsIdentifier::<Impl, IMPL_OFFSET>,
            SetSettingsIdentifier: SetSettingsIdentifier::<Impl, IMPL_OFFSET>,
            SuggestedStartLocation: SuggestedStartLocation::<Impl, IMPL_OFFSET>,
            SetSuggestedStartLocation: SetSuggestedStartLocation::<Impl, IMPL_OFFSET>,
            CommitButtonText: CommitButtonText::<Impl, IMPL_OFFSET>,
            SetCommitButtonText: SetCommitButtonText::<Impl, IMPL_OFFSET>,
            FileTypeFilter: FileTypeFilter::<Impl, IMPL_OFFSET>,
            PickSingleFolderAsync: PickSingleFolderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFolderPicker2_Impl: Sized {
    fn ContinuationData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn PickFolderAndContinue(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPicker2 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFolderPicker2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPicker2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPicker2_Vtbl {
        unsafe extern "system" fn ContinuationData<Impl: IFolderPicker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PickFolderAndContinue<Impl: IFolderPicker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PickFolderAndContinue().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFolderPicker2, BASE_OFFSET>(),
            ContinuationData: ContinuationData::<Impl, IMPL_OFFSET>,
            PickFolderAndContinue: PickFolderAndContinue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPicker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFolderPicker3_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPicker3 {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPicker3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFolderPicker3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPicker3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPicker3_Vtbl {
        unsafe extern "system" fn User<Impl: IFolderPicker3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFolderPicker3, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPicker3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IFolderPickerStatics_Impl: Sized {
    fn CreateForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<FolderPicker>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderPickerStatics {
    const NAME: &'static str = "Windows.Storage.Pickers.IFolderPickerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IFolderPickerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderPickerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderPickerStatics_Vtbl {
        unsafe extern "system" fn CreateForUser<Impl: IFolderPickerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFolderPickerStatics, BASE_OFFSET>(), CreateForUser: CreateForUser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderPickerStatics as ::windows::core::Interface>::IID
    }
}
