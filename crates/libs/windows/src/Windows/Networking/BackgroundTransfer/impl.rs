#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderImpl: Sized + IBackgroundTransferBaseImpl {
    fn CreateDownload(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<DownloadOperation>;
    fn CreateDownloadFromFile(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>, requestbodyfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<DownloadOperation>;
    fn CreateDownloadAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>, requestbodystream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DownloadOperation>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloader";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBackgroundDownloaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderVtbl {
        unsafe extern "system" fn CreateDownload<Impl: IBackgroundDownloaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateDownloadFromFile<Impl: IBackgroundDownloaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, requestbodyfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateDownloadAsync<Impl: IBackgroundDownloaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, requestbodystream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundDownloader>, ::windows::core::GetTrustLevel, CreateDownload::<Impl, IMPL_OFFSET>, CreateDownloadFromFile::<Impl, IMPL_OFFSET>, CreateDownloadAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait IBackgroundDownloader2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
    fn SetTransferGroup(&self, value: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<()>;
    fn SuccessToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetSuccessToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn FailureToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetFailureToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn SuccessTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetSuccessTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
    fn FailureTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetFailureTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloader2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloader2";
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl IBackgroundDownloader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloader2Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransferGroup<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferGroup(&*(&value as *const <BackgroundTransferGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessToastNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuccessToastNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureToastNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFailureToastNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessTileNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuccessTileNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureTileNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFailureTileNotification<Impl: IBackgroundDownloader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBackgroundDownloader2>,
            ::windows::core::GetTrustLevel,
            TransferGroup::<Impl, IMPL_OFFSET>,
            SetTransferGroup::<Impl, IMPL_OFFSET>,
            SuccessToastNotification::<Impl, IMPL_OFFSET>,
            SetSuccessToastNotification::<Impl, IMPL_OFFSET>,
            FailureToastNotification::<Impl, IMPL_OFFSET>,
            SetFailureToastNotification::<Impl, IMPL_OFFSET>,
            SuccessTileNotification::<Impl, IMPL_OFFSET>,
            SetSuccessTileNotification::<Impl, IMPL_OFFSET>,
            FailureTileNotification::<Impl, IMPL_OFFSET>,
            SetFailureTileNotification::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloader3Impl: Sized {
    fn CompletionGroup(&self) -> ::windows::core::Result<BackgroundTransferCompletionGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundDownloader3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloader3";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundDownloader3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloader3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloader3Vtbl {
        unsafe extern "system" fn CompletionGroup<Impl: IBackgroundDownloader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundDownloader3>, ::windows::core::GetTrustLevel, CompletionGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloader3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloaderFactoryImpl: Sized {
    fn CreateWithCompletionGroup(&self, completiongroup: &::core::option::Option<BackgroundTransferCompletionGroup>) -> ::windows::core::Result<BackgroundDownloader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundDownloaderFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundDownloaderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderFactoryVtbl {
        unsafe extern "system" fn CreateWithCompletionGroup<Impl: IBackgroundDownloaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiongroup: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundDownloaderFactory>, ::windows::core::GetTrustLevel, CreateWithCompletionGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderStaticMethodsImpl: Sized {
    fn GetCurrentDownloadsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
    fn GetCurrentDownloadsForGroupAsync(&self, group: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloaderStaticMethods {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundDownloaderStaticMethodsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderStaticMethodsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderStaticMethodsVtbl {
        unsafe extern "system" fn GetCurrentDownloadsAsync<Impl: IBackgroundDownloaderStaticMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentDownloadsForGroupAsync<Impl: IBackgroundDownloaderStaticMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundDownloaderStaticMethods>, ::windows::core::GetTrustLevel, GetCurrentDownloadsAsync::<Impl, IMPL_OFFSET>, GetCurrentDownloadsForGroupAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderStaticMethods2Impl: Sized {
    fn GetCurrentDownloadsForTransferGroupAsync(&self, group: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloaderStaticMethods2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundDownloaderStaticMethods2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderStaticMethods2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderStaticMethods2Vtbl {
        unsafe extern "system" fn GetCurrentDownloadsForTransferGroupAsync<Impl: IBackgroundDownloaderStaticMethods2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundDownloaderStaticMethods2>, ::windows::core::GetTrustLevel, GetCurrentDownloadsForTransferGroupAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderStaticMethods2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderUserConsentImpl: Sized {
    fn RequestUnconstrainedDownloadsAsync(&self, operations: &::core::option::Option<super::super::Foundation::Collections::IIterable<DownloadOperation>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundDownloaderUserConsent {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundDownloaderUserConsentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundDownloaderUserConsentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundDownloaderUserConsentVtbl {
        unsafe extern "system" fn RequestUnconstrainedDownloadsAsync<Impl: IBackgroundDownloaderUserConsentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundDownloaderUserConsent>, ::windows::core::GetTrustLevel, RequestUnconstrainedDownloadsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundDownloaderUserConsent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Credentials")]
pub trait IBackgroundTransferBaseImpl: Sized {
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMethod(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Credentials")]
impl ::windows::core::RuntimeName for IBackgroundTransferBase {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferBase";
}
#[cfg(feature = "Security_Credentials")]
impl IBackgroundTransferBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferBaseVtbl {
        unsafe extern "system" fn SetRequestHeader<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerCredential<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetServerCredential<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerCredential(&*(&credential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyCredential<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProxyCredential<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyCredential(&*(&credential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Method<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMethod<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMethod(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Group<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGroup<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CostPolicy<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCostPolicy<Impl: IBackgroundTransferBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCostPolicy(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBackgroundTransferBase>,
            ::windows::core::GetTrustLevel,
            SetRequestHeader::<Impl, IMPL_OFFSET>,
            ServerCredential::<Impl, IMPL_OFFSET>,
            SetServerCredential::<Impl, IMPL_OFFSET>,
            ProxyCredential::<Impl, IMPL_OFFSET>,
            SetProxyCredential::<Impl, IMPL_OFFSET>,
            Method::<Impl, IMPL_OFFSET>,
            SetMethod::<Impl, IMPL_OFFSET>,
            Group::<Impl, IMPL_OFFSET>,
            SetGroup::<Impl, IMPL_OFFSET>,
            CostPolicy::<Impl, IMPL_OFFSET>,
            SetCostPolicy::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
pub trait IBackgroundTransferCompletionGroupImpl: Sized {
    fn Trigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn Enable(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferCompletionGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup";
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "implement_exclusive"))]
impl IBackgroundTransferCompletionGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferCompletionGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferCompletionGroupVtbl {
        unsafe extern "system" fn Trigger<Impl: IBackgroundTransferCompletionGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IBackgroundTransferCompletionGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Enable<Impl: IBackgroundTransferCompletionGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferCompletionGroup>, ::windows::core::GetTrustLevel, Trigger::<Impl, IMPL_OFFSET>, IsEnabled::<Impl, IMPL_OFFSET>, Enable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferCompletionGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundTransferCompletionGroupTriggerDetailsImpl: Sized {
    fn Downloads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DownloadOperation>>;
    fn Uploads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UploadOperation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferCompletionGroupTriggerDetails {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundTransferCompletionGroupTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferCompletionGroupTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferCompletionGroupTriggerDetailsVtbl {
        unsafe extern "system" fn Downloads<Impl: IBackgroundTransferCompletionGroupTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Uploads<Impl: IBackgroundTransferCompletionGroupTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferCompletionGroupTriggerDetails>, ::windows::core::GetTrustLevel, Downloads::<Impl, IMPL_OFFSET>, Uploads::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferCompletionGroupTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IBackgroundTransferContentPartImpl: Sized {
    fn SetHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetFile(&self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferContentPart {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IBackgroundTransferContentPartVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferContentPartImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferContentPartVtbl {
        unsafe extern "system" fn SetHeader<Impl: IBackgroundTransferContentPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetText<Impl: IBackgroundTransferContentPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFile<Impl: IBackgroundTransferContentPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFile(&*(&value as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferContentPart>, ::windows::core::GetTrustLevel, SetHeader::<Impl, IMPL_OFFSET>, SetText::<Impl, IMPL_OFFSET>, SetFile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferContentPart as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTransferContentPartFactoryImpl: Sized {
    fn CreateWithName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
    fn CreateWithNameAndFileName(&self, name: &::windows::core::HSTRING, filename: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
}
impl ::windows::core::RuntimeName for IBackgroundTransferContentPartFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory";
}
impl IBackgroundTransferContentPartFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferContentPartFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferContentPartFactoryVtbl {
        unsafe extern "system" fn CreateWithName<Impl: IBackgroundTransferContentPartFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithNameAndFileName<Impl: IBackgroundTransferContentPartFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferContentPartFactory>, ::windows::core::GetTrustLevel, CreateWithName::<Impl, IMPL_OFFSET>, CreateWithNameAndFileName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferContentPartFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
pub trait IBackgroundTransferErrorStaticMethodsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus>;
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferErrorStaticMethods {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods";
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl IBackgroundTransferErrorStaticMethodsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferErrorStaticMethodsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferErrorStaticMethodsVtbl {
        unsafe extern "system" fn GetStatus<Impl: IBackgroundTransferErrorStaticMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferErrorStaticMethods>, ::windows::core::GetTrustLevel, GetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferErrorStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferGroupImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransferBehavior(&self) -> ::windows::core::Result<BackgroundTransferBehavior>;
    fn SetTransferBehavior(&self, value: BackgroundTransferBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTransferGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTransferGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferGroupVtbl {
        unsafe extern "system" fn Name<Impl: IBackgroundTransferGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransferBehavior<Impl: IBackgroundTransferGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransferBehavior<Impl: IBackgroundTransferGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferBehavior(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferGroup>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, TransferBehavior::<Impl, IMPL_OFFSET>, SetTransferBehavior::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferGroupStaticsImpl: Sized {
    fn CreateGroup(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTransferGroupStatics {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTransferGroupStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferGroupStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferGroupStaticsVtbl {
        unsafe extern "system" fn CreateGroup<Impl: IBackgroundTransferGroupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferGroupStatics>, ::windows::core::GetTrustLevel, CreateGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBackgroundTransferOperationImpl: Sized {
    fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestedUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
    fn GetResultStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetResponseInformation(&self) -> ::windows::core::Result<ResponseInformation>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IBackgroundTransferOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IBackgroundTransferOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferOperationVtbl {
        unsafe extern "system" fn Guid<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestedUri<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Method<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Group<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CostPolicy<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCostPolicy<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCostPolicy(value).into()
        }
        unsafe extern "system" fn GetResultStreamAt<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetResponseInformation<Impl: IBackgroundTransferOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBackgroundTransferOperation>,
            ::windows::core::GetTrustLevel,
            Guid::<Impl, IMPL_OFFSET>,
            RequestedUri::<Impl, IMPL_OFFSET>,
            Method::<Impl, IMPL_OFFSET>,
            Group::<Impl, IMPL_OFFSET>,
            CostPolicy::<Impl, IMPL_OFFSET>,
            SetCostPolicy::<Impl, IMPL_OFFSET>,
            GetResultStreamAt::<Impl, IMPL_OFFSET>,
            GetResponseInformation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferOperation as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTransferOperationPriorityImpl: Sized {
    fn Priority(&self) -> ::windows::core::Result<BackgroundTransferPriority>;
    fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTransferOperationPriority {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority";
}
impl IBackgroundTransferOperationPriorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperationPriorityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferOperationPriorityVtbl {
        unsafe extern "system" fn Priority<Impl: IBackgroundTransferOperationPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPriority<Impl: IBackgroundTransferOperationPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferOperationPriority>, ::windows::core::GetTrustLevel, Priority::<Impl, IMPL_OFFSET>, SetPriority::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferOperationPriority as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundTransferRangesDownloadedEventArgsImpl: Sized {
    fn WasDownloadRestarted(&self) -> ::windows::core::Result<bool>;
    fn AddedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTransferRangesDownloadedEventArgs {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundTransferRangesDownloadedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferRangesDownloadedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTransferRangesDownloadedEventArgsVtbl {
        unsafe extern "system" fn WasDownloadRestarted<Impl: IBackgroundTransferRangesDownloadedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddedRanges<Impl: IBackgroundTransferRangesDownloadedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IBackgroundTransferRangesDownloadedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundTransferRangesDownloadedEventArgs>, ::windows::core::GetTrustLevel, WasDownloadRestarted::<Impl, IMPL_OFFSET>, AddedRanges::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferRangesDownloadedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderImpl: Sized + IBackgroundTransferBaseImpl {
    fn CreateUpload(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, sourcefile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<UploadOperation>;
    fn CreateUploadFromStreamAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, sourcestream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithFormDataAndAutoBoundaryAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithSubTypeAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithSubTypeAndBoundaryAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, subtype: &::windows::core::HSTRING, boundary: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBackgroundUploaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderVtbl {
        unsafe extern "system" fn CreateUpload<Impl: IBackgroundUploaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sourcefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateUploadFromStreamAsync<Impl: IBackgroundUploaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sourcestream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateUploadWithFormDataAndAutoBoundaryAsync<Impl: IBackgroundUploaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateUploadWithSubTypeAsync<Impl: IBackgroundUploaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateUploadWithSubTypeAndBoundaryAsync<Impl: IBackgroundUploaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, boundary: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBackgroundUploader>,
            ::windows::core::GetTrustLevel,
            CreateUpload::<Impl, IMPL_OFFSET>,
            CreateUploadFromStreamAsync::<Impl, IMPL_OFFSET>,
            CreateUploadWithFormDataAndAutoBoundaryAsync::<Impl, IMPL_OFFSET>,
            CreateUploadWithSubTypeAsync::<Impl, IMPL_OFFSET>,
            CreateUploadWithSubTypeAndBoundaryAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait IBackgroundUploader2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
    fn SetTransferGroup(&self, value: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<()>;
    fn SuccessToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetSuccessToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn FailureToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetFailureToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn SuccessTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetSuccessTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
    fn FailureTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetFailureTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploader2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploader2";
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl IBackgroundUploader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploader2Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransferGroup<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransferGroup(&*(&value as *const <BackgroundTransferGroup as ::windows::core::Abi>::Abi as *const <BackgroundTransferGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessToastNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuccessToastNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureToastNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFailureToastNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureToastNotification(&*(&value as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuccessTileNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuccessTileNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuccessTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FailureTileNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFailureTileNotification<Impl: IBackgroundUploader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailureTileNotification(&*(&value as *const <super::super::UI::Notifications::TileNotification as ::windows::core::Abi>::Abi as *const <super::super::UI::Notifications::TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBackgroundUploader2>,
            ::windows::core::GetTrustLevel,
            TransferGroup::<Impl, IMPL_OFFSET>,
            SetTransferGroup::<Impl, IMPL_OFFSET>,
            SuccessToastNotification::<Impl, IMPL_OFFSET>,
            SetSuccessToastNotification::<Impl, IMPL_OFFSET>,
            FailureToastNotification::<Impl, IMPL_OFFSET>,
            SetFailureToastNotification::<Impl, IMPL_OFFSET>,
            SuccessTileNotification::<Impl, IMPL_OFFSET>,
            SetSuccessTileNotification::<Impl, IMPL_OFFSET>,
            FailureTileNotification::<Impl, IMPL_OFFSET>,
            SetFailureTileNotification::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploader3Impl: Sized {
    fn CompletionGroup(&self) -> ::windows::core::Result<BackgroundTransferCompletionGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundUploader3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploader3";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundUploader3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploader3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploader3Vtbl {
        unsafe extern "system" fn CompletionGroup<Impl: IBackgroundUploader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundUploader3>, ::windows::core::GetTrustLevel, CompletionGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploader3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploaderFactoryImpl: Sized {
    fn CreateWithCompletionGroup(&self, completiongroup: &::core::option::Option<BackgroundTransferCompletionGroup>) -> ::windows::core::Result<BackgroundUploader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundUploaderFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundUploaderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderFactoryVtbl {
        unsafe extern "system" fn CreateWithCompletionGroup<Impl: IBackgroundUploaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completiongroup: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundUploaderFactory>, ::windows::core::GetTrustLevel, CreateWithCompletionGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderStaticMethodsImpl: Sized {
    fn GetCurrentUploadsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
    fn GetCurrentUploadsForGroupAsync(&self, group: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploaderStaticMethods {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundUploaderStaticMethodsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderStaticMethodsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderStaticMethodsVtbl {
        unsafe extern "system" fn GetCurrentUploadsAsync<Impl: IBackgroundUploaderStaticMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentUploadsForGroupAsync<Impl: IBackgroundUploaderStaticMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundUploaderStaticMethods>, ::windows::core::GetTrustLevel, GetCurrentUploadsAsync::<Impl, IMPL_OFFSET>, GetCurrentUploadsForGroupAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderStaticMethods2Impl: Sized {
    fn GetCurrentUploadsForTransferGroupAsync(&self, group: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploaderStaticMethods2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundUploaderStaticMethods2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderStaticMethods2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderStaticMethods2Vtbl {
        unsafe extern "system" fn GetCurrentUploadsForTransferGroupAsync<Impl: IBackgroundUploaderStaticMethods2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundUploaderStaticMethods2>, ::windows::core::GetTrustLevel, GetCurrentUploadsForTransferGroupAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderStaticMethods2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderUserConsentImpl: Sized {
    fn RequestUnconstrainedUploadsAsync(&self, operations: &::core::option::Option<super::super::Foundation::Collections::IIterable<UploadOperation>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundUploaderUserConsent {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundUploaderUserConsentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundUploaderUserConsentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundUploaderUserConsentVtbl {
        unsafe extern "system" fn RequestUnconstrainedUploadsAsync<Impl: IBackgroundUploaderUserConsentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundUploaderUserConsent>, ::windows::core::GetTrustLevel, RequestUnconstrainedUploadsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundUploaderUserConsent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContentPrefetcherImpl: Sized {
    fn ContentUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn SetIndirectContentUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn IndirectContentUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentPrefetcher {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IContentPrefetcher";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContentPrefetcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetcherVtbl {
        unsafe extern "system" fn ContentUris<Impl: IContentPrefetcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIndirectContentUri<Impl: IContentPrefetcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndirectContentUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IndirectContentUri<Impl: IContentPrefetcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentPrefetcher>, ::windows::core::GetTrustLevel, ContentUris::<Impl, IMPL_OFFSET>, SetIndirectContentUri::<Impl, IMPL_OFFSET>, IndirectContentUri::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContentPrefetcherTimeImpl: Sized {
    fn LastSuccessfulPrefetchTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentPrefetcherTime {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IContentPrefetcherTime";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContentPrefetcherTimeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetcherTimeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetcherTimeVtbl {
        unsafe extern "system" fn LastSuccessfulPrefetchTime<Impl: IContentPrefetcherTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentPrefetcherTime>, ::windows::core::GetTrustLevel, LastSuccessfulPrefetchTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetcherTime as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDownloadOperationImpl: Sized + IBackgroundTransferOperationImpl {
    fn ResultFile(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn Progress(&self) -> ::windows::core::Result<BackgroundDownloadProgress>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>;
    fn AttachAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDownloadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDownloadOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperationVtbl {
        unsafe extern "system" fn ResultFile<Impl: IDownloadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Progress<Impl: IDownloadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundDownloadProgress) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAsync<Impl: IDownloadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AttachAsync<Impl: IDownloadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pause<Impl: IDownloadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IDownloadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDownloadOperation>, ::windows::core::GetTrustLevel, ResultFile::<Impl, IMPL_OFFSET>, Progress::<Impl, IMPL_OFFSET>, StartAsync::<Impl, IMPL_OFFSET>, AttachAsync::<Impl, IMPL_OFFSET>, Pause::<Impl, IMPL_OFFSET>, Resume::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadOperation2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation2Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IDownloadOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDownloadOperation2>, ::windows::core::GetTrustLevel, TransferGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "Web", feature = "implement_exclusive"))]
pub trait IDownloadOperation3Impl: Sized {
    fn IsRandomAccessRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsRandomAccessRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetResultRandomAccessStreamReference(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetDownloadedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>;
    fn RangesDownloaded(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRangesDownloaded(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetRequestedUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn RecoverableWebErrorStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>>;
    fn CurrentWebErrorStatus(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDownloadOperation3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "Web", feature = "implement_exclusive"))]
impl IDownloadOperation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation3Vtbl {
        unsafe extern "system" fn IsRandomAccessRequired<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRandomAccessRequired<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRandomAccessRequired(value).into()
        }
        unsafe extern "system" fn GetResultRandomAccessStreamReference<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDownloadedRanges<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RangesDownloaded<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRangesDownloaded<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRangesDownloaded(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetRequestedUri<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecoverableWebErrorStatuses<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentWebErrorStatus<Impl: IDownloadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDownloadOperation3>,
            ::windows::core::GetTrustLevel,
            IsRandomAccessRequired::<Impl, IMPL_OFFSET>,
            SetIsRandomAccessRequired::<Impl, IMPL_OFFSET>,
            GetResultRandomAccessStreamReference::<Impl, IMPL_OFFSET>,
            GetDownloadedRanges::<Impl, IMPL_OFFSET>,
            RangesDownloaded::<Impl, IMPL_OFFSET>,
            RemoveRangesDownloaded::<Impl, IMPL_OFFSET>,
            SetRequestedUri::<Impl, IMPL_OFFSET>,
            RecoverableWebErrorStatuses::<Impl, IMPL_OFFSET>,
            CurrentWebErrorStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation4Impl: Sized {
    fn MakeCurrentInTransferGroup(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadOperation4 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation4";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadOperation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation4Vtbl {
        unsafe extern "system" fn MakeCurrentInTransferGroup<Impl: IDownloadOperation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeCurrentInTransferGroup().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDownloadOperation4>, ::windows::core::GetTrustLevel, MakeCurrentInTransferGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation5Impl: Sized {
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveRequestHeader(&self, headername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadOperation5 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IDownloadOperation5";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadOperation5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadOperation5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadOperation5Vtbl {
        unsafe extern "system" fn SetRequestHeader<Impl: IDownloadOperation5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveRequestHeader<Impl: IDownloadOperation5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDownloadOperation5>, ::windows::core::GetTrustLevel, SetRequestHeader::<Impl, IMPL_OFFSET>, RemoveRequestHeader::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadOperation5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResponseInformationImpl: Sized {
    fn IsResumable(&self) -> ::windows::core::Result<bool>;
    fn ActualUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn StatusCode(&self) -> ::windows::core::Result<u32>;
    fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResponseInformation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IResponseInformation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResponseInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResponseInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResponseInformationVtbl {
        unsafe extern "system" fn IsResumable<Impl: IResponseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActualUri<Impl: IResponseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StatusCode<Impl: IResponseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Headers<Impl: IResponseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResponseInformation>, ::windows::core::GetTrustLevel, IsResumable::<Impl, IMPL_OFFSET>, ActualUri::<Impl, IMPL_OFFSET>, StatusCode::<Impl, IMPL_OFFSET>, Headers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResponseInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IUnconstrainedTransferRequestResultImpl: Sized {
    fn IsUnconstrained(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUnconstrainedTransferRequestResult {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IUnconstrainedTransferRequestResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnconstrainedTransferRequestResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnconstrainedTransferRequestResultVtbl {
        unsafe extern "system" fn IsUnconstrained<Impl: IUnconstrainedTransferRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUnconstrainedTransferRequestResult>, ::windows::core::GetTrustLevel, IsUnconstrained::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnconstrainedTransferRequestResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUploadOperationImpl: Sized + IBackgroundTransferOperationImpl {
    fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn Progress(&self) -> ::windows::core::Result<BackgroundUploadProgress>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>;
    fn AttachAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUploadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUploadOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperationVtbl {
        unsafe extern "system" fn SourceFile<Impl: IUploadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Progress<Impl: IUploadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundUploadProgress) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAsync<Impl: IUploadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AttachAsync<Impl: IUploadOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUploadOperation>, ::windows::core::GetTrustLevel, SourceFile::<Impl, IMPL_OFFSET>, Progress::<Impl, IMPL_OFFSET>, StartAsync::<Impl, IMPL_OFFSET>, AttachAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUploadOperation2 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl IUploadOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperation2Vtbl {
        unsafe extern "system" fn TransferGroup<Impl: IUploadOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUploadOperation2>, ::windows::core::GetTrustLevel, TransferGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation3Impl: Sized {
    fn MakeCurrentInTransferGroup(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUploadOperation3 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation3";
}
#[cfg(feature = "implement_exclusive")]
impl IUploadOperation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperation3Vtbl {
        unsafe extern "system" fn MakeCurrentInTransferGroup<Impl: IUploadOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeCurrentInTransferGroup().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUploadOperation3>, ::windows::core::GetTrustLevel, MakeCurrentInTransferGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation4Impl: Sized {
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveRequestHeader(&self, headername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUploadOperation4 {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IUploadOperation4";
}
#[cfg(feature = "implement_exclusive")]
impl IUploadOperation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUploadOperation4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUploadOperation4Vtbl {
        unsafe extern "system" fn SetRequestHeader<Impl: IUploadOperation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveRequestHeader<Impl: IUploadOperation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUploadOperation4>, ::windows::core::GetTrustLevel, SetRequestHeader::<Impl, IMPL_OFFSET>, RemoveRequestHeader::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUploadOperation4 as ::windows::core::Interface>::IID
    }
}
