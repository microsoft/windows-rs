#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBackgroundDownloader_Impl: Sized + IBackgroundTransferBase_Impl {
    fn CreateDownload(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<DownloadOperation>;
    fn CreateDownloadFromFile(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>, requestbodyfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<DownloadOperation>;
    fn CreateDownloadAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>, requestbodystream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DownloadOperation>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloader";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBackgroundDownloader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloader_Vtbl {
        unsafe extern "system" fn CreateDownload<Impl: IBackgroundDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDownload(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&resultfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDownloadFromFile<Impl: IBackgroundDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, requestbodyfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDownloadFromFile(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&resultfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&requestbodyfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDownloadAsync<Impl: IBackgroundDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, requestbodystream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDownloadAsync(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&resultfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&requestbodystream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundDownloader, BASE_OFFSET>(),
            CreateDownload: CreateDownload::<Impl, IMPL_OFFSET>,
            CreateDownloadFromFile: CreateDownloadFromFile::<Impl, IMPL_OFFSET>,
            CreateDownloadAsync: CreateDownloadAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait IBackgroundDownloader2_Impl: Sized {
    fn TransferGroup(&mut self) -> ::windows::core::Result<BackgroundTransferGroup>;
    fn SetTransferGroup(&mut self, value: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<()>;
    fn SuccessToastNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetSuccessToastNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn FailureToastNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetFailureToastNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn SuccessTileNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetSuccessTileNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
    fn FailureTileNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetFailureTileNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloader2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloader2";
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl IBackgroundDownloader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloader2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloader2_Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransferGroup<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferGroup(&*(&value as *const <BackgroundTransferGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessToastNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuccessToastNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuccessToastNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureToastNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FailureToastNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFailureToastNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessTileNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuccessTileNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuccessTileNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureTileNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FailureTileNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFailureTileNotification<Impl: IBackgroundDownloader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundDownloader2, BASE_OFFSET>(),
            TransferGroup: TransferGroup::<Impl, IMPL_OFFSET>,
            SetTransferGroup: SetTransferGroup::<Impl, IMPL_OFFSET>,
            SuccessToastNotification: SuccessToastNotification::<Impl, IMPL_OFFSET>,
            SetSuccessToastNotification: SetSuccessToastNotification::<Impl, IMPL_OFFSET>,
            FailureToastNotification: FailureToastNotification::<Impl, IMPL_OFFSET>,
            SetFailureToastNotification: SetFailureToastNotification::<Impl, IMPL_OFFSET>,
            SuccessTileNotification: SuccessTileNotification::<Impl, IMPL_OFFSET>,
            SetSuccessTileNotification: SetSuccessTileNotification::<Impl, IMPL_OFFSET>,
            FailureTileNotification: FailureTileNotification::<Impl, IMPL_OFFSET>,
            SetFailureTileNotification: SetFailureTileNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloader3_Impl: Sized {
    fn CompletionGroup(&mut self) -> ::windows::core::Result<BackgroundTransferCompletionGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundDownloader3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloader3";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundDownloader3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloader3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloader3_Vtbl {
        unsafe extern "system" fn CompletionGroup<Impl: IBackgroundDownloader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletionGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundDownloader3, BASE_OFFSET>(),
            CompletionGroup: CompletionGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloader3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloaderFactory_Impl: Sized {
    fn CreateWithCompletionGroup(&mut self, completiongroup: &::core::option::Option<BackgroundTransferCompletionGroup>) -> ::windows::core::Result<BackgroundDownloader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundDownloaderFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundDownloaderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderFactory_Vtbl {
        unsafe extern "system" fn CreateWithCompletionGroup<Impl: IBackgroundDownloaderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiongroup: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithCompletionGroup(&*(&completiongroup as *const <BackgroundTransferCompletionGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferCompletionGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundDownloaderFactory, BASE_OFFSET>(),
            CreateWithCompletionGroup: CreateWithCompletionGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderStaticMethods_Impl: Sized {
    fn GetCurrentDownloadsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
    fn GetCurrentDownloadsForGroupAsync(&mut self, group: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloaderStaticMethods {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundDownloaderStaticMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderStaticMethods_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderStaticMethods_Vtbl {
        unsafe extern "system" fn GetCurrentDownloadsAsync<Impl: IBackgroundDownloaderStaticMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentDownloadsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDownloadsForGroupAsync<Impl: IBackgroundDownloaderStaticMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentDownloadsForGroupAsync(&*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundDownloaderStaticMethods, BASE_OFFSET>(),
            GetCurrentDownloadsAsync: GetCurrentDownloadsAsync::<Impl, IMPL_OFFSET>,
            GetCurrentDownloadsForGroupAsync: GetCurrentDownloadsForGroupAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderStaticMethods2_Impl: Sized {
    fn GetCurrentDownloadsForTransferGroupAsync(&mut self, group: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloaderStaticMethods2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundDownloaderStaticMethods2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderStaticMethods2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderStaticMethods2_Vtbl {
        unsafe extern "system" fn GetCurrentDownloadsForTransferGroupAsync<Impl: IBackgroundDownloaderStaticMethods2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentDownloadsForTransferGroupAsync(&*(&group as *const <BackgroundTransferGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundDownloaderStaticMethods2, BASE_OFFSET>(),
            GetCurrentDownloadsForTransferGroupAsync: GetCurrentDownloadsForTransferGroupAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderStaticMethods2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderUserConsent_Impl: Sized {
    fn RequestUnconstrainedDownloadsAsync(&mut self, operations: &::core::option::Option<super::super::Foundation::Collections::IIterable<DownloadOperation>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloaderUserConsent {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundDownloaderUserConsent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderUserConsent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderUserConsent_Vtbl {
        unsafe extern "system" fn RequestUnconstrainedDownloadsAsync<Impl: IBackgroundDownloaderUserConsent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUnconstrainedDownloadsAsync(&*(&operations as *const <super::super::Foundation::Collections::IIterable<DownloadOperation> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<DownloadOperation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundDownloaderUserConsent, BASE_OFFSET>(),
            RequestUnconstrainedDownloadsAsync: RequestUnconstrainedDownloadsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderUserConsent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Credentials")]
pub trait IBackgroundTransferBase_Impl: Sized {
    fn SetRequestHeader(&mut self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServerCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&mut self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&mut self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn Method(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMethod(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CostPolicy(&mut self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&mut self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Credentials")]
impl ::windows::core::RuntimeName for IBackgroundTransferBase {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferBase";
}
#[cfg(feature = "Security_Credentials")]
impl IBackgroundTransferBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferBase_Vtbl {
        unsafe extern "system" fn SetRequestHeader<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerCredential<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerCredential(&*(&credential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyCredential<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyCredential(&*(&credential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Method<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Method() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethod<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMethod(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Group<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CostPolicy<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CostPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCostPolicy(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferBase, BASE_OFFSET>(),
            SetRequestHeader: SetRequestHeader::<Impl, IMPL_OFFSET>,
            ServerCredential: ServerCredential::<Impl, IMPL_OFFSET>,
            SetServerCredential: SetServerCredential::<Impl, IMPL_OFFSET>,
            ProxyCredential: ProxyCredential::<Impl, IMPL_OFFSET>,
            SetProxyCredential: SetProxyCredential::<Impl, IMPL_OFFSET>,
            Method: Method::<Impl, IMPL_OFFSET>,
            SetMethod: SetMethod::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            SetGroup: SetGroup::<Impl, IMPL_OFFSET>,
            CostPolicy: CostPolicy::<Impl, IMPL_OFFSET>,
            SetCostPolicy: SetCostPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
pub trait IBackgroundTransferCompletionGroup_Impl: Sized {
    fn Trigger(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn Enable(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferCompletionGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup";
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl IBackgroundTransferCompletionGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferCompletionGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferCompletionGroup_Vtbl {
        unsafe extern "system" fn Trigger<Impl: IBackgroundTransferCompletionGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IBackgroundTransferCompletionGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Enable<Impl: IBackgroundTransferCompletionGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferCompletionGroup, BASE_OFFSET>(),
            Trigger: Trigger::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferCompletionGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundTransferCompletionGroupTriggerDetails_Impl: Sized {
    fn Downloads(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DownloadOperation>>;
    fn Uploads(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UploadOperation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferCompletionGroupTriggerDetails {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundTransferCompletionGroupTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferCompletionGroupTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferCompletionGroupTriggerDetails_Vtbl {
        unsafe extern "system" fn Downloads<Impl: IBackgroundTransferCompletionGroupTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Downloads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uploads<Impl: IBackgroundTransferCompletionGroupTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uploads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferCompletionGroupTriggerDetails, BASE_OFFSET>(),
            Downloads: Downloads::<Impl, IMPL_OFFSET>,
            Uploads: Uploads::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferCompletionGroupTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IBackgroundTransferContentPart_Impl: Sized {
    fn SetHeader(&mut self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetFile(&mut self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferContentPart {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IBackgroundTransferContentPart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferContentPart_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferContentPart_Vtbl {
        unsafe extern "system" fn SetHeader<Impl: IBackgroundTransferContentPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetText<Impl: IBackgroundTransferContentPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFile<Impl: IBackgroundTransferContentPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFile(&*(&value as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferContentPart, BASE_OFFSET>(),
            SetHeader: SetHeader::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            SetFile: SetFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferContentPart as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTransferContentPartFactory_Impl: Sized {
    fn CreateWithName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
    fn CreateWithNameAndFileName(&mut self, name: &::windows::core::HSTRING, filename: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
}
impl ::windows::core::RuntimeName for IBackgroundTransferContentPartFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory";
}
impl IBackgroundTransferContentPartFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferContentPartFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferContentPartFactory_Vtbl {
        unsafe extern "system" fn CreateWithName<Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithNameAndFileName<Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithNameAndFileName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferContentPartFactory, BASE_OFFSET>(),
            CreateWithName: CreateWithName::<Impl, IMPL_OFFSET>,
            CreateWithNameAndFileName: CreateWithNameAndFileName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferContentPartFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
pub trait IBackgroundTransferErrorStaticMethods_Impl: Sized {
    fn GetStatus(&mut self, hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus>;
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferErrorStaticMethods {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods";
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl IBackgroundTransferErrorStaticMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferErrorStaticMethods_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferErrorStaticMethods_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: IBackgroundTransferErrorStaticMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferErrorStaticMethods, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferErrorStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferGroup_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransferBehavior(&mut self) -> ::windows::core::Result<BackgroundTransferBehavior>;
    fn SetTransferBehavior(&mut self, value: BackgroundTransferBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTransferGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTransferGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferGroup_Vtbl {
        unsafe extern "system" fn Name<Impl: IBackgroundTransferGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferBehavior<Impl: IBackgroundTransferGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransferBehavior<Impl: IBackgroundTransferGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferBehavior(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferGroup, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            TransferBehavior: TransferBehavior::<Impl, IMPL_OFFSET>,
            SetTransferBehavior: SetTransferBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferGroupStatics_Impl: Sized {
    fn CreateGroup(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTransferGroupStatics {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTransferGroupStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferGroupStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferGroupStatics_Vtbl {
        unsafe extern "system" fn CreateGroup<Impl: IBackgroundTransferGroupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGroup(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferGroupStatics, BASE_OFFSET>(),
            CreateGroup: CreateGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBackgroundTransferOperation_Impl: Sized {
    fn Guid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestedUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Method(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Group(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CostPolicy(&mut self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&mut self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
    fn GetResultStreamAt(&mut self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetResponseInformation(&mut self) -> ::windows::core::Result<ResponseInformation>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IBackgroundTransferOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IBackgroundTransferOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferOperation_Vtbl {
        unsafe extern "system" fn Guid<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUri<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Method<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Method() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CostPolicy<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CostPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCostPolicy(value).into()
        }
        unsafe extern "system" fn GetResultStreamAt<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResponseInformation<Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResponseInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferOperation, BASE_OFFSET>(),
            Guid: Guid::<Impl, IMPL_OFFSET>,
            RequestedUri: RequestedUri::<Impl, IMPL_OFFSET>,
            Method: Method::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            CostPolicy: CostPolicy::<Impl, IMPL_OFFSET>,
            SetCostPolicy: SetCostPolicy::<Impl, IMPL_OFFSET>,
            GetResultStreamAt: GetResultStreamAt::<Impl, IMPL_OFFSET>,
            GetResponseInformation: GetResponseInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferOperation as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTransferOperationPriority_Impl: Sized {
    fn Priority(&mut self) -> ::windows::core::Result<BackgroundTransferPriority>;
    fn SetPriority(&mut self, value: BackgroundTransferPriority) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTransferOperationPriority {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority";
}
impl IBackgroundTransferOperationPriority_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperationPriority_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferOperationPriority_Vtbl {
        unsafe extern "system" fn Priority<Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferOperationPriority, BASE_OFFSET>(),
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferOperationPriority as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundTransferRangesDownloadedEventArgs_Impl: Sized {
    fn WasDownloadRestarted(&mut self) -> ::windows::core::Result<bool>;
    fn AddedRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferRangesDownloadedEventArgs {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundTransferRangesDownloadedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferRangesDownloadedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferRangesDownloadedEventArgs_Vtbl {
        unsafe extern "system" fn WasDownloadRestarted<Impl: IBackgroundTransferRangesDownloadedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasDownloadRestarted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddedRanges<Impl: IBackgroundTransferRangesDownloadedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBackgroundTransferRangesDownloadedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferRangesDownloadedEventArgs, BASE_OFFSET>(),
            WasDownloadRestarted: WasDownloadRestarted::<Impl, IMPL_OFFSET>,
            AddedRanges: AddedRanges::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferRangesDownloadedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBackgroundUploader_Impl: Sized + IBackgroundTransferBase_Impl {
    fn CreateUpload(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, sourcefile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<UploadOperation>;
    fn CreateUploadFromStreamAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, sourcestream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithFormDataAndAutoBoundaryAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithSubTypeAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithSubTypeAndBoundaryAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, subtype: &::windows::core::HSTRING, boundary: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBackgroundUploader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploader_Vtbl {
        unsafe extern "system" fn CreateUpload<Impl: IBackgroundUploader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sourcefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUpload(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&sourcefile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUploadFromStreamAsync<Impl: IBackgroundUploader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sourcestream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUploadFromStreamAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&sourcestream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUploadWithFormDataAndAutoBoundaryAsync<Impl: IBackgroundUploader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUploadWithFormDataAndAutoBoundaryAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&parts as *const <super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUploadWithSubTypeAsync<Impl: IBackgroundUploader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUploadWithSubTypeAsync(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&parts as *const <super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart> as ::windows::core::DefaultType>::DefaultType),
                &*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUploadWithSubTypeAndBoundaryAsync<Impl: IBackgroundUploader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, boundary: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUploadWithSubTypeAndBoundaryAsync(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&parts as *const <super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart> as ::windows::core::DefaultType>::DefaultType),
                &*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&boundary as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundUploader, BASE_OFFSET>(),
            CreateUpload: CreateUpload::<Impl, IMPL_OFFSET>,
            CreateUploadFromStreamAsync: CreateUploadFromStreamAsync::<Impl, IMPL_OFFSET>,
            CreateUploadWithFormDataAndAutoBoundaryAsync: CreateUploadWithFormDataAndAutoBoundaryAsync::<Impl, IMPL_OFFSET>,
            CreateUploadWithSubTypeAsync: CreateUploadWithSubTypeAsync::<Impl, IMPL_OFFSET>,
            CreateUploadWithSubTypeAndBoundaryAsync: CreateUploadWithSubTypeAndBoundaryAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait IBackgroundUploader2_Impl: Sized {
    fn TransferGroup(&mut self) -> ::windows::core::Result<BackgroundTransferGroup>;
    fn SetTransferGroup(&mut self, value: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<()>;
    fn SuccessToastNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetSuccessToastNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn FailureToastNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetFailureToastNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn SuccessTileNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetSuccessTileNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
    fn FailureTileNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetFailureTileNotification(&mut self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploader2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploader2";
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl IBackgroundUploader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploader2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploader2_Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransferGroup<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferGroup(&*(&value as *const <BackgroundTransferGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessToastNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuccessToastNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuccessToastNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureToastNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FailureToastNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFailureToastNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessTileNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuccessTileNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuccessTileNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureTileNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FailureTileNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFailureTileNotification<Impl: IBackgroundUploader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundUploader2, BASE_OFFSET>(),
            TransferGroup: TransferGroup::<Impl, IMPL_OFFSET>,
            SetTransferGroup: SetTransferGroup::<Impl, IMPL_OFFSET>,
            SuccessToastNotification: SuccessToastNotification::<Impl, IMPL_OFFSET>,
            SetSuccessToastNotification: SetSuccessToastNotification::<Impl, IMPL_OFFSET>,
            FailureToastNotification: FailureToastNotification::<Impl, IMPL_OFFSET>,
            SetFailureToastNotification: SetFailureToastNotification::<Impl, IMPL_OFFSET>,
            SuccessTileNotification: SuccessTileNotification::<Impl, IMPL_OFFSET>,
            SetSuccessTileNotification: SetSuccessTileNotification::<Impl, IMPL_OFFSET>,
            FailureTileNotification: FailureTileNotification::<Impl, IMPL_OFFSET>,
            SetFailureTileNotification: SetFailureTileNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploader3_Impl: Sized {
    fn CompletionGroup(&mut self) -> ::windows::core::Result<BackgroundTransferCompletionGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundUploader3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploader3";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundUploader3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploader3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploader3_Vtbl {
        unsafe extern "system" fn CompletionGroup<Impl: IBackgroundUploader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletionGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundUploader3, BASE_OFFSET>(),
            CompletionGroup: CompletionGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploader3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploaderFactory_Impl: Sized {
    fn CreateWithCompletionGroup(&mut self, completiongroup: &::core::option::Option<BackgroundTransferCompletionGroup>) -> ::windows::core::Result<BackgroundUploader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundUploaderFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundUploaderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderFactory_Vtbl {
        unsafe extern "system" fn CreateWithCompletionGroup<Impl: IBackgroundUploaderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiongroup: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithCompletionGroup(&*(&completiongroup as *const <BackgroundTransferCompletionGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferCompletionGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundUploaderFactory, BASE_OFFSET>(),
            CreateWithCompletionGroup: CreateWithCompletionGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderStaticMethods_Impl: Sized {
    fn GetCurrentUploadsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
    fn GetCurrentUploadsForGroupAsync(&mut self, group: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploaderStaticMethods {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundUploaderStaticMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderStaticMethods_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderStaticMethods_Vtbl {
        unsafe extern "system" fn GetCurrentUploadsAsync<Impl: IBackgroundUploaderStaticMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentUploadsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentUploadsForGroupAsync<Impl: IBackgroundUploaderStaticMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentUploadsForGroupAsync(&*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundUploaderStaticMethods, BASE_OFFSET>(),
            GetCurrentUploadsAsync: GetCurrentUploadsAsync::<Impl, IMPL_OFFSET>,
            GetCurrentUploadsForGroupAsync: GetCurrentUploadsForGroupAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderStaticMethods2_Impl: Sized {
    fn GetCurrentUploadsForTransferGroupAsync(&mut self, group: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploaderStaticMethods2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundUploaderStaticMethods2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderStaticMethods2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderStaticMethods2_Vtbl {
        unsafe extern "system" fn GetCurrentUploadsForTransferGroupAsync<Impl: IBackgroundUploaderStaticMethods2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentUploadsForTransferGroupAsync(&*(&group as *const <BackgroundTransferGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundUploaderStaticMethods2, BASE_OFFSET>(),
            GetCurrentUploadsForTransferGroupAsync: GetCurrentUploadsForTransferGroupAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderStaticMethods2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderUserConsent_Impl: Sized {
    fn RequestUnconstrainedUploadsAsync(&mut self, operations: &::core::option::Option<super::super::Foundation::Collections::IIterable<UploadOperation>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploaderUserConsent {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundUploaderUserConsent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderUserConsent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderUserConsent_Vtbl {
        unsafe extern "system" fn RequestUnconstrainedUploadsAsync<Impl: IBackgroundUploaderUserConsent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUnconstrainedUploadsAsync(&*(&operations as *const <super::super::Foundation::Collections::IIterable<UploadOperation> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<UploadOperation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundUploaderUserConsent, BASE_OFFSET>(),
            RequestUnconstrainedUploadsAsync: RequestUnconstrainedUploadsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderUserConsent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContentPrefetcher_Impl: Sized {
    fn ContentUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn SetIndirectContentUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn IndirectContentUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentPrefetcher {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IContentPrefetcher";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContentPrefetcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetcher_Vtbl {
        unsafe extern "system" fn ContentUris<Impl: IContentPrefetcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndirectContentUri<Impl: IContentPrefetcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndirectContentUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IndirectContentUri<Impl: IContentPrefetcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndirectContentUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentPrefetcher, BASE_OFFSET>(),
            ContentUris: ContentUris::<Impl, IMPL_OFFSET>,
            SetIndirectContentUri: SetIndirectContentUri::<Impl, IMPL_OFFSET>,
            IndirectContentUri: IndirectContentUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContentPrefetcherTime_Impl: Sized {
    fn LastSuccessfulPrefetchTime(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentPrefetcherTime {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IContentPrefetcherTime";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContentPrefetcherTime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetcherTime_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetcherTime_Vtbl {
        unsafe extern "system" fn LastSuccessfulPrefetchTime<Impl: IContentPrefetcherTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSuccessfulPrefetchTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentPrefetcherTime, BASE_OFFSET>(),
            LastSuccessfulPrefetchTime: LastSuccessfulPrefetchTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetcherTime as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDownloadOperation_Impl: Sized + IBackgroundTransferOperation_Impl {
    fn ResultFile(&mut self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn Progress(&mut self) -> ::windows::core::Result<BackgroundDownloadProgress>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>;
    fn AttachAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDownloadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDownloadOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation_Vtbl {
        unsafe extern "system" fn ResultFile<Impl: IDownloadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IDownloadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundDownloadProgress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IDownloadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachAsync<Impl: IDownloadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IDownloadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IDownloadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadOperation, BASE_OFFSET>(),
            ResultFile: ResultFile::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            AttachAsync: AttachAsync::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation2_Impl: Sized {
    fn TransferGroup(&mut self) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadOperation2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadOperation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation2_Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IDownloadOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadOperation2, BASE_OFFSET>(), TransferGroup: TransferGroup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "Web", feature = "implement_exclusive"))]
pub trait IDownloadOperation3_Impl: Sized {
    fn IsRandomAccessRequired(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRandomAccessRequired(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetResultRandomAccessStreamReference(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetDownloadedRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>;
    fn RangesDownloaded(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRangesDownloaded(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetRequestedUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn RecoverableWebErrorStatuses(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>>;
    fn CurrentWebErrorStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDownloadOperation3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "Web", feature = "implement_exclusive"))]
impl IDownloadOperation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation3_Vtbl {
        unsafe extern "system" fn IsRandomAccessRequired<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRandomAccessRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRandomAccessRequired<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRandomAccessRequired(value).into()
        }
        unsafe extern "system" fn GetResultRandomAccessStreamReference<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultRandomAccessStreamReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDownloadedRanges<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDownloadedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangesDownloaded<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangesDownloaded(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRangesDownloaded<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRangesDownloaded(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetRequestedUri<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecoverableWebErrorStatuses<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoverableWebErrorStatuses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWebErrorStatus<Impl: IDownloadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWebErrorStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadOperation3, BASE_OFFSET>(),
            IsRandomAccessRequired: IsRandomAccessRequired::<Impl, IMPL_OFFSET>,
            SetIsRandomAccessRequired: SetIsRandomAccessRequired::<Impl, IMPL_OFFSET>,
            GetResultRandomAccessStreamReference: GetResultRandomAccessStreamReference::<Impl, IMPL_OFFSET>,
            GetDownloadedRanges: GetDownloadedRanges::<Impl, IMPL_OFFSET>,
            RangesDownloaded: RangesDownloaded::<Impl, IMPL_OFFSET>,
            RemoveRangesDownloaded: RemoveRangesDownloaded::<Impl, IMPL_OFFSET>,
            SetRequestedUri: SetRequestedUri::<Impl, IMPL_OFFSET>,
            RecoverableWebErrorStatuses: RecoverableWebErrorStatuses::<Impl, IMPL_OFFSET>,
            CurrentWebErrorStatus: CurrentWebErrorStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation4_Impl: Sized {
    fn MakeCurrentInTransferGroup(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadOperation4 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation4";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadOperation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation4_Vtbl {
        unsafe extern "system" fn MakeCurrentInTransferGroup<Impl: IDownloadOperation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeCurrentInTransferGroup().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadOperation4, BASE_OFFSET>(),
            MakeCurrentInTransferGroup: MakeCurrentInTransferGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation5_Impl: Sized {
    fn SetRequestHeader(&mut self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveRequestHeader(&mut self, headername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadOperation5 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation5";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadOperation5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation5_Vtbl {
        unsafe extern "system" fn SetRequestHeader<Impl: IDownloadOperation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveRequestHeader<Impl: IDownloadOperation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadOperation5, BASE_OFFSET>(),
            SetRequestHeader: SetRequestHeader::<Impl, IMPL_OFFSET>,
            RemoveRequestHeader: RemoveRequestHeader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResponseInformation_Impl: Sized {
    fn IsResumable(&mut self) -> ::windows::core::Result<bool>;
    fn ActualUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn StatusCode(&mut self) -> ::windows::core::Result<u32>;
    fn Headers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResponseInformation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IResponseInformation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResponseInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResponseInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResponseInformation_Vtbl {
        unsafe extern "system" fn IsResumable<Impl: IResponseInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResumable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualUri<Impl: IResponseInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IResponseInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Headers<Impl: IResponseInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Headers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResponseInformation, BASE_OFFSET>(),
            IsResumable: IsResumable::<Impl, IMPL_OFFSET>,
            ActualUri: ActualUri::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            Headers: Headers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResponseInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IUnconstrainedTransferRequestResult_Impl: Sized {
    fn IsUnconstrained(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUnconstrainedTransferRequestResult {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IUnconstrainedTransferRequestResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnconstrainedTransferRequestResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnconstrainedTransferRequestResult_Vtbl {
        unsafe extern "system" fn IsUnconstrained<Impl: IUnconstrainedTransferRequestResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUnconstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnconstrainedTransferRequestResult, BASE_OFFSET>(),
            IsUnconstrained: IsUnconstrained::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnconstrainedTransferRequestResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUploadOperation_Impl: Sized + IBackgroundTransferOperation_Impl {
    fn SourceFile(&mut self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn Progress(&mut self) -> ::windows::core::Result<BackgroundUploadProgress>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>;
    fn AttachAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUploadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUploadOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperation_Vtbl {
        unsafe extern "system" fn SourceFile<Impl: IUploadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IUploadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundUploadProgress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IUploadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachAsync<Impl: IUploadOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUploadOperation, BASE_OFFSET>(),
            SourceFile: SourceFile::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            AttachAsync: AttachAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation2_Impl: Sized {
    fn TransferGroup(&mut self) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUploadOperation2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl IUploadOperation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperation2_Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IUploadOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUploadOperation2, BASE_OFFSET>(), TransferGroup: TransferGroup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation3_Impl: Sized {
    fn MakeCurrentInTransferGroup(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUploadOperation3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation3";
}
#[cfg(feature = "implement_exclusive")]
impl IUploadOperation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperation3_Vtbl {
        unsafe extern "system" fn MakeCurrentInTransferGroup<Impl: IUploadOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeCurrentInTransferGroup().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUploadOperation3, BASE_OFFSET>(),
            MakeCurrentInTransferGroup: MakeCurrentInTransferGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation4_Impl: Sized {
    fn SetRequestHeader(&mut self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveRequestHeader(&mut self, headername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUploadOperation4 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation4";
}
#[cfg(feature = "implement_exclusive")]
impl IUploadOperation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperation4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperation4_Vtbl {
        unsafe extern "system" fn SetRequestHeader<Impl: IUploadOperation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveRequestHeader<Impl: IUploadOperation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUploadOperation4, BASE_OFFSET>(),
            SetRequestHeader: SetRequestHeader::<Impl, IMPL_OFFSET>,
            RemoveRequestHeader: RemoveRequestHeader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation4 as ::windows::core::Interface>::IID
    }
}
