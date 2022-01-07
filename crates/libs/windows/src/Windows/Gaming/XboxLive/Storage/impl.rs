#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveBlobGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveBlobGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobGetResultImpl, const OFFSET: isize>() -> IGameSaveBlobGetResultVtbl {
        unsafe extern "system" fn Status<Impl: IGameSaveBlobGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IGameSaveBlobGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveBlobGetResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobInfoImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Size(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveBlobInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveBlobInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobInfoImpl, const OFFSET: isize>() -> IGameSaveBlobInfoVtbl {
        unsafe extern "system" fn Name<Impl: IGameSaveBlobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<Impl: IGameSaveBlobInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveBlobInfo>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Size::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobInfoGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveBlobInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveBlobInfoGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobInfoGetResultImpl, const OFFSET: isize>() -> IGameSaveBlobInfoGetResultVtbl {
        unsafe extern "system" fn Status<Impl: IGameSaveBlobInfoGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IGameSaveBlobInfoGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveBlobInfoGetResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobInfoQueryImpl: Sized {
    fn GetBlobInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>;
    fn GetBlobInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>;
    fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveBlobInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveBlobInfoQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobInfoQueryImpl, const OFFSET: isize>() -> IGameSaveBlobInfoQueryVtbl {
        unsafe extern "system" fn GetBlobInfoAsync<Impl: IGameSaveBlobInfoQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlobInfoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobInfoWithIndexAndMaxAsync<Impl: IGameSaveBlobInfoQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlobInfoWithIndexAndMaxAsync(startindex, maxnumberofitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCountAsync<Impl: IGameSaveBlobInfoQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveBlobInfoQuery>, ::windows::core::GetTrustLevel, GetBlobInfoAsync::<Impl, OFFSET>, GetBlobInfoWithIndexAndMaxAsync::<Impl, OFFSET>, GetItemCountAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Provider(&self) -> ::windows::core::Result<GameSaveProvider>;
    fn SubmitUpdatesAsync(&self, blobstowrite: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>, blobstodelete: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn ReadAsync(&self, blobstoread: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn GetAsync(&self, blobstoread: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>>;
    fn SubmitPropertySetUpdatesAsync(&self, blobstowrite: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>, blobstodelete: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn CreateBlobInfoQuery(&self, blobnameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveBlobInfoQuery>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveContainer {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainer";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerImpl, const OFFSET: isize>() -> IGameSaveContainerVtbl {
        unsafe extern "system" fn Name<Impl: IGameSaveContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Provider<Impl: IGameSaveContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Provider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitUpdatesAsync<Impl: IGameSaveContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobstowrite: ::windows::core::RawPtr, blobstodelete: ::windows::core::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitUpdatesAsync(
                &*(&blobstowrite as *const <super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer> as ::windows::core::DefaultType>::DefaultType),
                &*(&blobstodelete as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsync<Impl: IGameSaveContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobstoread: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadAsync(&*(&blobstoread as *const <super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Impl: IGameSaveContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobstoread: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsync(&*(&blobstoread as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitPropertySetUpdatesAsync<Impl: IGameSaveContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobstowrite: ::windows::core::RawPtr, blobstodelete: ::windows::core::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitPropertySetUpdatesAsync(
                &*(&blobstowrite as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                &*(&blobstodelete as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobInfoQuery<Impl: IGameSaveContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobnameprefix: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobInfoQuery(&*(&blobnameprefix as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IGameSaveContainer>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            Provider::<Impl, OFFSET>,
            SubmitUpdatesAsync::<Impl, OFFSET>,
            ReadAsync::<Impl, OFFSET>,
            GetAsync::<Impl, OFFSET>,
            SubmitPropertySetUpdatesAsync::<Impl, OFFSET>,
            CreateBlobInfoQuery::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerInfoImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TotalSize(&self) -> ::windows::core::Result<u64>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastModifiedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NeedsSync(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveContainerInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveContainerInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerInfoImpl, const OFFSET: isize>() -> IGameSaveContainerInfoVtbl {
        unsafe extern "system" fn Name<Impl: IGameSaveContainerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TotalSize<Impl: IGameSaveContainerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IGameSaveContainerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastModifiedTime<Impl: IGameSaveContainerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeedsSync<Impl: IGameSaveContainerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeedsSync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveContainerInfo>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, TotalSize::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, LastModifiedTime::<Impl, OFFSET>, NeedsSync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerInfoGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveContainerInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveContainerInfoGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerInfoGetResultImpl, const OFFSET: isize>() -> IGameSaveContainerInfoGetResultVtbl {
        unsafe extern "system" fn Status<Impl: IGameSaveContainerInfoGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IGameSaveContainerInfoGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveContainerInfoGetResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerInfoQueryImpl: Sized {
    fn GetContainerInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>;
    fn GetContainerInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>;
    fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveContainerInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveContainerInfoQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerInfoQueryImpl, const OFFSET: isize>() -> IGameSaveContainerInfoQueryVtbl {
        unsafe extern "system" fn GetContainerInfoAsync<Impl: IGameSaveContainerInfoQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerInfoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerInfoWithIndexAndMaxAsync<Impl: IGameSaveContainerInfoQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerInfoWithIndexAndMaxAsync(startindex, maxnumberofitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCountAsync<Impl: IGameSaveContainerInfoQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveContainerInfoQuery>, ::windows::core::GetTrustLevel, GetContainerInfoAsync::<Impl, OFFSET>, GetContainerInfoWithIndexAndMaxAsync::<Impl, OFFSET>, GetItemCountAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveOperationResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveOperationResultImpl, const OFFSET: isize>() -> IGameSaveOperationResultVtbl {
        unsafe extern "system" fn Status<Impl: IGameSaveOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveOperationResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveProviderImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::super::System::User>;
    fn CreateContainer(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainer>;
    fn DeleteContainerAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn CreateContainerInfoQuery(&self) -> ::windows::core::Result<GameSaveContainerInfoQuery>;
    fn CreateContainerInfoQueryWithName(&self, containernameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainerInfoQuery>;
    fn GetRemainingBytesInQuotaAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<i64>>;
    fn ContainersChangedSinceLastSync(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveProvider {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveProviderImpl, const OFFSET: isize>() -> IGameSaveProviderVtbl {
        unsafe extern "system" fn User<Impl: IGameSaveProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateContainer<Impl: IGameSaveProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContainer(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteContainerAsync<Impl: IGameSaveProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteContainerAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContainerInfoQuery<Impl: IGameSaveProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContainerInfoQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContainerInfoQueryWithName<Impl: IGameSaveProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containernameprefix: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContainerInfoQueryWithName(&*(&containernameprefix as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemainingBytesInQuotaAsync<Impl: IGameSaveProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemainingBytesInQuotaAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainersChangedSinceLastSync<Impl: IGameSaveProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainersChangedSinceLastSync() {
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
            ::windows::core::GetRuntimeClassName::<IGameSaveProvider>,
            ::windows::core::GetTrustLevel,
            User::<Impl, OFFSET>,
            CreateContainer::<Impl, OFFSET>,
            DeleteContainerAsync::<Impl, OFFSET>,
            CreateContainerInfoQuery::<Impl, OFFSET>,
            CreateContainerInfoQueryWithName::<Impl, OFFSET>,
            GetRemainingBytesInQuotaAsync::<Impl, OFFSET>,
            ContainersChangedSinceLastSync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveProviderGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<GameSaveProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveProviderGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveProviderGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveProviderGetResultImpl, const OFFSET: isize>() -> IGameSaveProviderGetResultVtbl {
        unsafe extern "system" fn Status<Impl: IGameSaveProviderGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IGameSaveProviderGetResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveProviderGetResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveProviderStaticsImpl: Sized {
    fn GetForUserAsync(&self, user: &::core::option::Option<super::super::super::System::User>, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>;
    fn GetSyncOnDemandForUserAsync(&self, user: &::core::option::Option<super::super::super::System::User>, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveProviderStatics {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveProviderStaticsImpl, const OFFSET: isize>() -> IGameSaveProviderStaticsVtbl {
        unsafe extern "system" fn GetForUserAsync<Impl: IGameSaveProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, serviceconfigid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUserAsync(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&serviceconfigid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncOnDemandForUserAsync<Impl: IGameSaveProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, serviceconfigid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncOnDemandForUserAsync(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&serviceconfigid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameSaveProviderStatics>, ::windows::core::GetTrustLevel, GetForUserAsync::<Impl, OFFSET>, GetSyncOnDemandForUserAsync::<Impl, OFFSET>)
    }
}
