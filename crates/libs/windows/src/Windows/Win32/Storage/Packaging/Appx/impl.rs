pub trait IAppxBlockMapBlockImpl: Sized {
    fn GetHash();
    fn GetCompressedSize();
}
impl ::windows::core::RuntimeName for IAppxBlockMapBlock {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBlockMapBlock";
}
impl IAppxBlockMapBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapBlockImpl, const OFFSET: isize>() -> IAppxBlockMapBlockVtbl {
        unsafe extern "system" fn GetHash<Impl: IAppxBlockMapBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHash(::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompressedSize<Impl: IAppxBlockMapBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompressedSize(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBlockMapBlock>, ::windows::core::GetTrustLevel, GetHash::<Impl, OFFSET>, GetCompressedSize::<Impl, OFFSET>)
    }
}
pub trait IAppxBlockMapBlocksEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxBlockMapBlocksEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBlockMapBlocksEnumerator";
}
impl IAppxBlockMapBlocksEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapBlocksEnumeratorImpl, const OFFSET: isize>() -> IAppxBlockMapBlocksEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBlockMapBlocksEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBlockMapBlocksEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBlockMapBlocksEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBlockMapBlocksEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxBlockMapFileImpl: Sized {
    fn GetBlocks();
    fn GetLocalFileHeaderSize();
    fn GetName();
    fn GetUncompressedSize();
    fn ValidateFileHash();
}
impl ::windows::core::RuntimeName for IAppxBlockMapFile {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBlockMapFile";
}
impl IAppxBlockMapFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapFileImpl, const OFFSET: isize>() -> IAppxBlockMapFileVtbl {
        unsafe extern "system" fn GetBlocks<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlocks(::core::mem::transmute_copy(&blocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalFileHeaderSize<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfhsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalFileHeaderSize(::core::mem::transmute_copy(&lfhsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUncompressedSize<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUncompressedSize(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateFileHash<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filestream: ::windows::core::RawPtr, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateFileHash(&*(&filestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isvalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBlockMapFile>, ::windows::core::GetTrustLevel, GetBlocks::<Impl, OFFSET>, GetLocalFileHeaderSize::<Impl, OFFSET>, GetName::<Impl, OFFSET>, GetUncompressedSize::<Impl, OFFSET>, ValidateFileHash::<Impl, OFFSET>)
    }
}
pub trait IAppxBlockMapFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxBlockMapFilesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBlockMapFilesEnumerator";
}
impl IAppxBlockMapFilesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapFilesEnumeratorImpl, const OFFSET: isize>() -> IAppxBlockMapFilesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBlockMapFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBlockMapFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBlockMapFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBlockMapFilesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxBlockMapReaderImpl: Sized {
    fn GetFile();
    fn GetFiles();
    fn GetHashMethod();
    fn GetStream();
}
impl ::windows::core::RuntimeName for IAppxBlockMapReader {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBlockMapReader";
}
impl IAppxBlockMapReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>() -> IAppxBlockMapReaderVtbl {
        unsafe extern "system" fn GetFile<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFile(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFiles(::core::mem::transmute_copy(&enumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashMethod<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHashMethod(::core::mem::transmute_copy(&hashmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&blockmapstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBlockMapReader>, ::windows::core::GetTrustLevel, GetFile::<Impl, OFFSET>, GetFiles::<Impl, OFFSET>, GetHashMethod::<Impl, OFFSET>, GetStream::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleFactoryImpl: Sized {
    fn CreateBundleWriter();
    fn CreateBundleReader();
    fn CreateBundleManifestReader();
}
impl ::windows::core::RuntimeName for IAppxBundleFactory {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleFactory";
}
impl IAppxBundleFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleFactoryImpl, const OFFSET: isize>() -> IAppxBundleFactoryVtbl {
        unsafe extern "system" fn CreateBundleWriter<Impl: IAppxBundleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBundleWriter(&*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), bundleversion, ::core::mem::transmute_copy(&bundlewriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleReader<Impl: IAppxBundleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBundleReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bundlereader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleManifestReader<Impl: IAppxBundleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBundleManifestReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&manifestreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleFactory>, ::windows::core::GetTrustLevel, CreateBundleWriter::<Impl, OFFSET>, CreateBundleReader::<Impl, OFFSET>, CreateBundleManifestReader::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestOptionalBundleInfoImpl: Sized {
    fn GetPackageId();
    fn GetFileName();
    fn GetPackageInfoItems();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestOptionalBundleInfo {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestOptionalBundleInfo";
}
impl IAppxBundleManifestOptionalBundleInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestOptionalBundleInfoImpl, const OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfoVtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestOptionalBundleInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId(::core::mem::transmute_copy(&packageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Impl: IAppxBundleManifestOptionalBundleInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileName(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Impl: IAppxBundleManifestOptionalBundleInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageInfoItems(::core::mem::transmute_copy(&packageinfoitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestOptionalBundleInfo>, ::windows::core::GetTrustLevel, GetPackageId::<Impl, OFFSET>, GetFileName::<Impl, OFFSET>, GetPackageInfoItems::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestOptionalBundleInfoEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestOptionalBundleInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestOptionalBundleInfoEnumerator";
}
impl IAppxBundleManifestOptionalBundleInfoEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfoEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&optionalbundle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestOptionalBundleInfoEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestPackageInfoImpl: Sized {
    fn GetPackageType();
    fn GetPackageId();
    fn GetFileName();
    fn GetOffset();
    fn GetSize();
    fn GetResources();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestPackageInfo";
}
impl IAppxBundleManifestPackageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfoVtbl {
        unsafe extern "system" fn GetPackageType<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageType(::core::mem::transmute_copy(&packagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId(::core::mem::transmute_copy(&packageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileName(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOffset(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResources(::core::mem::transmute_copy(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestPackageInfo>, ::windows::core::GetTrustLevel, GetPackageType::<Impl, OFFSET>, GetPackageId::<Impl, OFFSET>, GetFileName::<Impl, OFFSET>, GetOffset::<Impl, OFFSET>, GetSize::<Impl, OFFSET>, GetResources::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestPackageInfo2Impl: Sized {
    fn GetIsPackageReference();
    fn GetIsNonQualifiedResourcePackage();
    fn GetIsDefaultApplicablePackage();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestPackageInfo2";
}
impl IAppxBundleManifestPackageInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo2Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo2Vtbl {
        unsafe extern "system" fn GetIsPackageReference<Impl: IAppxBundleManifestPackageInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsPackageReference(::core::mem::transmute_copy(&ispackagereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Impl: IAppxBundleManifestPackageInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsNonQualifiedResourcePackage(::core::mem::transmute_copy(&isnonqualifiedresourcepackage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDefaultApplicablePackage<Impl: IAppxBundleManifestPackageInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsDefaultApplicablePackage(::core::mem::transmute_copy(&isdefaultapplicablepackage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestPackageInfo2>, ::windows::core::GetTrustLevel, GetIsPackageReference::<Impl, OFFSET>, GetIsNonQualifiedResourcePackage::<Impl, OFFSET>, GetIsDefaultApplicablePackage::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestPackageInfo3Impl: Sized {
    fn GetTargetDeviceFamilies();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo3 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestPackageInfo3";
}
impl IAppxBundleManifestPackageInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo3Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo3Vtbl {
        unsafe extern "system" fn GetTargetDeviceFamilies<Impl: IAppxBundleManifestPackageInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetDeviceFamilies(::core::mem::transmute_copy(&targetdevicefamilies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestPackageInfo3>, ::windows::core::GetTrustLevel, GetTargetDeviceFamilies::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestPackageInfo4Impl: Sized {
    fn GetIsStub();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo4 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestPackageInfo4";
}
impl IAppxBundleManifestPackageInfo4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo4Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo4Vtbl {
        unsafe extern "system" fn GetIsStub<Impl: IAppxBundleManifestPackageInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsStub(::core::mem::transmute_copy(&isstub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestPackageInfo4>, ::windows::core::GetTrustLevel, GetIsStub::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestPackageInfoEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfoEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestPackageInfoEnumerator";
}
impl IAppxBundleManifestPackageInfoEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfoEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&packageinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestPackageInfoEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestReaderImpl: Sized {
    fn GetPackageId();
    fn GetPackageInfoItems();
    fn GetStream();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestReader {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestReader";
}
impl IAppxBundleManifestReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestReaderImpl, const OFFSET: isize>() -> IAppxBundleManifestReaderVtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId(::core::mem::transmute_copy(&packageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Impl: IAppxBundleManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageInfoItems(::core::mem::transmute_copy(&packageinfoitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxBundleManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&manifeststream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestReader>, ::windows::core::GetTrustLevel, GetPackageId::<Impl, OFFSET>, GetPackageInfoItems::<Impl, OFFSET>, GetStream::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleManifestReader2Impl: Sized {
    fn GetOptionalBundles();
}
impl ::windows::core::RuntimeName for IAppxBundleManifestReader2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleManifestReader2";
}
impl IAppxBundleManifestReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestReader2Impl, const OFFSET: isize>() -> IAppxBundleManifestReader2Vtbl {
        unsafe extern "system" fn GetOptionalBundles<Impl: IAppxBundleManifestReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionalBundles(::core::mem::transmute_copy(&optionalbundles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleManifestReader2>, ::windows::core::GetTrustLevel, GetOptionalBundles::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleReaderImpl: Sized {
    fn GetFootprintFile();
    fn GetBlockMap();
    fn GetManifest();
    fn GetPayloadPackages();
    fn GetPayloadPackage();
}
impl ::windows::core::RuntimeName for IAppxBundleReader {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleReader";
}
impl IAppxBundleReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleReaderImpl, const OFFSET: isize>() -> IAppxBundleReaderVtbl {
        unsafe extern "system" fn GetFootprintFile<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFootprintFile(filetype, ::core::mem::transmute_copy(&footprintfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlockMap<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlockMap(::core::mem::transmute_copy(&blockmapreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManifest(::core::mem::transmute_copy(&manifestreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackages<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payloadpackages: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadPackages(::core::mem::transmute_copy(&payloadpackages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackage<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, payloadpackage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadPackage(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&payloadpackage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleReader>, ::windows::core::GetTrustLevel, GetFootprintFile::<Impl, OFFSET>, GetBlockMap::<Impl, OFFSET>, GetManifest::<Impl, OFFSET>, GetPayloadPackages::<Impl, OFFSET>, GetPayloadPackage::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleWriterImpl: Sized {
    fn AddPayloadPackage();
    fn Close();
}
impl ::windows::core::RuntimeName for IAppxBundleWriter {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleWriter";
}
impl IAppxBundleWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriterImpl, const OFFSET: isize>() -> IAppxBundleWriterVtbl {
        unsafe extern "system" fn AddPayloadPackage<Impl: IAppxBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadPackage(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&packagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IAppxBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleWriter>, ::windows::core::GetTrustLevel, AddPayloadPackage::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleWriter2Impl: Sized {
    fn AddExternalPackageReference();
}
impl ::windows::core::RuntimeName for IAppxBundleWriter2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleWriter2";
}
impl IAppxBundleWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter2Impl, const OFFSET: isize>() -> IAppxBundleWriter2Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxBundleWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExternalPackageReference(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleWriter2>, ::windows::core::GetTrustLevel, AddExternalPackageReference::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleWriter3Impl: Sized {
    fn AddPackageReference();
    fn Close();
}
impl ::windows::core::RuntimeName for IAppxBundleWriter3 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleWriter3";
}
impl IAppxBundleWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter3Impl, const OFFSET: isize>() -> IAppxBundleWriter3Vtbl {
        unsafe extern "system" fn AddPackageReference<Impl: IAppxBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageReference(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IAppxBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethodstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close(&*(&hashmethodstring as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleWriter3>, ::windows::core::GetTrustLevel, AddPackageReference::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IAppxBundleWriter4Impl: Sized {
    fn AddPayloadPackage();
    fn AddPackageReference();
    fn AddExternalPackageReference();
}
impl ::windows::core::RuntimeName for IAppxBundleWriter4 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxBundleWriter4";
}
impl IAppxBundleWriter4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter4Impl, const OFFSET: isize>() -> IAppxBundleWriter4Vtbl {
        unsafe extern "system" fn AddPayloadPackage<Impl: IAppxBundleWriter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadPackage(
                &*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&packagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&isdefaultapplicablepackage as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPackageReference<Impl: IAppxBundleWriter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageReference(
                &*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&isdefaultapplicablepackage as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxBundleWriter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExternalPackageReference(
                &*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&isdefaultapplicablepackage as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxBundleWriter4>, ::windows::core::GetTrustLevel, AddPayloadPackage::<Impl, OFFSET>, AddPackageReference::<Impl, OFFSET>, AddExternalPackageReference::<Impl, OFFSET>)
    }
}
pub trait IAppxContentGroupImpl: Sized {
    fn GetName();
    fn GetFiles();
}
impl ::windows::core::RuntimeName for IAppxContentGroup {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxContentGroup";
}
impl IAppxContentGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupImpl, const OFFSET: isize>() -> IAppxContentGroupVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&groupname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Impl: IAppxContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFiles(::core::mem::transmute_copy(&enumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxContentGroup>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetFiles::<Impl, OFFSET>)
    }
}
pub trait IAppxContentGroupFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxContentGroupFilesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxContentGroupFilesEnumerator";
}
impl IAppxContentGroupFilesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupFilesEnumeratorImpl, const OFFSET: isize>() -> IAppxContentGroupFilesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxContentGroupFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxContentGroupFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxContentGroupFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxContentGroupFilesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxContentGroupMapReaderImpl: Sized {
    fn GetRequiredGroup();
    fn GetAutomaticGroups();
}
impl ::windows::core::RuntimeName for IAppxContentGroupMapReader {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxContentGroupMapReader";
}
impl IAppxContentGroupMapReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupMapReaderImpl, const OFFSET: isize>() -> IAppxContentGroupMapReaderVtbl {
        unsafe extern "system" fn GetRequiredGroup<Impl: IAppxContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiredGroup(::core::mem::transmute_copy(&requiredgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Impl: IAppxContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomaticGroups(::core::mem::transmute_copy(&automaticgroupsenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxContentGroupMapReader>, ::windows::core::GetTrustLevel, GetRequiredGroup::<Impl, OFFSET>, GetAutomaticGroups::<Impl, OFFSET>)
    }
}
pub trait IAppxContentGroupMapWriterImpl: Sized {
    fn AddAutomaticGroup();
    fn AddAutomaticFile();
    fn Close();
}
impl ::windows::core::RuntimeName for IAppxContentGroupMapWriter {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxContentGroupMapWriter";
}
impl IAppxContentGroupMapWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupMapWriterImpl, const OFFSET: isize>() -> IAppxContentGroupMapWriterVtbl {
        unsafe extern "system" fn AddAutomaticGroup<Impl: IAppxContentGroupMapWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAutomaticGroup(&*(&groupname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAutomaticFile<Impl: IAppxContentGroupMapWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAutomaticFile(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IAppxContentGroupMapWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxContentGroupMapWriter>, ::windows::core::GetTrustLevel, AddAutomaticGroup::<Impl, OFFSET>, AddAutomaticFile::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IAppxContentGroupsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxContentGroupsEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxContentGroupsEnumerator";
}
impl IAppxContentGroupsEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupsEnumeratorImpl, const OFFSET: isize>() -> IAppxContentGroupsEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxContentGroupsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxContentGroupsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxContentGroupsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxContentGroupsEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptedBundleWriterImpl: Sized {
    fn AddPayloadPackageEncrypted();
    fn Close();
}
impl ::windows::core::RuntimeName for IAppxEncryptedBundleWriter {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptedBundleWriter";
}
impl IAppxEncryptedBundleWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriterImpl, const OFFSET: isize>() -> IAppxEncryptedBundleWriterVtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Impl: IAppxEncryptedBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadPackageEncrypted(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&packagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IAppxEncryptedBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptedBundleWriter>, ::windows::core::GetTrustLevel, AddPayloadPackageEncrypted::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptedBundleWriter2Impl: Sized {
    fn AddExternalPackageReference();
}
impl ::windows::core::RuntimeName for IAppxEncryptedBundleWriter2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptedBundleWriter2";
}
impl IAppxEncryptedBundleWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriter2Impl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter2Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxEncryptedBundleWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExternalPackageReference(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptedBundleWriter2>, ::windows::core::GetTrustLevel, AddExternalPackageReference::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptedBundleWriter3Impl: Sized {
    fn AddPayloadPackageEncrypted();
    fn AddExternalPackageReference();
}
impl ::windows::core::RuntimeName for IAppxEncryptedBundleWriter3 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptedBundleWriter3";
}
impl IAppxEncryptedBundleWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriter3Impl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter3Vtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Impl: IAppxEncryptedBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadPackageEncrypted(
                &*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&packagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&isdefaultapplicablepackage as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxEncryptedBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddExternalPackageReference(
                &*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&isdefaultapplicablepackage as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptedBundleWriter3>, ::windows::core::GetTrustLevel, AddPayloadPackageEncrypted::<Impl, OFFSET>, AddExternalPackageReference::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptedPackageWriterImpl: Sized {
    fn AddPayloadFileEncrypted();
    fn Close();
}
impl ::windows::core::RuntimeName for IAppxEncryptedPackageWriter {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptedPackageWriter";
}
impl IAppxEncryptedPackageWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedPackageWriterImpl, const OFFSET: isize>() -> IAppxEncryptedPackageWriterVtbl {
        unsafe extern "system" fn AddPayloadFileEncrypted<Impl: IAppxEncryptedPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadFileEncrypted(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), compressionoption, &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IAppxEncryptedPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptedPackageWriter>, ::windows::core::GetTrustLevel, AddPayloadFileEncrypted::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptedPackageWriter2Impl: Sized {
    fn AddPayloadFilesEncrypted();
}
impl ::windows::core::RuntimeName for IAppxEncryptedPackageWriter2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptedPackageWriter2";
}
impl IAppxEncryptedPackageWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedPackageWriter2Impl, const OFFSET: isize>() -> IAppxEncryptedPackageWriter2Vtbl {
        unsafe extern "system" fn AddPayloadFilesEncrypted<Impl: IAppxEncryptedPackageWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadFilesEncrypted(filecount, &*(&payloadfiles as *const <APPX_PACKAGE_WRITER_PAYLOAD_STREAM as ::windows::core::Abi>::Abi as *const <APPX_PACKAGE_WRITER_PAYLOAD_STREAM as ::windows::core::DefaultType>::DefaultType), memorylimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptedPackageWriter2>, ::windows::core::GetTrustLevel, AddPayloadFilesEncrypted::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptionFactoryImpl: Sized {
    fn EncryptPackage();
    fn DecryptPackage();
    fn CreateEncryptedPackageWriter();
    fn CreateEncryptedPackageReader();
    fn EncryptBundle();
    fn DecryptBundle();
    fn CreateEncryptedBundleWriter();
    fn CreateEncryptedBundleReader();
}
impl ::windows::core::RuntimeName for IAppxEncryptionFactory {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptionFactory";
}
impl IAppxEncryptionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>() -> IAppxEncryptionFactoryVtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptPackage(
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecryptPackage<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptPackage(
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageWriter(
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&manifeststream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageReader<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&packagereader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptBundle(
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecryptBundle<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptBundle(
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedBundleWriter(
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                bundleversion,
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&bundlewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleReader<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedBundleReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bundlereader)) {
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
            ::windows::core::GetRuntimeClassName::<IAppxEncryptionFactory>,
            ::windows::core::GetTrustLevel,
            EncryptPackage::<Impl, OFFSET>,
            DecryptPackage::<Impl, OFFSET>,
            CreateEncryptedPackageWriter::<Impl, OFFSET>,
            CreateEncryptedPackageReader::<Impl, OFFSET>,
            EncryptBundle::<Impl, OFFSET>,
            DecryptBundle::<Impl, OFFSET>,
            CreateEncryptedBundleWriter::<Impl, OFFSET>,
            CreateEncryptedBundleReader::<Impl, OFFSET>,
        )
    }
}
pub trait IAppxEncryptionFactory2Impl: Sized {
    fn CreateEncryptedPackageWriter();
}
impl ::windows::core::RuntimeName for IAppxEncryptionFactory2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptionFactory2";
}
impl IAppxEncryptionFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory2Impl, const OFFSET: isize>() -> IAppxEncryptionFactory2Vtbl {
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageWriter(
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&manifeststream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&contentgroupmapstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptionFactory2>, ::windows::core::GetTrustLevel, CreateEncryptedPackageWriter::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptionFactory3Impl: Sized {
    fn EncryptPackage();
    fn CreateEncryptedPackageWriter();
    fn EncryptBundle();
    fn CreateEncryptedBundleWriter();
}
impl ::windows::core::RuntimeName for IAppxEncryptionFactory3 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptionFactory3";
}
impl IAppxEncryptionFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>() -> IAppxEncryptionFactory3Vtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptPackage(
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageWriter(
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&manifeststream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&contentgroupmapstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptBundle(
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedBundleWriter(
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                bundleversion,
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&bundlewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptionFactory3>, ::windows::core::GetTrustLevel, EncryptPackage::<Impl, OFFSET>, CreateEncryptedPackageWriter::<Impl, OFFSET>, EncryptBundle::<Impl, OFFSET>, CreateEncryptedBundleWriter::<Impl, OFFSET>)
    }
}
pub trait IAppxEncryptionFactory4Impl: Sized {
    fn EncryptPackage();
}
impl ::windows::core::RuntimeName for IAppxEncryptionFactory4 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxEncryptionFactory4";
}
impl IAppxEncryptionFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory4Impl, const OFFSET: isize>() -> IAppxEncryptionFactory4Vtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptPackage(
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&exemptedfiles as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_EXEMPTIONS as ::windows::core::DefaultType>::DefaultType),
                memorylimit,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxEncryptionFactory4>, ::windows::core::GetTrustLevel, EncryptPackage::<Impl, OFFSET>)
    }
}
pub trait IAppxFactoryImpl: Sized {
    fn CreatePackageWriter();
    fn CreatePackageReader();
    fn CreateManifestReader();
    fn CreateBlockMapReader();
    fn CreateValidatedBlockMapReader();
}
impl ::windows::core::RuntimeName for IAppxFactory {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxFactory";
}
impl IAppxFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFactoryImpl, const OFFSET: isize>() -> IAppxFactoryVtbl {
        unsafe extern "system" fn CreatePackageWriter<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriter(&*(&outputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&settings as *const <APPX_PACKAGE_SETTINGS as ::windows::core::Abi>::Abi as *const <APPX_PACKAGE_SETTINGS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&packagewriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&packagereader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManifestReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateManifestReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&manifestreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlockMapReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlockMapReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&blockmapreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateValidatedBlockMapReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: ::windows::core::RawPtr, signaturefilename: super::super::super::Foundation::PWSTR, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateValidatedBlockMapReader(&*(&blockmapstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&signaturefilename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&blockmapreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxFactory>, ::windows::core::GetTrustLevel, CreatePackageWriter::<Impl, OFFSET>, CreatePackageReader::<Impl, OFFSET>, CreateManifestReader::<Impl, OFFSET>, CreateBlockMapReader::<Impl, OFFSET>, CreateValidatedBlockMapReader::<Impl, OFFSET>)
    }
}
pub trait IAppxFactory2Impl: Sized {
    fn CreateContentGroupMapReader();
    fn CreateSourceContentGroupMapReader();
    fn CreateContentGroupMapWriter();
}
impl ::windows::core::RuntimeName for IAppxFactory2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxFactory2";
}
impl IAppxFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFactory2Impl, const OFFSET: isize>() -> IAppxFactory2Vtbl {
        unsafe extern "system" fn CreateContentGroupMapReader<Impl: IAppxFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, contentgroupmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContentGroupMapReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&contentgroupmapreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSourceContentGroupMapReader<Impl: IAppxFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, reader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSourceContentGroupMapReader(&*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&reader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContentGroupMapWriter<Impl: IAppxFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contentgroupmapwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContentGroupMapWriter(&*(&stream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&contentgroupmapwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxFactory2>, ::windows::core::GetTrustLevel, CreateContentGroupMapReader::<Impl, OFFSET>, CreateSourceContentGroupMapReader::<Impl, OFFSET>, CreateContentGroupMapWriter::<Impl, OFFSET>)
    }
}
pub trait IAppxFileImpl: Sized {
    fn GetCompressionOption();
    fn GetContentType();
    fn GetName();
    fn GetSize();
    fn GetStream();
}
impl ::windows::core::RuntimeName for IAppxFile {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxFile";
}
impl IAppxFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFileImpl, const OFFSET: isize>() -> IAppxFileVtbl {
        unsafe extern "system" fn GetCompressionOption<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompressionOption(::core::mem::transmute_copy(&compressionoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentType(::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxFile>, ::windows::core::GetTrustLevel, GetCompressionOption::<Impl, OFFSET>, GetContentType::<Impl, OFFSET>, GetName::<Impl, OFFSET>, GetSize::<Impl, OFFSET>, GetStream::<Impl, OFFSET>)
    }
}
pub trait IAppxFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxFilesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxFilesEnumerator";
}
impl IAppxFilesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFilesEnumeratorImpl, const OFFSET: isize>() -> IAppxFilesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxFilesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestApplicationImpl: Sized {
    fn GetStringValue();
    fn GetAppUserModelId();
}
impl ::windows::core::RuntimeName for IAppxManifestApplication {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestApplication";
}
impl IAppxManifestApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestApplicationImpl, const OFFSET: isize>() -> IAppxManifestApplicationVtbl {
        unsafe extern "system" fn GetStringValue<Impl: IAppxManifestApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(&*(&name as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppUserModelId<Impl: IAppxManifestApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppUserModelId(::core::mem::transmute_copy(&appusermodelid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestApplication>, ::windows::core::GetTrustLevel, GetStringValue::<Impl, OFFSET>, GetAppUserModelId::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestApplicationsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestApplicationsEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestApplicationsEnumerator";
}
impl IAppxManifestApplicationsEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestApplicationsEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestApplicationsEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestApplicationsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&application)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestApplicationsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestApplicationsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestApplicationsEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestCapabilitiesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestCapabilitiesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestCapabilitiesEnumerator";
}
impl IAppxManifestCapabilitiesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestCapabilitiesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestCapabilitiesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&capability)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestCapabilitiesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestDeviceCapabilitiesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestDeviceCapabilitiesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestDeviceCapabilitiesEnumerator";
}
impl IAppxManifestDeviceCapabilitiesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestDeviceCapabilitiesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&devicecapability)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestDeviceCapabilitiesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestDriverConstraintImpl: Sized {
    fn GetName();
    fn GetMinVersion();
    fn GetMinDate();
}
impl ::windows::core::RuntimeName for IAppxManifestDriverConstraint {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestDriverConstraint";
}
impl IAppxManifestDriverConstraintVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverConstraintImpl, const OFFSET: isize>() -> IAppxManifestDriverConstraintVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestDriverConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestDriverConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion(::core::mem::transmute_copy(&minversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDate<Impl: IAppxManifestDriverConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mindate: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinDate(::core::mem::transmute_copy(&mindate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestDriverConstraint>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetMinVersion::<Impl, OFFSET>, GetMinDate::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestDriverConstraintsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestDriverConstraintsEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestDriverConstraintsEnumerator";
}
impl IAppxManifestDriverConstraintsEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestDriverConstraintsEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&driverconstraint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestDriverConstraintsEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestDriverDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestDriverDependenciesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestDriverDependenciesEnumerator";
}
impl IAppxManifestDriverDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestDriverDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&driverdependency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestDriverDependenciesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestDriverDependencyImpl: Sized {
    fn GetDriverConstraints();
}
impl ::windows::core::RuntimeName for IAppxManifestDriverDependency {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestDriverDependency";
}
impl IAppxManifestDriverDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverDependencyImpl, const OFFSET: isize>() -> IAppxManifestDriverDependencyVtbl {
        unsafe extern "system" fn GetDriverConstraints<Impl: IAppxManifestDriverDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDriverConstraints(::core::mem::transmute_copy(&driverconstraints)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestDriverDependency>, ::windows::core::GetTrustLevel, GetDriverConstraints::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestHostRuntimeDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestHostRuntimeDependenciesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestHostRuntimeDependenciesEnumerator";
}
impl IAppxManifestHostRuntimeDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&hostruntimedependency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestHostRuntimeDependenciesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestHostRuntimeDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetMinVersion();
}
impl ::windows::core::RuntimeName for IAppxManifestHostRuntimeDependency {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestHostRuntimeDependency";
}
impl IAppxManifestHostRuntimeDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependencyImpl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestHostRuntimeDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestHostRuntimeDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher(::core::mem::transmute_copy(&publisher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestHostRuntimeDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion(::core::mem::transmute_copy(&minversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestHostRuntimeDependency>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetPublisher::<Impl, OFFSET>, GetMinVersion::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestHostRuntimeDependency2Impl: Sized {
    fn GetPackageFamilyName();
}
impl ::windows::core::RuntimeName for IAppxManifestHostRuntimeDependency2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestHostRuntimeDependency2";
}
impl IAppxManifestHostRuntimeDependency2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependency2Impl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependency2Vtbl {
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestHostRuntimeDependency2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFamilyName(::core::mem::transmute_copy(&packagefamilyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestHostRuntimeDependency2>, ::windows::core::GetTrustLevel, GetPackageFamilyName::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestMainPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestMainPackageDependenciesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestMainPackageDependenciesEnumerator";
}
impl IAppxManifestMainPackageDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestMainPackageDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&mainpackagedependency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestMainPackageDependenciesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestMainPackageDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetPackageFamilyName();
}
impl ::windows::core::RuntimeName for IAppxManifestMainPackageDependency {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestMainPackageDependency";
}
impl IAppxManifestMainPackageDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestMainPackageDependencyImpl, const OFFSET: isize>() -> IAppxManifestMainPackageDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestMainPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestMainPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher(::core::mem::transmute_copy(&publisher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestMainPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFamilyName(::core::mem::transmute_copy(&packagefamilyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestMainPackageDependency>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetPublisher::<Impl, OFFSET>, GetPackageFamilyName::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestOSPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestOSPackageDependenciesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestOSPackageDependenciesEnumerator";
}
impl IAppxManifestOSPackageDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestOSPackageDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&ospackagedependency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestOSPackageDependenciesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestOSPackageDependencyImpl: Sized {
    fn GetName();
    fn GetVersion();
}
impl ::windows::core::RuntimeName for IAppxManifestOSPackageDependency {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestOSPackageDependency";
}
impl IAppxManifestOSPackageDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOSPackageDependencyImpl, const OFFSET: isize>() -> IAppxManifestOSPackageDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestOSPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IAppxManifestOSPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&version)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestOSPackageDependency>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetVersion::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestOptionalPackageInfoImpl: Sized {
    fn GetIsOptionalPackage();
    fn GetMainPackageName();
}
impl ::windows::core::RuntimeName for IAppxManifestOptionalPackageInfo {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestOptionalPackageInfo";
}
impl IAppxManifestOptionalPackageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOptionalPackageInfoImpl, const OFFSET: isize>() -> IAppxManifestOptionalPackageInfoVtbl {
        unsafe extern "system" fn GetIsOptionalPackage<Impl: IAppxManifestOptionalPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsOptionalPackage(::core::mem::transmute_copy(&isoptionalpackage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainPackageName<Impl: IAppxManifestOptionalPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainPackageName(::core::mem::transmute_copy(&mainpackagename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestOptionalPackageInfo>, ::windows::core::GetTrustLevel, GetIsOptionalPackage::<Impl, OFFSET>, GetMainPackageName::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestPackageDependenciesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestPackageDependenciesEnumerator";
}
impl IAppxManifestPackageDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestPackageDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&dependency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestPackageDependenciesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestPackageDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetMinVersion();
}
impl ::windows::core::RuntimeName for IAppxManifestPackageDependency {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestPackageDependency";
}
impl IAppxManifestPackageDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependencyImpl, const OFFSET: isize>() -> IAppxManifestPackageDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher(::core::mem::transmute_copy(&publisher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion(::core::mem::transmute_copy(&minversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestPackageDependency>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetPublisher::<Impl, OFFSET>, GetMinVersion::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestPackageDependency2Impl: Sized + IAppxManifestPackageDependencyImpl {
    fn GetMaxMajorVersionTested();
}
impl ::windows::core::RuntimeName for IAppxManifestPackageDependency2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestPackageDependency2";
}
impl IAppxManifestPackageDependency2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependency2Impl, const OFFSET: isize>() -> IAppxManifestPackageDependency2Vtbl {
        unsafe extern "system" fn GetMaxMajorVersionTested<Impl: IAppxManifestPackageDependency2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxmajorversiontested: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxMajorVersionTested(::core::mem::transmute_copy(&maxmajorversiontested)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestPackageDependency2>, ::windows::core::GetTrustLevel, GetMaxMajorVersionTested::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestPackageDependency3Impl: Sized {
    fn GetIsOptional();
}
impl ::windows::core::RuntimeName for IAppxManifestPackageDependency3 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestPackageDependency3";
}
impl IAppxManifestPackageDependency3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependency3Impl, const OFFSET: isize>() -> IAppxManifestPackageDependency3Vtbl {
        unsafe extern "system" fn GetIsOptional<Impl: IAppxManifestPackageDependency3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsOptional(::core::mem::transmute_copy(&isoptional)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestPackageDependency3>, ::windows::core::GetTrustLevel, GetIsOptional::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestPackageIdImpl: Sized {
    fn GetName();
    fn GetArchitecture();
    fn GetPublisher();
    fn GetVersion();
    fn GetResourceId();
    fn ComparePublisher();
    fn GetPackageFullName();
    fn GetPackageFamilyName();
}
impl ::windows::core::RuntimeName for IAppxManifestPackageId {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestPackageId";
}
impl IAppxManifestPackageIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>() -> IAppxManifestPackageIdVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArchitecture<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArchitecture(::core::mem::transmute_copy(&architecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher(::core::mem::transmute_copy(&publisher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&packageversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceId<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceId(::core::mem::transmute_copy(&resourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComparePublisher<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, other: super::super::super::Foundation::PWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComparePublisher(&*(&other as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&issame)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFullName<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFullName(::core::mem::transmute_copy(&packagefullname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFamilyName(::core::mem::transmute_copy(&packagefamilyname)) {
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
            ::windows::core::GetRuntimeClassName::<IAppxManifestPackageId>,
            ::windows::core::GetTrustLevel,
            GetName::<Impl, OFFSET>,
            GetArchitecture::<Impl, OFFSET>,
            GetPublisher::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
            GetResourceId::<Impl, OFFSET>,
            ComparePublisher::<Impl, OFFSET>,
            GetPackageFullName::<Impl, OFFSET>,
            GetPackageFamilyName::<Impl, OFFSET>,
        )
    }
}
pub trait IAppxManifestPackageId2Impl: Sized + IAppxManifestPackageIdImpl {
    fn GetArchitecture2();
}
impl ::windows::core::RuntimeName for IAppxManifestPackageId2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestPackageId2";
}
impl IAppxManifestPackageId2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageId2Impl, const OFFSET: isize>() -> IAppxManifestPackageId2Vtbl {
        unsafe extern "system" fn GetArchitecture2<Impl: IAppxManifestPackageId2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArchitecture2(::core::mem::transmute_copy(&architecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestPackageId2>, ::windows::core::GetTrustLevel, GetArchitecture2::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestPropertiesImpl: Sized {
    fn GetBoolValue();
    fn GetStringValue();
}
impl ::windows::core::RuntimeName for IAppxManifestProperties {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestProperties";
}
impl IAppxManifestPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPropertiesImpl, const OFFSET: isize>() -> IAppxManifestPropertiesVtbl {
        unsafe extern "system" fn GetBoolValue<Impl: IAppxManifestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolValue(&*(&name as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: IAppxManifestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(&*(&name as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestProperties>, ::windows::core::GetTrustLevel, GetBoolValue::<Impl, OFFSET>, GetStringValue::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestQualifiedResourceImpl: Sized {
    fn GetLanguage();
    fn GetScale();
    fn GetDXFeatureLevel();
}
impl ::windows::core::RuntimeName for IAppxManifestQualifiedResource {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestQualifiedResource";
}
impl IAppxManifestQualifiedResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestQualifiedResourceImpl, const OFFSET: isize>() -> IAppxManifestQualifiedResourceVtbl {
        unsafe extern "system" fn GetLanguage<Impl: IAppxManifestQualifiedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage(::core::mem::transmute_copy(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScale<Impl: IAppxManifestQualifiedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scale: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScale(::core::mem::transmute_copy(&scale)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDXFeatureLevel<Impl: IAppxManifestQualifiedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDXFeatureLevel(::core::mem::transmute_copy(&dxfeaturelevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestQualifiedResource>, ::windows::core::GetTrustLevel, GetLanguage::<Impl, OFFSET>, GetScale::<Impl, OFFSET>, GetDXFeatureLevel::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestQualifiedResourcesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestQualifiedResourcesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestQualifiedResourcesEnumerator";
}
impl IAppxManifestQualifiedResourcesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestQualifiedResourcesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestQualifiedResourcesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestReaderImpl: Sized {
    fn GetPackageId();
    fn GetProperties();
    fn GetPackageDependencies();
    fn GetCapabilities();
    fn GetResources();
    fn GetDeviceCapabilities();
    fn GetPrerequisite();
    fn GetApplications();
    fn GetStream();
}
impl ::windows::core::RuntimeName for IAppxManifestReader {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestReader";
}
impl IAppxManifestReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReaderImpl, const OFFSET: isize>() -> IAppxManifestReaderVtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId(::core::mem::transmute_copy(&packageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&packageproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageDependencies<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageDependencies(::core::mem::transmute_copy(&dependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(::core::mem::transmute_copy(&capabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResources(::core::mem::transmute_copy(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCapabilities<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCapabilities(::core::mem::transmute_copy(&devicecapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisite<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrerequisite(&*(&name as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplications<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplications(::core::mem::transmute_copy(&applications)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&manifeststream)) {
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
            ::windows::core::GetRuntimeClassName::<IAppxManifestReader>,
            ::windows::core::GetTrustLevel,
            GetPackageId::<Impl, OFFSET>,
            GetProperties::<Impl, OFFSET>,
            GetPackageDependencies::<Impl, OFFSET>,
            GetCapabilities::<Impl, OFFSET>,
            GetResources::<Impl, OFFSET>,
            GetDeviceCapabilities::<Impl, OFFSET>,
            GetPrerequisite::<Impl, OFFSET>,
            GetApplications::<Impl, OFFSET>,
            GetStream::<Impl, OFFSET>,
        )
    }
}
pub trait IAppxManifestReader2Impl: Sized + IAppxManifestReaderImpl {
    fn GetQualifiedResources();
}
impl ::windows::core::RuntimeName for IAppxManifestReader2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestReader2";
}
impl IAppxManifestReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader2Impl, const OFFSET: isize>() -> IAppxManifestReader2Vtbl {
        unsafe extern "system" fn GetQualifiedResources<Impl: IAppxManifestReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQualifiedResources(::core::mem::transmute_copy(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestReader2>, ::windows::core::GetTrustLevel, GetQualifiedResources::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestReader3Impl: Sized + IAppxManifestReader2Impl + IAppxManifestReaderImpl {
    fn GetCapabilitiesByCapabilityClass();
    fn GetTargetDeviceFamilies();
}
impl ::windows::core::RuntimeName for IAppxManifestReader3 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestReader3";
}
impl IAppxManifestReader3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader3Impl, const OFFSET: isize>() -> IAppxManifestReader3Vtbl {
        unsafe extern "system" fn GetCapabilitiesByCapabilityClass<Impl: IAppxManifestReader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilitiesByCapabilityClass(capabilityclass, ::core::mem::transmute_copy(&capabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetDeviceFamilies<Impl: IAppxManifestReader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetDeviceFamilies(::core::mem::transmute_copy(&targetdevicefamilies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestReader3>, ::windows::core::GetTrustLevel, GetCapabilitiesByCapabilityClass::<Impl, OFFSET>, GetTargetDeviceFamilies::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestReader4Impl: Sized + IAppxManifestReader3Impl + IAppxManifestReader2Impl + IAppxManifestReaderImpl {
    fn GetOptionalPackageInfo();
}
impl ::windows::core::RuntimeName for IAppxManifestReader4 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestReader4";
}
impl IAppxManifestReader4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader4Impl, const OFFSET: isize>() -> IAppxManifestReader4Vtbl {
        unsafe extern "system" fn GetOptionalPackageInfo<Impl: IAppxManifestReader4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionalPackageInfo(::core::mem::transmute_copy(&optionalpackageinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestReader4>, ::windows::core::GetTrustLevel, GetOptionalPackageInfo::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestReader5Impl: Sized {
    fn GetMainPackageDependencies();
}
impl ::windows::core::RuntimeName for IAppxManifestReader5 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestReader5";
}
impl IAppxManifestReader5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader5Impl, const OFFSET: isize>() -> IAppxManifestReader5Vtbl {
        unsafe extern "system" fn GetMainPackageDependencies<Impl: IAppxManifestReader5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainPackageDependencies(::core::mem::transmute_copy(&mainpackagedependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestReader5>, ::windows::core::GetTrustLevel, GetMainPackageDependencies::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestReader6Impl: Sized {
    fn GetIsNonQualifiedResourcePackage();
}
impl ::windows::core::RuntimeName for IAppxManifestReader6 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestReader6";
}
impl IAppxManifestReader6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader6Impl, const OFFSET: isize>() -> IAppxManifestReader6Vtbl {
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Impl: IAppxManifestReader6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsNonQualifiedResourcePackage(::core::mem::transmute_copy(&isnonqualifiedresourcepackage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestReader6>, ::windows::core::GetTrustLevel, GetIsNonQualifiedResourcePackage::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestReader7Impl: Sized {
    fn GetDriverDependencies();
    fn GetOSPackageDependencies();
    fn GetHostRuntimeDependencies();
}
impl ::windows::core::RuntimeName for IAppxManifestReader7 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestReader7";
}
impl IAppxManifestReader7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader7Impl, const OFFSET: isize>() -> IAppxManifestReader7Vtbl {
        unsafe extern "system" fn GetDriverDependencies<Impl: IAppxManifestReader7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDriverDependencies(::core::mem::transmute_copy(&driverdependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOSPackageDependencies<Impl: IAppxManifestReader7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOSPackageDependencies(::core::mem::transmute_copy(&ospackagedependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostRuntimeDependencies<Impl: IAppxManifestReader7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostRuntimeDependencies(::core::mem::transmute_copy(&hostruntimedependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestReader7>, ::windows::core::GetTrustLevel, GetDriverDependencies::<Impl, OFFSET>, GetOSPackageDependencies::<Impl, OFFSET>, GetHostRuntimeDependencies::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestResourcesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestResourcesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestResourcesEnumerator";
}
impl IAppxManifestResourcesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestResourcesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestResourcesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestResourcesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestTargetDeviceFamiliesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
impl ::windows::core::RuntimeName for IAppxManifestTargetDeviceFamiliesEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestTargetDeviceFamiliesEnumerator";
}
impl IAppxManifestTargetDeviceFamiliesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const OFFSET: isize>() -> IAppxManifestTargetDeviceFamiliesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&targetdevicefamily)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent(::core::mem::transmute_copy(&hascurrent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestTargetDeviceFamiliesEnumerator>, ::windows::core::GetTrustLevel, GetCurrent::<Impl, OFFSET>, GetHasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IAppxManifestTargetDeviceFamilyImpl: Sized {
    fn GetName();
    fn GetMinVersion();
    fn GetMaxVersionTested();
}
impl ::windows::core::RuntimeName for IAppxManifestTargetDeviceFamily {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxManifestTargetDeviceFamily";
}
impl IAppxManifestTargetDeviceFamilyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestTargetDeviceFamilyImpl, const OFFSET: isize>() -> IAppxManifestTargetDeviceFamilyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestTargetDeviceFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestTargetDeviceFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion(::core::mem::transmute_copy(&minversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxVersionTested<Impl: IAppxManifestTargetDeviceFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxversiontested: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxVersionTested(::core::mem::transmute_copy(&maxversiontested)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxManifestTargetDeviceFamily>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetMinVersion::<Impl, OFFSET>, GetMaxVersionTested::<Impl, OFFSET>)
    }
}
pub trait IAppxPackageEditorImpl: Sized {
    fn SetWorkingDirectory();
    fn CreateDeltaPackage();
    fn CreateDeltaPackageUsingBaselineBlockMap();
    fn UpdatePackage();
    fn UpdateEncryptedPackage();
    fn UpdatePackageManifest();
}
impl ::windows::core::RuntimeName for IAppxPackageEditor {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxPackageEditor";
}
impl IAppxPackageEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageEditorImpl, const OFFSET: isize>() -> IAppxPackageEditorVtbl {
        unsafe extern "system" fn SetWorkingDirectory<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWorkingDirectory(&*(&workingdirectory as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeltaPackage<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: ::windows::core::RawPtr, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeltaPackage(
                &*(&updatedpackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&baselinepackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&deltapackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeltaPackageUsingBaselineBlockMap<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: ::windows::core::RawPtr, baselineblockmapstream: ::windows::core::RawPtr, baselinepackagefullname: super::super::super::Foundation::PWSTR, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeltaPackageUsingBaselineBlockMap(
                &*(&updatedpackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&baselineblockmapstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&baselinepackagefullname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&deltapackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdatePackage<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdatePackage(&*(&baselinepackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&deltapackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), updateoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateEncryptedPackage<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineencryptedpackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateEncryptedPackage(
                &*(&baselineencryptedpackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&deltapackagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                updateoption,
                &*(&settings as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::Abi>::Abi as *const <APPX_ENCRYPTED_PACKAGE_SETTINGS2 as ::windows::core::DefaultType>::DefaultType),
                &*(&keyinfo as *const <APPX_KEY_INFO as ::windows::core::Abi>::Abi as *const <APPX_KEY_INFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdatePackageManifest<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestream: ::windows::core::RawPtr, updatedmanifeststream: ::windows::core::RawPtr, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdatePackageManifest(
                &*(&packagestream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&updatedmanifeststream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&ispackageencrypted as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                options,
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
            ::windows::core::GetRuntimeClassName::<IAppxPackageEditor>,
            ::windows::core::GetTrustLevel,
            SetWorkingDirectory::<Impl, OFFSET>,
            CreateDeltaPackage::<Impl, OFFSET>,
            CreateDeltaPackageUsingBaselineBlockMap::<Impl, OFFSET>,
            UpdatePackage::<Impl, OFFSET>,
            UpdateEncryptedPackage::<Impl, OFFSET>,
            UpdatePackageManifest::<Impl, OFFSET>,
        )
    }
}
pub trait IAppxPackageReaderImpl: Sized {
    fn GetBlockMap();
    fn GetFootprintFile();
    fn GetPayloadFile();
    fn GetPayloadFiles();
    fn GetManifest();
}
impl ::windows::core::RuntimeName for IAppxPackageReader {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxPackageReader";
}
impl IAppxPackageReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageReaderImpl, const OFFSET: isize>() -> IAppxPackageReaderVtbl {
        unsafe extern "system" fn GetBlockMap<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlockMap(::core::mem::transmute_copy(&blockmapreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFootprintFile<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFootprintFile(r#type, ::core::mem::transmute_copy(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFile<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadFile(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFiles<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadFiles(::core::mem::transmute_copy(&filesenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManifest(::core::mem::transmute_copy(&manifestreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxPackageReader>, ::windows::core::GetTrustLevel, GetBlockMap::<Impl, OFFSET>, GetFootprintFile::<Impl, OFFSET>, GetPayloadFile::<Impl, OFFSET>, GetPayloadFiles::<Impl, OFFSET>, GetManifest::<Impl, OFFSET>)
    }
}
pub trait IAppxPackageWriterImpl: Sized {
    fn AddPayloadFile();
    fn Close();
}
impl ::windows::core::RuntimeName for IAppxPackageWriter {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxPackageWriter";
}
impl IAppxPackageWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriterImpl, const OFFSET: isize>() -> IAppxPackageWriterVtbl {
        unsafe extern "system" fn AddPayloadFile<Impl: IAppxPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, contenttype: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadFile(
                &*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&contenttype as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                compressionoption,
                &*(&inputstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IAppxPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close(&*(&manifest as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxPackageWriter>, ::windows::core::GetTrustLevel, AddPayloadFile::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IAppxPackageWriter2Impl: Sized {
    fn Close();
}
impl ::windows::core::RuntimeName for IAppxPackageWriter2 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxPackageWriter2";
}
impl IAppxPackageWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriter2Impl, const OFFSET: isize>() -> IAppxPackageWriter2Vtbl {
        unsafe extern "system" fn Close<Impl: IAppxPackageWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: ::windows::core::RawPtr, contentgroupmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close(&*(&manifest as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&contentgroupmap as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxPackageWriter2>, ::windows::core::GetTrustLevel, Close::<Impl, OFFSET>)
    }
}
pub trait IAppxPackageWriter3Impl: Sized {
    fn AddPayloadFiles();
}
impl ::windows::core::RuntimeName for IAppxPackageWriter3 {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxPackageWriter3";
}
impl IAppxPackageWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriter3Impl, const OFFSET: isize>() -> IAppxPackageWriter3Vtbl {
        unsafe extern "system" fn AddPayloadFiles<Impl: IAppxPackageWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPayloadFiles(filecount, &*(&payloadfiles as *const <APPX_PACKAGE_WRITER_PAYLOAD_STREAM as ::windows::core::Abi>::Abi as *const <APPX_PACKAGE_WRITER_PAYLOAD_STREAM as ::windows::core::DefaultType>::DefaultType), memorylimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxPackageWriter3>, ::windows::core::GetTrustLevel, AddPayloadFiles::<Impl, OFFSET>)
    }
}
pub trait IAppxPackagingDiagnosticEventSinkImpl: Sized {
    fn ReportContextChange();
    fn ReportError();
}
impl ::windows::core::RuntimeName for IAppxPackagingDiagnosticEventSink {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxPackagingDiagnosticEventSink";
}
impl IAppxPackagingDiagnosticEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackagingDiagnosticEventSinkImpl, const OFFSET: isize>() -> IAppxPackagingDiagnosticEventSinkVtbl {
        unsafe extern "system" fn ReportContextChange<Impl: IAppxPackagingDiagnosticEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: super::super::super::Foundation::PSTR, contextmessage: super::super::super::Foundation::PWSTR, detailsmessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportContextChange(
                changetype,
                contextid,
                &*(&contextname as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&contextmessage as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&detailsmessage as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportError<Impl: IAppxPackagingDiagnosticEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errormessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportError(&*(&errormessage as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxPackagingDiagnosticEventSink>, ::windows::core::GetTrustLevel, ReportContextChange::<Impl, OFFSET>, ReportError::<Impl, OFFSET>)
    }
}
pub trait IAppxPackagingDiagnosticEventSinkManagerImpl: Sized {
    fn SetSinkForProcess();
}
impl ::windows::core::RuntimeName for IAppxPackagingDiagnosticEventSinkManager {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxPackagingDiagnosticEventSinkManager";
}
impl IAppxPackagingDiagnosticEventSinkManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackagingDiagnosticEventSinkManagerImpl, const OFFSET: isize>() -> IAppxPackagingDiagnosticEventSinkManagerVtbl {
        unsafe extern "system" fn SetSinkForProcess<Impl: IAppxPackagingDiagnosticEventSinkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSinkForProcess(&*(&sink as *const <IAppxPackagingDiagnosticEventSink as ::windows::core::Abi>::Abi as *const <IAppxPackagingDiagnosticEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxPackagingDiagnosticEventSinkManager>, ::windows::core::GetTrustLevel, SetSinkForProcess::<Impl, OFFSET>)
    }
}
pub trait IAppxSourceContentGroupMapReaderImpl: Sized {
    fn GetRequiredGroup();
    fn GetAutomaticGroups();
}
impl ::windows::core::RuntimeName for IAppxSourceContentGroupMapReader {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Appx.IAppxSourceContentGroupMapReader";
}
impl IAppxSourceContentGroupMapReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxSourceContentGroupMapReaderImpl, const OFFSET: isize>() -> IAppxSourceContentGroupMapReaderVtbl {
        unsafe extern "system" fn GetRequiredGroup<Impl: IAppxSourceContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiredGroup(::core::mem::transmute_copy(&requiredgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Impl: IAppxSourceContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomaticGroups(::core::mem::transmute_copy(&automaticgroupsenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppxSourceContentGroupMapReader>, ::windows::core::GetTrustLevel, GetRequiredGroup::<Impl, OFFSET>, GetAutomaticGroups::<Impl, OFFSET>)
    }
}
