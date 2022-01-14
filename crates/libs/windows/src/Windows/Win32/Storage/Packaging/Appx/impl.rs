pub trait IAppxBlockMapBlock_Impl: Sized {
    fn GetHash(&mut self, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetCompressedSize(&mut self) -> ::windows::core::Result<u32>;
}
impl IAppxBlockMapBlock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapBlock_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapBlock_Vtbl {
        unsafe extern "system" fn GetHash<Impl: IAppxBlockMapBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHash(::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&buffer)).into()
        }
        unsafe extern "system" fn GetCompressedSize<Impl: IAppxBlockMapBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompressedSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetHash: GetHash::<Impl, IMPL_OFFSET>,
            GetCompressedSize: GetCompressedSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapBlock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBlockMapBlocksEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxBlockMapBlock>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBlockMapBlocksEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapBlocksEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapBlocksEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *block = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapBlocksEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBlockMapFile_Impl: Sized {
    fn GetBlocks(&mut self) -> ::windows::core::Result<IAppxBlockMapBlocksEnumerator>;
    fn GetLocalFileHeaderSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetUncompressedSize(&mut self) -> ::windows::core::Result<u64>;
    fn ValidateFileHash(&mut self, filestream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBlockMapFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapFile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapFile_Vtbl {
        unsafe extern "system" fn GetBlocks<Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *blocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalFileHeaderSize<Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfhsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalFileHeaderSize() {
                ::core::result::Result::Ok(ok__) => {
                    *lfhsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUncompressedSize<Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUncompressedSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateFileHash<Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filestream: ::windows::core::RawPtr, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateFileHash(::core::mem::transmute(&filestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *isvalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBlocks: GetBlocks::<Impl, IMPL_OFFSET>,
            GetLocalFileHeaderSize: GetLocalFileHeaderSize::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetUncompressedSize: GetUncompressedSize::<Impl, IMPL_OFFSET>,
            ValidateFileHash: ValidateFileHash::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBlockMapFilesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxBlockMapFile>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBlockMapFilesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapFilesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapFilesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *file = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapFilesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBlockMapReader_Impl: Sized {
    fn GetFile(&mut self, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IAppxBlockMapFile>;
    fn GetFiles(&mut self) -> ::windows::core::Result<IAppxBlockMapFilesEnumerator>;
    fn GetHashMethod(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBlockMapReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapReader_Vtbl {
        unsafe extern "system" fn GetFile<Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFile(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *file = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *enumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashMethod<Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHashMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *hashmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *blockmapstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFile: GetFile::<Impl, IMPL_OFFSET>,
            GetFiles: GetFiles::<Impl, IMPL_OFFSET>,
            GetHashMethod: GetHashMethod::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory_Impl: Sized {
    fn CreateBundleWriter(&mut self, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, bundleversion: u64) -> ::windows::core::Result<IAppxBundleWriter>;
    fn CreateBundleReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxBundleReader>;
    fn CreateBundleManifestReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxBundleManifestReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleFactory_Vtbl {
        unsafe extern "system" fn CreateBundleWriter<Impl: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBundleWriter(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&bundleversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *bundlewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleReader<Impl: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBundleReader(::core::mem::transmute(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *bundlereader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleManifestReader<Impl: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBundleManifestReader(::core::mem::transmute(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *manifestreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateBundleWriter: CreateBundleWriter::<Impl, IMPL_OFFSET>,
            CreateBundleReader: CreateBundleReader::<Impl, IMPL_OFFSET>,
            CreateBundleManifestReader: CreateBundleManifestReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestOptionalBundleInfo_Impl: Sized {
    fn GetPackageId(&mut self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetFileName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetPackageInfoItems(&mut self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestOptionalBundleInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfo_Vtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    *packageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *filename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageInfoItems() {
                ::core::result::Result::Ok(ok__) => {
                    *packageinfoitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPackageId: GetPackageId::<Impl, IMPL_OFFSET>,
            GetFileName: GetFileName::<Impl, IMPL_OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestOptionalBundleInfoEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfo>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *optionalbundle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo_Impl: Sized {
    fn GetPackageType(&mut self) -> ::windows::core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE>;
    fn GetPackageId(&mut self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetFileName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetOffset(&mut self) -> ::windows::core::Result<u64>;
    fn GetSize(&mut self) -> ::windows::core::Result<u64>;
    fn GetResources(&mut self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfo_Vtbl {
        unsafe extern "system" fn GetPackageType<Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageType() {
                ::core::result::Result::Ok(ok__) => {
                    *packagetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    *packageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *filename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *offset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResources() {
                ::core::result::Result::Ok(ok__) => {
                    *resources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPackageType: GetPackageType::<Impl, IMPL_OFFSET>,
            GetPackageId: GetPackageId::<Impl, IMPL_OFFSET>,
            GetFileName: GetFileName::<Impl, IMPL_OFFSET>,
            GetOffset: GetOffset::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetResources: GetResources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo2_Impl: Sized {
    fn GetIsPackageReference(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsNonQualifiedResourcePackage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsDefaultApplicablePackage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfo2_Vtbl {
        unsafe extern "system" fn GetIsPackageReference<Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsPackageReference() {
                ::core::result::Result::Ok(ok__) => {
                    *ispackagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsNonQualifiedResourcePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *isnonqualifiedresourcepackage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDefaultApplicablePackage<Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsDefaultApplicablePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *isdefaultapplicablepackage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIsPackageReference: GetIsPackageReference::<Impl, IMPL_OFFSET>,
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Impl, IMPL_OFFSET>,
            GetIsDefaultApplicablePackage: GetIsDefaultApplicablePackage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait IAppxBundleManifestPackageInfo3_Impl: Sized {
    fn GetTargetDeviceFamilies(&mut self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
impl IAppxBundleManifestPackageInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfo3_Vtbl {
        unsafe extern "system" fn GetTargetDeviceFamilies<Impl: IAppxBundleManifestPackageInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetDeviceFamilies() {
                ::core::result::Result::Ok(ok__) => {
                    *targetdevicefamilies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo4_Impl: Sized {
    fn GetIsStub(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfo4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfo4_Vtbl {
        unsafe extern "system" fn GetIsStub<Impl: IAppxBundleManifestPackageInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsStub() {
                ::core::result::Result::Ok(ok__) => {
                    *isstub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIsStub: GetIsStub::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfoEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxBundleManifestPackageInfo>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfoEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *packageinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleManifestReader_Impl: Sized {
    fn GetPackageId(&mut self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetPackageInfoItems(&mut self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleManifestReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestReader_Vtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    *packageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Impl: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageInfoItems() {
                ::core::result::Result::Ok(ok__) => {
                    *packageinfoitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *manifeststream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPackageId: GetPackageId::<Impl, IMPL_OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader as ::windows::core::Interface>::IID
    }
}
pub trait IAppxBundleManifestReader2_Impl: Sized {
    fn GetOptionalBundles(&mut self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator>;
}
impl IAppxBundleManifestReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestReader2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestReader2_Vtbl {
        unsafe extern "system" fn GetOptionalBundles<Impl: IAppxBundleManifestReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionalBundles() {
                ::core::result::Result::Ok(ok__) => {
                    *optionalbundles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOptionalBundles: GetOptionalBundles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleReader_Impl: Sized {
    fn GetFootprintFile(&mut self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile>;
    fn GetBlockMap(&mut self) -> ::windows::core::Result<IAppxBlockMapReader>;
    fn GetManifest(&mut self) -> ::windows::core::Result<IAppxBundleManifestReader>;
    fn GetPayloadPackages(&mut self) -> ::windows::core::Result<IAppxFilesEnumerator>;
    fn GetPayloadPackage(&mut self, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IAppxFile>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleReader_Vtbl {
        unsafe extern "system" fn GetFootprintFile<Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFootprintFile(::core::mem::transmute_copy(&filetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *footprintfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlockMap<Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlockMap() {
                ::core::result::Result::Ok(ok__) => {
                    *blockmapreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManifest() {
                ::core::result::Result::Ok(ok__) => {
                    *manifestreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackages<Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payloadpackages: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadPackages() {
                ::core::result::Result::Ok(ok__) => {
                    *payloadpackages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackage<Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, payloadpackage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadPackage(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *payloadpackage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFootprintFile: GetFootprintFile::<Impl, IMPL_OFFSET>,
            GetBlockMap: GetBlockMap::<Impl, IMPL_OFFSET>,
            GetManifest: GetManifest::<Impl, IMPL_OFFSET>,
            GetPayloadPackages: GetPayloadPackages::<Impl, IMPL_OFFSET>,
            GetPayloadPackage: GetPayloadPackage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBundleWriter_Impl: Sized {
    fn AddPayloadPackage(&mut self, filename: super::super::super::Foundation::PWSTR, packagestream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriter_Vtbl {
        unsafe extern "system" fn AddPayloadPackage<Impl: IAppxBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadPackage(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&packagestream)).into()
        }
        unsafe extern "system" fn Close<Impl: IAppxBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBundleWriter2_Impl: Sized {
    fn AddExternalPackageReference(&mut self, filename: super::super::super::Foundation::PWSTR, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriter2_Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxBundleWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExternalPackageReference(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&inputstream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddExternalPackageReference: AddExternalPackageReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBundleWriter3_Impl: Sized {
    fn AddPackageReference(&mut self, filename: super::super::super::Foundation::PWSTR, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&mut self, hashmethodstring: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriter3_Vtbl {
        unsafe extern "system" fn AddPackageReference<Impl: IAppxBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPackageReference(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Impl: IAppxBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethodstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute_copy(&hashmethodstring)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPackageReference: AddPackageReference::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBundleWriter4_Impl: Sized {
    fn AddPayloadPackage(&mut self, filename: super::super::super::Foundation::PWSTR, packagestream: &::core::option::Option<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddPackageReference(&mut self, filename: super::super::super::Foundation::PWSTR, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddExternalPackageReference(&mut self, filename: super::super::super::Foundation::PWSTR, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriter4_Vtbl {
        unsafe extern "system" fn AddPayloadPackage<Impl: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadPackage(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&packagestream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddPackageReference<Impl: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPackageReference(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExternalPackageReference(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Impl, IMPL_OFFSET>,
            AddPackageReference: AddPackageReference::<Impl, IMPL_OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroup_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetFiles(&mut self) -> ::windows::core::Result<IAppxContentGroupFilesEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroup_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxContentGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *groupname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Impl: IAppxContentGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *enumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetFiles: GetFiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroupFilesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupFilesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupFilesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupFilesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *file = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupFilesEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IAppxContentGroupMapReader_Impl: Sized {
    fn GetRequiredGroup(&mut self) -> ::windows::core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&mut self) -> ::windows::core::Result<IAppxContentGroupsEnumerator>;
}
impl IAppxContentGroupMapReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupMapReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupMapReader_Vtbl {
        unsafe extern "system" fn GetRequiredGroup<Impl: IAppxContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiredGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *requiredgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Impl: IAppxContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomaticGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *automaticgroupsenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Impl, IMPL_OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupMapReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroupMapWriter_Impl: Sized {
    fn AddAutomaticGroup(&mut self, groupname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddAutomaticFile(&mut self, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupMapWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupMapWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupMapWriter_Vtbl {
        unsafe extern "system" fn AddAutomaticGroup<Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAutomaticGroup(::core::mem::transmute_copy(&groupname)).into()
        }
        unsafe extern "system" fn AddAutomaticFile<Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAutomaticFile(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn Close<Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAutomaticGroup: AddAutomaticGroup::<Impl, IMPL_OFFSET>,
            AddAutomaticFile: AddAutomaticFile::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupMapWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroupsEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxContentGroup>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupsEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupsEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupsEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupsEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptedBundleWriter_Impl: Sized {
    fn AddPayloadPackageEncrypted(&mut self, filename: super::super::super::Foundation::PWSTR, packagestream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedBundleWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedBundleWriter_Vtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadPackageEncrypted(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&packagestream)).into()
        }
        unsafe extern "system" fn Close<Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptedBundleWriter2_Impl: Sized {
    fn AddExternalPackageReference(&mut self, filename: super::super::super::Foundation::PWSTR, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedBundleWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedBundleWriter2_Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxEncryptedBundleWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExternalPackageReference(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&inputstream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddExternalPackageReference: AddExternalPackageReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptedBundleWriter3_Impl: Sized {
    fn AddPayloadPackageEncrypted(&mut self, filename: super::super::super::Foundation::PWSTR, packagestream: &::core::option::Option<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddExternalPackageReference(&mut self, filename: super::super::super::Foundation::PWSTR, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedBundleWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriter3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedBundleWriter3_Vtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadPackageEncrypted(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&packagestream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExternalPackageReference(::core::mem::transmute_copy(&filename), ::core::mem::transmute(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Impl, IMPL_OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptedPackageWriter_Impl: Sized {
    fn AddPayloadFileEncrypted(&mut self, filename: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedPackageWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedPackageWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedPackageWriter_Vtbl {
        unsafe extern "system" fn AddPayloadFileEncrypted<Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadFileEncrypted(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&compressionoption), ::core::mem::transmute(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPayloadFileEncrypted: AddPayloadFileEncrypted::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptedPackageWriter2_Impl: Sized {
    fn AddPayloadFilesEncrypted(&mut self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedPackageWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedPackageWriter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedPackageWriter2_Vtbl {
        unsafe extern "system" fn AddPayloadFilesEncrypted<Impl: IAppxEncryptedPackageWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadFilesEncrypted(::core::mem::transmute_copy(&filecount), ::core::mem::transmute_copy(&payloadfiles), ::core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddPayloadFilesEncrypted: AddPayloadFilesEncrypted::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory_Impl: Sized {
    fn EncryptPackage(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn DecryptPackage(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>;
    fn CreateEncryptedPackageWriter(&mut self, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, manifeststream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>;
    fn CreateEncryptedPackageReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxPackageReader>;
    fn EncryptBundle(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn DecryptBundle(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>;
    fn CreateEncryptedBundleWriter(&mut self, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter>;
    fn CreateEncryptedBundleReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxBundleReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactory_Vtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncryptPackage(::core::mem::transmute(&inputstream), ::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn DecryptPackage<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecryptPackage(::core::mem::transmute(&inputstream), ::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageWriter(::core::mem::transmute(&outputstream), ::core::mem::transmute(&manifeststream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageReader<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageReader(::core::mem::transmute(&inputstream), ::core::mem::transmute_copy(&keyinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagereader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncryptBundle(::core::mem::transmute(&inputstream), ::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn DecryptBundle<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecryptBundle(::core::mem::transmute(&inputstream), ::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedBundleWriter(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&bundleversion), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *bundlewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleReader<Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedBundleReader(::core::mem::transmute(&inputstream), ::core::mem::transmute_copy(&keyinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *bundlereader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EncryptPackage: EncryptPackage::<Impl, IMPL_OFFSET>,
            DecryptPackage: DecryptPackage::<Impl, IMPL_OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Impl, IMPL_OFFSET>,
            CreateEncryptedPackageReader: CreateEncryptedPackageReader::<Impl, IMPL_OFFSET>,
            EncryptBundle: EncryptBundle::<Impl, IMPL_OFFSET>,
            DecryptBundle: DecryptBundle::<Impl, IMPL_OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Impl, IMPL_OFFSET>,
            CreateEncryptedBundleReader: CreateEncryptedBundleReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory2_Impl: Sized {
    fn CreateEncryptedPackageWriter(&mut self, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, manifeststream: &::core::option::Option<super::super::super::System::Com::IStream>, contentgroupmapstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactory2_Vtbl {
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageWriter(::core::mem::transmute(&outputstream), ::core::mem::transmute(&manifeststream), ::core::mem::transmute(&contentgroupmapstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory3_Impl: Sized {
    fn EncryptPackage(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn CreateEncryptedPackageWriter(&mut self, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, manifeststream: &::core::option::Option<super::super::super::System::Com::IStream>, contentgroupmapstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>;
    fn EncryptBundle(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn CreateEncryptedBundleWriter(&mut self, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactory3_Vtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncryptPackage(::core::mem::transmute(&inputstream), ::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedPackageWriter(::core::mem::transmute(&outputstream), ::core::mem::transmute(&manifeststream), ::core::mem::transmute(&contentgroupmapstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncryptBundle(::core::mem::transmute(&inputstream), ::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncryptedBundleWriter(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&bundleversion), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *bundlewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EncryptPackage: EncryptPackage::<Impl, IMPL_OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Impl, IMPL_OFFSET>,
            EncryptBundle: EncryptBundle::<Impl, IMPL_OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory4_Impl: Sized {
    fn EncryptPackage(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactory4_Vtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncryptPackage(::core::mem::transmute(&inputstream), ::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles), ::core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EncryptPackage: EncryptPackage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxFactory_Impl: Sized {
    fn CreatePackageWriter(&mut self, outputstream: &::core::option::Option<super::super::super::System::Com::IStream>, settings: *const APPX_PACKAGE_SETTINGS) -> ::windows::core::Result<IAppxPackageWriter>;
    fn CreatePackageReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxPackageReader>;
    fn CreateManifestReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxManifestReader>;
    fn CreateBlockMapReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxBlockMapReader>;
    fn CreateValidatedBlockMapReader(&mut self, blockmapstream: &::core::option::Option<super::super::super::System::Com::IStream>, signaturefilename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IAppxBlockMapReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFactory_Vtbl {
        unsafe extern "system" fn CreatePackageWriter<Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriter(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageReader<Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageReader(::core::mem::transmute(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagereader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManifestReader<Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateManifestReader(::core::mem::transmute(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *manifestreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlockMapReader<Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlockMapReader(::core::mem::transmute(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *blockmapreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateValidatedBlockMapReader<Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: ::windows::core::RawPtr, signaturefilename: super::super::super::Foundation::PWSTR, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateValidatedBlockMapReader(::core::mem::transmute(&blockmapstream), ::core::mem::transmute_copy(&signaturefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *blockmapreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePackageWriter: CreatePackageWriter::<Impl, IMPL_OFFSET>,
            CreatePackageReader: CreatePackageReader::<Impl, IMPL_OFFSET>,
            CreateManifestReader: CreateManifestReader::<Impl, IMPL_OFFSET>,
            CreateBlockMapReader: CreateBlockMapReader::<Impl, IMPL_OFFSET>,
            CreateValidatedBlockMapReader: CreateValidatedBlockMapReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory2_Impl: Sized {
    fn CreateContentGroupMapReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxContentGroupMapReader>;
    fn CreateSourceContentGroupMapReader(&mut self, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxSourceContentGroupMapReader>;
    fn CreateContentGroupMapWriter(&mut self, stream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxContentGroupMapWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFactory2_Vtbl {
        unsafe extern "system" fn CreateContentGroupMapReader<Impl: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, contentgroupmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContentGroupMapReader(::core::mem::transmute(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *contentgroupmapreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSourceContentGroupMapReader<Impl: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, reader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSourceContentGroupMapReader(::core::mem::transmute(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *reader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContentGroupMapWriter<Impl: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contentgroupmapwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContentGroupMapWriter(::core::mem::transmute(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *contentgroupmapwriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateContentGroupMapReader: CreateContentGroupMapReader::<Impl, IMPL_OFFSET>,
            CreateSourceContentGroupMapReader: CreateSourceContentGroupMapReader::<Impl, IMPL_OFFSET>,
            CreateContentGroupMapWriter: CreateContentGroupMapWriter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxFile_Impl: Sized {
    fn GetCompressionOption(&mut self) -> ::windows::core::Result<APPX_COMPRESSION_OPTION>;
    fn GetContentType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetSize(&mut self) -> ::windows::core::Result<u64>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFile_Vtbl {
        unsafe extern "system" fn GetCompressionOption<Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompressionOption() {
                ::core::result::Result::Ok(ok__) => {
                    *compressionoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *contenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *filename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCompressionOption: GetCompressionOption::<Impl, IMPL_OFFSET>,
            GetContentType: GetContentType::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxFilesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxFile>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxFilesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFilesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFilesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *file = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFilesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestApplication_Impl: Sized {
    fn GetStringValue(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetAppUserModelId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestApplication_Vtbl {
        unsafe extern "system" fn GetStringValue<Impl: IAppxManifestApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppUserModelId<Impl: IAppxManifestApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *appusermodelid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStringValue: GetStringValue::<Impl, IMPL_OFFSET>,
            GetAppUserModelId: GetAppUserModelId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestApplicationsEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestApplication>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestApplicationsEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestApplicationsEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestApplicationsEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *application = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestApplicationsEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestCapabilitiesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestCapabilitiesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *capability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestCapabilitiesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDeviceCapabilitiesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *devicecapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDeviceCapabilitiesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDriverConstraint_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetMinVersion(&mut self) -> ::windows::core::Result<u64>;
    fn GetMinDate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverConstraint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverConstraint_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverConstraint_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *minversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDate<Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mindate: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinDate() {
                ::core::result::Result::Ok(ok__) => {
                    *mindate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetMinVersion: GetMinVersion::<Impl, IMPL_OFFSET>,
            GetMinDate: GetMinDate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraint as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDriverConstraintsEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestDriverConstraint>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverConstraintsEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverConstraintsEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *driverconstraint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraintsEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDriverDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestDriverDependency>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *driverdependency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependenciesEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IAppxManifestDriverDependency_Impl: Sized {
    fn GetDriverConstraints(&mut self) -> ::windows::core::Result<IAppxManifestDriverConstraintsEnumerator>;
}
impl IAppxManifestDriverDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverDependency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverDependency_Vtbl {
        unsafe extern "system" fn GetDriverConstraints<Impl: IAppxManifestDriverDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDriverConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *driverconstraints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDriverConstraints: GetDriverConstraints::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestHostRuntimeDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependency>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hostruntimedependency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependenciesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestHostRuntimeDependency_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetPublisher(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetMinVersion(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestHostRuntimeDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestHostRuntimeDependency_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    *publisher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *minversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetPublisher: GetPublisher::<Impl, IMPL_OFFSET>,
            GetMinVersion: GetMinVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestHostRuntimeDependency2_Impl: Sized {
    fn GetPackageFamilyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestHostRuntimeDependency2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependency2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestHostRuntimeDependency2_Vtbl {
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestHostRuntimeDependency2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *packagefamilyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPackageFamilyName: GetPackageFamilyName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestMainPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestMainPackageDependency>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *mainpackagedependency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependenciesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestMainPackageDependency_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetPublisher(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetPackageFamilyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestMainPackageDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestMainPackageDependency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestMainPackageDependency_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    *publisher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *packagefamilyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetPublisher: GetPublisher::<Impl, IMPL_OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestOSPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestOSPackageDependency>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *ospackagedependency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependenciesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestOSPackageDependency_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetVersion(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOSPackageDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOSPackageDependency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestOSPackageDependency_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestOptionalPackageInfo_Impl: Sized {
    fn GetIsOptionalPackage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetMainPackageName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOptionalPackageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOptionalPackageInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestOptionalPackageInfo_Vtbl {
        unsafe extern "system" fn GetIsOptionalPackage<Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsOptionalPackage() {
                ::core::result::Result::Ok(ok__) => {
                    *isoptionalpackage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainPackageName<Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainPackageName() {
                ::core::result::Result::Ok(ok__) => {
                    *mainpackagename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIsOptionalPackage: GetIsOptionalPackage::<Impl, IMPL_OFFSET>,
            GetMainPackageName: GetMainPackageName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestOptionalPackageInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestPackageDependency>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *dependency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependenciesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependency_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetPublisher(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetMinVersion(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependency_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    *publisher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *minversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetPublisher: GetPublisher::<Impl, IMPL_OFFSET>,
            GetMinVersion: GetMinVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependency2_Impl: Sized + IAppxManifestPackageDependency_Impl {
    fn GetMaxMajorVersionTested(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependency2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependency2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependency2_Vtbl {
        unsafe extern "system" fn GetMaxMajorVersionTested<Impl: IAppxManifestPackageDependency2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxmajorversiontested: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxMajorVersionTested() {
                ::core::result::Result::Ok(ok__) => {
                    *maxmajorversiontested = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAppxManifestPackageDependency_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMaxMajorVersionTested: GetMaxMajorVersionTested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependency3_Impl: Sized {
    fn GetIsOptional(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependency3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependency3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependency3_Vtbl {
        unsafe extern "system" fn GetIsOptional<Impl: IAppxManifestPackageDependency3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsOptional() {
                ::core::result::Result::Ok(ok__) => {
                    *isoptional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIsOptional: GetIsOptional::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageId_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetArchitecture(&mut self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE>;
    fn GetPublisher(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetVersion(&mut self) -> ::windows::core::Result<u64>;
    fn GetResourceId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn ComparePublisher(&mut self, other: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetPackageFullName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetPackageFamilyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageId_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArchitecture<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArchitecture() {
                ::core::result::Result::Ok(ok__) => {
                    *architecture = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    *publisher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *packageversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceId<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *resourceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComparePublisher<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, other: super::super::super::Foundation::PWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComparePublisher(::core::mem::transmute_copy(&other)) {
                ::core::result::Result::Ok(ok__) => {
                    *issame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFullName<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFullName() {
                ::core::result::Result::Ok(ok__) => {
                    *packagefullname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *packagefamilyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetArchitecture: GetArchitecture::<Impl, IMPL_OFFSET>,
            GetPublisher: GetPublisher::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            GetResourceId: GetResourceId::<Impl, IMPL_OFFSET>,
            ComparePublisher: ComparePublisher::<Impl, IMPL_OFFSET>,
            GetPackageFullName: GetPackageFullName::<Impl, IMPL_OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageId2_Impl: Sized + IAppxManifestPackageId_Impl {
    fn GetArchitecture2(&mut self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE2>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageId2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageId2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageId2_Vtbl {
        unsafe extern "system" fn GetArchitecture2<Impl: IAppxManifestPackageId2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArchitecture2() {
                ::core::result::Result::Ok(ok__) => {
                    *architecture = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IAppxManifestPackageId_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetArchitecture2: GetArchitecture2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageId2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestProperties_Impl: Sized {
    fn GetBoolValue(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetStringValue(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestProperties_Vtbl {
        unsafe extern "system" fn GetBoolValue<Impl: IAppxManifestProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolValue(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: IAppxManifestProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBoolValue: GetBoolValue::<Impl, IMPL_OFFSET>,
            GetStringValue: GetStringValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestQualifiedResource_Impl: Sized {
    fn GetLanguage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetScale(&mut self) -> ::windows::core::Result<u32>;
    fn GetDXFeatureLevel(&mut self) -> ::windows::core::Result<DX_FEATURE_LEVEL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestQualifiedResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestQualifiedResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestQualifiedResource_Vtbl {
        unsafe extern "system" fn GetLanguage<Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScale<Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scale: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScale() {
                ::core::result::Result::Ok(ok__) => {
                    *scale = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDXFeatureLevel<Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDXFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *dxfeaturelevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            GetScale: GetScale::<Impl, IMPL_OFFSET>,
            GetDXFeatureLevel: GetDXFeatureLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestQualifiedResourcesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestQualifiedResource>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestQualifiedResourcesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestQualifiedResourcesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResourcesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxManifestReader_Impl: Sized {
    fn GetPackageId(&mut self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetProperties(&mut self) -> ::windows::core::Result<IAppxManifestProperties>;
    fn GetPackageDependencies(&mut self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator>;
    fn GetCapabilities(&mut self) -> ::windows::core::Result<APPX_CAPABILITIES>;
    fn GetResources(&mut self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator>;
    fn GetDeviceCapabilities(&mut self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator>;
    fn GetPrerequisite(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<u64>;
    fn GetApplications(&mut self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader_Vtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    *packageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *packageproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageDependencies<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *dependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *capabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResources() {
                ::core::result::Result::Ok(ok__) => {
                    *resources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCapabilities<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *devicecapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisite<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrerequisite(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplications<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplications() {
                ::core::result::Result::Ok(ok__) => {
                    *applications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *manifeststream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPackageId: GetPackageId::<Impl, IMPL_OFFSET>,
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetPackageDependencies: GetPackageDependencies::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            GetResources: GetResources::<Impl, IMPL_OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Impl, IMPL_OFFSET>,
            GetPrerequisite: GetPrerequisite::<Impl, IMPL_OFFSET>,
            GetApplications: GetApplications::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxManifestReader2_Impl: Sized + IAppxManifestReader_Impl {
    fn GetQualifiedResources(&mut self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader2_Vtbl {
        unsafe extern "system" fn GetQualifiedResources<Impl: IAppxManifestReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQualifiedResources() {
                ::core::result::Result::Ok(ok__) => {
                    *resources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAppxManifestReader_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetQualifiedResources: GetQualifiedResources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxManifestReader3_Impl: Sized + IAppxManifestReader_Impl + IAppxManifestReader2_Impl {
    fn GetCapabilitiesByCapabilityClass(&mut self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::core::Result<IAppxManifestCapabilitiesEnumerator>;
    fn GetTargetDeviceFamilies(&mut self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReader3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader3_Vtbl {
        unsafe extern "system" fn GetCapabilitiesByCapabilityClass<Impl: IAppxManifestReader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilitiesByCapabilityClass(::core::mem::transmute_copy(&capabilityclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *capabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetDeviceFamilies<Impl: IAppxManifestReader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetDeviceFamilies() {
                ::core::result::Result::Ok(ok__) => {
                    *targetdevicefamilies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAppxManifestReader2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCapabilitiesByCapabilityClass: GetCapabilitiesByCapabilityClass::<Impl, IMPL_OFFSET>,
            GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxManifestReader4_Impl: Sized + IAppxManifestReader_Impl + IAppxManifestReader2_Impl + IAppxManifestReader3_Impl {
    fn GetOptionalPackageInfo(&mut self) -> ::windows::core::Result<IAppxManifestOptionalPackageInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReader4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader4_Vtbl {
        unsafe extern "system" fn GetOptionalPackageInfo<Impl: IAppxManifestReader4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionalPackageInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *optionalpackageinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAppxManifestReader3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOptionalPackageInfo: GetOptionalPackageInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader4 as ::windows::core::Interface>::IID
    }
}
pub trait IAppxManifestReader5_Impl: Sized {
    fn GetMainPackageDependencies(&mut self) -> ::windows::core::Result<IAppxManifestMainPackageDependenciesEnumerator>;
}
impl IAppxManifestReader5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader5_Vtbl {
        unsafe extern "system" fn GetMainPackageDependencies<Impl: IAppxManifestReader5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainPackageDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *mainpackagedependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMainPackageDependencies: GetMainPackageDependencies::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestReader6_Impl: Sized {
    fn GetIsNonQualifiedResourcePackage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestReader6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader6_Vtbl {
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Impl: IAppxManifestReader6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsNonQualifiedResourcePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *isnonqualifiedresourcepackage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader6 as ::windows::core::Interface>::IID
    }
}
pub trait IAppxManifestReader7_Impl: Sized {
    fn GetDriverDependencies(&mut self) -> ::windows::core::Result<IAppxManifestDriverDependenciesEnumerator>;
    fn GetOSPackageDependencies(&mut self) -> ::windows::core::Result<IAppxManifestOSPackageDependenciesEnumerator>;
    fn GetHostRuntimeDependencies(&mut self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependenciesEnumerator>;
}
impl IAppxManifestReader7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader7_Vtbl {
        unsafe extern "system" fn GetDriverDependencies<Impl: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDriverDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *driverdependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOSPackageDependencies<Impl: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOSPackageDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *ospackagedependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostRuntimeDependencies<Impl: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostRuntimeDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *hostruntimedependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDriverDependencies: GetDriverDependencies::<Impl, IMPL_OFFSET>,
            GetOSPackageDependencies: GetOSPackageDependencies::<Impl, IMPL_OFFSET>,
            GetHostRuntimeDependencies: GetHostRuntimeDependencies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestResourcesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestResourcesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestResourcesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestResourcesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestResourcesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestTargetDeviceFamiliesEnumerator_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamily>;
    fn GetHasCurrent(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *targetdevicefamily = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *hascurrent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            GetHasCurrent: GetHasCurrent::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestTargetDeviceFamily_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetMinVersion(&mut self) -> ::windows::core::Result<u64>;
    fn GetMaxVersionTested(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestTargetDeviceFamily_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestTargetDeviceFamily_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestTargetDeviceFamily_Vtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *minversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxVersionTested<Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxversiontested: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxVersionTested() {
                ::core::result::Result::Ok(ok__) => {
                    *maxversiontested = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetMinVersion: GetMinVersion::<Impl, IMPL_OFFSET>,
            GetMaxVersionTested: GetMaxVersionTested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamily as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxPackageEditor_Impl: Sized {
    fn SetWorkingDirectory(&mut self, workingdirectory: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateDeltaPackage(&mut self, updatedpackagestream: &::core::option::Option<super::super::super::System::Com::IStream>, baselinepackagestream: &::core::option::Option<super::super::super::System::Com::IStream>, deltapackagestream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CreateDeltaPackageUsingBaselineBlockMap(&mut self, updatedpackagestream: &::core::option::Option<super::super::super::System::Com::IStream>, baselineblockmapstream: &::core::option::Option<super::super::super::System::Com::IStream>, baselinepackagefullname: super::super::super::Foundation::PWSTR, deltapackagestream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn UpdatePackage(&mut self, baselinepackagestream: &::core::option::Option<super::super::super::System::Com::IStream>, deltapackagestream: &::core::option::Option<super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::Result<()>;
    fn UpdateEncryptedPackage(&mut self, baselineencryptedpackagestream: &::core::option::Option<super::super::super::System::Com::IStream>, deltapackagestream: &::core::option::Option<super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>;
    fn UpdatePackageManifest(&mut self, packagestream: &::core::option::Option<super::super::super::System::Com::IStream>, updatedmanifeststream: &::core::option::Option<super::super::super::System::Com::IStream>, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxPackageEditor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageEditor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageEditor_Vtbl {
        unsafe extern "system" fn SetWorkingDirectory<Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&workingdirectory)).into()
        }
        unsafe extern "system" fn CreateDeltaPackage<Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: ::windows::core::RawPtr, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDeltaPackage(::core::mem::transmute(&updatedpackagestream), ::core::mem::transmute(&baselinepackagestream), ::core::mem::transmute(&deltapackagestream)).into()
        }
        unsafe extern "system" fn CreateDeltaPackageUsingBaselineBlockMap<Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: ::windows::core::RawPtr, baselineblockmapstream: ::windows::core::RawPtr, baselinepackagefullname: super::super::super::Foundation::PWSTR, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDeltaPackageUsingBaselineBlockMap(::core::mem::transmute(&updatedpackagestream), ::core::mem::transmute(&baselineblockmapstream), ::core::mem::transmute_copy(&baselinepackagefullname), ::core::mem::transmute(&deltapackagestream)).into()
        }
        unsafe extern "system" fn UpdatePackage<Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdatePackage(::core::mem::transmute(&baselinepackagestream), ::core::mem::transmute(&deltapackagestream), ::core::mem::transmute_copy(&updateoption)).into()
        }
        unsafe extern "system" fn UpdateEncryptedPackage<Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineencryptedpackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateEncryptedPackage(::core::mem::transmute(&baselineencryptedpackagestream), ::core::mem::transmute(&deltapackagestream), ::core::mem::transmute_copy(&updateoption), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn UpdatePackageManifest<Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestream: ::windows::core::RawPtr, updatedmanifeststream: ::windows::core::RawPtr, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdatePackageManifest(::core::mem::transmute(&packagestream), ::core::mem::transmute(&updatedmanifeststream), ::core::mem::transmute_copy(&ispackageencrypted), ::core::mem::transmute_copy(&options)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetWorkingDirectory: SetWorkingDirectory::<Impl, IMPL_OFFSET>,
            CreateDeltaPackage: CreateDeltaPackage::<Impl, IMPL_OFFSET>,
            CreateDeltaPackageUsingBaselineBlockMap: CreateDeltaPackageUsingBaselineBlockMap::<Impl, IMPL_OFFSET>,
            UpdatePackage: UpdatePackage::<Impl, IMPL_OFFSET>,
            UpdateEncryptedPackage: UpdateEncryptedPackage::<Impl, IMPL_OFFSET>,
            UpdatePackageManifest: UpdatePackageManifest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxPackageReader_Impl: Sized {
    fn GetBlockMap(&mut self) -> ::windows::core::Result<IAppxBlockMapReader>;
    fn GetFootprintFile(&mut self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile>;
    fn GetPayloadFile(&mut self, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IAppxFile>;
    fn GetPayloadFiles(&mut self) -> ::windows::core::Result<IAppxFilesEnumerator>;
    fn GetManifest(&mut self) -> ::windows::core::Result<IAppxManifestReader>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxPackageReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageReader_Vtbl {
        unsafe extern "system" fn GetBlockMap<Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlockMap() {
                ::core::result::Result::Ok(ok__) => {
                    *blockmapreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFootprintFile<Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFootprintFile(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *file = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFile<Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadFile(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *file = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFiles<Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayloadFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *filesenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManifest() {
                ::core::result::Result::Ok(ok__) => {
                    *manifestreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBlockMap: GetBlockMap::<Impl, IMPL_OFFSET>,
            GetFootprintFile: GetFootprintFile::<Impl, IMPL_OFFSET>,
            GetPayloadFile: GetPayloadFile::<Impl, IMPL_OFFSET>,
            GetPayloadFiles: GetPayloadFiles::<Impl, IMPL_OFFSET>,
            GetManifest: GetManifest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxPackageWriter_Impl: Sized {
    fn AddPayloadFile(&mut self, filename: super::super::super::Foundation::PWSTR, contenttype: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&mut self, manifest: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxPackageWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageWriter_Vtbl {
        unsafe extern "system" fn AddPayloadFile<Impl: IAppxPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, contenttype: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&contenttype), ::core::mem::transmute_copy(&compressionoption), ::core::mem::transmute(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Impl: IAppxPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute(&manifest)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddPayloadFile: AddPayloadFile::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter2_Impl: Sized {
    fn Close(&mut self, manifest: &::core::option::Option<super::super::super::System::Com::IStream>, contentgroupmap: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageWriter2_Vtbl {
        unsafe extern "system" fn Close<Impl: IAppxPackageWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: ::windows::core::RawPtr, contentgroupmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(::core::mem::transmute(&manifest), ::core::mem::transmute(&contentgroupmap)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Close: Close::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxPackageWriter3_Impl: Sized {
    fn AddPayloadFiles(&mut self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxPackageWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriter3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageWriter3_Vtbl {
        unsafe extern "system" fn AddPayloadFiles<Impl: IAppxPackageWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPayloadFiles(::core::mem::transmute_copy(&filecount), ::core::mem::transmute_copy(&payloadfiles), ::core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddPayloadFiles: AddPayloadFiles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxPackagingDiagnosticEventSink_Impl: Sized {
    fn ReportContextChange(&mut self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: super::super::super::Foundation::PSTR, contextmessage: super::super::super::Foundation::PWSTR, detailsmessage: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ReportError(&mut self, errormessage: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxPackagingDiagnosticEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackagingDiagnosticEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackagingDiagnosticEventSink_Vtbl {
        unsafe extern "system" fn ReportContextChange<Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: super::super::super::Foundation::PSTR, contextmessage: super::super::super::Foundation::PWSTR, detailsmessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportContextChange(::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&contextid), ::core::mem::transmute_copy(&contextname), ::core::mem::transmute_copy(&contextmessage), ::core::mem::transmute_copy(&detailsmessage)).into()
        }
        unsafe extern "system" fn ReportError<Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errormessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(::core::mem::transmute_copy(&errormessage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReportContextChange: ReportContextChange::<Impl, IMPL_OFFSET>,
            ReportError: ReportError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSink as ::windows::core::Interface>::IID
    }
}
pub trait IAppxPackagingDiagnosticEventSinkManager_Impl: Sized {
    fn SetSinkForProcess(&mut self, sink: &::core::option::Option<IAppxPackagingDiagnosticEventSink>) -> ::windows::core::Result<()>;
}
impl IAppxPackagingDiagnosticEventSinkManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackagingDiagnosticEventSinkManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackagingDiagnosticEventSinkManager_Vtbl {
        unsafe extern "system" fn SetSinkForProcess<Impl: IAppxPackagingDiagnosticEventSinkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSinkForProcess(::core::mem::transmute(&sink)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSinkForProcess: SetSinkForProcess::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSinkManager as ::windows::core::Interface>::IID
    }
}
pub trait IAppxSourceContentGroupMapReader_Impl: Sized {
    fn GetRequiredGroup(&mut self) -> ::windows::core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&mut self) -> ::windows::core::Result<IAppxContentGroupsEnumerator>;
}
impl IAppxSourceContentGroupMapReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxSourceContentGroupMapReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxSourceContentGroupMapReader_Vtbl {
        unsafe extern "system" fn GetRequiredGroup<Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiredGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *requiredgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomaticGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *automaticgroupsenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Impl, IMPL_OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxSourceContentGroupMapReader as ::windows::core::Interface>::IID
    }
}
