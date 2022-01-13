#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGameSaveBlobGetResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveBlobGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGameSaveBlobGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobGetResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveBlobGetResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveBlobGetResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveBlobGetResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobInfoImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Size(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveBlobInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveBlobInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveBlobInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveBlobInfo, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveBlobInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGameSaveBlobInfoGetResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveBlobInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGameSaveBlobInfoGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobInfoGetResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveBlobInfoGetResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveBlobInfoGetResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveBlobInfoGetResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameSaveBlobInfoQueryImpl: Sized {
    fn GetBlobInfoAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>;
    fn GetBlobInfoWithIndexAndMaxAsync(&mut self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>;
    fn GetItemCountAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveBlobInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameSaveBlobInfoQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveBlobInfoQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveBlobInfoQueryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveBlobInfoQuery, BASE_OFFSET>(),
            GetBlobInfoAsync: GetBlobInfoAsync::<Impl, IMPL_OFFSET>,
            GetBlobInfoWithIndexAndMaxAsync: GetBlobInfoWithIndexAndMaxAsync::<Impl, IMPL_OFFSET>,
            GetItemCountAsync: GetItemCountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveBlobInfoQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGameSaveContainerImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Provider(&mut self) -> ::windows::core::Result<GameSaveProvider>;
    fn SubmitUpdatesAsync(&mut self, blobstowrite: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>, blobstodelete: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn ReadAsync(&mut self, blobstoread: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn GetAsync(&mut self, blobstoread: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>>;
    fn SubmitPropertySetUpdatesAsync(&mut self, blobstowrite: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>, blobstodelete: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn CreateBlobInfoQuery(&mut self, blobnameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveBlobInfoQuery>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveContainer {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGameSaveContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveContainerVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveContainer, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Provider: Provider::<Impl, IMPL_OFFSET>,
            SubmitUpdatesAsync: SubmitUpdatesAsync::<Impl, IMPL_OFFSET>,
            ReadAsync: ReadAsync::<Impl, IMPL_OFFSET>,
            GetAsync: GetAsync::<Impl, IMPL_OFFSET>,
            SubmitPropertySetUpdatesAsync: SubmitPropertySetUpdatesAsync::<Impl, IMPL_OFFSET>,
            CreateBlobInfoQuery: CreateBlobInfoQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameSaveContainerInfoImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TotalSize(&mut self) -> ::windows::core::Result<u64>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastModifiedTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NeedsSync(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveContainerInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameSaveContainerInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveContainerInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveContainerInfo, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            TotalSize: TotalSize::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            LastModifiedTime: LastModifiedTime::<Impl, IMPL_OFFSET>,
            NeedsSync: NeedsSync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveContainerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGameSaveContainerInfoGetResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveContainerInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGameSaveContainerInfoGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerInfoGetResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveContainerInfoGetResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveContainerInfoGetResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveContainerInfoGetResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameSaveContainerInfoQueryImpl: Sized {
    fn GetContainerInfoAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>;
    fn GetContainerInfoWithIndexAndMaxAsync(&mut self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>;
    fn GetItemCountAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveContainerInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameSaveContainerInfoQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveContainerInfoQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveContainerInfoQueryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveContainerInfoQuery, BASE_OFFSET>(),
            GetContainerInfoAsync: GetContainerInfoAsync::<Impl, IMPL_OFFSET>,
            GetContainerInfoWithIndexAndMaxAsync: GetContainerInfoWithIndexAndMaxAsync::<Impl, IMPL_OFFSET>,
            GetItemCountAsync: GetItemCountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveContainerInfoQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveOperationResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GameSaveErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveOperationResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveOperationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveOperationResultVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveOperationResult, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IGameSaveProviderImpl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::super::System::User>;
    fn CreateContainer(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainer>;
    fn DeleteContainerAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn CreateContainerInfoQuery(&mut self) -> ::windows::core::Result<GameSaveContainerInfoQuery>;
    fn CreateContainerInfoQueryWithName(&mut self, containernameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainerInfoQuery>;
    fn GetRemainingBytesInQuotaAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<i64>>;
    fn ContainersChangedSinceLastSync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveProvider {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveProvider";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IGameSaveProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveProviderVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveProvider, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            CreateContainer: CreateContainer::<Impl, IMPL_OFFSET>,
            DeleteContainerAsync: DeleteContainerAsync::<Impl, IMPL_OFFSET>,
            CreateContainerInfoQuery: CreateContainerInfoQuery::<Impl, IMPL_OFFSET>,
            CreateContainerInfoQueryWithName: CreateContainerInfoQueryWithName::<Impl, IMPL_OFFSET>,
            GetRemainingBytesInQuotaAsync: GetRemainingBytesInQuotaAsync::<Impl, IMPL_OFFSET>,
            ContainersChangedSinceLastSync: ContainersChangedSinceLastSync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveProviderGetResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&mut self) -> ::windows::core::Result<GameSaveProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameSaveProviderGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGameSaveProviderGetResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveProviderGetResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveProviderGetResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveProviderGetResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveProviderGetResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IGameSaveProviderStaticsImpl: Sized {
    fn GetForUserAsync(&mut self, user: &::core::option::Option<super::super::super::System::User>, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>;
    fn GetSyncOnDemandForUserAsync(&mut self, user: &::core::option::Option<super::super::super::System::User>, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameSaveProviderStatics {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IGameSaveProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameSaveProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameSaveProviderStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameSaveProviderStatics, BASE_OFFSET>(),
            GetForUserAsync: GetForUserAsync::<Impl, IMPL_OFFSET>,
            GetSyncOnDemandForUserAsync: GetSyncOnDemandForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameSaveProviderStatics as ::windows::core::Interface>::IID
    }
}
