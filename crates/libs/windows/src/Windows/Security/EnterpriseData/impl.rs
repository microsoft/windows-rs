#[cfg(feature = "implement_exclusive")]
pub trait IBufferProtectUnprotectResultImpl: Sized {
    fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ProtectionInfo(&self) -> ::windows::core::Result<DataProtectionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBufferProtectUnprotectResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IBufferProtectUnprotectResult";
}
#[cfg(feature = "implement_exclusive")]
impl IBufferProtectUnprotectResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBufferProtectUnprotectResultImpl, const OFFSET: isize>() -> IBufferProtectUnprotectResultVtbl {
        unsafe extern "system" fn Buffer<Impl: IBufferProtectUnprotectResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionInfo<Impl: IBufferProtectUnprotectResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBufferProtectUnprotectResult>, ::windows::core::GetTrustLevel, Buffer::<Impl, OFFSET>, ProtectionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionInfoImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DataProtectionStatus>;
    fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IDataProtectionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IDataProtectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataProtectionInfoImpl, const OFFSET: isize>() -> IDataProtectionInfoVtbl {
        unsafe extern "system" fn Status<Impl: IDataProtectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataProtectionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Identity<Impl: IDataProtectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataProtectionInfo>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Identity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionManagerStaticsImpl: Sized {
    fn ProtectAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>, identity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>;
    fn UnprotectAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>;
    fn ProtectStreamAsync(&self, unprotectedstream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, identity: &::windows::core::HSTRING, protectedstream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
    fn UnprotectStreamAsync(&self, protectedstream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, unprotectedstream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
    fn GetProtectionInfoAsync(&self, protecteddata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
    fn GetStreamProtectionInfoAsync(&self, protectedstream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataProtectionManagerStatics {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IDataProtectionManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDataProtectionManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataProtectionManagerStaticsImpl, const OFFSET: isize>() -> IDataProtectionManagerStaticsVtbl {
        unsafe extern "system" fn ProtectAsync<Impl: IDataProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectAsync(&*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprotectAsync<Impl: IDataProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprotectAsync(&*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectStreamAsync<Impl: IDataProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unprotectedstream: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectedstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectStreamAsync(
                &*(&unprotectedstream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
                &*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&protectedstream as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprotectStreamAsync<Impl: IDataProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectedstream: ::windows::core::RawPtr, unprotectedstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprotectStreamAsync(&*(&protectedstream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&unprotectedstream as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtectionInfoAsync<Impl: IDataProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protecteddata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtectionInfoAsync(&*(&protecteddata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamProtectionInfoAsync<Impl: IDataProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectedstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamProtectionInfoAsync(&*(&protectedstream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IDataProtectionManagerStatics>,
            ::windows::core::GetTrustLevel,
            ProtectAsync::<Impl, OFFSET>,
            UnprotectAsync::<Impl, OFFSET>,
            ProtectStreamAsync::<Impl, OFFSET>,
            UnprotectStreamAsync::<Impl, OFFSET>,
            GetProtectionInfoAsync::<Impl, OFFSET>,
            GetStreamProtectionInfoAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionInfoImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<FileProtectionStatus>;
    fn IsRoamable(&self) -> ::windows::core::Result<bool>;
    fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileProtectionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IFileProtectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileProtectionInfoImpl, const OFFSET: isize>() -> IFileProtectionInfoVtbl {
        unsafe extern "system" fn Status<Impl: IFileProtectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FileProtectionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsRoamable<Impl: IFileProtectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoamable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identity<Impl: IFileProtectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileProtectionInfo>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, IsRoamable::<Impl, OFFSET>, Identity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionInfo2Impl: Sized {
    fn IsProtectWhileOpenSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileProtectionInfo2 {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileProtectionInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IFileProtectionInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileProtectionInfo2Impl, const OFFSET: isize>() -> IFileProtectionInfo2Vtbl {
        unsafe extern "system" fn IsProtectWhileOpenSupported<Impl: IFileProtectionInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProtectWhileOpenSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileProtectionInfo2>, ::windows::core::GetTrustLevel, IsProtectWhileOpenSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionManagerStaticsImpl: Sized {
    fn ProtectAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>, identity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
    fn CopyProtectionAsync(&self, source: &::core::option::Option<super::super::Storage::IStorageItem>, target: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetProtectionInfoAsync(&self, source: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
    fn SaveFileAsContainerAsync(&self, protectedfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>;
    fn LoadFileFromContainerAsync(&self, containerfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>;
    fn LoadFileFromContainerWithTargetAsync(&self, containerfile: &::core::option::Option<super::super::Storage::IStorageFile>, target: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>;
    fn CreateProtectedAndOpenAsync(&self, parentfolder: &::core::option::Option<super::super::Storage::IStorageFolder>, desiredname: &::windows::core::HSTRING, identity: &::windows::core::HSTRING, collisionoption: super::super::Storage::CreationCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileProtectionManagerStatics {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileProtectionManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFileProtectionManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>() -> IFileProtectionManagerStaticsVtbl {
        unsafe extern "system" fn ProtectAsync<Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectAsync(&*(&target as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyProtectionAsync<Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyProtectionAsync(&*(&source as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&target as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtectionInfoAsync<Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtectionInfoAsync(&*(&source as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFileAsContainerAsync<Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectedfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveFileAsContainerAsync(&*(&protectedfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFileFromContainerAsync<Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containerfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFileFromContainerAsync(&*(&containerfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFileFromContainerWithTargetAsync<Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containerfile: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFileFromContainerWithTargetAsync(&*(&containerfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&target as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProtectedAndOpenAsync<Impl: IFileProtectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentfolder: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, collisionoption: super::super::Storage::CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProtectedAndOpenAsync(
                &*(&parentfolder as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType),
                &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                collisionoption,
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
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFileProtectionManagerStatics>,
            ::windows::core::GetTrustLevel,
            ProtectAsync::<Impl, OFFSET>,
            CopyProtectionAsync::<Impl, OFFSET>,
            GetProtectionInfoAsync::<Impl, OFFSET>,
            SaveFileAsContainerAsync::<Impl, OFFSET>,
            LoadFileFromContainerAsync::<Impl, OFFSET>,
            LoadFileFromContainerWithTargetAsync::<Impl, OFFSET>,
            CreateProtectedAndOpenAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionManagerStatics2Impl: Sized {
    fn IsContainerAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync(&self, containerfile: &::core::option::Option<super::super::Storage::IStorageFile>, target: &::core::option::Option<super::super::Storage::IStorageItem>, collisionoption: super::super::Storage::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>;
    fn SaveFileAsContainerWithSharingAsync(&self, protectedfile: &::core::option::Option<super::super::Storage::IStorageFile>, sharedwithidentities: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileProtectionManagerStatics2 {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileProtectionManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFileProtectionManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileProtectionManagerStatics2Impl, const OFFSET: isize>() -> IFileProtectionManagerStatics2Vtbl {
        unsafe extern "system" fn IsContainerAsync<Impl: IFileProtectionManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContainerAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync<Impl: IFileProtectionManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containerfile: ::windows::core::RawPtr, target: ::windows::core::RawPtr, collisionoption: super::super::Storage::NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFileFromContainerWithTargetAndNameCollisionOptionAsync(&*(&containerfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&target as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), collisionoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFileAsContainerWithSharingAsync<Impl: IFileProtectionManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectedfile: ::windows::core::RawPtr, sharedwithidentities: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveFileAsContainerWithSharingAsync(&*(&protectedfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&sharedwithidentities as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileProtectionManagerStatics2>, ::windows::core::GetTrustLevel, IsContainerAsync::<Impl, OFFSET>, LoadFileFromContainerWithTargetAndNameCollisionOptionAsync::<Impl, OFFSET>, SaveFileAsContainerWithSharingAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileProtectionManagerStatics3Impl: Sized {
    fn UnprotectAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
    fn UnprotectWithOptionsAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>, options: &::core::option::Option<FileUnprotectOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileProtectionManagerStatics3 {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileProtectionManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IFileProtectionManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileProtectionManagerStatics3Impl, const OFFSET: isize>() -> IFileProtectionManagerStatics3Vtbl {
        unsafe extern "system" fn UnprotectAsync<Impl: IFileProtectionManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprotectAsync(&*(&target as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprotectWithOptionsAsync<Impl: IFileProtectionManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprotectWithOptionsAsync(&*(&target as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <FileUnprotectOptions as ::windows::core::Abi>::Abi as *const <FileUnprotectOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileProtectionManagerStatics3>, ::windows::core::GetTrustLevel, UnprotectAsync::<Impl, OFFSET>, UnprotectWithOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IFileRevocationManagerStaticsImpl: Sized {
    fn ProtectAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>, enterpriseidentity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>;
    fn CopyProtectionAsync(&self, sourcestorageitem: &::core::option::Option<super::super::Storage::IStorageItem>, targetstorageitem: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Revoke(&self, enterpriseidentity: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetStatusAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileRevocationManagerStatics {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileRevocationManagerStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IFileRevocationManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileRevocationManagerStaticsImpl, const OFFSET: isize>() -> IFileRevocationManagerStaticsVtbl {
        unsafe extern "system" fn ProtectAsync<Impl: IFileRevocationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, enterpriseidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectAsync(&*(&storageitem as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&enterpriseidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyProtectionAsync<Impl: IFileRevocationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestorageitem: ::windows::core::RawPtr, targetstorageitem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyProtectionAsync(&*(&sourcestorageitem as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&targetstorageitem as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revoke<Impl: IFileRevocationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enterpriseidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Revoke(&*(&enterpriseidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetStatusAsync<Impl: IFileRevocationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusAsync(&*(&storageitem as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileRevocationManagerStatics>, ::windows::core::GetTrustLevel, ProtectAsync::<Impl, OFFSET>, CopyProtectionAsync::<Impl, OFFSET>, Revoke::<Impl, OFFSET>, GetStatusAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUnprotectOptionsImpl: Sized {
    fn SetAudit(&self, value: bool) -> ::windows::core::Result<()>;
    fn Audit(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileUnprotectOptions {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileUnprotectOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFileUnprotectOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileUnprotectOptionsImpl, const OFFSET: isize>() -> IFileUnprotectOptionsVtbl {
        unsafe extern "system" fn SetAudit<Impl: IFileUnprotectOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudit(value).into()
        }
        unsafe extern "system" fn Audit<Impl: IFileUnprotectOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Audit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileUnprotectOptions>, ::windows::core::GetTrustLevel, SetAudit::<Impl, OFFSET>, Audit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileUnprotectOptionsFactoryImpl: Sized {
    fn Create(&self, audit: bool) -> ::windows::core::Result<FileUnprotectOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileUnprotectOptionsFactory {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IFileUnprotectOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFileUnprotectOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileUnprotectOptionsFactoryImpl, const OFFSET: isize>() -> IFileUnprotectOptionsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IFileUnprotectOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audit: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(audit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileUnprotectOptionsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedAccessResumedEventArgsImpl: Sized {
    fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectedAccessResumedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectedAccessResumedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectedAccessResumedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedAccessResumedEventArgsImpl, const OFFSET: isize>() -> IProtectedAccessResumedEventArgsVtbl {
        unsafe extern "system" fn Identities<Impl: IProtectedAccessResumedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectedAccessResumedEventArgs>, ::windows::core::GetTrustLevel, Identities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedAccessSuspendingEventArgsImpl: Sized {
    fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectedAccessSuspendingEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectedAccessSuspendingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectedAccessSuspendingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedAccessSuspendingEventArgsImpl, const OFFSET: isize>() -> IProtectedAccessSuspendingEventArgsVtbl {
        unsafe extern "system" fn Identities<Impl: IProtectedAccessSuspendingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: IProtectedAccessSuspendingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IProtectedAccessSuspendingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectedAccessSuspendingEventArgs>, ::windows::core::GetTrustLevel, Identities::<Impl, OFFSET>, Deadline::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedContainerExportResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ProtectedImportExportStatus>;
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectedContainerExportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectedContainerExportResult";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectedContainerExportResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedContainerExportResultImpl, const OFFSET: isize>() -> IProtectedContainerExportResultVtbl {
        unsafe extern "system" fn Status<Impl: IProtectedContainerExportResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn File<Impl: IProtectedContainerExportResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectedContainerExportResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, File::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedContainerImportResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ProtectedImportExportStatus>;
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectedContainerImportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectedContainerImportResult";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectedContainerImportResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedContainerImportResultImpl, const OFFSET: isize>() -> IProtectedContainerImportResultVtbl {
        unsafe extern "system" fn Status<Impl: IProtectedContainerImportResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn File<Impl: IProtectedContainerImportResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectedContainerImportResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, File::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedContentRevokedEventArgsImpl: Sized {
    fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectedContentRevokedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectedContentRevokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectedContentRevokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedContentRevokedEventArgsImpl, const OFFSET: isize>() -> IProtectedContentRevokedEventArgsVtbl {
        unsafe extern "system" fn Identities<Impl: IProtectedContentRevokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectedContentRevokedEventArgs>, ::windows::core::GetTrustLevel, Identities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectedFileCreateResultImpl: Sized {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn Stream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn ProtectionInfo(&self) -> ::windows::core::Result<FileProtectionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectedFileCreateResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectedFileCreateResult";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectedFileCreateResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectedFileCreateResultImpl, const OFFSET: isize>() -> IProtectedFileCreateResultVtbl {
        unsafe extern "system" fn File<Impl: IProtectedFileCreateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: IProtectedFileCreateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionInfo<Impl: IProtectedFileCreateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectedFileCreateResult>, ::windows::core::GetTrustLevel, File::<Impl, OFFSET>, Stream::<Impl, OFFSET>, ProtectionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyAuditInfoImpl: Sized {
    fn SetAction(&self, value: ProtectionPolicyAuditAction) -> ::windows::core::Result<()>;
    fn Action(&self) -> ::windows::core::Result<ProtectionPolicyAuditAction>;
    fn SetDataDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DataDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSourceDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyAuditInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyAuditInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyAuditInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>() -> IProtectionPolicyAuditInfoVtbl {
        unsafe extern "system" fn SetAction<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProtectionPolicyAuditAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAction(value).into()
        }
        unsafe extern "system" fn Action<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProtectionPolicyAuditAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Action() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataDescription<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DataDescription<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceDescription<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceDescription<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetDescription<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetDescription<Impl: IProtectionPolicyAuditInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetDescription() {
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
            ::windows::core::GetRuntimeClassName::<IProtectionPolicyAuditInfo>,
            ::windows::core::GetTrustLevel,
            SetAction::<Impl, OFFSET>,
            Action::<Impl, OFFSET>,
            SetDataDescription::<Impl, OFFSET>,
            DataDescription::<Impl, OFFSET>,
            SetSourceDescription::<Impl, OFFSET>,
            SourceDescription::<Impl, OFFSET>,
            SetTargetDescription::<Impl, OFFSET>,
            TargetDescription::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyAuditInfoFactoryImpl: Sized {
    fn Create(&self, action: ProtectionPolicyAuditAction, datadescription: &::windows::core::HSTRING, sourcedescription: &::windows::core::HSTRING, targetdescription: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyAuditInfo>;
    fn CreateWithActionAndDataDescription(&self, action: ProtectionPolicyAuditAction, datadescription: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyAuditInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyAuditInfoFactory {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyAuditInfoFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyAuditInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyAuditInfoFactoryImpl, const OFFSET: isize>() -> IProtectionPolicyAuditInfoFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IProtectionPolicyAuditInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sourcedescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetdescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                action,
                &*(&datadescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&sourcedescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&targetdescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithActionAndDataDescription<Impl: IProtectionPolicyAuditInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithActionAndDataDescription(action, &*(&datadescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectionPolicyAuditInfoFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>, CreateWithActionAndDataDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerImpl: Sized {
    fn SetIdentity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyManager";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerImpl, const OFFSET: isize>() -> IProtectionPolicyManagerVtbl {
        unsafe extern "system" fn SetIdentity<Impl: IProtectionPolicyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIdentity(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Identity<Impl: IProtectionPolicyManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectionPolicyManager>, ::windows::core::GetTrustLevel, SetIdentity::<Impl, OFFSET>, Identity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManager2Impl: Sized {
    fn SetShowEnterpriseIndicator(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowEnterpriseIndicator(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyManager2 {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManager2Impl, const OFFSET: isize>() -> IProtectionPolicyManager2Vtbl {
        unsafe extern "system" fn SetShowEnterpriseIndicator<Impl: IProtectionPolicyManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowEnterpriseIndicator(value).into()
        }
        unsafe extern "system" fn ShowEnterpriseIndicator<Impl: IProtectionPolicyManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowEnterpriseIndicator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectionPolicyManager2>, ::windows::core::GetTrustLevel, SetShowEnterpriseIndicator::<Impl, OFFSET>, ShowEnterpriseIndicator::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStaticsImpl: Sized {
    fn IsIdentityManaged(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryApplyProcessUIPolicy(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn ClearProcessUIPolicy(&self) -> ::windows::core::Result<()>;
    fn CreateCurrentThreadNetworkContext(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ThreadNetworkContext>;
    fn GetPrimaryManagedIdentityForNetworkEndpointAsync(&self, endpointhost: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RevokeContent(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<ProtectionPolicyManager>;
    fn ProtectedAccessSuspending(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectedAccessSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProtectedAccessResumed(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectedAccessResumed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProtectedContentRevoked(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectedContentRevoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CheckAccess(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyEvaluationResult>;
    fn RequestAccessAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerStatics {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>() -> IProtectionPolicyManagerStaticsVtbl {
        unsafe extern "system" fn IsIdentityManaged<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIdentityManaged(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryApplyProcessUIPolicy<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryApplyProcessUIPolicy(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearProcessUIPolicy<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearProcessUIPolicy().into()
        }
        unsafe extern "system" fn CreateCurrentThreadNetworkContext<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCurrentThreadNetworkContext(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrimaryManagedIdentityForNetworkEndpointAsync<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointhost: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrimaryManagedIdentityForNetworkEndpointAsync(&*(&endpointhost as *const <super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeContent<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeContent(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetForCurrentView<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectedAccessSuspending<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectedAccessSuspending(&*(&handler as *const <super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProtectedAccessSuspending<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProtectedAccessSuspending(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProtectedAccessResumed<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectedAccessResumed(&*(&handler as *const <super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProtectedAccessResumed<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProtectedAccessResumed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProtectedContentRevoked<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectedContentRevoked(&*(&handler as *const <super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProtectedContentRevoked<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProtectedContentRevoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckAccess<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckAccess(&*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&targetidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IProtectionPolicyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync(&*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&targetidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IProtectionPolicyManagerStatics>,
            ::windows::core::GetTrustLevel,
            IsIdentityManaged::<Impl, OFFSET>,
            TryApplyProcessUIPolicy::<Impl, OFFSET>,
            ClearProcessUIPolicy::<Impl, OFFSET>,
            CreateCurrentThreadNetworkContext::<Impl, OFFSET>,
            GetPrimaryManagedIdentityForNetworkEndpointAsync::<Impl, OFFSET>,
            RevokeContent::<Impl, OFFSET>,
            GetForCurrentView::<Impl, OFFSET>,
            ProtectedAccessSuspending::<Impl, OFFSET>,
            RemoveProtectedAccessSuspending::<Impl, OFFSET>,
            ProtectedAccessResumed::<Impl, OFFSET>,
            RemoveProtectedAccessResumed::<Impl, OFFSET>,
            ProtectedContentRevoked::<Impl, OFFSET>,
            RemoveProtectedContentRevoked::<Impl, OFFSET>,
            CheckAccess::<Impl, OFFSET>,
            RequestAccessAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStatics2Impl: Sized {
    fn HasContentBeenRevokedSince(&self, identity: &::windows::core::HSTRING, since: &super::super::Foundation::DateTime) -> ::windows::core::Result<bool>;
    fn CheckAccessForApp(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionPolicyEvaluationResult>;
    fn RequestAccessForAppAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn GetEnforcementLevel(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<EnforcementLevel>;
    fn IsUserDecryptionAllowed(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsProtectionUnderLockRequired(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn PolicyChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePolicyChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsProtectionEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerStatics2 {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>() -> IProtectionPolicyManagerStatics2Vtbl {
        unsafe extern "system" fn HasContentBeenRevokedSince<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, since: super::super::Foundation::DateTime, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasContentBeenRevokedSince(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&since as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckAccessForApp<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckAccessForApp(&*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessForAppAsync<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessForAppAsync(&*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnforcementLevel<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut EnforcementLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnforcementLevel(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserDecryptionAllowed<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserDecryptionAllowed(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsProtectionUnderLockRequired<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProtectionUnderLockRequired(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyChanged<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePolicyChanged<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePolicyChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsProtectionEnabled<Impl: IProtectionPolicyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProtectionEnabled() {
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
            ::windows::core::GetRuntimeClassName::<IProtectionPolicyManagerStatics2>,
            ::windows::core::GetTrustLevel,
            HasContentBeenRevokedSince::<Impl, OFFSET>,
            CheckAccessForApp::<Impl, OFFSET>,
            RequestAccessForAppAsync::<Impl, OFFSET>,
            GetEnforcementLevel::<Impl, OFFSET>,
            IsUserDecryptionAllowed::<Impl, OFFSET>,
            IsProtectionUnderLockRequired::<Impl, OFFSET>,
            PolicyChanged::<Impl, OFFSET>,
            RemovePolicyChanged::<Impl, OFFSET>,
            IsProtectionEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStatics3Impl: Sized {
    fn RequestAccessWithAuditingInfoAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessWithMessageAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessForAppWithAuditingInfoAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessForAppWithMessageAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn LogAuditEvent(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerStatics3 {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerStatics3Impl, const OFFSET: isize>() -> IProtectionPolicyManagerStatics3Vtbl {
        unsafe extern "system" fn RequestAccessWithAuditingInfoAsync<Impl: IProtectionPolicyManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessWithAuditingInfoAsync(
                &*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&targetidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessWithMessageAsync<Impl: IProtectionPolicyManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessWithMessageAsync(
                &*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&targetidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&messagefromapp as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessForAppWithAuditingInfoAsync<Impl: IProtectionPolicyManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessForAppWithAuditingInfoAsync(
                &*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessForAppWithMessageAsync<Impl: IProtectionPolicyManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessForAppWithMessageAsync(
                &*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&messagefromapp as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogAuditEvent<Impl: IProtectionPolicyManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .LogAuditEvent(
                    &*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&targetidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IProtectionPolicyManagerStatics3>,
            ::windows::core::GetTrustLevel,
            RequestAccessWithAuditingInfoAsync::<Impl, OFFSET>,
            RequestAccessWithMessageAsync::<Impl, OFFSET>,
            RequestAccessForAppWithAuditingInfoAsync::<Impl, OFFSET>,
            RequestAccessForAppWithMessageAsync::<Impl, OFFSET>,
            LogAuditEvent::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionPolicyManagerStatics4Impl: Sized {
    fn IsRoamableProtectionEnabled(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn RequestAccessWithBehaviorAsync(&self, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessForAppWithBehaviorAsync(&self, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForAppAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, apppackagefamilyname: &::windows::core::HSTRING, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForProcessAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, processid: u32, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync(&self, sourceitemlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, processid: u32, auditinfo: &::core::option::Option<ProtectionPolicyAuditInfo>, messagefromapp: &::windows::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>;
    fn IsFileProtectionRequiredAsync(&self, target: &::core::option::Option<super::super::Storage::IStorageItem>, identity: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn IsFileProtectionRequiredForNewFileAsync(&self, parentfolder: &::core::option::Option<super::super::Storage::IStorageFolder>, identity: &::windows::core::HSTRING, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn PrimaryManagedIdentity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetPrimaryManagedIdentityForIdentity(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerStatics4 {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IProtectionPolicyManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionPolicyManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>() -> IProtectionPolicyManagerStatics4Vtbl {
        unsafe extern "system" fn IsRoamableProtectionEnabled<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoamableProtectionEnabled(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessWithBehaviorAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessWithBehaviorAsync(
                &*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&targetidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&messagefromapp as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                behavior,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessForAppWithBehaviorAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessForAppWithBehaviorAsync(
                &*(&sourceidentity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&messagefromapp as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                behavior,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForAppAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessToFilesForAppAsync(
                &*(&sourceitemlist as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::DefaultType>::DefaultType),
                &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessToFilesForAppWithMessageAndBehaviorAsync(
                &*(&sourceitemlist as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::DefaultType>::DefaultType),
                &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&messagefromapp as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                behavior,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, processid: u32, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessToFilesForProcessAsync(&*(&sourceitemlist as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::DefaultType>::DefaultType), processid, &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, processid: u32, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessToFilesForProcessWithMessageAndBehaviorAsync(
                &*(&sourceitemlist as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::DefaultType>::DefaultType),
                processid,
                &*(&auditinfo as *const <ProtectionPolicyAuditInfo as ::windows::core::Abi>::Abi as *const <ProtectionPolicyAuditInfo as ::windows::core::DefaultType>::DefaultType),
                &*(&messagefromapp as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                behavior,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileProtectionRequiredAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFileProtectionRequiredAsync(&*(&target as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileProtectionRequiredForNewFileAsync<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentfolder: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFileProtectionRequiredForNewFileAsync(
                &*(&parentfolder as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType),
                &*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryManagedIdentity<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryManagedIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrimaryManagedIdentityForIdentity<Impl: IProtectionPolicyManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrimaryManagedIdentityForIdentity(&*(&identity as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IProtectionPolicyManagerStatics4>,
            ::windows::core::GetTrustLevel,
            IsRoamableProtectionEnabled::<Impl, OFFSET>,
            RequestAccessWithBehaviorAsync::<Impl, OFFSET>,
            RequestAccessForAppWithBehaviorAsync::<Impl, OFFSET>,
            RequestAccessToFilesForAppAsync::<Impl, OFFSET>,
            RequestAccessToFilesForAppWithMessageAndBehaviorAsync::<Impl, OFFSET>,
            RequestAccessToFilesForProcessAsync::<Impl, OFFSET>,
            RequestAccessToFilesForProcessWithMessageAndBehaviorAsync::<Impl, OFFSET>,
            IsFileProtectionRequiredAsync::<Impl, OFFSET>,
            IsFileProtectionRequiredForNewFileAsync::<Impl, OFFSET>,
            PrimaryManagedIdentity::<Impl, OFFSET>,
            GetPrimaryManagedIdentityForIdentity::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThreadNetworkContextImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThreadNetworkContext {
    const NAME: &'static str = "Windows.Security.EnterpriseData.IThreadNetworkContext";
}
#[cfg(feature = "implement_exclusive")]
impl IThreadNetworkContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThreadNetworkContextImpl, const OFFSET: isize>() -> IThreadNetworkContextVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IThreadNetworkContext>, ::windows::core::GetTrustLevel)
    }
}
