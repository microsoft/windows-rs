pub trait IAssemblyCache_Impl: Sized {
    fn UninstallAssembly(&self, dwflags: u32, pszassemblyname: &::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::Result<()>;
    fn QueryAssemblyInfo(&self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: &::windows::core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::Result<()>;
    fn CreateAssemblyCacheItem(&self, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::core::option::Option<IAssemblyCacheItem>, pszassemblyname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Reserved(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn InstallAssembly(&self, dwflags: u32, pszmanifestfilepath: &::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::Result<()>;
}
impl IAssemblyCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCache_Impl, const OFFSET: isize>() -> IAssemblyCache_Vtbl {
        unsafe extern "system" fn UninstallAssembly<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszassemblyname: ::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UninstallAssembly(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszassemblyname), ::core::mem::transmute_copy(&prefdata), ::core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn QueryAssemblyInfo<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: ::windows::core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryAssemblyInfo(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszassemblyname), ::core::mem::transmute_copy(&pasminfo)).into()
        }
        unsafe extern "system" fn CreateAssemblyCacheItem<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::windows::core::RawPtr, pszassemblyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAssemblyCacheItem(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&ppasmitem), ::core::mem::transmute(&pszassemblyname)).into()
        }
        unsafe extern "system" fn Reserved<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Reserved() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallAssembly<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszmanifestfilepath: ::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallAssembly(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszmanifestfilepath), ::core::mem::transmute_copy(&prefdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            UninstallAssembly: UninstallAssembly::<Identity, Impl, OFFSET>,
            QueryAssemblyInfo: QueryAssemblyInfo::<Identity, Impl, OFFSET>,
            CreateAssemblyCacheItem: CreateAssemblyCacheItem::<Identity, Impl, OFFSET>,
            Reserved: Reserved::<Identity, Impl, OFFSET>,
            InstallAssembly: InstallAssembly::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyCacheItem_Impl: Sized {
    fn CreateStream(&self, dwflags: u32, pszstreamname: &::windows::core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut ::core::option::Option<super::Com::IStream>, pulimaxsize: *mut u64) -> ::windows::core::Result<()>;
    fn Commit(&self, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::Result<()>;
    fn AbortItem(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyCacheItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>() -> IAssemblyCacheItem_Vtbl {
        unsafe extern "system" fn CreateStream<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszstreamname: ::windows::core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut ::windows::core::RawPtr, pulimaxsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateStream(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszstreamname), ::core::mem::transmute_copy(&dwformat), ::core::mem::transmute_copy(&dwformatflags), ::core::mem::transmute_copy(&ppistream), ::core::mem::transmute_copy(&pulimaxsize)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn AbortItem<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbortItem().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            AbortItem: AbortItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyCacheItem as ::windows::core::Interface>::IID
    }
}
pub trait IAssemblyName_Impl: Sized {
    fn SetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::Result<()>;
    fn Finalize(&self) -> ::windows::core::Result<()>;
    fn GetDisplayName(&self, szdisplayname: ::windows::core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::Result<()>;
    fn Reserved(&self, refiid: *const ::windows::core::GUID, punkreserved1: &::core::option::Option<::windows::core::IUnknown>, punkreserved2: &::core::option::Option<::windows::core::IUnknown>, szreserved: &::windows::core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetName(&self, lpcwbuffer: *mut u32, pwzname: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetVersion(&self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::Result<()>;
    fn IsEqual(&self, pname: &::core::option::Option<IAssemblyName>, dwcmpflags: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IAssemblyName>;
}
impl IAssemblyName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>() -> IAssemblyName_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&cbproperty)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&pcbproperty)).into()
        }
        unsafe extern "system" fn Finalize<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finalize().into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdisplayname: ::windows::core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&szdisplayname), ::core::mem::transmute_copy(&pccdisplayname), ::core::mem::transmute_copy(&dwdisplayflags)).into()
        }
        unsafe extern "system" fn Reserved<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, punkreserved1: *mut ::core::ffi::c_void, punkreserved2: *mut ::core::ffi::c_void, szreserved: ::windows::core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reserved(::core::mem::transmute_copy(&refiid), ::core::mem::transmute(&punkreserved1), ::core::mem::transmute(&punkreserved2), ::core::mem::transmute(&szreserved), ::core::mem::transmute_copy(&llreserved), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&cbreserved), ::core::mem::transmute_copy(&ppreserved)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&lpcwbuffer), ::core::mem::transmute_copy(&pwzname)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&pdwversionhi), ::core::mem::transmute_copy(&pdwversionlow)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::RawPtr, dwcmpflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&dwcmpflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            Finalize: Finalize::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            Reserved: Reserved::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyName as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmDependency_Impl: Sized {
    fn Next(&self, cfetch: u32, rgmsmdependencies: *mut ::core::option::Option<IMsmDependency>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMsmDependency>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>() -> IEnumMsmDependency_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmdependencies), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pemsmdependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmError_Impl: Sized {
    fn Next(&self, cfetch: u32, rgmsmerrors: *mut ::core::option::Option<IMsmError>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMsmError>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmError_Impl, const OFFSET: isize>() -> IEnumMsmError_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmerrors), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmerrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pemsmerrors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumMsmString_Impl: Sized {
    fn Next(&self, cfetch: u32, rgbstrstrings: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMsmString>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumMsmString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmString_Impl, const OFFSET: isize>() -> IEnumMsmString_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgbstrstrings), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmstrings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pemsmstrings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmDependencies_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, item: i32) -> ::windows::core::Result<IMsmDependency>;
    fn Count(&self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmDependencies_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependencies_Impl, const OFFSET: isize>() -> IMsmDependencies_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *r#return = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmDependencies as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmDependency_Impl: Sized + super::Com::IDispatch_Impl {
    fn Module(&self, module: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Language(&self, language: *mut i16) -> ::windows::core::Result<()>;
    fn Version(&self, version: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependency_Impl, const OFFSET: isize>() -> IMsmDependency_Vtbl {
        unsafe extern "system" fn Module<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Module(::core::mem::transmute_copy(&module)).into()
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Language(::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&version)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Module: Module::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmDependency as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmError_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&self, errortype: *mut msmErrorType) -> ::windows::core::Result<()>;
    fn Path(&self, errorpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Language(&self, errorlanguage: *mut i16) -> ::windows::core::Result<()>;
    fn DatabaseTable(&self, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DatabaseKeys(&self) -> ::windows::core::Result<IMsmStrings>;
    fn ModuleTable(&self, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ModuleKeys(&self) -> ::windows::core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>() -> IMsmError_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortype: *mut msmErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Type(::core::mem::transmute_copy(&errortype)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Path(::core::mem::transmute_copy(&errorpath)).into()
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorlanguage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Language(::core::mem::transmute_copy(&errorlanguage)).into()
        }
        unsafe extern "system" fn DatabaseTable<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DatabaseTable(::core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn DatabaseKeys<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DatabaseKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *errorkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleTable<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModuleTable(::core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn ModuleKeys<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModuleKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *errorkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            DatabaseTable: DatabaseTable::<Identity, Impl, OFFSET>,
            DatabaseKeys: DatabaseKeys::<Identity, Impl, OFFSET>,
            ModuleTable: ModuleTable::<Identity, Impl, OFFSET>,
            ModuleKeys: ModuleKeys::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmError as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmErrors_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, item: i32) -> ::windows::core::Result<IMsmError>;
    fn Count(&self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmErrors_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmErrors_Impl, const OFFSET: isize>() -> IMsmErrors_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *r#return = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmErrors as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmGetFiles_Impl: Sized + super::Com::IDispatch_Impl {
    fn ModuleFiles(&self) -> ::windows::core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmGetFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmGetFiles_Impl, const OFFSET: isize>() -> IMsmGetFiles_Vtbl {
        unsafe extern "system" fn ModuleFiles<Identity: ::windows::core::IUnknownImpl, Impl: IMsmGetFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModuleFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *files = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ModuleFiles: ModuleFiles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmGetFiles as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmMerge_Impl: Sized + super::Com::IDispatch_Impl {
    fn OpenDatabase(&self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OpenModule(&self, path: &super::super::Foundation::BSTR, language: i16) -> ::windows::core::Result<()>;
    fn CloseDatabase(&self, commit: i16) -> ::windows::core::Result<()>;
    fn CloseModule(&self) -> ::windows::core::Result<()>;
    fn OpenLog(&self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CloseLog(&self) -> ::windows::core::Result<()>;
    fn Log(&self, message: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Errors(&self) -> ::windows::core::Result<IMsmErrors>;
    fn Dependencies(&self) -> ::windows::core::Result<IMsmDependencies>;
    fn Merge(&self, feature: &super::super::Foundation::BSTR, redirectdir: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Connect(&self, feature: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExtractCAB(&self, filename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExtractFiles(&self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmMerge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>() -> IMsmMerge_Vtbl {
        unsafe extern "system" fn OpenDatabase<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenDatabase(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn OpenModule<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, language: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenModule(::core::mem::transmute(&path), ::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn CloseDatabase<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commit: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseDatabase(::core::mem::transmute_copy(&commit)).into()
        }
        unsafe extern "system" fn CloseModule<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseModule().into()
        }
        unsafe extern "system" fn OpenLog<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenLog(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn CloseLog<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseLog().into()
        }
        unsafe extern "system" fn Log<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Log(::core::mem::transmute(&message)).into()
        }
        unsafe extern "system" fn Errors<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Errors() {
                ::core::result::Result::Ok(ok__) => {
                    *errors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *dependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, redirectdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Merge(::core::mem::transmute(&feature), ::core::mem::transmute(&redirectdir)).into()
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute(&feature)).into()
        }
        unsafe extern "system" fn ExtractCAB<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExtractCAB(::core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn ExtractFiles<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExtractFiles(::core::mem::transmute(&path)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenDatabase: OpenDatabase::<Identity, Impl, OFFSET>,
            OpenModule: OpenModule::<Identity, Impl, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, Impl, OFFSET>,
            CloseModule: CloseModule::<Identity, Impl, OFFSET>,
            OpenLog: OpenLog::<Identity, Impl, OFFSET>,
            CloseLog: CloseLog::<Identity, Impl, OFFSET>,
            Log: Log::<Identity, Impl, OFFSET>,
            Errors: Errors::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            ExtractCAB: ExtractCAB::<Identity, Impl, OFFSET>,
            ExtractFiles: ExtractFiles::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmMerge as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmStrings_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, item: i32, r#return: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Count(&self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmStrings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmStrings_Impl, const OFFSET: isize>() -> IMsmStrings_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&r#return)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmStrings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMApplicationInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn InstanceID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OfferID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DefaultTask(&self, pdefaulttask: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppTitle(&self, papptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IconPath(&self, pappiconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NotificationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AppInstallType(&self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn State(&self) -> ::windows::core::Result<PM_APPLICATION_STATE>;
    fn IsRevoked(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn UpdateAvailable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InstallDate(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn IsUninstallable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsThemable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsTrial(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InstallPath(&self, pinstallpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DataRoot(&self, pdataroot: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Genre(&self) -> ::windows::core::Result<PM_APP_GENRE>;
    fn Publisher(&self, ppublisher: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Author(&self, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&self, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppPlatMajorVersion(&self) -> ::windows::core::Result<u8>;
    fn AppPlatMinorVersion(&self) -> ::windows::core::Result<u8>;
    fn PublisherID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsMultiCore(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SID(&self, psid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppPlatMajorVersionLightUp(&self) -> ::windows::core::Result<u8>;
    fn AppPlatMinorVersionLightUp(&self) -> ::windows::core::Result<u8>;
    fn set_UpdateAvailable(&self, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_NotificationState(&self, isnotified: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IconPath(&self, appiconpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn set_UninstallableState(&self, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinableOnKidZone(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOriginallyPreInstalled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsInstallOnSD(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutOnSD(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutBackupRestore(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_EnterpriseDisabled(&self, isdisabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_EnterpriseUninstallable(&self, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnterpriseDisabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EnterpriseUninstallable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsVisibleOnAppList(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsInboxApp(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn StorageID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StartAppBlob(&self, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::Result<()>;
    fn IsMovable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DeploymentAppEnumerationHubFilter(&self) -> ::windows::core::Result<PM_TILE_HUBTYPE>;
    fn ModifiedDate(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn IsOriginallyRestored(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ShouldDeferMdilBind(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsFullyPreInstall(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsMdilMaintenanceNeeded(&self, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_Title(&self, apptitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMApplicationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>() -> IPMApplicationInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceID<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstanceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferID<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pofferid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OfferID() {
                ::core::result::Result::Ok(ok__) => {
                    *pofferid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTask<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaulttask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DefaultTask(::core::mem::transmute_copy(&pdefaulttask)).into()
        }
        unsafe extern "system" fn AppTitle<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AppTitle(::core::mem::transmute_copy(&papptitle)).into()
        }
        unsafe extern "system" fn IconPath<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappiconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IconPath(::core::mem::transmute_copy(&pappiconpath)).into()
        }
        unsafe extern "system" fn NotificationState<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NotificationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pisnotified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInstallType<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppInstallType() {
                ::core::result::Result::Ok(ok__) => {
                    *pappinstalltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRevoked<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRevoked() {
                ::core::result::Result::Ok(ok__) => {
                    *pisrevoked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisupdateavailable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallDate<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstallDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pinstalldate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisuninstallable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsThemable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisthemable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrial<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTrial() {
                ::core::result::Result::Ok(ok__) => {
                    *pistrial = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPath<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallPath(::core::mem::transmute_copy(&pinstallpath)).into()
        }
        unsafe extern "system" fn DataRoot<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DataRoot(::core::mem::transmute_copy(&pdataroot)).into()
        }
        unsafe extern "system" fn Genre<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Genre() {
                ::core::result::Result::Ok(ok__) => {
                    *pgenre = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publisher<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublisher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Publisher(::core::mem::transmute_copy(&ppublisher)).into()
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Author(::core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersion<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppPlatMajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pmajorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersion<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppPlatMinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pminorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublisherid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMultiCore<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMultiCore() {
                ::core::result::Result::Ok(ok__) => {
                    *pismulticore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SID<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersionLightUp<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppPlatMajorVersionLightUp() {
                ::core::result::Result::Ok(ok__) => {
                    *pmajorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersionLightUp<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppPlatMinorVersionLightUp() {
                ::core::result::Result::Ok(ok__) => {
                    *pminorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_UpdateAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_UpdateAvailable(::core::mem::transmute_copy(&isupdateavailable)).into()
        }
        unsafe extern "system" fn set_NotificationState<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_NotificationState(::core::mem::transmute_copy(&isnotified)).into()
        }
        unsafe extern "system" fn set_IconPath<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appiconpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_IconPath(::core::mem::transmute(&appiconpath)).into()
        }
        unsafe extern "system" fn set_UninstallableState<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_UninstallableState(::core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn IsPinableOnKidZone<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPinableOnKidZone() {
                ::core::result::Result::Ok(ok__) => {
                    *pispinable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyPreInstalled<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOriginallyPreInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *pispreinstalled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInstallOnSD<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInstallOnSD() {
                ::core::result::Result::Ok(ok__) => {
                    *pisinstallonsd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutOnSD<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOptoutOnSD() {
                ::core::result::Result::Ok(ok__) => {
                    *pisoptoutonsd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutBackupRestore<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOptoutBackupRestore() {
                ::core::result::Result::Ok(ok__) => {
                    *pisoptoutbackuprestore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_EnterpriseDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_EnterpriseDisabled(::core::mem::transmute_copy(&isdisabled)).into()
        }
        unsafe extern "system" fn set_EnterpriseUninstallable<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_EnterpriseUninstallable(::core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn EnterpriseDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnterpriseDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *isdisabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseUninstallable<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnterpriseUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    *isuninstallable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisibleOnAppList<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsVisibleOnAppList() {
                ::core::result::Result::Ok(ok__) => {
                    *pisvisible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInboxApp<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInboxApp() {
                ::core::result::Result::Ok(ok__) => {
                    *pisinboxapp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageID<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorageid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StorageID() {
                ::core::result::Result::Ok(ok__) => {
                    *pstorageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAppBlob<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartAppBlob(::core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsMovable<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMovable() {
                ::core::result::Result::Ok(ok__) => {
                    *pismovable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeploymentAppEnumerationHubFilter<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeploymentAppEnumerationHubFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *hubtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedDate<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pmodifieddate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyRestored<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOriginallyRestored() {
                ::core::result::Result::Ok(ok__) => {
                    *pisrestored = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldDeferMdilBind<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShouldDeferMdilBind() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdefermdilbind = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullyPreInstall<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFullyPreInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisfullypreinstall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsMdilMaintenanceNeeded<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_IsMdilMaintenanceNeeded(::core::mem::transmute_copy(&fismdilmaintenanceneeded)).into()
        }
        unsafe extern "system" fn set_Title<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apptitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_Title(::core::mem::transmute(&apptitle)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            InstanceID: InstanceID::<Identity, Impl, OFFSET>,
            OfferID: OfferID::<Identity, Impl, OFFSET>,
            DefaultTask: DefaultTask::<Identity, Impl, OFFSET>,
            AppTitle: AppTitle::<Identity, Impl, OFFSET>,
            IconPath: IconPath::<Identity, Impl, OFFSET>,
            NotificationState: NotificationState::<Identity, Impl, OFFSET>,
            AppInstallType: AppInstallType::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            IsRevoked: IsRevoked::<Identity, Impl, OFFSET>,
            UpdateAvailable: UpdateAvailable::<Identity, Impl, OFFSET>,
            InstallDate: InstallDate::<Identity, Impl, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, Impl, OFFSET>,
            IsThemable: IsThemable::<Identity, Impl, OFFSET>,
            IsTrial: IsTrial::<Identity, Impl, OFFSET>,
            InstallPath: InstallPath::<Identity, Impl, OFFSET>,
            DataRoot: DataRoot::<Identity, Impl, OFFSET>,
            Genre: Genre::<Identity, Impl, OFFSET>,
            Publisher: Publisher::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            InvocationInfo: InvocationInfo::<Identity, Impl, OFFSET>,
            AppPlatMajorVersion: AppPlatMajorVersion::<Identity, Impl, OFFSET>,
            AppPlatMinorVersion: AppPlatMinorVersion::<Identity, Impl, OFFSET>,
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            IsMultiCore: IsMultiCore::<Identity, Impl, OFFSET>,
            SID: SID::<Identity, Impl, OFFSET>,
            AppPlatMajorVersionLightUp: AppPlatMajorVersionLightUp::<Identity, Impl, OFFSET>,
            AppPlatMinorVersionLightUp: AppPlatMinorVersionLightUp::<Identity, Impl, OFFSET>,
            set_UpdateAvailable: set_UpdateAvailable::<Identity, Impl, OFFSET>,
            set_NotificationState: set_NotificationState::<Identity, Impl, OFFSET>,
            set_IconPath: set_IconPath::<Identity, Impl, OFFSET>,
            set_UninstallableState: set_UninstallableState::<Identity, Impl, OFFSET>,
            IsPinableOnKidZone: IsPinableOnKidZone::<Identity, Impl, OFFSET>,
            IsOriginallyPreInstalled: IsOriginallyPreInstalled::<Identity, Impl, OFFSET>,
            IsInstallOnSD: IsInstallOnSD::<Identity, Impl, OFFSET>,
            IsOptoutOnSD: IsOptoutOnSD::<Identity, Impl, OFFSET>,
            IsOptoutBackupRestore: IsOptoutBackupRestore::<Identity, Impl, OFFSET>,
            set_EnterpriseDisabled: set_EnterpriseDisabled::<Identity, Impl, OFFSET>,
            set_EnterpriseUninstallable: set_EnterpriseUninstallable::<Identity, Impl, OFFSET>,
            EnterpriseDisabled: EnterpriseDisabled::<Identity, Impl, OFFSET>,
            EnterpriseUninstallable: EnterpriseUninstallable::<Identity, Impl, OFFSET>,
            IsVisibleOnAppList: IsVisibleOnAppList::<Identity, Impl, OFFSET>,
            IsInboxApp: IsInboxApp::<Identity, Impl, OFFSET>,
            StorageID: StorageID::<Identity, Impl, OFFSET>,
            StartAppBlob: StartAppBlob::<Identity, Impl, OFFSET>,
            IsMovable: IsMovable::<Identity, Impl, OFFSET>,
            DeploymentAppEnumerationHubFilter: DeploymentAppEnumerationHubFilter::<Identity, Impl, OFFSET>,
            ModifiedDate: ModifiedDate::<Identity, Impl, OFFSET>,
            IsOriginallyRestored: IsOriginallyRestored::<Identity, Impl, OFFSET>,
            ShouldDeferMdilBind: ShouldDeferMdilBind::<Identity, Impl, OFFSET>,
            IsFullyPreInstall: IsFullyPreInstall::<Identity, Impl, OFFSET>,
            set_IsMdilMaintenanceNeeded: set_IsMdilMaintenanceNeeded::<Identity, Impl, OFFSET>,
            set_Title: set_Title::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMApplicationInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMApplicationInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMApplicationInfo>;
}
impl IPMApplicationInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfoEnumerator_Impl, const OFFSET: isize>() -> IPMApplicationInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMApplicationInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundServiceAgentInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BSAID(&self) -> ::windows::core::Result<u32>;
    fn BGSpecifier(&self, pbgspecifier: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGName(&self, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGSource(&self, pbgsource: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGType(&self, pbgtype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsPeriodic(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsScheduled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsScheduleAllowed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Description(&self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsLaunchOnBoot(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsScheduled(&self, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IsScheduleAllowed(&self, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMBackgroundServiceAgentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>() -> IPMBackgroundServiceAgentInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BSAID<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsaid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BSAID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsaid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGSpecifier<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgspecifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BGSpecifier(::core::mem::transmute_copy(&pbgspecifier)).into()
        }
        unsafe extern "system" fn BGName<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BGName(::core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn BGSource<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BGSource(::core::mem::transmute_copy(&pbgsource)).into()
        }
        unsafe extern "system" fn BGType<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BGType(::core::mem::transmute_copy(&pbgtype)).into()
        }
        unsafe extern "system" fn IsPeriodic<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPeriodic() {
                ::core::result::Result::Ok(ok__) => {
                    *pisperiodic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduled<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsScheduled() {
                ::core::result::Result::Ok(ok__) => {
                    *pisscheduled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduleAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsScheduleAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *pisscheduleallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn IsLaunchOnBoot<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLaunchOnBoot() {
                ::core::result::Result::Ok(ok__) => {
                    *plaunchonboot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsScheduled<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_IsScheduled(::core::mem::transmute_copy(&isscheduled)).into()
        }
        unsafe extern "system" fn set_IsScheduleAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_IsScheduleAllowed(::core::mem::transmute_copy(&isscheduleallowed)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            BSAID: BSAID::<Identity, Impl, OFFSET>,
            BGSpecifier: BGSpecifier::<Identity, Impl, OFFSET>,
            BGName: BGName::<Identity, Impl, OFFSET>,
            BGSource: BGSource::<Identity, Impl, OFFSET>,
            BGType: BGType::<Identity, Impl, OFFSET>,
            IsPeriodic: IsPeriodic::<Identity, Impl, OFFSET>,
            IsScheduled: IsScheduled::<Identity, Impl, OFFSET>,
            IsScheduleAllowed: IsScheduleAllowed::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            IsLaunchOnBoot: IsLaunchOnBoot::<Identity, Impl, OFFSET>,
            set_IsScheduled: set_IsScheduled::<Identity, Impl, OFFSET>,
            set_IsScheduleAllowed: set_IsScheduleAllowed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMBackgroundServiceAgentInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo>;
}
impl IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: isize>() -> IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbsainfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbsainfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundWorkerInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGName(&self, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxStartupLatency(&self) -> ::windows::core::Result<u32>;
    fn ExpectedRuntime(&self) -> ::windows::core::Result<u32>;
    fn IsBootWorker(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMBackgroundWorkerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>() -> IPMBackgroundWorkerInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BGName<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BGName(::core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn MaxStartupLatency<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxstartuplatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxStartupLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxstartuplatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedRuntime<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpectedruntime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpectedRuntime() {
                ::core::result::Result::Ok(ok__) => {
                    *pexpectedruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBootWorker<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsBootWorker() {
                ::core::result::Result::Ok(ok__) => {
                    *pisbootworker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            BGName: BGName::<Identity, Impl, OFFSET>,
            MaxStartupLatency: MaxStartupLatency::<Identity, Impl, OFFSET>,
            ExpectedRuntime: ExpectedRuntime::<Identity, Impl, OFFSET>,
            IsBootWorker: IsBootWorker::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMBackgroundWorkerInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMBackgroundWorkerInfo>;
}
impl IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: isize>() -> IPMBackgroundWorkerInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbwinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbwinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPMDeploymentManager_Impl: Sized {
    fn ReportDownloadBegin(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ReportDownloadProgress(&self, productid: &::windows::core::GUID, usprogress: u16) -> ::windows::core::Result<()>;
    fn ReportDownloadComplete(&self, productid: &::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn BeginInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()>;
    fn BeginUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()>;
    fn BeginDeployPackage(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()>;
    fn BeginUpdateDeployedPackageLegacy(&self, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::Result<()>;
    fn BeginUninstall(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginEnterpriseAppInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()>;
    fn BeginEnterpriseAppUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()>;
    fn BeginUpdateLicense(&self, productid: &::windows::core::GUID, offerid: &::windows::core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallenge(&self, packagepath: &super::super::Foundation::BSTR, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallengeByProductID(&self, productid: &::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallengeByProductID2(&self, productid: &::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()>;
    fn RevokeLicense(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RebindMdilBinaries(&self, productid: &::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RebindAllMdilBinaries(&self, productid: &::windows::core::GUID, instanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegenerateXbf(&self, productid: &::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GenerateXbfForCurrentLocale(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginProvision(&self, productid: &::windows::core::GUID, xmlpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BeginDeprovision(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ReindexSQLCEDatabases(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetApplicationsNeedMaintenance(&self, requiredmaintenanceoperations: u32) -> ::windows::core::Result<u32>;
    fn UpdateChamberProfile(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnterprisePolicyIsApplicationAllowed(&self, productid: &::windows::core::GUID, publishername: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BeginUpdateDeployedPackage(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()>;
    fn ReportRestoreCancelled(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ResolveResourceString(&self, resourcestring: &::windows::core::PCWSTR, presolvedresourcestring: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UpdateCapabilitiesForModernApps(&self) -> ::windows::core::Result<()>;
    fn ReportDownloadStatusUpdate(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginUninstallWithOptions(&self, productid: &::windows::core::GUID, removaloptions: u32) -> ::windows::core::Result<()>;
    fn BindDeferredMdilBinaries(&self) -> ::windows::core::Result<()>;
    fn GenerateXamlLightupXbfForCurrentLocale(&self, packagefamilyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddLicenseForAppx(&self, productid: &::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::Result<()>;
    fn FixJunctionsForAppsOnSDCard(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPMDeploymentManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>() -> IPMDeploymentManager_Vtbl {
        unsafe extern "system" fn ReportDownloadBegin<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportDownloadBegin(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ReportDownloadProgress<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, usprogress: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportDownloadProgress(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&usprogress)).into()
        }
        unsafe extern "system" fn ReportDownloadComplete<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportDownloadComplete(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn BeginInstall<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginInstall(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUpdate(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginDeployPackage<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginDeployPackage(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateDeployedPackageLegacy<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUpdateDeployedPackageLegacy(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUninstall<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUninstall(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppInstall<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginEnterpriseAppInstall(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginEnterpriseAppUpdate(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateLicense<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, offerid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUpdateLicense(::core::mem::transmute(&productid), ::core::mem::transmute(&offerid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallenge<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .GetLicenseChallenge(::core::mem::transmute(&packagepath), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcbchallenge), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue))
                .into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLicenseChallengeByProductID(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID2<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .GetLicenseChallengeByProductID2(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue))
                .into()
        }
        unsafe extern "system" fn RevokeLicense<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RevokeLicense(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn RebindMdilBinaries<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RebindMdilBinaries(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&filenames)).into()
        }
        unsafe extern "system" fn RebindAllMdilBinaries<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, instanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RebindAllMdilBinaries(::core::mem::transmute(&productid), ::core::mem::transmute(&instanceid)).into()
        }
        unsafe extern "system" fn RegenerateXbf<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegenerateXbf(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&assemblypaths)).into()
        }
        unsafe extern "system" fn GenerateXbfForCurrentLocale<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateXbfForCurrentLocale(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginProvision<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, xmlpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginProvision(::core::mem::transmute(&productid), ::core::mem::transmute(&xmlpath)).into()
        }
        unsafe extern "system" fn BeginDeprovision<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginDeprovision(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ReindexSQLCEDatabases<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReindexSQLCEDatabases(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn SetApplicationsNeedMaintenance<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetApplicationsNeedMaintenance(::core::mem::transmute_copy(&requiredmaintenanceoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcapplications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateChamberProfile<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateChamberProfile(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn EnterprisePolicyIsApplicationAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, publishername: ::windows::core::PCWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnterprisePolicyIsApplicationAllowed(::core::mem::transmute(&productid), ::core::mem::transmute(&publishername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pisallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackage<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUpdateDeployedPackage(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn ReportRestoreCancelled<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportRestoreCancelled(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ResolveResourceString<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcestring: ::windows::core::PCWSTR, presolvedresourcestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveResourceString(::core::mem::transmute(&resourcestring), ::core::mem::transmute_copy(&presolvedresourcestring)).into()
        }
        unsafe extern "system" fn UpdateCapabilitiesForModernApps<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateCapabilitiesForModernApps().into()
        }
        unsafe extern "system" fn ReportDownloadStatusUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportDownloadStatusUpdate(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginUninstallWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, removaloptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUninstallWithOptions(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&removaloptions)).into()
        }
        unsafe extern "system" fn BindDeferredMdilBinaries<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindDeferredMdilBinaries().into()
        }
        unsafe extern "system" fn GenerateXamlLightupXbfForCurrentLocale<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateXamlLightupXbfForCurrentLocale(::core::mem::transmute(&packagefamilyname)).into()
        }
        unsafe extern "system" fn AddLicenseForAppx<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLicenseForAppx(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense), ::core::mem::transmute_copy(&pbplayreadyheader), ::core::mem::transmute_copy(&cbplayreadyheader)).into()
        }
        unsafe extern "system" fn FixJunctionsForAppsOnSDCard<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FixJunctionsForAppsOnSDCard().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReportDownloadBegin: ReportDownloadBegin::<Identity, Impl, OFFSET>,
            ReportDownloadProgress: ReportDownloadProgress::<Identity, Impl, OFFSET>,
            ReportDownloadComplete: ReportDownloadComplete::<Identity, Impl, OFFSET>,
            BeginInstall: BeginInstall::<Identity, Impl, OFFSET>,
            BeginUpdate: BeginUpdate::<Identity, Impl, OFFSET>,
            BeginDeployPackage: BeginDeployPackage::<Identity, Impl, OFFSET>,
            BeginUpdateDeployedPackageLegacy: BeginUpdateDeployedPackageLegacy::<Identity, Impl, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, Impl, OFFSET>,
            BeginEnterpriseAppInstall: BeginEnterpriseAppInstall::<Identity, Impl, OFFSET>,
            BeginEnterpriseAppUpdate: BeginEnterpriseAppUpdate::<Identity, Impl, OFFSET>,
            BeginUpdateLicense: BeginUpdateLicense::<Identity, Impl, OFFSET>,
            GetLicenseChallenge: GetLicenseChallenge::<Identity, Impl, OFFSET>,
            GetLicenseChallengeByProductID: GetLicenseChallengeByProductID::<Identity, Impl, OFFSET>,
            GetLicenseChallengeByProductID2: GetLicenseChallengeByProductID2::<Identity, Impl, OFFSET>,
            RevokeLicense: RevokeLicense::<Identity, Impl, OFFSET>,
            RebindMdilBinaries: RebindMdilBinaries::<Identity, Impl, OFFSET>,
            RebindAllMdilBinaries: RebindAllMdilBinaries::<Identity, Impl, OFFSET>,
            RegenerateXbf: RegenerateXbf::<Identity, Impl, OFFSET>,
            GenerateXbfForCurrentLocale: GenerateXbfForCurrentLocale::<Identity, Impl, OFFSET>,
            BeginProvision: BeginProvision::<Identity, Impl, OFFSET>,
            BeginDeprovision: BeginDeprovision::<Identity, Impl, OFFSET>,
            ReindexSQLCEDatabases: ReindexSQLCEDatabases::<Identity, Impl, OFFSET>,
            SetApplicationsNeedMaintenance: SetApplicationsNeedMaintenance::<Identity, Impl, OFFSET>,
            UpdateChamberProfile: UpdateChamberProfile::<Identity, Impl, OFFSET>,
            EnterprisePolicyIsApplicationAllowed: EnterprisePolicyIsApplicationAllowed::<Identity, Impl, OFFSET>,
            BeginUpdateDeployedPackage: BeginUpdateDeployedPackage::<Identity, Impl, OFFSET>,
            ReportRestoreCancelled: ReportRestoreCancelled::<Identity, Impl, OFFSET>,
            ResolveResourceString: ResolveResourceString::<Identity, Impl, OFFSET>,
            UpdateCapabilitiesForModernApps: UpdateCapabilitiesForModernApps::<Identity, Impl, OFFSET>,
            ReportDownloadStatusUpdate: ReportDownloadStatusUpdate::<Identity, Impl, OFFSET>,
            BeginUninstallWithOptions: BeginUninstallWithOptions::<Identity, Impl, OFFSET>,
            BindDeferredMdilBinaries: BindDeferredMdilBinaries::<Identity, Impl, OFFSET>,
            GenerateXamlLightupXbfForCurrentLocale: GenerateXamlLightupXbfForCurrentLocale::<Identity, Impl, OFFSET>,
            AddLicenseForAppx: AddLicenseForAppx::<Identity, Impl, OFFSET>,
            FixJunctionsForAppsOnSDCard: FixJunctionsForAppsOnSDCard::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMDeploymentManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMEnumerationManager_Impl: Sized {
    fn AllApplications(&self, ppappenum: *mut ::core::option::Option<IPMApplicationInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllTiles(&self, pptileenum: *mut ::core::option::Option<IPMTileInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllTasks(&self, pptaskenum: *mut ::core::option::Option<IPMTaskInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllExtensions(&self, ppextensionenum: *mut ::core::option::Option<IPMExtensionInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllBackgroundServiceAgents(&self, ppbsaenum: *mut ::core::option::Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllBackgroundWorkers(&self, ppbswenum: *mut ::core::option::Option<IPMBackgroundWorkerInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn ApplicationInfo(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo>;
    fn TileInfo(&self, productid: &::windows::core::GUID, tileid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPMTileInfo>;
    fn TaskInfo(&self, productid: &::windows::core::GUID, taskid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPMTaskInfo>;
    fn TaskInfoEx(&self, productid: &::windows::core::GUID, taskid: &::windows::core::PCWSTR) -> ::windows::core::Result<IPMTaskInfo>;
    fn BackgroundServiceAgentInfo(&self, bsaid: u32) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo>;
    fn AllLiveTileJobs(&self) -> ::windows::core::Result<IPMLiveTileJobInfoEnumerator>;
    fn LiveTileJob(&self, productid: &::windows::core::GUID, tileid: &super::super::Foundation::BSTR, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> ::windows::core::Result<IPMLiveTileJobInfo>;
    fn ApplicationInfoExternal(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo>;
    fn FileHandlerGenericLogo(&self, filetype: &super::super::Foundation::BSTR, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationInfoFromAccessClaims(&self, sysappid0: &super::super::Foundation::BSTR, sysappid1: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPMApplicationInfo>;
    fn StartTileEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()>;
    fn StartAppEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMEnumerationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>() -> IPMEnumerationManager_Vtbl {
        unsafe extern "system" fn AllApplications<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllApplications(::core::mem::transmute_copy(&ppappenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn AllTiles<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptileenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllTiles(::core::mem::transmute_copy(&pptileenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn AllTasks<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllTasks(::core::mem::transmute_copy(&pptaskenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn AllExtensions<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensionenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllExtensions(::core::mem::transmute_copy(&ppextensionenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn AllBackgroundServiceAgents<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbsaenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllBackgroundServiceAgents(::core::mem::transmute_copy(&ppbsaenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn AllBackgroundWorkers<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbswenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllBackgroundWorkers(::core::mem::transmute_copy(&ppbswenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn ApplicationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationInfo(::core::mem::transmute(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptileinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TileInfo(::core::mem::transmute(&productid), ::core::mem::transmute(&tileid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptileinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskInfo(::core::mem::transmute(&productid), ::core::mem::transmute(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskInfoEx<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::windows::core::PCWSTR, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskInfoEx(::core::mem::transmute(&productid), ::core::mem::transmute(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundServiceAgentInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsaid: u32, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackgroundServiceAgentInfo(::core::mem::transmute_copy(&bsaid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllLiveTileJobs<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplivetilejobenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllLiveTileJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pplivetilejobenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LiveTileJob<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LiveTileJob(::core::mem::transmute(&productid), ::core::mem::transmute(&tileid), ::core::mem::transmute_copy(&recurrencetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplivetilejobinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationInfoExternal<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationInfoExternal(::core::mem::transmute(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileHandlerGenericLogo<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FileHandlerGenericLogo(::core::mem::transmute(&filetype), ::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn ApplicationInfoFromAccessClaims<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sysappid0: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sysappid1: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationInfoFromAccessClaims(::core::mem::transmute(&sysappid0), ::core::mem::transmute(&sysappid1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTileEnumeratorBlob<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartTileEnumeratorBlob(::core::mem::transmute(&filter), ::core::mem::transmute_copy(&pctiles), ::core::mem::transmute_copy(&pptileblobs)).into()
        }
        unsafe extern "system" fn StartAppEnumeratorBlob<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartAppEnumeratorBlob(::core::mem::transmute(&filter), ::core::mem::transmute_copy(&pcapps), ::core::mem::transmute_copy(&ppappblobs)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllApplications: AllApplications::<Identity, Impl, OFFSET>,
            AllTiles: AllTiles::<Identity, Impl, OFFSET>,
            AllTasks: AllTasks::<Identity, Impl, OFFSET>,
            AllExtensions: AllExtensions::<Identity, Impl, OFFSET>,
            AllBackgroundServiceAgents: AllBackgroundServiceAgents::<Identity, Impl, OFFSET>,
            AllBackgroundWorkers: AllBackgroundWorkers::<Identity, Impl, OFFSET>,
            ApplicationInfo: ApplicationInfo::<Identity, Impl, OFFSET>,
            TileInfo: TileInfo::<Identity, Impl, OFFSET>,
            TaskInfo: TaskInfo::<Identity, Impl, OFFSET>,
            TaskInfoEx: TaskInfoEx::<Identity, Impl, OFFSET>,
            BackgroundServiceAgentInfo: BackgroundServiceAgentInfo::<Identity, Impl, OFFSET>,
            AllLiveTileJobs: AllLiveTileJobs::<Identity, Impl, OFFSET>,
            LiveTileJob: LiveTileJob::<Identity, Impl, OFFSET>,
            ApplicationInfoExternal: ApplicationInfoExternal::<Identity, Impl, OFFSET>,
            FileHandlerGenericLogo: FileHandlerGenericLogo::<Identity, Impl, OFFSET>,
            ApplicationInfoFromAccessClaims: ApplicationInfoFromAccessClaims::<Identity, Impl, OFFSET>,
            StartTileEnumeratorBlob: StartTileEnumeratorBlob::<Identity, Impl, OFFSET>,
            StartAppEnumeratorBlob: StartAppEnumeratorBlob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMEnumerationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionCachedFileUpdaterInfo_Impl: Sized {
    fn SupportsUpdates(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: isize>() -> IPMExtensionCachedFileUpdaterInfo_Vtbl {
        unsafe extern "system" fn SupportsUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportsUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsupdates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SupportsUpdates: SupportsUpdates::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionCachedFileUpdaterInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionContractInfo_Impl: Sized {
    fn InvocationInfo(&self, paumid: *mut super::super::Foundation::BSTR, pargs: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionContractInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionContractInfo_Impl, const OFFSET: isize>() -> IPMExtensionContractInfo_Vtbl {
        unsafe extern "system" fn InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionContractInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paumid: *mut super::super::Foundation::BSTR, pargs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&paumid), ::core::mem::transmute_copy(&pargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), InvocationInfo: InvocationInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionContractInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileExtensionInfo_Impl: Sized {
    fn Name(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayName(&self, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Logo(&self, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ContentType(&self, filetype: &super::super::Foundation::BSTR, pcontenttype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FileType(&self, contenttype: &super::super::Foundation::BSTR, pfiletype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllFileTypes(&self, pcbtypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileExtensionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>() -> IPMExtensionFileExtensionInfo_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayName(::core::mem::transmute_copy(&pdisplayname)).into()
        }
        unsafe extern "system" fn Logo<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Logo(::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn ContentType<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontenttype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContentType(::core::mem::transmute(&filetype), ::core::mem::transmute_copy(&pcontenttype)).into()
        }
        unsafe extern "system" fn FileType<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfiletype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FileType(::core::mem::transmute(&contenttype), ::core::mem::transmute_copy(&pfiletype)).into()
        }
        unsafe extern "system" fn InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn AllFileTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pcbtypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Logo: Logo::<Identity, Impl, OFFSET>,
            ContentType: ContentType::<Identity, Impl, OFFSET>,
            FileType: FileType::<Identity, Impl, OFFSET>,
            InvocationInfo: InvocationInfo::<Identity, Impl, OFFSET>,
            AllFileTypes: AllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileExtensionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileOpenPickerInfo_Impl: Sized {
    fn AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileOpenPickerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>() -> IPMExtensionFileOpenPickerInfo_Vtbl {
        unsafe extern "system" fn AllFileTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsalltypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllFileTypes: AllFileTypes::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileOpenPickerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileSavePickerInfo_Impl: Sized {
    fn AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileSavePickerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>() -> IPMExtensionFileSavePickerInfo_Vtbl {
        unsafe extern "system" fn AllFileTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsalltypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllFileTypes: AllFileTypes::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileSavePickerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionInfo_Impl: Sized {
    fn SupplierPID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupplierTaskID(&self, psuppliertid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Title(&self, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IconPath(&self, piconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExtraFile(&self, pfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>() -> IPMExtensionInfo_Vtbl {
        unsafe extern "system" fn SupplierPID<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupplierpid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupplierPID() {
                ::core::result::Result::Ok(ok__) => {
                    *psupplierpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupplierTaskID<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psuppliertid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SupplierTaskID(::core::mem::transmute_copy(&psuppliertid)).into()
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Title(::core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn IconPath<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IconPath(::core::mem::transmute_copy(&piconpath)).into()
        }
        unsafe extern "system" fn ExtraFile<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExtraFile(::core::mem::transmute_copy(&pfilepath)).into()
        }
        unsafe extern "system" fn InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SupplierPID: SupplierPID::<Identity, Impl, OFFSET>,
            SupplierTaskID: SupplierTaskID::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            IconPath: IconPath::<Identity, Impl, OFFSET>,
            ExtraFile: ExtraFile::<Identity, Impl, OFFSET>,
            InvocationInfo: InvocationInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMExtensionInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMExtensionInfo>;
}
impl IPMExtensionInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfoEnumerator_Impl, const OFFSET: isize>() -> IPMExtensionInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppextensioninfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionProtocolInfo_Impl: Sized {
    fn Protocol(&self, pprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionProtocolInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>() -> IPMExtensionProtocolInfo_Vtbl {
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Protocol(::core::mem::transmute_copy(&pprotocol)).into()
        }
        unsafe extern "system" fn InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            InvocationInfo: InvocationInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionProtocolInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionShareTargetInfo_Impl: Sized {
    fn AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllDataFormats(&self, pcdataformats: *mut u32, ppdataformats: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionShareTargetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>() -> IPMExtensionShareTargetInfo_Vtbl {
        unsafe extern "system" fn AllFileTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn AllDataFormats<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllDataFormats(::core::mem::transmute_copy(&pcdataformats), ::core::mem::transmute_copy(&ppdataformats)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsalltypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllFileTypes: AllFileTypes::<Identity, Impl, OFFSET>,
            AllDataFormats: AllDataFormats::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionShareTargetInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMLiveTileJobInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TileID(&self, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NextSchedule(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn set_NextSchedule(&self, ftnextschedule: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn StartSchedule(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn set_StartSchedule(&self, ftstartschedule: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn IntervalDuration(&self) -> ::windows::core::Result<u32>;
    fn set_IntervalDuration(&self, ulintervalduration: u32) -> ::windows::core::Result<()>;
    fn RunForever(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_RunForever(&self, frunforever: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MaxRunCount(&self) -> ::windows::core::Result<u32>;
    fn set_MaxRunCount(&self, ulmaxruncount: u32) -> ::windows::core::Result<()>;
    fn RunCount(&self) -> ::windows::core::Result<u32>;
    fn set_RunCount(&self, ulruncount: u32) -> ::windows::core::Result<()>;
    fn RecurrenceType(&self) -> ::windows::core::Result<u32>;
    fn set_RecurrenceType(&self, ulrecurrencetype: u32) -> ::windows::core::Result<()>;
    fn TileXML(&self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::Result<()>;
    fn set_TileXML(&self, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::Result<()>;
    fn UrlXML(&self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::Result<()>;
    fn set_UrlXML(&self, purlxml: *const u8, cburlxml: u32) -> ::windows::core::Result<()>;
    fn AttemptCount(&self) -> ::windows::core::Result<u32>;
    fn set_AttemptCount(&self, ulattemptcount: u32) -> ::windows::core::Result<()>;
    fn DownloadState(&self) -> ::windows::core::Result<u32>;
    fn set_DownloadState(&self, uldownloadstate: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMLiveTileJobInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>() -> IPMLiveTileJobInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TileID(::core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn NextSchedule<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    *pnextschedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_NextSchedule<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_NextSchedule(::core::mem::transmute(&ftnextschedule)).into()
        }
        unsafe extern "system" fn StartSchedule<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    *pstartschedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_StartSchedule<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_StartSchedule(::core::mem::transmute(&ftstartschedule)).into()
        }
        unsafe extern "system" fn IntervalDuration<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintervalduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IntervalDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pintervalduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IntervalDuration<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulintervalduration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_IntervalDuration(::core::mem::transmute_copy(&ulintervalduration)).into()
        }
        unsafe extern "system" fn RunForever<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RunForever() {
                ::core::result::Result::Ok(ok__) => {
                    *isrunforever = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunForever<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_RunForever(::core::mem::transmute_copy(&frunforever)).into()
        }
        unsafe extern "system" fn MaxRunCount<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxRunCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxruncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_MaxRunCount<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_MaxRunCount(::core::mem::transmute_copy(&ulmaxruncount)).into()
        }
        unsafe extern "system" fn RunCount<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RunCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pruncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunCount<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_RunCount(::core::mem::transmute_copy(&ulruncount)).into()
        }
        unsafe extern "system" fn RecurrenceType<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precurrencetype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecurrenceType() {
                ::core::result::Result::Ok(ok__) => {
                    *precurrencetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RecurrenceType<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecurrencetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_RecurrenceType(::core::mem::transmute_copy(&ulrecurrencetype)).into()
        }
        unsafe extern "system" fn TileXML<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TileXML(::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&pcbtilexml)).into()
        }
        unsafe extern "system" fn set_TileXML<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_TileXML(::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&cbtilexml)).into()
        }
        unsafe extern "system" fn UrlXML<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UrlXML(::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&pcburlxml)).into()
        }
        unsafe extern "system" fn set_UrlXML<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_UrlXML(::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&cburlxml)).into()
        }
        unsafe extern "system" fn AttemptCount<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattemptcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttemptCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pattemptcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_AttemptCount<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulattemptcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_AttemptCount(::core::mem::transmute_copy(&ulattemptcount)).into()
        }
        unsafe extern "system" fn DownloadState<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdownloadstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdownloadstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_DownloadState<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uldownloadstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_DownloadState(::core::mem::transmute_copy(&uldownloadstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TileID: TileID::<Identity, Impl, OFFSET>,
            NextSchedule: NextSchedule::<Identity, Impl, OFFSET>,
            set_NextSchedule: set_NextSchedule::<Identity, Impl, OFFSET>,
            StartSchedule: StartSchedule::<Identity, Impl, OFFSET>,
            set_StartSchedule: set_StartSchedule::<Identity, Impl, OFFSET>,
            IntervalDuration: IntervalDuration::<Identity, Impl, OFFSET>,
            set_IntervalDuration: set_IntervalDuration::<Identity, Impl, OFFSET>,
            RunForever: RunForever::<Identity, Impl, OFFSET>,
            set_RunForever: set_RunForever::<Identity, Impl, OFFSET>,
            MaxRunCount: MaxRunCount::<Identity, Impl, OFFSET>,
            set_MaxRunCount: set_MaxRunCount::<Identity, Impl, OFFSET>,
            RunCount: RunCount::<Identity, Impl, OFFSET>,
            set_RunCount: set_RunCount::<Identity, Impl, OFFSET>,
            RecurrenceType: RecurrenceType::<Identity, Impl, OFFSET>,
            set_RecurrenceType: set_RecurrenceType::<Identity, Impl, OFFSET>,
            TileXML: TileXML::<Identity, Impl, OFFSET>,
            set_TileXML: set_TileXML::<Identity, Impl, OFFSET>,
            UrlXML: UrlXML::<Identity, Impl, OFFSET>,
            set_UrlXML: set_UrlXML::<Identity, Impl, OFFSET>,
            AttemptCount: AttemptCount::<Identity, Impl, OFFSET>,
            set_AttemptCount: set_AttemptCount::<Identity, Impl, OFFSET>,
            DownloadState: DownloadState::<Identity, Impl, OFFSET>,
            set_DownloadState: set_DownloadState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMLiveTileJobInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMLiveTileJobInfo>;
}
impl IPMLiveTileJobInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: isize>() -> IPMLiveTileJobInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplivetilejobinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pplivetilejobinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTaskInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NavigationPage(&self, pnavigationpage: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskTransition(&self) -> ::windows::core::Result<PM_TASK_TRANSITION>;
    fn RuntimeType(&self) -> ::windows::core::Result<PACKMAN_RUNTIME>;
    fn ActivationPolicy(&self) -> ::windows::core::Result<PM_ACTIVATION_POLICY>;
    fn TaskType(&self) -> ::windows::core::Result<PM_TASK_TYPE>;
    fn InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImagePath(&self, pimagepath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImageParams(&self, pimageparams: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InstallRootFolder(&self, pinstallrootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DataRootFolder(&self, pdatarootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsSingleInstanceHost(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsInteropEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ApplicationState(&self) -> ::windows::core::Result<PM_APPLICATION_STATE>;
    fn InstallType(&self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn Version(&self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::Result<()>;
    fn BitsPerPixel(&self) -> ::windows::core::Result<u16>;
    fn SuppressesDehydration(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BackgroundExecutionAbilities(&self, pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsOptedForExtendedMem(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMTaskInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>() -> IPMTaskInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn NavigationPage<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnavigationpage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NavigationPage(::core::mem::transmute_copy(&pnavigationpage)).into()
        }
        unsafe extern "system" fn TaskTransition<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *ptasktransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuntimeType<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RuntimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *pruntimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActivationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pactivationpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskType<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptasktype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn ImagePath<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimagepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImagePath(::core::mem::transmute_copy(&pimagepath)).into()
        }
        unsafe extern "system" fn ImageParams<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageparams: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImageParams(::core::mem::transmute_copy(&pimageparams)).into()
        }
        unsafe extern "system" fn InstallRootFolder<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallrootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallRootFolder(::core::mem::transmute_copy(&pinstallrootfolder)).into()
        }
        unsafe extern "system" fn DataRootFolder<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatarootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DataRootFolder(::core::mem::transmute_copy(&pdatarootfolder)).into()
        }
        unsafe extern "system" fn IsSingleInstanceHost<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSingleInstanceHost() {
                ::core::result::Result::Ok(ok__) => {
                    *pissingleinstancehost = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInteropEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInteropEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pisinteropenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationState<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationState() {
                ::core::result::Result::Ok(ok__) => {
                    *papplicationstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallType<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstallType() {
                ::core::result::Result::Ok(ok__) => {
                    *pinstalltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&ptargetmajorversion), ::core::mem::transmute_copy(&ptargetminorversion)).into()
        }
        unsafe extern "system" fn BitsPerPixel<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitsperpixel: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BitsPerPixel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbitsperpixel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressesDehydration<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SuppressesDehydration() {
                ::core::result::Result::Ok(ok__) => {
                    *psuppressesdehydration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundExecutionAbilities<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackgroundExecutionAbilities(::core::mem::transmute_copy(&pbackgroundexecutionabilities)).into()
        }
        unsafe extern "system" fn IsOptedForExtendedMem<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOptedForExtendedMem() {
                ::core::result::Result::Ok(ok__) => {
                    *pisoptedin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            NavigationPage: NavigationPage::<Identity, Impl, OFFSET>,
            TaskTransition: TaskTransition::<Identity, Impl, OFFSET>,
            RuntimeType: RuntimeType::<Identity, Impl, OFFSET>,
            ActivationPolicy: ActivationPolicy::<Identity, Impl, OFFSET>,
            TaskType: TaskType::<Identity, Impl, OFFSET>,
            InvocationInfo: InvocationInfo::<Identity, Impl, OFFSET>,
            ImagePath: ImagePath::<Identity, Impl, OFFSET>,
            ImageParams: ImageParams::<Identity, Impl, OFFSET>,
            InstallRootFolder: InstallRootFolder::<Identity, Impl, OFFSET>,
            DataRootFolder: DataRootFolder::<Identity, Impl, OFFSET>,
            IsSingleInstanceHost: IsSingleInstanceHost::<Identity, Impl, OFFSET>,
            IsInteropEnabled: IsInteropEnabled::<Identity, Impl, OFFSET>,
            ApplicationState: ApplicationState::<Identity, Impl, OFFSET>,
            InstallType: InstallType::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            BitsPerPixel: BitsPerPixel::<Identity, Impl, OFFSET>,
            SuppressesDehydration: SuppressesDehydration::<Identity, Impl, OFFSET>,
            BackgroundExecutionAbilities: BackgroundExecutionAbilities::<Identity, Impl, OFFSET>,
            IsOptedForExtendedMem: IsOptedForExtendedMem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTaskInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMTaskInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMTaskInfo>;
}
impl IPMTaskInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfoEnumerator_Impl, const OFFSET: isize>() -> IPMTaskInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTaskInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTileInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TileID(&self, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TemplateType(&self) -> ::windows::core::Result<TILE_TEMPLATE_TYPE>;
    fn HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<u32>;
    fn IsNotified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TileType(&self) -> ::windows::core::Result<PM_STARTTILE_TYPE>;
    fn IsThemable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PropertyById(&self, propid: u32) -> ::windows::core::Result<IPMTilePropertyInfo>;
    fn InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PropertyEnum(&self) -> ::windows::core::Result<IPMTilePropertyEnumerator>;
    fn HubTileSize(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<PM_TILE_SIZE>;
    fn set_HubPosition(&self, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::Result<()>;
    fn set_NotifiedState(&self, notified: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::Result<()>;
    fn set_InvocationInfo(&self, taskname: &super::super::Foundation::BSTR, taskparameters: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartTileBlob(&self, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()>;
    fn IsRestoring(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsAutoRestoreDisabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsRestoring(&self, restoring: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IsAutoRestoreDisabled(&self, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMTileInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>() -> IPMTileInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TileID(::core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn TemplateType<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TemplateType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptemplatetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubPinnedState<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HubPinnedState(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubPosition<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HubPosition(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotified<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsNotified() {
                ::core::result::Result::Ok(ok__) => {
                    *pisnotified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *pisdefault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn TileType<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TileType() {
                ::core::result::Result::Ok(ok__) => {
                    *pstarttiletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsThemable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisthemable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyById<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, pppropinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyById(::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn PropertyEnum<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptilepropenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pptilepropenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubTileSize<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HubTileSize(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_HubPosition<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_HubPosition(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn set_NotifiedState<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_NotifiedState(::core::mem::transmute_copy(&notified)).into()
        }
        unsafe extern "system" fn set_HubPinnedState<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_HubPinnedState(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&pinned)).into()
        }
        unsafe extern "system" fn set_HubTileSize<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_HubTileSize(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn set_InvocationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_InvocationInfo(::core::mem::transmute(&taskname), ::core::mem::transmute(&taskparameters)).into()
        }
        unsafe extern "system" fn StartTileBlob<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartTileBlob(::core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsRestoring<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRestoring() {
                ::core::result::Result::Ok(ok__) => {
                    *pisrestoring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAutoRestoreDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAutoRestoreDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pisautorestoredisabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsRestoring<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_IsRestoring(::core::mem::transmute_copy(&restoring)).into()
        }
        unsafe extern "system" fn set_IsAutoRestoreDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_IsAutoRestoreDisabled(::core::mem::transmute_copy(&autorestoredisabled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TileID: TileID::<Identity, Impl, OFFSET>,
            TemplateType: TemplateType::<Identity, Impl, OFFSET>,
            HubPinnedState: HubPinnedState::<Identity, Impl, OFFSET>,
            HubPosition: HubPosition::<Identity, Impl, OFFSET>,
            IsNotified: IsNotified::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            TileType: TileType::<Identity, Impl, OFFSET>,
            IsThemable: IsThemable::<Identity, Impl, OFFSET>,
            PropertyById: PropertyById::<Identity, Impl, OFFSET>,
            InvocationInfo: InvocationInfo::<Identity, Impl, OFFSET>,
            PropertyEnum: PropertyEnum::<Identity, Impl, OFFSET>,
            HubTileSize: HubTileSize::<Identity, Impl, OFFSET>,
            set_HubPosition: set_HubPosition::<Identity, Impl, OFFSET>,
            set_NotifiedState: set_NotifiedState::<Identity, Impl, OFFSET>,
            set_HubPinnedState: set_HubPinnedState::<Identity, Impl, OFFSET>,
            set_HubTileSize: set_HubTileSize::<Identity, Impl, OFFSET>,
            set_InvocationInfo: set_InvocationInfo::<Identity, Impl, OFFSET>,
            StartTileBlob: StartTileBlob::<Identity, Impl, OFFSET>,
            IsRestoring: IsRestoring::<Identity, Impl, OFFSET>,
            IsAutoRestoreDisabled: IsAutoRestoreDisabled::<Identity, Impl, OFFSET>,
            set_IsRestoring: set_IsRestoring::<Identity, Impl, OFFSET>,
            set_IsAutoRestoreDisabled: set_IsAutoRestoreDisabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTileInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMTileInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMTileInfo>;
}
impl IPMTileInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfoEnumerator_Impl, const OFFSET: isize>() -> IPMTileInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptileinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pptileinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTileInfoEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IPMTilePropertyEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMTilePropertyInfo>;
}
impl IPMTilePropertyEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyEnumerator_Impl, const OFFSET: isize>() -> IPMTilePropertyEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTilePropertyEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTilePropertyInfo_Impl: Sized {
    fn PropertyID(&self) -> ::windows::core::Result<u32>;
    fn PropertyValue(&self, ppropvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn set_Property(&self, propvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMTilePropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>() -> IPMTilePropertyInfo_Vtbl {
        unsafe extern "system" fn PropertyID<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppropid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PropertyValue(::core::mem::transmute_copy(&ppropvalue)).into()
        }
        unsafe extern "system" fn set_Property<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).set_Property(::core::mem::transmute(&propvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PropertyID: PropertyID::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            set_Property: set_Property::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTilePropertyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IValidate_Impl: Sized {
    fn OpenDatabase(&self, szdatabase: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OpenCUB(&self, szcubfile: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn CloseDatabase(&self) -> ::windows::core::Result<()>;
    fn CloseCUB(&self) -> ::windows::core::Result<()>;
    fn SetDisplay(&self, pdisplayfunction: &LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetStatus(&self, pstatusfunction: &LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Validate(&self, wzices: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IValidate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>() -> IValidate_Vtbl {
        unsafe extern "system" fn OpenDatabase<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdatabase: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenDatabase(::core::mem::transmute(&szdatabase)).into()
        }
        unsafe extern "system" fn OpenCUB<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcubfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenCUB(::core::mem::transmute(&szcubfile)).into()
        }
        unsafe extern "system" fn CloseDatabase<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseDatabase().into()
        }
        unsafe extern "system" fn CloseCUB<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseCUB().into()
        }
        unsafe extern "system" fn SetDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayfunction: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplay(::core::mem::transmute(&pdisplayfunction), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatusfunction: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatus(::core::mem::transmute(&pstatusfunction), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzices: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Validate(::core::mem::transmute(&wzices)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OpenDatabase: OpenDatabase::<Identity, Impl, OFFSET>,
            OpenCUB: OpenCUB::<Identity, Impl, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, Impl, OFFSET>,
            CloseCUB: CloseCUB::<Identity, Impl, OFFSET>,
            SetDisplay: SetDisplay::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValidate as ::windows::core::Interface>::IID
    }
}
