#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxBlockMapBlock_Impl: Sized {
    fn GetHash(&self, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetCompressedSize(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IAppxBlockMapBlock {}
impl IAppxBlockMapBlock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapBlock_Impl, const OFFSET: isize>() -> IAppxBlockMapBlock_Vtbl {
        unsafe extern "system" fn GetHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHash(::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&buffer)).into()
        }
        unsafe extern "system" fn GetCompressedSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCompressedSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHash: GetHash::<Identity, Impl, OFFSET>,
            GetCompressedSize: GetCompressedSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapBlock as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBlockMapBlocksEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxBlockMapBlock>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxBlockMapBlocksEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBlockMapBlocksEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>() -> IAppxBlockMapBlocksEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(block, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapBlocksEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBlockMapFile_Impl: Sized {
    fn GetBlocks(&self) -> ::windows::core::Result<IAppxBlockMapBlocksEnumerator>;
    fn GetLocalFileHeaderSize(&self) -> ::windows::core::Result<u32>;
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetUncompressedSize(&self) -> ::windows::core::Result<u64>;
    fn ValidateFileHash(&self, filestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IAppxBlockMapFile {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBlockMapFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>() -> IAppxBlockMapFile_Vtbl {
        unsafe extern "system" fn GetBlocks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blocks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalFileHeaderSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfhsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalFileHeaderSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lfhsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUncompressedSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUncompressedSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateFileHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filestream: *mut ::core::ffi::c_void, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValidateFileHash(::windows::core::from_raw_borrowed(&filestream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isvalid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBlocks: GetBlocks::<Identity, Impl, OFFSET>,
            GetLocalFileHeaderSize: GetLocalFileHeaderSize::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetUncompressedSize: GetUncompressedSize::<Identity, Impl, OFFSET>,
            ValidateFileHash: ValidateFileHash::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapFile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBlockMapFilesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxBlockMapFile>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxBlockMapFilesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBlockMapFilesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>() -> IAppxBlockMapFilesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapFilesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBlockMapReader_Impl: Sized {
    fn GetFile(&self, filename: &::windows::core::PCWSTR) -> ::windows::core::Result<IAppxBlockMapFile>;
    fn GetFiles(&self) -> ::windows::core::Result<IAppxBlockMapFilesEnumerator>;
    fn GetHashMethod(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxBlockMapReader {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBlockMapReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>() -> IAppxBlockMapReader_Vtbl {
        unsafe extern "system" fn GetFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFile(::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethod: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHashMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hashmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapstream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetFiles: GetFiles::<Identity, Impl, OFFSET>,
            GetHashMethod: GetHashMethod::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBlockMapReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory_Impl: Sized {
    fn CreateBundleWriter(&self, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, bundleversion: u64) -> ::windows::core::Result<IAppxBundleWriter>;
    fn CreateBundleReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxBundleReader>;
    fn CreateBundleManifestReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxBundleManifestReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxBundleFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: isize>() -> IAppxBundleFactory_Vtbl {
        unsafe extern "system" fn CreateBundleWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBundleWriter(::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&bundleversion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlewriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBundleReader(::windows::core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlereader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleManifestReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBundleManifestReader(::windows::core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBundleWriter: CreateBundleWriter::<Identity, Impl, OFFSET>,
            CreateBundleReader: CreateBundleReader::<Identity, Impl, OFFSET>,
            CreateBundleManifestReader: CreateBundleManifestReader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxBundleManifestOptionalBundleInfo_Impl: Sized {
    fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPackageInfoItems(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxBundleManifestOptionalBundleInfo {}
impl IAppxBundleManifestOptionalBundleInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfo_Vtbl {
        unsafe extern "system" fn GetPackageId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageInfoItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageinfoitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestOptionalBundleInfoEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfo>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxBundleManifestOptionalBundleInfoEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(optionalbundle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxBundleManifestPackageInfo_Impl: Sized {
    fn GetPackageType(&self) -> ::windows::core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE>;
    fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetOffset(&self) -> ::windows::core::Result<u64>;
    fn GetSize(&self) -> ::windows::core::Result<u64>;
    fn GetResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo {}
impl IAppxBundleManifestPackageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo_Vtbl {
        unsafe extern "system" fn GetPackageType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resources, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageType: GetPackageType::<Identity, Impl, OFFSET>,
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            GetOffset: GetOffset::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetResources: GetResources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo2_Impl: Sized {
    fn GetIsPackageReference(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsDefaultApplicablePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo2 {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo2_Vtbl {
        unsafe extern "system" fn GetIsPackageReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsPackageReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ispackagereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsNonQualifiedResourcePackage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnonqualifiedresourcepackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDefaultApplicablePackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsDefaultApplicablePackage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdefaultapplicablepackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsPackageReference: GetIsPackageReference::<Identity, Impl, OFFSET>,
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, Impl, OFFSET>,
            GetIsDefaultApplicablePackage: GetIsDefaultApplicablePackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxBundleManifestPackageInfo3_Impl: Sized {
    fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo3 {}
impl IAppxBundleManifestPackageInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo3_Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo3_Vtbl {
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetDeviceFamilies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetdevicefamilies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo4_Impl: Sized {
    fn GetIsStub(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfo4 {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfo4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo4_Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo4_Vtbl {
        unsafe extern "system" fn GetIsStub<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsStub() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isstub, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIsStub: GetIsStub::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfoEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfo>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxBundleManifestPackageInfoEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfoEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleManifestReader_Impl: Sized {
    fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetPackageInfoItems(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator>;
    fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxBundleManifestReader {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleManifestReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: isize>() -> IAppxBundleManifestReader_Vtbl {
        unsafe extern "system" fn GetPackageId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageInfoItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageinfoitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifeststream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxBundleManifestReader2_Impl: Sized {
    fn GetOptionalBundles(&self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxBundleManifestReader2 {}
impl IAppxBundleManifestReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestReader2_Impl, const OFFSET: isize>() -> IAppxBundleManifestReader2_Vtbl {
        unsafe extern "system" fn GetOptionalBundles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleManifestReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundles: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionalBundles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(optionalbundles, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOptionalBundles: GetOptionalBundles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxBundleReader_Impl: Sized {
    fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile>;
    fn GetBlockMap(&self) -> ::windows::core::Result<IAppxBlockMapReader>;
    fn GetManifest(&self) -> ::windows::core::Result<IAppxBundleManifestReader>;
    fn GetPayloadPackages(&self) -> ::windows::core::Result<IAppxFilesEnumerator>;
    fn GetPayloadPackage(&self, filename: &::windows::core::PCWSTR) -> ::windows::core::Result<IAppxFile>;
}
impl ::windows::core::RuntimeName for IAppxBundleReader {}
impl IAppxBundleReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: isize>() -> IAppxBundleReader_Vtbl {
        unsafe extern "system" fn GetFootprintFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFootprintFile(::core::mem::transmute_copy(&filetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(footprintfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlockMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlockMap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetManifest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payloadpackages: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPayloadPackages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(payloadpackages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, payloadpackage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPayloadPackage(::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(payloadpackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFootprintFile: GetFootprintFile::<Identity, Impl, OFFSET>,
            GetBlockMap: GetBlockMap::<Identity, Impl, OFFSET>,
            GetManifest: GetManifest::<Identity, Impl, OFFSET>,
            GetPayloadPackages: GetPayloadPackages::<Identity, Impl, OFFSET>,
            GetPayloadPackage: GetPayloadPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter_Impl: Sized {
    fn AddPayloadPackage(&self, filename: &::windows::core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxBundleWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter_Impl, const OFFSET: isize>() -> IAppxBundleWriter_Vtbl {
        unsafe extern "system" fn AddPayloadPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadPackage(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&packagestream)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter2_Impl: Sized {
    fn AddExternalPackageReference(&self, filename: &::windows::core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxBundleWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter2_Impl, const OFFSET: isize>() -> IAppxBundleWriter2_Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddExternalPackageReference(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&inputstream)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter3_Impl: Sized {
    fn AddPackageReference(&self, filename: &::windows::core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&self, hashmethodstring: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxBundleWriter3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter3_Impl, const OFFSET: isize>() -> IAppxBundleWriter3_Vtbl {
        unsafe extern "system" fn AddPackageReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPackageReference(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethodstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::core::mem::transmute(&hashmethodstring)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPackageReference: AddPackageReference::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBundleWriter4_Impl: Sized {
    fn AddPayloadPackage(&self, filename: &::windows::core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddPackageReference(&self, filename: &::windows::core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddExternalPackageReference(&self, filename: &::windows::core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IAppxBundleWriter4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: isize>() -> IAppxBundleWriter4_Vtbl {
        unsafe extern "system" fn AddPayloadPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadPackage(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&packagestream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddPackageReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPackageReference(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddExternalPackageReference(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Identity, Impl, OFFSET>,
            AddPackageReference: AddPackageReference::<Identity, Impl, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleWriter4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxContentGroup_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetFiles(&self) -> ::windows::core::Result<IAppxContentGroupFilesEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxContentGroup {}
impl IAppxContentGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroup_Impl, const OFFSET: isize>() -> IAppxContentGroup_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(groupname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetFiles: GetFiles::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroup as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroupFilesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxContentGroupFilesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupFilesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>() -> IAppxContentGroupFilesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupFilesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxContentGroupMapReader_Impl: Sized {
    fn GetRequiredGroup(&self) -> ::windows::core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&self) -> ::windows::core::Result<IAppxContentGroupsEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxContentGroupMapReader {}
impl IAppxContentGroupMapReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupMapReader_Impl, const OFFSET: isize>() -> IAppxContentGroupMapReader_Vtbl {
        unsafe extern "system" fn GetRequiredGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequiredGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requiredgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAutomaticGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(automaticgroupsenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Identity, Impl, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupMapReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxContentGroupMapWriter_Impl: Sized {
    fn AddAutomaticGroup(&self, groupname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AddAutomaticFile(&self, filename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppxContentGroupMapWriter {}
impl IAppxContentGroupMapWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>() -> IAppxContentGroupMapWriter_Vtbl {
        unsafe extern "system" fn AddAutomaticGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAutomaticGroup(::core::mem::transmute(&groupname)).into()
        }
        unsafe extern "system" fn AddAutomaticFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAutomaticFile(::core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAutomaticGroup: AddAutomaticGroup::<Identity, Impl, OFFSET>,
            AddAutomaticFile: AddAutomaticFile::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupMapWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroupsEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxContentGroup>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxContentGroupsEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupsEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>() -> IAppxContentGroupsEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxContentGroupsEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter_Impl: Sized {
    fn AddPayloadPackageEncrypted(&self, filename: &::windows::core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxEncryptedBundleWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter_Vtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadPackageEncrypted(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&packagestream)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter2_Impl: Sized {
    fn AddExternalPackageReference(&self, filename: &::windows::core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxEncryptedBundleWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter2_Impl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter2_Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddExternalPackageReference(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&inputstream)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptedBundleWriter3_Impl: Sized {
    fn AddPayloadPackageEncrypted(&self, filename: &::windows::core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddExternalPackageReference(&self, filename: &::windows::core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IAppxEncryptedBundleWriter3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedBundleWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter3_Vtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadPackageEncrypted(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&packagestream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddExternalPackageReference(::core::mem::transmute(&filename), ::windows::core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, Impl, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter_Impl: Sized {
    fn AddPayloadFileEncrypted(&self, filename: &::windows::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxEncryptedPackageWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedPackageWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>() -> IAppxEncryptedPackageWriter_Vtbl {
        unsafe extern "system" fn AddPayloadFileEncrypted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadFileEncrypted(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&compressionoption), ::windows::core::from_raw_borrowed(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadFileEncrypted: AddPayloadFileEncrypted::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter2_Impl: Sized {
    fn AddPayloadFilesEncrypted(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxEncryptedPackageWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedPackageWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedPackageWriter2_Impl, const OFFSET: isize>() -> IAppxEncryptedPackageWriter2_Vtbl {
        unsafe extern "system" fn AddPayloadFilesEncrypted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptedPackageWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadFilesEncrypted(::core::mem::transmute_copy(&filecount), ::core::mem::transmute_copy(&payloadfiles), ::core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPayloadFilesEncrypted: AddPayloadFilesEncrypted::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory_Impl: Sized {
    fn EncryptPackage(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn DecryptPackage(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>;
    fn CreateEncryptedPackageWriter(&self, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, manifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>;
    fn CreateEncryptedPackageReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxPackageReader>;
    fn EncryptBundle(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn DecryptBundle(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>;
    fn CreateEncryptedBundleWriter(&self, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter>;
    fn CreateEncryptedBundleReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxBundleReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IAppxEncryptionFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>() -> IAppxEncryptionFactory_Vtbl {
        unsafe extern "system" fn EncryptPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EncryptPackage(::windows::core::from_raw_borrowed(&inputstream), ::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn DecryptPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DecryptPackage(::windows::core::from_raw_borrowed(&inputstream), ::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncryptedPackageWriter(::windows::core::from_raw_borrowed(&outputstream), ::windows::core::from_raw_borrowed(&manifeststream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncryptedPackageReader(::windows::core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&keyinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagereader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EncryptBundle(::windows::core::from_raw_borrowed(&inputstream), ::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn DecryptBundle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DecryptBundle(::windows::core::from_raw_borrowed(&inputstream), ::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncryptedBundleWriter(::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&bundleversion), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlewriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncryptedBundleReader(::windows::core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&keyinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlereader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncryptPackage: EncryptPackage::<Identity, Impl, OFFSET>,
            DecryptPackage: DecryptPackage::<Identity, Impl, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, Impl, OFFSET>,
            CreateEncryptedPackageReader: CreateEncryptedPackageReader::<Identity, Impl, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, Impl, OFFSET>,
            DecryptBundle: DecryptBundle::<Identity, Impl, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, Impl, OFFSET>,
            CreateEncryptedBundleReader: CreateEncryptedBundleReader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory2_Impl: Sized {
    fn CreateEncryptedPackageWriter(&self, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, manifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, contentgroupmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IAppxEncryptionFactory2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory2_Impl, const OFFSET: isize>() -> IAppxEncryptionFactory2_Vtbl {
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncryptedPackageWriter(::windows::core::from_raw_borrowed(&outputstream), ::windows::core::from_raw_borrowed(&manifeststream), ::windows::core::from_raw_borrowed(&contentgroupmapstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory3_Impl: Sized {
    fn EncryptPackage(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn CreateEncryptedPackageWriter(&self, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, manifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, contentgroupmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>;
    fn EncryptBundle(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>;
    fn CreateEncryptedBundleWriter(&self, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxEncryptionFactory3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>() -> IAppxEncryptionFactory3_Vtbl {
        unsafe extern "system" fn EncryptPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EncryptPackage(::windows::core::from_raw_borrowed(&inputstream), ::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncryptedPackageWriter(::windows::core::from_raw_borrowed(&outputstream), ::windows::core::from_raw_borrowed(&manifeststream), ::windows::core::from_raw_borrowed(&contentgroupmapstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EncryptBundle(::windows::core::from_raw_borrowed(&inputstream), ::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncryptedBundleWriter(::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&bundleversion), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlewriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncryptPackage: EncryptPackage::<Identity, Impl, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, Impl, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, Impl, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory4_Impl: Sized {
    fn EncryptPackage(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxEncryptionFactory4 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory4_Impl, const OFFSET: isize>() -> IAppxEncryptionFactory4_Vtbl {
        unsafe extern "system" fn EncryptPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxEncryptionFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EncryptPackage(::windows::core::from_raw_borrowed(&inputstream), ::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles), ::core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EncryptPackage: EncryptPackage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxFactory_Impl: Sized {
    fn CreatePackageWriter(&self, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_PACKAGE_SETTINGS) -> ::windows::core::Result<IAppxPackageWriter>;
    fn CreatePackageReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxPackageReader>;
    fn CreateManifestReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxManifestReader>;
    fn CreateBlockMapReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxBlockMapReader>;
    fn CreateValidatedBlockMapReader(&self, blockmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, signaturefilename: &::windows::core::PCWSTR) -> ::windows::core::Result<IAppxBlockMapReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IAppxFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: isize>() -> IAppxFactory_Vtbl {
        unsafe extern "system" fn CreatePackageWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageWriter(::windows::core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageReader(::windows::core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagereader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManifestReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateManifestReader(::windows::core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlockMapReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlockMapReader(::windows::core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateValidatedBlockMapReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: *mut ::core::ffi::c_void, signaturefilename: ::windows::core::PCWSTR, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateValidatedBlockMapReader(::windows::core::from_raw_borrowed(&blockmapstream), ::core::mem::transmute(&signaturefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackageWriter: CreatePackageWriter::<Identity, Impl, OFFSET>,
            CreatePackageReader: CreatePackageReader::<Identity, Impl, OFFSET>,
            CreateManifestReader: CreateManifestReader::<Identity, Impl, OFFSET>,
            CreateBlockMapReader: CreateBlockMapReader::<Identity, Impl, OFFSET>,
            CreateValidatedBlockMapReader: CreateValidatedBlockMapReader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory2_Impl: Sized {
    fn CreateContentGroupMapReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxContentGroupMapReader>;
    fn CreateSourceContentGroupMapReader(&self, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxSourceContentGroupMapReader>;
    fn CreateContentGroupMapWriter(&self, stream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<IAppxContentGroupMapWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxFactory2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: isize>() -> IAppxFactory2_Vtbl {
        unsafe extern "system" fn CreateContentGroupMapReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, contentgroupmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateContentGroupMapReader(::windows::core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentgroupmapreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSourceContentGroupMapReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, reader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSourceContentGroupMapReader(::windows::core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContentGroupMapWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, contentgroupmapwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateContentGroupMapWriter(::windows::core::from_raw_borrowed(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentgroupmapwriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContentGroupMapReader: CreateContentGroupMapReader::<Identity, Impl, OFFSET>,
            CreateSourceContentGroupMapReader: CreateSourceContentGroupMapReader::<Identity, Impl, OFFSET>,
            CreateContentGroupMapWriter: CreateContentGroupMapWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFactory2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFile_Impl: Sized {
    fn GetCompressionOption(&self) -> ::windows::core::Result<APPX_COMPRESSION_OPTION>;
    fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSize(&self) -> ::windows::core::Result<u64>;
    fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxFile {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: isize>() -> IAppxFile_Vtbl {
        unsafe extern "system" fn GetCompressionOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCompressionOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compressionoption, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCompressionOption: GetCompressionOption::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxFilesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxFile>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxFilesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxFilesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: isize>() -> IAppxFilesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxFilesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestApplication_Impl: Sized {
    fn GetStringValue(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetAppUserModelId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IAppxManifestApplication {}
impl IAppxManifestApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestApplication_Impl, const OFFSET: isize>() -> IAppxManifestApplication_Vtbl {
        unsafe extern "system" fn GetStringValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringValue(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppUserModelId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(appusermodelid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            GetAppUserModelId: GetAppUserModelId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestApplication as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestApplicationsEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestApplication>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestApplicationsEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestApplicationsEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestApplicationsEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(application, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestApplicationsEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestCapabilitiesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestCapabilitiesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestCapabilitiesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capability: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capability, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestCapabilitiesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDeviceCapabilitiesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestDeviceCapabilitiesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapability: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecapability, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDeviceCapabilitiesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestDriverConstraint_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMinVersion(&self) -> ::windows::core::Result<u64>;
    fn GetMinDate(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IAppxManifestDriverConstraint {}
impl IAppxManifestDriverConstraint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>() -> IAppxManifestDriverConstraint_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mindate: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mindate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
            GetMinDate: GetMinDate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraint as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDriverConstraintsEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestDriverConstraint>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestDriverConstraintsEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverConstraintsEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestDriverConstraintsEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverconstraint, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraintsEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDriverDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestDriverDependency>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestDriverDependenciesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestDriverDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverdependency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependenciesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestDriverDependency_Impl: Sized {
    fn GetDriverConstraints(&self) -> ::windows::core::Result<IAppxManifestDriverConstraintsEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxManifestDriverDependency {}
impl IAppxManifestDriverDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverDependency_Impl, const OFFSET: isize>() -> IAppxManifestDriverDependency_Vtbl {
        unsafe extern "system" fn GetDriverConstraints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestDriverDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraints: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDriverConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverconstraints, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDriverConstraints: GetDriverConstraints::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependency as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestHostRuntimeDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependency>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestHostRuntimeDependenciesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hostruntimedependency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependenciesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestHostRuntimeDependency_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMinVersion(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IAppxManifestHostRuntimeDependency {}
impl IAppxManifestHostRuntimeDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependency_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestHostRuntimeDependency2_Impl: Sized {
    fn GetPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IAppxManifestHostRuntimeDependency2 {}
impl IAppxManifestHostRuntimeDependency2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency2_Impl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependency2_Vtbl {
        unsafe extern "system" fn GetPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPackageFamilyName: GetPackageFamilyName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestMainPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestMainPackageDependency>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestMainPackageDependenciesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mainpackagedependency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependenciesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestMainPackageDependency_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IAppxManifestMainPackageDependency {}
impl IAppxManifestMainPackageDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>() -> IAppxManifestMainPackageDependency_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependency as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestOSPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestOSPackageDependency>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestOSPackageDependenciesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ospackagedependency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependenciesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestOSPackageDependency_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetVersion(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IAppxManifestOSPackageDependency {}
impl IAppxManifestOSPackageDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>() -> IAppxManifestOSPackageDependency_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependency as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestOptionalPackageInfo_Impl: Sized {
    fn GetIsOptionalPackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetMainPackageName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestOptionalPackageInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOptionalPackageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>() -> IAppxManifestOptionalPackageInfo_Vtbl {
        unsafe extern "system" fn GetIsOptionalPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsOptionalPackage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isoptionalpackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainPackageName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMainPackageName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mainpackagename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsOptionalPackage: GetIsOptionalPackage::<Identity, Impl, OFFSET>,
            GetMainPackageName: GetMainPackageName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestOptionalPackageInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestPackageDependency>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestPackageDependenciesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestPackageDependenciesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dependency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependenciesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestPackageDependency_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMinVersion(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IAppxManifestPackageDependency {}
impl IAppxManifestPackageDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: isize>() -> IAppxManifestPackageDependency_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestPackageDependency2_Impl: Sized + IAppxManifestPackageDependency_Impl {
    fn GetMaxMajorVersionTested(&self) -> ::windows::core::Result<u16>;
}
impl ::windows::core::RuntimeName for IAppxManifestPackageDependency2 {}
impl IAppxManifestPackageDependency2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency2_Impl, const OFFSET: isize>() -> IAppxManifestPackageDependency2_Vtbl {
        unsafe extern "system" fn GetMaxMajorVersionTested<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxmajorversiontested: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxMajorVersionTested() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxmajorversiontested, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAppxManifestPackageDependency_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMaxMajorVersionTested: GetMaxMajorVersionTested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency2 as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestPackageDependency as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependency3_Impl: Sized {
    fn GetIsOptional(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestPackageDependency3 {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependency3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency3_Impl, const OFFSET: isize>() -> IAppxManifestPackageDependency3_Vtbl {
        unsafe extern "system" fn GetIsOptional<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageDependency3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsOptional() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isoptional, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIsOptional: GetIsOptional::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageId_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetArchitecture(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE>;
    fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetVersion(&self) -> ::windows::core::Result<u64>;
    fn GetResourceId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn ComparePublisher(&self, other: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetPackageFullName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestPackageId {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>() -> IAppxManifestPackageId_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArchitecture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetArchitecture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(architecture, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPublisher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResourceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComparePublisher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, other: ::windows::core::PCWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComparePublisher(::core::mem::transmute(&other)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issame, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFullName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageFullName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefullname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetArchitecture: GetArchitecture::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetResourceId: GetResourceId::<Identity, Impl, OFFSET>,
            ComparePublisher: ComparePublisher::<Identity, Impl, OFFSET>,
            GetPackageFullName: GetPackageFullName::<Identity, Impl, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageId as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageId2_Impl: Sized + IAppxManifestPackageId_Impl {
    fn GetArchitecture2(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE2>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestPackageId2 {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageId2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId2_Impl, const OFFSET: isize>() -> IAppxManifestPackageId2_Vtbl {
        unsafe extern "system" fn GetArchitecture2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestPackageId2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetArchitecture2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(architecture, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAppxManifestPackageId_Vtbl::new::<Identity, Impl, OFFSET>(), GetArchitecture2: GetArchitecture2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageId2 as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestPackageId as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestProperties_Impl: Sized {
    fn GetBoolValue(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetStringValue(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestProperties {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestProperties_Impl, const OFFSET: isize>() -> IAppxManifestProperties_Vtbl {
        unsafe extern "system" fn GetBoolValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoolValue(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringValue(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBoolValue: GetBoolValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestProperties as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestQualifiedResource_Impl: Sized {
    fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetScale(&self) -> ::windows::core::Result<u32>;
    fn GetDXFeatureLevel(&self) -> ::windows::core::Result<DX_FEATURE_LEVEL>;
}
impl ::windows::core::RuntimeName for IAppxManifestQualifiedResource {}
impl IAppxManifestQualifiedResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>() -> IAppxManifestQualifiedResource_Vtbl {
        unsafe extern "system" fn GetLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scale: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scale, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDXFeatureLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDXFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dxfeaturelevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            GetScale: GetScale::<Identity, Impl, OFFSET>,
            GetDXFeatureLevel: GetDXFeatureLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestQualifiedResourcesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestQualifiedResource>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestQualifiedResourcesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestQualifiedResourcesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestQualifiedResourcesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResourcesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader_Impl: Sized {
    fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId>;
    fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties>;
    fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator>;
    fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES>;
    fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator>;
    fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator>;
    fn GetPrerequisite(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<u64>;
    fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator>;
    fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxManifestReader {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>() -> IAppxManifestReader_Vtbl {
        unsafe extern "system" fn GetPackageId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageDependencies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resources, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecapabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrerequisite(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applications: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(applications, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifeststream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetPackageDependencies: GetPackageDependencies::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetResources: GetResources::<Identity, Impl, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, Impl, OFFSET>,
            GetPrerequisite: GetPrerequisite::<Identity, Impl, OFFSET>,
            GetApplications: GetApplications::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader2_Impl: Sized + IAppxManifestReader_Impl {
    fn GetQualifiedResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxManifestReader2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader2_Impl, const OFFSET: isize>() -> IAppxManifestReader2_Vtbl {
        unsafe extern "system" fn GetQualifiedResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQualifiedResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resources, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAppxManifestReader_Vtbl::new::<Identity, Impl, OFFSET>(), GetQualifiedResources: GetQualifiedResources::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader2 as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader3_Impl: Sized + IAppxManifestReader2_Impl {
    fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::core::Result<IAppxManifestCapabilitiesEnumerator>;
    fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxManifestReader3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader3_Impl, const OFFSET: isize>() -> IAppxManifestReader3_Vtbl {
        unsafe extern "system" fn GetCapabilitiesByCapabilityClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilitiesByCapabilityClass(::core::mem::transmute_copy(&capabilityclass)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetDeviceFamilies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetdevicefamilies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAppxManifestReader2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCapabilitiesByCapabilityClass: GetCapabilitiesByCapabilityClass::<Identity, Impl, OFFSET>,
            GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader3 as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestReader as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestReader2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader4_Impl: Sized + IAppxManifestReader3_Impl {
    fn GetOptionalPackageInfo(&self) -> ::windows::core::Result<IAppxManifestOptionalPackageInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxManifestReader4 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader4_Impl, const OFFSET: isize>() -> IAppxManifestReader4_Vtbl {
        unsafe extern "system" fn GetOptionalPackageInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackageinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionalPackageInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(optionalpackageinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAppxManifestReader3_Vtbl::new::<Identity, Impl, OFFSET>(), GetOptionalPackageInfo: GetOptionalPackageInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader4 as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestReader as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestReader2 as ::windows::core::ComInterface>::IID || iid == &<IAppxManifestReader3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestReader5_Impl: Sized {
    fn GetMainPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestMainPackageDependenciesEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxManifestReader5 {}
impl IAppxManifestReader5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader5_Impl, const OFFSET: isize>() -> IAppxManifestReader5_Vtbl {
        unsafe extern "system" fn GetMainPackageDependencies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMainPackageDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mainpackagedependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMainPackageDependencies: GetMainPackageDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestReader6_Impl: Sized {
    fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestReader6 {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestReader6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader6_Impl, const OFFSET: isize>() -> IAppxManifestReader6_Vtbl {
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsNonQualifiedResourcePackage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnonqualifiedresourcepackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader6 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestReader7_Impl: Sized {
    fn GetDriverDependencies(&self) -> ::windows::core::Result<IAppxManifestDriverDependenciesEnumerator>;
    fn GetOSPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestOSPackageDependenciesEnumerator>;
    fn GetHostRuntimeDependencies(&self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependenciesEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxManifestReader7 {}
impl IAppxManifestReader7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: isize>() -> IAppxManifestReader7_Vtbl {
        unsafe extern "system" fn GetDriverDependencies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDriverDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverdependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOSPackageDependencies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOSPackageDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ospackagedependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostRuntimeDependencies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHostRuntimeDependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hostruntimedependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDriverDependencies: GetDriverDependencies::<Identity, Impl, OFFSET>,
            GetOSPackageDependencies: GetOSPackageDependencies::<Identity, Impl, OFFSET>,
            GetHostRuntimeDependencies: GetHostRuntimeDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader7 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestResourcesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestResourcesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestResourcesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestResourcesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestResourcesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestTargetDeviceFamiliesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamily>;
    fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAppxManifestTargetDeviceFamiliesEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>() -> IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamily: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetdevicefamily, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxManifestTargetDeviceFamily_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetMinVersion(&self) -> ::windows::core::Result<u64>;
    fn GetMaxVersionTested(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IAppxManifestTargetDeviceFamily {}
impl IAppxManifestTargetDeviceFamily_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>() -> IAppxManifestTargetDeviceFamily_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxVersionTested<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxversiontested: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxVersionTested() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxversiontested, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
            GetMaxVersionTested: GetMaxVersionTested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamily as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxPackageEditor_Impl: Sized {
    fn SetWorkingDirectory(&self, workingdirectory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn CreateDeltaPackage(&self, updatedpackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, baselinepackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CreateDeltaPackageUsingBaselineBlockMap(&self, updatedpackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, baselineblockmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, baselinepackagefullname: &::windows::core::PCWSTR, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn UpdatePackage(&self, baselinepackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::Result<()>;
    fn UpdateEncryptedPackage(&self, baselineencryptedpackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>;
    fn UpdatePackageManifest(&self, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, updatedmanifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IAppxPackageEditor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxPackageEditor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: isize>() -> IAppxPackageEditor_Vtbl {
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWorkingDirectory(::core::mem::transmute(&workingdirectory)).into()
        }
        unsafe extern "system" fn CreateDeltaPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: *mut ::core::ffi::c_void, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDeltaPackage(::windows::core::from_raw_borrowed(&updatedpackagestream), ::windows::core::from_raw_borrowed(&baselinepackagestream), ::windows::core::from_raw_borrowed(&deltapackagestream)).into()
        }
        unsafe extern "system" fn CreateDeltaPackageUsingBaselineBlockMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: *mut ::core::ffi::c_void, baselineblockmapstream: *mut ::core::ffi::c_void, baselinepackagefullname: ::windows::core::PCWSTR, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDeltaPackageUsingBaselineBlockMap(::windows::core::from_raw_borrowed(&updatedpackagestream), ::windows::core::from_raw_borrowed(&baselineblockmapstream), ::core::mem::transmute(&baselinepackagefullname), ::windows::core::from_raw_borrowed(&deltapackagestream)).into()
        }
        unsafe extern "system" fn UpdatePackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdatePackage(::windows::core::from_raw_borrowed(&baselinepackagestream), ::windows::core::from_raw_borrowed(&deltapackagestream), ::core::mem::transmute_copy(&updateoption)).into()
        }
        unsafe extern "system" fn UpdateEncryptedPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineencryptedpackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateEncryptedPackage(::windows::core::from_raw_borrowed(&baselineencryptedpackagestream), ::windows::core::from_raw_borrowed(&deltapackagestream), ::core::mem::transmute_copy(&updateoption), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn UpdatePackageManifest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestream: *mut ::core::ffi::c_void, updatedmanifeststream: *mut ::core::ffi::c_void, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdatePackageManifest(::windows::core::from_raw_borrowed(&packagestream), ::windows::core::from_raw_borrowed(&updatedmanifeststream), ::core::mem::transmute_copy(&ispackageencrypted), ::core::mem::transmute_copy(&options)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            CreateDeltaPackage: CreateDeltaPackage::<Identity, Impl, OFFSET>,
            CreateDeltaPackageUsingBaselineBlockMap: CreateDeltaPackageUsingBaselineBlockMap::<Identity, Impl, OFFSET>,
            UpdatePackage: UpdatePackage::<Identity, Impl, OFFSET>,
            UpdateEncryptedPackage: UpdateEncryptedPackage::<Identity, Impl, OFFSET>,
            UpdatePackageManifest: UpdatePackageManifest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageEditor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxPackageReader_Impl: Sized {
    fn GetBlockMap(&self) -> ::windows::core::Result<IAppxBlockMapReader>;
    fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile>;
    fn GetPayloadFile(&self, filename: &::windows::core::PCWSTR) -> ::windows::core::Result<IAppxFile>;
    fn GetPayloadFiles(&self) -> ::windows::core::Result<IAppxFilesEnumerator>;
    fn GetManifest(&self) -> ::windows::core::Result<IAppxManifestReader>;
}
impl ::windows::core::RuntimeName for IAppxPackageReader {}
impl IAppxPackageReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: isize>() -> IAppxPackageReader_Vtbl {
        unsafe extern "system" fn GetBlockMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlockMap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFootprintFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFootprintFile(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPayloadFile(::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPayloadFiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetManifest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBlockMap: GetBlockMap::<Identity, Impl, OFFSET>,
            GetFootprintFile: GetFootprintFile::<Identity, Impl, OFFSET>,
            GetPayloadFile: GetPayloadFile::<Identity, Impl, OFFSET>,
            GetPayloadFiles: GetPayloadFiles::<Identity, Impl, OFFSET>,
            GetManifest: GetManifest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter_Impl: Sized {
    fn AddPayloadFile(&self, filename: &::windows::core::PCWSTR, contenttype: &::windows::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Close(&self, manifest: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxPackageWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageWriter_Impl, const OFFSET: isize>() -> IAppxPackageWriter_Vtbl {
        unsafe extern "system" fn AddPayloadFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, contenttype: ::windows::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadFile(::core::mem::transmute(&filename), ::core::mem::transmute(&contenttype), ::core::mem::transmute_copy(&compressionoption), ::windows::core::from_raw_borrowed(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::windows::core::from_raw_borrowed(&manifest)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadFile: AddPayloadFile::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter2_Impl: Sized {
    fn Close(&self, manifest: ::core::option::Option<&super::super::super::System::Com::IStream>, contentgroupmap: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxPackageWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageWriter2_Impl, const OFFSET: isize>() -> IAppxPackageWriter2_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: *mut ::core::ffi::c_void, contentgroupmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close(::windows::core::from_raw_borrowed(&manifest), ::windows::core::from_raw_borrowed(&contentgroupmap)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter3_Impl: Sized {
    fn AddPayloadFiles(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppxPackageWriter3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageWriter3_Impl, const OFFSET: isize>() -> IAppxPackageWriter3_Vtbl {
        unsafe extern "system" fn AddPayloadFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackageWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPayloadFiles(::core::mem::transmute_copy(&filecount), ::core::mem::transmute_copy(&payloadfiles), ::core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPayloadFiles: AddPayloadFiles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxPackagingDiagnosticEventSink_Impl: Sized {
    fn ReportContextChange(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: &::windows::core::PCSTR, contextmessage: &::windows::core::PCWSTR, detailsmessage: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ReportError(&self, errormessage: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppxPackagingDiagnosticEventSink {}
impl IAppxPackagingDiagnosticEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>() -> IAppxPackagingDiagnosticEventSink_Vtbl {
        unsafe extern "system" fn ReportContextChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: ::windows::core::PCSTR, contextmessage: ::windows::core::PCWSTR, detailsmessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportContextChange(::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&contextid), ::core::mem::transmute(&contextname), ::core::mem::transmute(&contextmessage), ::core::mem::transmute(&detailsmessage)).into()
        }
        unsafe extern "system" fn ReportError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errormessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportError(::core::mem::transmute(&errormessage)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportContextChange: ReportContextChange::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxPackagingDiagnosticEventSinkManager_Impl: Sized {
    fn SetSinkForProcess(&self, sink: ::core::option::Option<&IAppxPackagingDiagnosticEventSink>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppxPackagingDiagnosticEventSinkManager {}
impl IAppxPackagingDiagnosticEventSinkManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSinkManager_Impl, const OFFSET: isize>() -> IAppxPackagingDiagnosticEventSinkManager_Vtbl {
        unsafe extern "system" fn SetSinkForProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSinkManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSinkForProcess(::windows::core::from_raw_borrowed(&sink)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSinkForProcess: SetSinkForProcess::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSinkManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"implement\"`*"]
pub trait IAppxSourceContentGroupMapReader_Impl: Sized {
    fn GetRequiredGroup(&self) -> ::windows::core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&self) -> ::windows::core::Result<IAppxContentGroupsEnumerator>;
}
impl ::windows::core::RuntimeName for IAppxSourceContentGroupMapReader {}
impl IAppxSourceContentGroupMapReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>() -> IAppxSourceContentGroupMapReader_Vtbl {
        unsafe extern "system" fn GetRequiredGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequiredGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requiredgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAutomaticGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(automaticgroupsenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Identity, Impl, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxSourceContentGroupMapReader as ::windows::core::ComInterface>::IID
    }
}
