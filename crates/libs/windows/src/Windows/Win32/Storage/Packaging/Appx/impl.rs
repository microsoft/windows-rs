pub trait IAppxBlockMapBlockImpl: Sized {
    fn GetHash();
    fn GetCompressedSize();
}
impl IAppxBlockMapBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapBlockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapBlockVtbl {
        unsafe extern "system" fn GetHash<Impl: IAppxBlockMapBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompressedSize<Impl: IAppxBlockMapBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBlockMapBlocksEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBlockMapBlocksEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapBlocksEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapBlocksEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBlockMapBlocksEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBlockMapBlocksEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBlockMapBlocksEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBlockMapFileImpl: Sized {
    fn GetBlocks();
    fn GetLocalFileHeaderSize();
    fn GetName();
    fn GetUncompressedSize();
    fn ValidateFileHash();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBlockMapFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapFileVtbl {
        unsafe extern "system" fn GetBlocks<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalFileHeaderSize<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfhsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUncompressedSize<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateFileHash<Impl: IAppxBlockMapFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filestream: ::windows::core::RawPtr, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBlockMapFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBlockMapFilesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapFilesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapFilesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBlockMapFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBlockMapFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBlockMapFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBlockMapReaderImpl: Sized {
    fn GetFile();
    fn GetFiles();
    fn GetHashMethod();
    fn GetStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBlockMapReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBlockMapReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBlockMapReaderVtbl {
        unsafe extern "system" fn GetFile<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFiles<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHashMethod<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IAppxBlockMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleFactoryImpl: Sized {
    fn CreateBundleWriter();
    fn CreateBundleReader();
    fn CreateBundleManifestReader();
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleFactoryVtbl {
        unsafe extern "system" fn CreateBundleWriter<Impl: IAppxBundleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBundleReader<Impl: IAppxBundleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBundleManifestReader<Impl: IAppxBundleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleManifestOptionalBundleInfoImpl: Sized {
    fn GetPackageId();
    fn GetFileName();
    fn GetPackageInfoItems();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestOptionalBundleInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestOptionalBundleInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfoVtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestOptionalBundleInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileName<Impl: IAppxBundleManifestOptionalBundleInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageInfoItems<Impl: IAppxBundleManifestOptionalBundleInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleManifestOptionalBundleInfoEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestOptionalBundleInfoEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfoEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBundleManifestOptionalBundleInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleManifestPackageInfoImpl: Sized {
    fn GetPackageType();
    fn GetPackageId();
    fn GetFileName();
    fn GetOffset();
    fn GetSize();
    fn GetResources();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfoVtbl {
        unsafe extern "system" fn GetPackageType<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileName<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOffset<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResources<Impl: IAppxBundleManifestPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleManifestPackageInfo2Impl: Sized {
    fn GetIsPackageReference();
    fn GetIsNonQualifiedResourcePackage();
    fn GetIsDefaultApplicablePackage();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfo2Vtbl {
        unsafe extern "system" fn GetIsPackageReference<Impl: IAppxBundleManifestPackageInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Impl: IAppxBundleManifestPackageInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsDefaultApplicablePackage<Impl: IAppxBundleManifestPackageInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleManifestPackageInfo3Impl: Sized {
    fn GetTargetDeviceFamilies();
}
impl IAppxBundleManifestPackageInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfo3Vtbl {
        unsafe extern "system" fn GetTargetDeviceFamilies<Impl: IAppxBundleManifestPackageInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo4Impl: Sized {
    fn GetIsStub();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfo4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfo4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfo4Vtbl {
        unsafe extern "system" fn GetIsStub<Impl: IAppxBundleManifestPackageInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIsStub: GetIsStub::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfoEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleManifestPackageInfoEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestPackageInfoEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxBundleManifestPackageInfoEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleManifestReaderImpl: Sized {
    fn GetPackageId();
    fn GetPackageInfoItems();
    fn GetStream();
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleManifestReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestReaderVtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxBundleManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageInfoItems<Impl: IAppxBundleManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IAppxBundleManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleManifestReader2Impl: Sized {
    fn GetOptionalBundles();
}
impl IAppxBundleManifestReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleManifestReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleManifestReader2Vtbl {
        unsafe extern "system" fn GetOptionalBundles<Impl: IAppxBundleManifestReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalbundles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOptionalBundles: GetOptionalBundles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleReaderImpl: Sized {
    fn GetFootprintFile();
    fn GetBlockMap();
    fn GetManifest();
    fn GetPayloadPackages();
    fn GetPayloadPackage();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxBundleReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleReaderVtbl {
        unsafe extern "system" fn GetFootprintFile<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBlockMap<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetManifest<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPayloadPackages<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payloadpackages: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPayloadPackage<Impl: IAppxBundleReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, payloadpackage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleWriterImpl: Sized {
    fn AddPayloadPackage();
    fn Close();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriterVtbl {
        unsafe extern "system" fn AddPayloadPackage<Impl: IAppxBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IAppxBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleWriter2Impl: Sized {
    fn AddExternalPackageReference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriter2Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxBundleWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleWriter3Impl: Sized {
    fn AddPackageReference();
    fn Close();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriter3Vtbl {
        unsafe extern "system" fn AddPackageReference<Impl: IAppxBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IAppxBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashmethodstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxBundleWriter4Impl: Sized {
    fn AddPayloadPackage();
    fn AddPackageReference();
    fn AddExternalPackageReference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxBundleWriter4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxBundleWriter4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxBundleWriter4Vtbl {
        unsafe extern "system" fn AddPayloadPackage<Impl: IAppxBundleWriter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPackageReference<Impl: IAppxBundleWriter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxBundleWriter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxContentGroupImpl: Sized {
    fn GetName();
    fn GetFiles();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFiles<Impl: IAppxContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxContentGroupFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupFilesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupFilesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupFilesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxContentGroupFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxContentGroupFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxContentGroupFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxContentGroupMapReaderImpl: Sized {
    fn GetRequiredGroup();
    fn GetAutomaticGroups();
}
impl IAppxContentGroupMapReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupMapReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupMapReaderVtbl {
        unsafe extern "system" fn GetRequiredGroup<Impl: IAppxContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutomaticGroups<Impl: IAppxContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxContentGroupMapWriterImpl: Sized {
    fn AddAutomaticGroup();
    fn AddAutomaticFile();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupMapWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupMapWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupMapWriterVtbl {
        unsafe extern "system" fn AddAutomaticGroup<Impl: IAppxContentGroupMapWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAutomaticFile<Impl: IAppxContentGroupMapWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IAppxContentGroupMapWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxContentGroupsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxContentGroupsEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxContentGroupsEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxContentGroupsEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxContentGroupsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxContentGroupsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxContentGroupsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptedBundleWriterImpl: Sized {
    fn AddPayloadPackageEncrypted();
    fn Close();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedBundleWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedBundleWriterVtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Impl: IAppxEncryptedBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IAppxEncryptedBundleWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptedBundleWriter2Impl: Sized {
    fn AddExternalPackageReference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedBundleWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedBundleWriter2Vtbl {
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxEncryptedBundleWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptedBundleWriter3Impl: Sized {
    fn AddPayloadPackageEncrypted();
    fn AddExternalPackageReference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedBundleWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedBundleWriter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedBundleWriter3Vtbl {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Impl: IAppxEncryptedBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddExternalPackageReference<Impl: IAppxEncryptedBundleWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptedPackageWriterImpl: Sized {
    fn AddPayloadFileEncrypted();
    fn Close();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedPackageWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedPackageWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedPackageWriterVtbl {
        unsafe extern "system" fn AddPayloadFileEncrypted<Impl: IAppxEncryptedPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IAppxEncryptedPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptedPackageWriter2Impl: Sized {
    fn AddPayloadFilesEncrypted();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptedPackageWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptedPackageWriter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptedPackageWriter2Vtbl {
        unsafe extern "system" fn AddPayloadFilesEncrypted<Impl: IAppxEncryptedPackageWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddPayloadFilesEncrypted: AddPayloadFilesEncrypted::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactoryVtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecryptPackage<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncryptedPackageReader<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptBundle<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecryptBundle<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncryptedBundleReader<Impl: IAppxEncryptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptionFactory2Impl: Sized {
    fn CreateEncryptedPackageWriter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactory2Vtbl {
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptionFactory3Impl: Sized {
    fn EncryptPackage();
    fn CreateEncryptedPackageWriter();
    fn EncryptBundle();
    fn CreateEncryptedBundleWriter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactory3Vtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptBundle<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Impl: IAppxEncryptionFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxEncryptionFactory4Impl: Sized {
    fn EncryptPackage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxEncryptionFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxEncryptionFactory4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxEncryptionFactory4Vtbl {
        unsafe extern "system" fn EncryptPackage<Impl: IAppxEncryptionFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EncryptPackage: EncryptPackage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxFactoryImpl: Sized {
    fn CreatePackageWriter();
    fn CreatePackageReader();
    fn CreateManifestReader();
    fn CreateBlockMapReader();
    fn CreateValidatedBlockMapReader();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFactoryVtbl {
        unsafe extern "system" fn CreatePackageWriter<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateManifestReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBlockMapReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateValidatedBlockMapReader<Impl: IAppxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapstream: ::windows::core::RawPtr, signaturefilename: super::super::super::Foundation::PWSTR, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxFactory2Impl: Sized {
    fn CreateContentGroupMapReader();
    fn CreateSourceContentGroupMapReader();
    fn CreateContentGroupMapWriter();
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFactory2Vtbl {
        unsafe extern "system" fn CreateContentGroupMapReader<Impl: IAppxFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, contentgroupmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSourceContentGroupMapReader<Impl: IAppxFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, reader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateContentGroupMapWriter<Impl: IAppxFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contentgroupmapwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxFileImpl: Sized {
    fn GetCompressionOption();
    fn GetContentType();
    fn GetName();
    fn GetSize();
    fn GetStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFileVtbl {
        unsafe extern "system" fn GetCompressionOption<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentType<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IAppxFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxFilesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxFilesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxFilesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxFilesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxFilesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestApplicationImpl: Sized {
    fn GetStringValue();
    fn GetAppUserModelId();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestApplicationVtbl {
        unsafe extern "system" fn GetStringValue<Impl: IAppxManifestApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAppUserModelId<Impl: IAppxManifestApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestApplicationsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestApplicationsEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestApplicationsEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestApplicationsEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestApplicationsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestApplicationsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestApplicationsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestCapabilitiesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestCapabilitiesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestCapabilitiesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestCapabilitiesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestDeviceCapabilitiesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDeviceCapabilitiesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDeviceCapabilitiesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDeviceCapabilitiesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestDriverConstraintImpl: Sized {
    fn GetName();
    fn GetMinVersion();
    fn GetMinDate();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverConstraintVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverConstraintImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverConstraintVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestDriverConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestDriverConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinDate<Impl: IAppxManifestDriverConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mindate: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestDriverConstraintsEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverConstraintsEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverConstraintsEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDriverConstraintsEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestDriverDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestDriverDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestDriverDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestDriverDependencyImpl: Sized {
    fn GetDriverConstraints();
}
impl IAppxManifestDriverDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestDriverDependencyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestDriverDependencyVtbl {
        unsafe extern "system" fn GetDriverConstraints<Impl: IAppxManifestDriverDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDriverConstraints: GetDriverConstraints::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestHostRuntimeDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestHostRuntimeDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestHostRuntimeDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestHostRuntimeDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestHostRuntimeDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetMinVersion();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestHostRuntimeDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependencyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestHostRuntimeDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestHostRuntimeDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestHostRuntimeDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestHostRuntimeDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestHostRuntimeDependency2Impl: Sized {
    fn GetPackageFamilyName();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestHostRuntimeDependency2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestHostRuntimeDependency2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestHostRuntimeDependency2Vtbl {
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestHostRuntimeDependency2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPackageFamilyName: GetPackageFamilyName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestMainPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestMainPackageDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestMainPackageDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestMainPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestMainPackageDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetPackageFamilyName();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestMainPackageDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestMainPackageDependencyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestMainPackageDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestMainPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestMainPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestMainPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestOSPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOSPackageDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestOSPackageDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestOSPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestOSPackageDependencyImpl: Sized {
    fn GetName();
    fn GetVersion();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOSPackageDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOSPackageDependencyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestOSPackageDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestOSPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersion<Impl: IAppxManifestOSPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestOptionalPackageInfoImpl: Sized {
    fn GetIsOptionalPackage();
    fn GetMainPackageName();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestOptionalPackageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestOptionalPackageInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestOptionalPackageInfoVtbl {
        unsafe extern "system" fn GetIsOptionalPackage<Impl: IAppxManifestOptionalPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMainPackageName<Impl: IAppxManifestOptionalPackageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestPackageDependenciesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependenciesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependenciesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestPackageDependenciesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestPackageDependencyImpl: Sized {
    fn GetName();
    fn GetPublisher();
    fn GetMinVersion();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependencyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependencyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependencyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestPackageDependencyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestPackageDependency2Impl: Sized + IAppxManifestPackageDependencyImpl {
    fn GetMaxMajorVersionTested();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependency2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependency2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependency2Vtbl {
        unsafe extern "system" fn GetMaxMajorVersionTested<Impl: IAppxManifestPackageDependency2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxmajorversiontested: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IAppxManifestPackageDependencyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMaxMajorVersionTested: GetMaxMajorVersionTested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependency3Impl: Sized {
    fn GetIsOptional();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageDependency3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageDependency3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageDependency3Vtbl {
        unsafe extern "system" fn GetIsOptional<Impl: IAppxManifestPackageDependency3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIsOptional: GetIsOptional::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageIdVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetArchitecture<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPublisher<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersion<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceId<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComparePublisher<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, other: super::super::super::Foundation::PWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageFullName<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageFamilyName<Impl: IAppxManifestPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestPackageId2Impl: Sized + IAppxManifestPackageIdImpl {
    fn GetArchitecture2();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPackageId2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPackageId2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPackageId2Vtbl {
        unsafe extern "system" fn GetArchitecture2<Impl: IAppxManifestPackageId2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IAppxManifestPackageIdVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetArchitecture2: GetArchitecture2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestPackageId2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPropertiesImpl: Sized {
    fn GetBoolValue();
    fn GetStringValue();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestPropertiesVtbl {
        unsafe extern "system" fn GetBoolValue<Impl: IAppxManifestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringValue<Impl: IAppxManifestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestQualifiedResourceImpl: Sized {
    fn GetLanguage();
    fn GetScale();
    fn GetDXFeatureLevel();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestQualifiedResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestQualifiedResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestQualifiedResourceVtbl {
        unsafe extern "system" fn GetLanguage<Impl: IAppxManifestQualifiedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScale<Impl: IAppxManifestQualifiedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scale: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDXFeatureLevel<Impl: IAppxManifestQualifiedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestQualifiedResourcesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestQualifiedResourcesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestQualifiedResourcesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestQualifiedResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReaderVtbl {
        unsafe extern "system" fn GetPackageId<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageDependencies<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCapabilities<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResources<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceCapabilities<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrerequisite<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplications<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IAppxManifestReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestReader2Impl: Sized + IAppxManifestReaderImpl {
    fn GetQualifiedResources();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader2Vtbl {
        unsafe extern "system" fn GetQualifiedResources<Impl: IAppxManifestReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IAppxManifestReaderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetQualifiedResources: GetQualifiedResources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxManifestReader3Impl: Sized + IAppxManifestReaderImpl + IAppxManifestReader2Impl {
    fn GetCapabilitiesByCapabilityClass();
    fn GetTargetDeviceFamilies();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReader3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader3Vtbl {
        unsafe extern "system" fn GetCapabilitiesByCapabilityClass<Impl: IAppxManifestReader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetDeviceFamilies<Impl: IAppxManifestReader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IAppxManifestReader2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCapabilitiesByCapabilityClass: GetCapabilitiesByCapabilityClass::<Impl, IMPL_OFFSET>,
            GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxManifestReader4Impl: Sized + IAppxManifestReaderImpl + IAppxManifestReader2Impl + IAppxManifestReader3Impl {
    fn GetOptionalPackageInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxManifestReader4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader4Vtbl {
        unsafe extern "system" fn GetOptionalPackageInfo<Impl: IAppxManifestReader4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IAppxManifestReader3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOptionalPackageInfo: GetOptionalPackageInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader4 as ::windows::core::Interface>::IID
    }
}
pub trait IAppxManifestReader5Impl: Sized {
    fn GetMainPackageDependencies();
}
impl IAppxManifestReader5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader5Vtbl {
        unsafe extern "system" fn GetMainPackageDependencies<Impl: IAppxManifestReader5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMainPackageDependencies: GetMainPackageDependencies::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxManifestReader5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestReader6Impl: Sized {
    fn GetIsNonQualifiedResourcePackage();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestReader6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader6Vtbl {
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Impl: IAppxManifestReader6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestReader7Impl: Sized {
    fn GetDriverDependencies();
    fn GetOSPackageDependencies();
    fn GetHostRuntimeDependencies();
}
impl IAppxManifestReader7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestReader7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestReader7Vtbl {
        unsafe extern "system" fn GetDriverDependencies<Impl: IAppxManifestReader7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, driverdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOSPackageDependencies<Impl: IAppxManifestReader7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ospackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHostRuntimeDependencies<Impl: IAppxManifestReader7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostruntimedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestResourcesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestResourcesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestResourcesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestResourcesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestResourcesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestTargetDeviceFamiliesEnumeratorImpl: Sized {
    fn GetCurrent();
    fn GetHasCurrent();
    fn MoveNext();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestTargetDeviceFamiliesEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestTargetDeviceFamiliesEnumeratorVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetdevicefamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHasCurrent<Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IAppxManifestTargetDeviceFamiliesEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxManifestTargetDeviceFamilyImpl: Sized {
    fn GetName();
    fn GetMinVersion();
    fn GetMaxVersionTested();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxManifestTargetDeviceFamilyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxManifestTargetDeviceFamilyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxManifestTargetDeviceFamilyVtbl {
        unsafe extern "system" fn GetName<Impl: IAppxManifestTargetDeviceFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinVersion<Impl: IAppxManifestTargetDeviceFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxVersionTested<Impl: IAppxManifestTargetDeviceFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxversiontested: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxPackageEditorImpl: Sized {
    fn SetWorkingDirectory();
    fn CreateDeltaPackage();
    fn CreateDeltaPackageUsingBaselineBlockMap();
    fn UpdatePackage();
    fn UpdateEncryptedPackage();
    fn UpdatePackageManifest();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxPackageEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageEditorVtbl {
        unsafe extern "system" fn SetWorkingDirectory<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeltaPackage<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: ::windows::core::RawPtr, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeltaPackageUsingBaselineBlockMap<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpackagestream: ::windows::core::RawPtr, baselineblockmapstream: ::windows::core::RawPtr, baselinepackagefullname: super::super::super::Foundation::PWSTR, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdatePackage<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateEncryptedPackage<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineencryptedpackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdatePackageManifest<Impl: IAppxPackageEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestream: ::windows::core::RawPtr, updatedmanifeststream: ::windows::core::RawPtr, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxPackageReaderImpl: Sized {
    fn GetBlockMap();
    fn GetFootprintFile();
    fn GetPayloadFile();
    fn GetPayloadFiles();
    fn GetManifest();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxPackageReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageReaderVtbl {
        unsafe extern "system" fn GetBlockMap<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFootprintFile<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPayloadFile<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPayloadFiles<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetManifest<Impl: IAppxPackageReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxPackageWriterImpl: Sized {
    fn AddPayloadFile();
    fn Close();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxPackageWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageWriterVtbl {
        unsafe extern "system" fn AddPayloadFile<Impl: IAppxPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, contenttype: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IAppxPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxPackageWriter2Impl: Sized {
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageWriter2Vtbl {
        unsafe extern "system" fn Close<Impl: IAppxPackageWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifest: ::windows::core::RawPtr, contentgroupmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Close: Close::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxPackageWriter3Impl: Sized {
    fn AddPayloadFiles();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAppxPackageWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackageWriter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackageWriter3Vtbl {
        unsafe extern "system" fn AddPayloadFiles<Impl: IAppxPackageWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddPayloadFiles: AddPayloadFiles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackageWriter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxPackagingDiagnosticEventSinkImpl: Sized {
    fn ReportContextChange();
    fn ReportError();
}
#[cfg(feature = "Win32_Foundation")]
impl IAppxPackagingDiagnosticEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackagingDiagnosticEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackagingDiagnosticEventSinkVtbl {
        unsafe extern "system" fn ReportContextChange<Impl: IAppxPackagingDiagnosticEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: super::super::super::Foundation::PSTR, contextmessage: super::super::super::Foundation::PWSTR, detailsmessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportError<Impl: IAppxPackagingDiagnosticEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errormessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAppxPackagingDiagnosticEventSinkManagerImpl: Sized {
    fn SetSinkForProcess();
}
impl IAppxPackagingDiagnosticEventSinkManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxPackagingDiagnosticEventSinkManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxPackagingDiagnosticEventSinkManagerVtbl {
        unsafe extern "system" fn SetSinkForProcess<Impl: IAppxPackagingDiagnosticEventSinkManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSinkForProcess: SetSinkForProcess::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSinkManager as ::windows::core::Interface>::IID
    }
}
pub trait IAppxSourceContentGroupMapReaderImpl: Sized {
    fn GetRequiredGroup();
    fn GetAutomaticGroups();
}
impl IAppxSourceContentGroupMapReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppxSourceContentGroupMapReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppxSourceContentGroupMapReaderVtbl {
        unsafe extern "system" fn GetRequiredGroup<Impl: IAppxSourceContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutomaticGroups<Impl: IAppxSourceContentGroupMapReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
