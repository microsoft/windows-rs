#[cfg(feature = "Win32_Foundation")]
pub trait IAssemblyCache_Impl: Sized {
    fn UninstallAssembly(&mut self, dwflags: u32, pszassemblyname: super::super::Foundation::PWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::Result<()>;
    fn QueryAssemblyInfo(&mut self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: super::super::Foundation::PWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::Result<()>;
    fn CreateAssemblyCacheItem(&mut self, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::core::option::Option<IAssemblyCacheItem>, pszassemblyname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Reserved(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn InstallAssembly(&mut self, dwflags: u32, pszmanifestfilepath: super::super::Foundation::PWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAssemblyCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCache_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssemblyCache_Vtbl {
        unsafe extern "system" fn UninstallAssembly<Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszassemblyname: super::super::Foundation::PWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UninstallAssembly(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszassemblyname), ::core::mem::transmute_copy(&prefdata), ::core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn QueryAssemblyInfo<Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: super::super::Foundation::PWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryAssemblyInfo(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszassemblyname), ::core::mem::transmute_copy(&pasminfo)).into()
        }
        unsafe extern "system" fn CreateAssemblyCacheItem<Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::windows::core::RawPtr, pszassemblyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateAssemblyCacheItem(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&ppasmitem), ::core::mem::transmute_copy(&pszassemblyname)).into()
        }
        unsafe extern "system" fn Reserved<Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reserved() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallAssembly<Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszmanifestfilepath: super::super::Foundation::PWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallAssembly(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszmanifestfilepath), ::core::mem::transmute_copy(&prefdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            UninstallAssembly: UninstallAssembly::<Impl, IMPL_OFFSET>,
            QueryAssemblyInfo: QueryAssemblyInfo::<Impl, IMPL_OFFSET>,
            CreateAssemblyCacheItem: CreateAssemblyCacheItem::<Impl, IMPL_OFFSET>,
            Reserved: Reserved::<Impl, IMPL_OFFSET>,
            InstallAssembly: InstallAssembly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyCache as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAssemblyCacheItem_Impl: Sized {
    fn CreateStream(&mut self, dwflags: u32, pszstreamname: super::super::Foundation::PWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut ::core::option::Option<super::Com::IStream>, pulimaxsize: *mut u64) -> ::windows::core::Result<()>;
    fn Commit(&mut self, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::Result<()>;
    fn AbortItem(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAssemblyCacheItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyCacheItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssemblyCacheItem_Vtbl {
        unsafe extern "system" fn CreateStream<Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszstreamname: super::super::Foundation::PWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut ::windows::core::RawPtr, pulimaxsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateStream(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszstreamname), ::core::mem::transmute_copy(&dwformat), ::core::mem::transmute_copy(&dwformatflags), ::core::mem::transmute_copy(&ppistream), ::core::mem::transmute_copy(&pulimaxsize)).into()
        }
        unsafe extern "system" fn Commit<Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn AbortItem<Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortItem().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateStream: CreateStream::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            AbortItem: AbortItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyCacheItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAssemblyName_Impl: Sized {
    fn SetProperty(&mut self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::Result<()>;
    fn Finalize(&mut self) -> ::windows::core::Result<()>;
    fn GetDisplayName(&mut self, szdisplayname: super::super::Foundation::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::Result<()>;
    fn Reserved(&mut self, refiid: *const ::windows::core::GUID, punkreserved1: &::core::option::Option<::windows::core::IUnknown>, punkreserved2: &::core::option::Option<::windows::core::IUnknown>, szreserved: super::super::Foundation::PWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetName(&mut self, lpcwbuffer: *mut u32, pwzname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetVersion(&mut self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::Result<()>;
    fn IsEqual(&mut self, pname: &::core::option::Option<IAssemblyName>, dwcmpflags: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IAssemblyName>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAssemblyName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyName_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssemblyName_Vtbl {
        unsafe extern "system" fn SetProperty<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&cbproperty)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&pcbproperty)).into()
        }
        unsafe extern "system" fn Finalize<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finalize().into()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdisplayname: super::super::Foundation::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&szdisplayname), ::core::mem::transmute_copy(&pccdisplayname), ::core::mem::transmute_copy(&dwdisplayflags)).into()
        }
        unsafe extern "system" fn Reserved<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, punkreserved1: *mut ::core::ffi::c_void, punkreserved2: *mut ::core::ffi::c_void, szreserved: super::super::Foundation::PWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved(::core::mem::transmute_copy(&refiid), ::core::mem::transmute(&punkreserved1), ::core::mem::transmute(&punkreserved2), ::core::mem::transmute_copy(&szreserved), ::core::mem::transmute_copy(&llreserved), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&cbreserved), ::core::mem::transmute_copy(&ppreserved)).into()
        }
        unsafe extern "system" fn GetName<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&lpcwbuffer), ::core::mem::transmute_copy(&pwzname)).into()
        }
        unsafe extern "system" fn GetVersion<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&pdwversionhi), ::core::mem::transmute_copy(&pdwversionlow)).into()
        }
        unsafe extern "system" fn IsEqual<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::RawPtr, dwcmpflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&dwcmpflags)).into()
        }
        unsafe extern "system" fn Clone<Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            Finalize: Finalize::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            Reserved: Reserved::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyName as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmDependency_Impl: Sized {
    fn Next(&mut self, cfetch: u32, rgmsmdependencies: *mut ::core::option::Option<IMsmDependency>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumMsmDependency>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmDependency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumMsmDependency_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmdependencies), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pemsmdependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmDependency as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmError_Impl: Sized {
    fn Next(&mut self, cfetch: u32, rgmsmerrors: *mut ::core::option::Option<IMsmError>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumMsmError>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumMsmError_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmerrors), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmerrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pemsmerrors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumMsmString_Impl: Sized {
    fn Next(&mut self, cfetch: u32, rgbstrstrings: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumMsmString>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumMsmString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMsmString_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumMsmString_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgbstrstrings), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmstrings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pemsmstrings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmDependencies_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, item: i32) -> ::windows::core::Result<IMsmDependency>;
    fn Count(&mut self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmDependencies_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependencies_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMsmDependencies_Vtbl {
        unsafe extern "system" fn Item<Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *r#return = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmDependencies as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmDependency_Impl: Sized + super::Com::IDispatch_Impl {
    fn Module(&mut self, module: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Language(&mut self, language: *mut i16) -> ::windows::core::Result<()>;
    fn Version(&mut self, version: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmDependency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMsmDependency_Vtbl {
        unsafe extern "system" fn Module<Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Module(::core::mem::transmute_copy(&module)).into()
        }
        unsafe extern "system" fn Language<Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Language(::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn Version<Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&version)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Module: Module::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmDependency as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmError_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&mut self, errortype: *mut msmErrorType) -> ::windows::core::Result<()>;
    fn Path(&mut self, errorpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Language(&mut self, errorlanguage: *mut i16) -> ::windows::core::Result<()>;
    fn DatabaseTable(&mut self, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DatabaseKeys(&mut self) -> ::windows::core::Result<IMsmStrings>;
    fn ModuleTable(&mut self, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ModuleKeys(&mut self) -> ::windows::core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMsmError_Vtbl {
        unsafe extern "system" fn Type<Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortype: *mut msmErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Type(::core::mem::transmute_copy(&errortype)).into()
        }
        unsafe extern "system" fn Path<Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Path(::core::mem::transmute_copy(&errorpath)).into()
        }
        unsafe extern "system" fn Language<Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorlanguage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Language(::core::mem::transmute_copy(&errorlanguage)).into()
        }
        unsafe extern "system" fn DatabaseTable<Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DatabaseTable(::core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn DatabaseKeys<Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DatabaseKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *errorkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleTable<Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModuleTable(::core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn ModuleKeys<Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *errorkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            DatabaseTable: DatabaseTable::<Impl, IMPL_OFFSET>,
            DatabaseKeys: DatabaseKeys::<Impl, IMPL_OFFSET>,
            ModuleTable: ModuleTable::<Impl, IMPL_OFFSET>,
            ModuleKeys: ModuleKeys::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmError as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmErrors_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, item: i32) -> ::windows::core::Result<IMsmError>;
    fn Count(&mut self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmErrors_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmErrors_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMsmErrors_Vtbl {
        unsafe extern "system" fn Item<Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *r#return = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmErrors as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmGetFiles_Impl: Sized + super::Com::IDispatch_Impl {
    fn ModuleFiles(&mut self) -> ::windows::core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmGetFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmGetFiles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMsmGetFiles_Vtbl {
        unsafe extern "system" fn ModuleFiles<Impl: IMsmGetFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *files = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ModuleFiles: ModuleFiles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmGetFiles as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmMerge_Impl: Sized + super::Com::IDispatch_Impl {
    fn OpenDatabase(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OpenModule(&mut self, path: &super::super::Foundation::BSTR, language: i16) -> ::windows::core::Result<()>;
    fn CloseDatabase(&mut self, commit: i16) -> ::windows::core::Result<()>;
    fn CloseModule(&mut self) -> ::windows::core::Result<()>;
    fn OpenLog(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CloseLog(&mut self) -> ::windows::core::Result<()>;
    fn Log(&mut self, message: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Errors(&mut self) -> ::windows::core::Result<IMsmErrors>;
    fn Dependencies(&mut self) -> ::windows::core::Result<IMsmDependencies>;
    fn Merge(&mut self, feature: &super::super::Foundation::BSTR, redirectdir: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Connect(&mut self, feature: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExtractCAB(&mut self, filename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExtractFiles(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmMerge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmMerge_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMsmMerge_Vtbl {
        unsafe extern "system" fn OpenDatabase<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenDatabase(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn OpenModule<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, language: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenModule(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn CloseDatabase<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commit: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseDatabase(::core::mem::transmute_copy(&commit)).into()
        }
        unsafe extern "system" fn CloseModule<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseModule().into()
        }
        unsafe extern "system" fn OpenLog<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenLog(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn CloseLog<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseLog().into()
        }
        unsafe extern "system" fn Log<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Log(::core::mem::transmute_copy(&message)).into()
        }
        unsafe extern "system" fn Errors<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Errors() {
                ::core::result::Result::Ok(ok__) => {
                    *errors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *dependencies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, redirectdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Merge(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&redirectdir)).into()
        }
        unsafe extern "system" fn Connect<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&feature)).into()
        }
        unsafe extern "system" fn ExtractCAB<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExtractCAB(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn ExtractFiles<Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExtractFiles(::core::mem::transmute_copy(&path)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OpenDatabase: OpenDatabase::<Impl, IMPL_OFFSET>,
            OpenModule: OpenModule::<Impl, IMPL_OFFSET>,
            CloseDatabase: CloseDatabase::<Impl, IMPL_OFFSET>,
            CloseModule: CloseModule::<Impl, IMPL_OFFSET>,
            OpenLog: OpenLog::<Impl, IMPL_OFFSET>,
            CloseLog: CloseLog::<Impl, IMPL_OFFSET>,
            Log: Log::<Impl, IMPL_OFFSET>,
            Errors: Errors::<Impl, IMPL_OFFSET>,
            Dependencies: Dependencies::<Impl, IMPL_OFFSET>,
            Merge: Merge::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            ExtractCAB: ExtractCAB::<Impl, IMPL_OFFSET>,
            ExtractFiles: ExtractFiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmMerge as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmStrings_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, item: i32, r#return: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Count(&mut self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmStrings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsmStrings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMsmStrings_Vtbl {
        unsafe extern "system" fn Item<Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&r#return)).into()
        }
        unsafe extern "system" fn Count<Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmStrings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMApplicationInfo_Impl: Sized {
    fn ProductID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn InstanceID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OfferID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DefaultTask(&mut self, pdefaulttask: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppTitle(&mut self, papptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IconPath(&mut self, pappiconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NotificationState(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AppInstallType(&mut self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn State(&mut self) -> ::windows::core::Result<PM_APPLICATION_STATE>;
    fn IsRevoked(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn UpdateAvailable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InstallDate(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn IsUninstallable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsThemable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsTrial(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InstallPath(&mut self, pinstallpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DataRoot(&mut self, pdataroot: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Genre(&mut self) -> ::windows::core::Result<PM_APP_GENRE>;
    fn Publisher(&mut self, ppublisher: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Author(&mut self, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&mut self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppPlatMajorVersion(&mut self) -> ::windows::core::Result<u8>;
    fn AppPlatMinorVersion(&mut self) -> ::windows::core::Result<u8>;
    fn PublisherID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsMultiCore(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SID(&mut self, psid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppPlatMajorVersionLightUp(&mut self) -> ::windows::core::Result<u8>;
    fn AppPlatMinorVersionLightUp(&mut self) -> ::windows::core::Result<u8>;
    fn set_UpdateAvailable(&mut self, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_NotificationState(&mut self, isnotified: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IconPath(&mut self, appiconpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn set_UninstallableState(&mut self, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinableOnKidZone(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOriginallyPreInstalled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsInstallOnSD(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutOnSD(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutBackupRestore(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_EnterpriseDisabled(&mut self, isdisabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_EnterpriseUninstallable(&mut self, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnterpriseDisabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EnterpriseUninstallable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsVisibleOnAppList(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsInboxApp(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn StorageID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StartAppBlob(&mut self, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::Result<()>;
    fn IsMovable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DeploymentAppEnumerationHubFilter(&mut self) -> ::windows::core::Result<PM_TILE_HUBTYPE>;
    fn ModifiedDate(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn IsOriginallyRestored(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ShouldDeferMdilBind(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsFullyPreInstall(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsMdilMaintenanceNeeded(&mut self, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_Title(&mut self, apptitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMApplicationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMApplicationInfo_Vtbl {
        unsafe extern "system" fn ProductID<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceID<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferID<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pofferid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferID() {
                ::core::result::Result::Ok(ok__) => {
                    *pofferid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTask<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaulttask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DefaultTask(::core::mem::transmute_copy(&pdefaulttask)).into()
        }
        unsafe extern "system" fn AppTitle<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppTitle(::core::mem::transmute_copy(&papptitle)).into()
        }
        unsafe extern "system" fn IconPath<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappiconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IconPath(::core::mem::transmute_copy(&pappiconpath)).into()
        }
        unsafe extern "system" fn NotificationState<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pisnotified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInstallType<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppInstallType() {
                ::core::result::Result::Ok(ok__) => {
                    *pappinstalltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRevoked<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRevoked() {
                ::core::result::Result::Ok(ok__) => {
                    *pisrevoked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAvailable<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisupdateavailable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallDate<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pinstalldate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisuninstallable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThemable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisthemable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrial<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTrial() {
                ::core::result::Result::Ok(ok__) => {
                    *pistrial = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPath<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallPath(::core::mem::transmute_copy(&pinstallpath)).into()
        }
        unsafe extern "system" fn DataRoot<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DataRoot(::core::mem::transmute_copy(&pdataroot)).into()
        }
        unsafe extern "system" fn Genre<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Genre() {
                ::core::result::Result::Ok(ok__) => {
                    *pgenre = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publisher<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublisher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Publisher(::core::mem::transmute_copy(&ppublisher)).into()
        }
        unsafe extern "system" fn Author<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Author(::core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn Description<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn Version<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersion<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppPlatMajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pmajorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersion<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppPlatMinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pminorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherID<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublisherid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMultiCore<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMultiCore() {
                ::core::result::Result::Ok(ok__) => {
                    *pismulticore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SID<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersionLightUp<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppPlatMajorVersionLightUp() {
                ::core::result::Result::Ok(ok__) => {
                    *pmajorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersionLightUp<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppPlatMinorVersionLightUp() {
                ::core::result::Result::Ok(ok__) => {
                    *pminorver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_UpdateAvailable<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_UpdateAvailable(::core::mem::transmute_copy(&isupdateavailable)).into()
        }
        unsafe extern "system" fn set_NotificationState<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_NotificationState(::core::mem::transmute_copy(&isnotified)).into()
        }
        unsafe extern "system" fn set_IconPath<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appiconpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_IconPath(::core::mem::transmute_copy(&appiconpath)).into()
        }
        unsafe extern "system" fn set_UninstallableState<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_UninstallableState(::core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn IsPinableOnKidZone<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinableOnKidZone() {
                ::core::result::Result::Ok(ok__) => {
                    *pispinable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyPreInstalled<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOriginallyPreInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *pispreinstalled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInstallOnSD<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInstallOnSD() {
                ::core::result::Result::Ok(ok__) => {
                    *pisinstallonsd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutOnSD<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOptoutOnSD() {
                ::core::result::Result::Ok(ok__) => {
                    *pisoptoutonsd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutBackupRestore<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOptoutBackupRestore() {
                ::core::result::Result::Ok(ok__) => {
                    *pisoptoutbackuprestore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_EnterpriseDisabled<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_EnterpriseDisabled(::core::mem::transmute_copy(&isdisabled)).into()
        }
        unsafe extern "system" fn set_EnterpriseUninstallable<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_EnterpriseUninstallable(::core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn EnterpriseDisabled<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *isdisabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseUninstallable<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    *isuninstallable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisibleOnAppList<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisibleOnAppList() {
                ::core::result::Result::Ok(ok__) => {
                    *pisvisible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInboxApp<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInboxApp() {
                ::core::result::Result::Ok(ok__) => {
                    *pisinboxapp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageID<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorageid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageID() {
                ::core::result::Result::Ok(ok__) => {
                    *pstorageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAppBlob<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAppBlob(::core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsMovable<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMovable() {
                ::core::result::Result::Ok(ok__) => {
                    *pismovable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeploymentAppEnumerationHubFilter<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeploymentAppEnumerationHubFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *hubtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedDate<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pmodifieddate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyRestored<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOriginallyRestored() {
                ::core::result::Result::Ok(ok__) => {
                    *pisrestored = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldDeferMdilBind<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldDeferMdilBind() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdefermdilbind = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullyPreInstall<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFullyPreInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisfullypreinstall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsMdilMaintenanceNeeded<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_IsMdilMaintenanceNeeded(::core::mem::transmute_copy(&fismdilmaintenanceneeded)).into()
        }
        unsafe extern "system" fn set_Title<Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apptitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_Title(::core::mem::transmute_copy(&apptitle)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProductID: ProductID::<Impl, IMPL_OFFSET>,
            InstanceID: InstanceID::<Impl, IMPL_OFFSET>,
            OfferID: OfferID::<Impl, IMPL_OFFSET>,
            DefaultTask: DefaultTask::<Impl, IMPL_OFFSET>,
            AppTitle: AppTitle::<Impl, IMPL_OFFSET>,
            IconPath: IconPath::<Impl, IMPL_OFFSET>,
            NotificationState: NotificationState::<Impl, IMPL_OFFSET>,
            AppInstallType: AppInstallType::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            IsRevoked: IsRevoked::<Impl, IMPL_OFFSET>,
            UpdateAvailable: UpdateAvailable::<Impl, IMPL_OFFSET>,
            InstallDate: InstallDate::<Impl, IMPL_OFFSET>,
            IsUninstallable: IsUninstallable::<Impl, IMPL_OFFSET>,
            IsThemable: IsThemable::<Impl, IMPL_OFFSET>,
            IsTrial: IsTrial::<Impl, IMPL_OFFSET>,
            InstallPath: InstallPath::<Impl, IMPL_OFFSET>,
            DataRoot: DataRoot::<Impl, IMPL_OFFSET>,
            Genre: Genre::<Impl, IMPL_OFFSET>,
            Publisher: Publisher::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            InvocationInfo: InvocationInfo::<Impl, IMPL_OFFSET>,
            AppPlatMajorVersion: AppPlatMajorVersion::<Impl, IMPL_OFFSET>,
            AppPlatMinorVersion: AppPlatMinorVersion::<Impl, IMPL_OFFSET>,
            PublisherID: PublisherID::<Impl, IMPL_OFFSET>,
            IsMultiCore: IsMultiCore::<Impl, IMPL_OFFSET>,
            SID: SID::<Impl, IMPL_OFFSET>,
            AppPlatMajorVersionLightUp: AppPlatMajorVersionLightUp::<Impl, IMPL_OFFSET>,
            AppPlatMinorVersionLightUp: AppPlatMinorVersionLightUp::<Impl, IMPL_OFFSET>,
            set_UpdateAvailable: set_UpdateAvailable::<Impl, IMPL_OFFSET>,
            set_NotificationState: set_NotificationState::<Impl, IMPL_OFFSET>,
            set_IconPath: set_IconPath::<Impl, IMPL_OFFSET>,
            set_UninstallableState: set_UninstallableState::<Impl, IMPL_OFFSET>,
            IsPinableOnKidZone: IsPinableOnKidZone::<Impl, IMPL_OFFSET>,
            IsOriginallyPreInstalled: IsOriginallyPreInstalled::<Impl, IMPL_OFFSET>,
            IsInstallOnSD: IsInstallOnSD::<Impl, IMPL_OFFSET>,
            IsOptoutOnSD: IsOptoutOnSD::<Impl, IMPL_OFFSET>,
            IsOptoutBackupRestore: IsOptoutBackupRestore::<Impl, IMPL_OFFSET>,
            set_EnterpriseDisabled: set_EnterpriseDisabled::<Impl, IMPL_OFFSET>,
            set_EnterpriseUninstallable: set_EnterpriseUninstallable::<Impl, IMPL_OFFSET>,
            EnterpriseDisabled: EnterpriseDisabled::<Impl, IMPL_OFFSET>,
            EnterpriseUninstallable: EnterpriseUninstallable::<Impl, IMPL_OFFSET>,
            IsVisibleOnAppList: IsVisibleOnAppList::<Impl, IMPL_OFFSET>,
            IsInboxApp: IsInboxApp::<Impl, IMPL_OFFSET>,
            StorageID: StorageID::<Impl, IMPL_OFFSET>,
            StartAppBlob: StartAppBlob::<Impl, IMPL_OFFSET>,
            IsMovable: IsMovable::<Impl, IMPL_OFFSET>,
            DeploymentAppEnumerationHubFilter: DeploymentAppEnumerationHubFilter::<Impl, IMPL_OFFSET>,
            ModifiedDate: ModifiedDate::<Impl, IMPL_OFFSET>,
            IsOriginallyRestored: IsOriginallyRestored::<Impl, IMPL_OFFSET>,
            ShouldDeferMdilBind: ShouldDeferMdilBind::<Impl, IMPL_OFFSET>,
            IsFullyPreInstall: IsFullyPreInstall::<Impl, IMPL_OFFSET>,
            set_IsMdilMaintenanceNeeded: set_IsMdilMaintenanceNeeded::<Impl, IMPL_OFFSET>,
            set_Title: set_Title::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMApplicationInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMApplicationInfoEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMApplicationInfo>;
}
impl IPMApplicationInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMApplicationInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMApplicationInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMApplicationInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMApplicationInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundServiceAgentInfo_Impl: Sized {
    fn ProductID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&mut self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BSAID(&mut self) -> ::windows::core::Result<u32>;
    fn BGSpecifier(&mut self, pbgspecifier: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGName(&mut self, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGSource(&mut self, pbgsource: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGType(&mut self, pbgtype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsPeriodic(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsScheduled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsScheduleAllowed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Description(&mut self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsLaunchOnBoot(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsScheduled(&mut self, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IsScheduleAllowed(&mut self, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMBackgroundServiceAgentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMBackgroundServiceAgentInfo_Vtbl {
        unsafe extern "system" fn ProductID<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BSAID<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsaid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BSAID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsaid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGSpecifier<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgspecifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BGSpecifier(::core::mem::transmute_copy(&pbgspecifier)).into()
        }
        unsafe extern "system" fn BGName<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BGName(::core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn BGSource<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BGSource(::core::mem::transmute_copy(&pbgsource)).into()
        }
        unsafe extern "system" fn BGType<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BGType(::core::mem::transmute_copy(&pbgtype)).into()
        }
        unsafe extern "system" fn IsPeriodic<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPeriodic() {
                ::core::result::Result::Ok(ok__) => {
                    *pisperiodic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduled<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScheduled() {
                ::core::result::Result::Ok(ok__) => {
                    *pisscheduled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduleAllowed<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScheduleAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *pisscheduleallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn IsLaunchOnBoot<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLaunchOnBoot() {
                ::core::result::Result::Ok(ok__) => {
                    *plaunchonboot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsScheduled<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_IsScheduled(::core::mem::transmute_copy(&isscheduled)).into()
        }
        unsafe extern "system" fn set_IsScheduleAllowed<Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_IsScheduleAllowed(::core::mem::transmute_copy(&isscheduleallowed)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProductID: ProductID::<Impl, IMPL_OFFSET>,
            TaskID: TaskID::<Impl, IMPL_OFFSET>,
            BSAID: BSAID::<Impl, IMPL_OFFSET>,
            BGSpecifier: BGSpecifier::<Impl, IMPL_OFFSET>,
            BGName: BGName::<Impl, IMPL_OFFSET>,
            BGSource: BGSource::<Impl, IMPL_OFFSET>,
            BGType: BGType::<Impl, IMPL_OFFSET>,
            IsPeriodic: IsPeriodic::<Impl, IMPL_OFFSET>,
            IsScheduled: IsScheduled::<Impl, IMPL_OFFSET>,
            IsScheduleAllowed: IsScheduleAllowed::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            IsLaunchOnBoot: IsLaunchOnBoot::<Impl, IMPL_OFFSET>,
            set_IsScheduled: set_IsScheduled::<Impl, IMPL_OFFSET>,
            set_IsScheduleAllowed: set_IsScheduleAllowed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMBackgroundServiceAgentInfoEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo>;
}
impl IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbsainfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbsainfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundWorkerInfo_Impl: Sized {
    fn ProductID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&mut self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BGName(&mut self, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxStartupLatency(&mut self) -> ::windows::core::Result<u32>;
    fn ExpectedRuntime(&mut self) -> ::windows::core::Result<u32>;
    fn IsBootWorker(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMBackgroundWorkerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMBackgroundWorkerInfo_Vtbl {
        unsafe extern "system" fn ProductID<Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BGName<Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BGName(::core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn MaxStartupLatency<Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxstartuplatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxStartupLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxstartuplatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedRuntime<Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpectedruntime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpectedRuntime() {
                ::core::result::Result::Ok(ok__) => {
                    *pexpectedruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBootWorker<Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBootWorker() {
                ::core::result::Result::Ok(ok__) => {
                    *pisbootworker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProductID: ProductID::<Impl, IMPL_OFFSET>,
            TaskID: TaskID::<Impl, IMPL_OFFSET>,
            BGName: BGName::<Impl, IMPL_OFFSET>,
            MaxStartupLatency: MaxStartupLatency::<Impl, IMPL_OFFSET>,
            ExpectedRuntime: ExpectedRuntime::<Impl, IMPL_OFFSET>,
            IsBootWorker: IsBootWorker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMBackgroundWorkerInfoEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMBackgroundWorkerInfo>;
}
impl IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMBackgroundWorkerInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbwinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbwinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPMDeploymentManager_Impl: Sized {
    fn ReportDownloadBegin(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ReportDownloadProgress(&mut self, productid: &::windows::core::GUID, usprogress: u16) -> ::windows::core::Result<()>;
    fn ReportDownloadComplete(&mut self, productid: &::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn BeginInstall(&mut self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()>;
    fn BeginUpdate(&mut self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()>;
    fn BeginDeployPackage(&mut self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()>;
    fn BeginUpdateDeployedPackageLegacy(&mut self, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::Result<()>;
    fn BeginUninstall(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginEnterpriseAppInstall(&mut self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()>;
    fn BeginEnterpriseAppUpdate(&mut self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()>;
    fn BeginUpdateLicense(&mut self, productid: &::windows::core::GUID, offerid: &::windows::core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallenge(&mut self, packagepath: &super::super::Foundation::BSTR, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallengeByProductID(&mut self, productid: &::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallengeByProductID2(&mut self, productid: &::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()>;
    fn RevokeLicense(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RebindMdilBinaries(&mut self, productid: &::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RebindAllMdilBinaries(&mut self, productid: &::windows::core::GUID, instanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegenerateXbf(&mut self, productid: &::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GenerateXbfForCurrentLocale(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginProvision(&mut self, productid: &::windows::core::GUID, xmlpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BeginDeprovision(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ReindexSQLCEDatabases(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetApplicationsNeedMaintenance(&mut self, requiredmaintenanceoperations: u32) -> ::windows::core::Result<u32>;
    fn UpdateChamberProfile(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnterprisePolicyIsApplicationAllowed(&mut self, productid: &::windows::core::GUID, publishername: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BeginUpdateDeployedPackage(&mut self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()>;
    fn ReportRestoreCancelled(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ResolveResourceString(&mut self, resourcestring: super::super::Foundation::PWSTR, presolvedresourcestring: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UpdateCapabilitiesForModernApps(&mut self) -> ::windows::core::Result<()>;
    fn ReportDownloadStatusUpdate(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginUninstallWithOptions(&mut self, productid: &::windows::core::GUID, removaloptions: u32) -> ::windows::core::Result<()>;
    fn BindDeferredMdilBinaries(&mut self) -> ::windows::core::Result<()>;
    fn GenerateXamlLightupXbfForCurrentLocale(&mut self, packagefamilyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddLicenseForAppx(&mut self, productid: &::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::Result<()>;
    fn FixJunctionsForAppsOnSDCard(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPMDeploymentManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMDeploymentManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMDeploymentManager_Vtbl {
        unsafe extern "system" fn ReportDownloadBegin<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportDownloadBegin(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn ReportDownloadProgress<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, usprogress: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportDownloadProgress(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&usprogress)).into()
        }
        unsafe extern "system" fn ReportDownloadComplete<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportDownloadComplete(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn BeginInstall<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginInstall(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdate<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUpdate(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginDeployPackage<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginDeployPackage(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateDeployedPackageLegacy<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUpdateDeployedPackageLegacy(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUninstall<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUninstall(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppInstall<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEnterpriseAppInstall(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppUpdate<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEnterpriseAppUpdate(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateLicense<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, offerid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUpdateLicense(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&offerid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallenge<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .GetLicenseChallenge(::core::mem::transmute_copy(&packagepath), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcbchallenge), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue))
                .into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLicenseChallengeByProductID(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID2<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .GetLicenseChallengeByProductID2(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue))
                .into()
        }
        unsafe extern "system" fn RevokeLicense<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeLicense(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn RebindMdilBinaries<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RebindMdilBinaries(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&filenames)).into()
        }
        unsafe extern "system" fn RebindAllMdilBinaries<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, instanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RebindAllMdilBinaries(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&instanceid)).into()
        }
        unsafe extern "system" fn RegenerateXbf<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegenerateXbf(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&assemblypaths)).into()
        }
        unsafe extern "system" fn GenerateXbfForCurrentLocale<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateXbfForCurrentLocale(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn BeginProvision<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, xmlpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginProvision(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&xmlpath)).into()
        }
        unsafe extern "system" fn BeginDeprovision<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginDeprovision(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn ReindexSQLCEDatabases<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReindexSQLCEDatabases(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn SetApplicationsNeedMaintenance<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationsNeedMaintenance(::core::mem::transmute_copy(&requiredmaintenanceoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcapplications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateChamberProfile<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateChamberProfile(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn EnterprisePolicyIsApplicationAllowed<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, publishername: super::super::Foundation::PWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterprisePolicyIsApplicationAllowed(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&publishername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pisallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackage<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUpdateDeployedPackage(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn ReportRestoreCancelled<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportRestoreCancelled(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn ResolveResourceString<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcestring: super::super::Foundation::PWSTR, presolvedresourcestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveResourceString(::core::mem::transmute_copy(&resourcestring), ::core::mem::transmute_copy(&presolvedresourcestring)).into()
        }
        unsafe extern "system" fn UpdateCapabilitiesForModernApps<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateCapabilitiesForModernApps().into()
        }
        unsafe extern "system" fn ReportDownloadStatusUpdate<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportDownloadStatusUpdate(::core::mem::transmute_copy(&productid)).into()
        }
        unsafe extern "system" fn BeginUninstallWithOptions<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, removaloptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUninstallWithOptions(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&removaloptions)).into()
        }
        unsafe extern "system" fn BindDeferredMdilBinaries<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindDeferredMdilBinaries().into()
        }
        unsafe extern "system" fn GenerateXamlLightupXbfForCurrentLocale<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateXamlLightupXbfForCurrentLocale(::core::mem::transmute_copy(&packagefamilyname)).into()
        }
        unsafe extern "system" fn AddLicenseForAppx<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLicenseForAppx(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense), ::core::mem::transmute_copy(&pbplayreadyheader), ::core::mem::transmute_copy(&cbplayreadyheader)).into()
        }
        unsafe extern "system" fn FixJunctionsForAppsOnSDCard<Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FixJunctionsForAppsOnSDCard().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReportDownloadBegin: ReportDownloadBegin::<Impl, IMPL_OFFSET>,
            ReportDownloadProgress: ReportDownloadProgress::<Impl, IMPL_OFFSET>,
            ReportDownloadComplete: ReportDownloadComplete::<Impl, IMPL_OFFSET>,
            BeginInstall: BeginInstall::<Impl, IMPL_OFFSET>,
            BeginUpdate: BeginUpdate::<Impl, IMPL_OFFSET>,
            BeginDeployPackage: BeginDeployPackage::<Impl, IMPL_OFFSET>,
            BeginUpdateDeployedPackageLegacy: BeginUpdateDeployedPackageLegacy::<Impl, IMPL_OFFSET>,
            BeginUninstall: BeginUninstall::<Impl, IMPL_OFFSET>,
            BeginEnterpriseAppInstall: BeginEnterpriseAppInstall::<Impl, IMPL_OFFSET>,
            BeginEnterpriseAppUpdate: BeginEnterpriseAppUpdate::<Impl, IMPL_OFFSET>,
            BeginUpdateLicense: BeginUpdateLicense::<Impl, IMPL_OFFSET>,
            GetLicenseChallenge: GetLicenseChallenge::<Impl, IMPL_OFFSET>,
            GetLicenseChallengeByProductID: GetLicenseChallengeByProductID::<Impl, IMPL_OFFSET>,
            GetLicenseChallengeByProductID2: GetLicenseChallengeByProductID2::<Impl, IMPL_OFFSET>,
            RevokeLicense: RevokeLicense::<Impl, IMPL_OFFSET>,
            RebindMdilBinaries: RebindMdilBinaries::<Impl, IMPL_OFFSET>,
            RebindAllMdilBinaries: RebindAllMdilBinaries::<Impl, IMPL_OFFSET>,
            RegenerateXbf: RegenerateXbf::<Impl, IMPL_OFFSET>,
            GenerateXbfForCurrentLocale: GenerateXbfForCurrentLocale::<Impl, IMPL_OFFSET>,
            BeginProvision: BeginProvision::<Impl, IMPL_OFFSET>,
            BeginDeprovision: BeginDeprovision::<Impl, IMPL_OFFSET>,
            ReindexSQLCEDatabases: ReindexSQLCEDatabases::<Impl, IMPL_OFFSET>,
            SetApplicationsNeedMaintenance: SetApplicationsNeedMaintenance::<Impl, IMPL_OFFSET>,
            UpdateChamberProfile: UpdateChamberProfile::<Impl, IMPL_OFFSET>,
            EnterprisePolicyIsApplicationAllowed: EnterprisePolicyIsApplicationAllowed::<Impl, IMPL_OFFSET>,
            BeginUpdateDeployedPackage: BeginUpdateDeployedPackage::<Impl, IMPL_OFFSET>,
            ReportRestoreCancelled: ReportRestoreCancelled::<Impl, IMPL_OFFSET>,
            ResolveResourceString: ResolveResourceString::<Impl, IMPL_OFFSET>,
            UpdateCapabilitiesForModernApps: UpdateCapabilitiesForModernApps::<Impl, IMPL_OFFSET>,
            ReportDownloadStatusUpdate: ReportDownloadStatusUpdate::<Impl, IMPL_OFFSET>,
            BeginUninstallWithOptions: BeginUninstallWithOptions::<Impl, IMPL_OFFSET>,
            BindDeferredMdilBinaries: BindDeferredMdilBinaries::<Impl, IMPL_OFFSET>,
            GenerateXamlLightupXbfForCurrentLocale: GenerateXamlLightupXbfForCurrentLocale::<Impl, IMPL_OFFSET>,
            AddLicenseForAppx: AddLicenseForAppx::<Impl, IMPL_OFFSET>,
            FixJunctionsForAppsOnSDCard: FixJunctionsForAppsOnSDCard::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMDeploymentManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMEnumerationManager_Impl: Sized {
    fn AllApplications(&mut self, ppappenum: *mut ::core::option::Option<IPMApplicationInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllTiles(&mut self, pptileenum: *mut ::core::option::Option<IPMTileInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllTasks(&mut self, pptaskenum: *mut ::core::option::Option<IPMTaskInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllExtensions(&mut self, ppextensionenum: *mut ::core::option::Option<IPMExtensionInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllBackgroundServiceAgents(&mut self, ppbsaenum: *mut ::core::option::Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn AllBackgroundWorkers(&mut self, ppbswenum: *mut ::core::option::Option<IPMBackgroundWorkerInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn ApplicationInfo(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo>;
    fn TileInfo(&mut self, productid: &::windows::core::GUID, tileid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPMTileInfo>;
    fn TaskInfo(&mut self, productid: &::windows::core::GUID, taskid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPMTaskInfo>;
    fn TaskInfoEx(&mut self, productid: &::windows::core::GUID, taskid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IPMTaskInfo>;
    fn BackgroundServiceAgentInfo(&mut self, bsaid: u32) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo>;
    fn AllLiveTileJobs(&mut self) -> ::windows::core::Result<IPMLiveTileJobInfoEnumerator>;
    fn LiveTileJob(&mut self, productid: &::windows::core::GUID, tileid: &super::super::Foundation::BSTR, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> ::windows::core::Result<IPMLiveTileJobInfo>;
    fn ApplicationInfoExternal(&mut self, productid: &::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo>;
    fn FileHandlerGenericLogo(&mut self, filetype: &super::super::Foundation::BSTR, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationInfoFromAccessClaims(&mut self, sysappid0: &super::super::Foundation::BSTR, sysappid1: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPMApplicationInfo>;
    fn StartTileEnumeratorBlob(&mut self, filter: &PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()>;
    fn StartAppEnumeratorBlob(&mut self, filter: &PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMEnumerationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMEnumerationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMEnumerationManager_Vtbl {
        unsafe extern "system" fn AllApplications<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllApplications(::core::mem::transmute_copy(&ppappenum), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn AllTiles<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptileenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllTiles(::core::mem::transmute_copy(&pptileenum), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn AllTasks<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllTasks(::core::mem::transmute_copy(&pptaskenum), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn AllExtensions<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensionenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllExtensions(::core::mem::transmute_copy(&ppextensionenum), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn AllBackgroundServiceAgents<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbsaenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllBackgroundServiceAgents(::core::mem::transmute_copy(&ppbsaenum), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn AllBackgroundWorkers<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbswenum: *mut ::windows::core::RawPtr, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllBackgroundWorkers(::core::mem::transmute_copy(&ppbswenum), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn ApplicationInfo<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationInfo(::core::mem::transmute_copy(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileInfo<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptileinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileInfo(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&tileid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptileinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskInfo<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskInfo(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskInfoEx<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: super::super::Foundation::PWSTR, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskInfoEx(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundServiceAgentInfo<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsaid: u32, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundServiceAgentInfo(::core::mem::transmute_copy(&bsaid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllLiveTileJobs<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplivetilejobenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllLiveTileJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pplivetilejobenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LiveTileJob<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LiveTileJob(::core::mem::transmute_copy(&productid), ::core::mem::transmute_copy(&tileid), ::core::mem::transmute_copy(&recurrencetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplivetilejobinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationInfoExternal<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationInfoExternal(::core::mem::transmute_copy(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileHandlerGenericLogo<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FileHandlerGenericLogo(::core::mem::transmute_copy(&filetype), ::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn ApplicationInfoFromAccessClaims<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sysappid0: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sysappid1: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppappinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationInfoFromAccessClaims(::core::mem::transmute_copy(&sysappid0), ::core::mem::transmute_copy(&sysappid1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppappinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTileEnumeratorBlob<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTileEnumeratorBlob(::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&pctiles), ::core::mem::transmute_copy(&pptileblobs)).into()
        }
        unsafe extern "system" fn StartAppEnumeratorBlob<Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAppEnumeratorBlob(::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&pcapps), ::core::mem::transmute_copy(&ppappblobs)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllApplications: AllApplications::<Impl, IMPL_OFFSET>,
            AllTiles: AllTiles::<Impl, IMPL_OFFSET>,
            AllTasks: AllTasks::<Impl, IMPL_OFFSET>,
            AllExtensions: AllExtensions::<Impl, IMPL_OFFSET>,
            AllBackgroundServiceAgents: AllBackgroundServiceAgents::<Impl, IMPL_OFFSET>,
            AllBackgroundWorkers: AllBackgroundWorkers::<Impl, IMPL_OFFSET>,
            ApplicationInfo: ApplicationInfo::<Impl, IMPL_OFFSET>,
            TileInfo: TileInfo::<Impl, IMPL_OFFSET>,
            TaskInfo: TaskInfo::<Impl, IMPL_OFFSET>,
            TaskInfoEx: TaskInfoEx::<Impl, IMPL_OFFSET>,
            BackgroundServiceAgentInfo: BackgroundServiceAgentInfo::<Impl, IMPL_OFFSET>,
            AllLiveTileJobs: AllLiveTileJobs::<Impl, IMPL_OFFSET>,
            LiveTileJob: LiveTileJob::<Impl, IMPL_OFFSET>,
            ApplicationInfoExternal: ApplicationInfoExternal::<Impl, IMPL_OFFSET>,
            FileHandlerGenericLogo: FileHandlerGenericLogo::<Impl, IMPL_OFFSET>,
            ApplicationInfoFromAccessClaims: ApplicationInfoFromAccessClaims::<Impl, IMPL_OFFSET>,
            StartTileEnumeratorBlob: StartTileEnumeratorBlob::<Impl, IMPL_OFFSET>,
            StartAppEnumeratorBlob: StartAppEnumeratorBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMEnumerationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionCachedFileUpdaterInfo_Impl: Sized {
    fn SupportsUpdates(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionCachedFileUpdaterInfo_Vtbl {
        unsafe extern "system" fn SupportsUpdates<Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsupdates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SupportsUpdates: SupportsUpdates::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionCachedFileUpdaterInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionContractInfo_Impl: Sized {
    fn InvocationInfo(&mut self, paumid: *mut super::super::Foundation::BSTR, pargs: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionContractInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionContractInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionContractInfo_Vtbl {
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionContractInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paumid: *mut super::super::Foundation::BSTR, pargs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&paumid), ::core::mem::transmute_copy(&pargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InvocationInfo: InvocationInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionContractInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileExtensionInfo_Impl: Sized {
    fn Name(&mut self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Logo(&mut self, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ContentType(&mut self, filetype: &super::super::Foundation::BSTR, pcontenttype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FileType(&mut self, contenttype: &super::super::Foundation::BSTR, pfiletype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&mut self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllFileTypes(&mut self, pcbtypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileExtensionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileExtensionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionFileExtensionInfo_Vtbl {
        unsafe extern "system" fn Name<Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayName(::core::mem::transmute_copy(&pdisplayname)).into()
        }
        unsafe extern "system" fn Logo<Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Logo(::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn ContentType<Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontenttype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContentType(::core::mem::transmute_copy(&filetype), ::core::mem::transmute_copy(&pcontenttype)).into()
        }
        unsafe extern "system" fn FileType<Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfiletype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FileType(::core::mem::transmute_copy(&contenttype), ::core::mem::transmute_copy(&pfiletype)).into()
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pcbtypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Logo: Logo::<Impl, IMPL_OFFSET>,
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            FileType: FileType::<Impl, IMPL_OFFSET>,
            InvocationInfo: InvocationInfo::<Impl, IMPL_OFFSET>,
            AllFileTypes: AllFileTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileExtensionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileOpenPickerInfo_Impl: Sized {
    fn AllFileTypes(&mut self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileOpenPickerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileOpenPickerInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionFileOpenPickerInfo_Vtbl {
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsalltypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllFileTypes: AllFileTypes::<Impl, IMPL_OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileOpenPickerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileSavePickerInfo_Impl: Sized {
    fn AllFileTypes(&mut self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileSavePickerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionFileSavePickerInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionFileSavePickerInfo_Vtbl {
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsalltypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllFileTypes: AllFileTypes::<Impl, IMPL_OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileSavePickerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionInfo_Impl: Sized {
    fn SupplierPID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupplierTaskID(&mut self, psuppliertid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Title(&mut self, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IconPath(&mut self, piconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExtraFile(&mut self, pfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&mut self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionInfo_Vtbl {
        unsafe extern "system" fn SupplierPID<Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupplierpid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupplierPID() {
                ::core::result::Result::Ok(ok__) => {
                    *psupplierpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupplierTaskID<Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psuppliertid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SupplierTaskID(::core::mem::transmute_copy(&psuppliertid)).into()
        }
        unsafe extern "system" fn Title<Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Title(::core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn IconPath<Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IconPath(::core::mem::transmute_copy(&piconpath)).into()
        }
        unsafe extern "system" fn ExtraFile<Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExtraFile(::core::mem::transmute_copy(&pfilepath)).into()
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SupplierPID: SupplierPID::<Impl, IMPL_OFFSET>,
            SupplierTaskID: SupplierTaskID::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            IconPath: IconPath::<Impl, IMPL_OFFSET>,
            ExtraFile: ExtraFile::<Impl, IMPL_OFFSET>,
            InvocationInfo: InvocationInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMExtensionInfoEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMExtensionInfo>;
}
impl IPMExtensionInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMExtensionInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppextensioninfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionProtocolInfo_Impl: Sized {
    fn Protocol(&mut self, pprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InvocationInfo(&mut self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionProtocolInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionProtocolInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionProtocolInfo_Vtbl {
        unsafe extern "system" fn Protocol<Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Protocol(::core::mem::transmute_copy(&pprotocol)).into()
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            InvocationInfo: InvocationInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionProtocolInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionShareTargetInfo_Impl: Sized {
    fn AllFileTypes(&mut self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllDataFormats(&mut self, pcdataformats: *mut u32, ppdataformats: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionShareTargetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMExtensionShareTargetInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMExtensionShareTargetInfo_Vtbl {
        unsafe extern "system" fn AllFileTypes<Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn AllDataFormats<Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllDataFormats(::core::mem::transmute_copy(&pcdataformats), ::core::mem::transmute_copy(&ppdataformats)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *psupportsalltypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllFileTypes: AllFileTypes::<Impl, IMPL_OFFSET>,
            AllDataFormats: AllDataFormats::<Impl, IMPL_OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionShareTargetInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMLiveTileJobInfo_Impl: Sized {
    fn ProductID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TileID(&mut self, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NextSchedule(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn set_NextSchedule(&mut self, ftnextschedule: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn StartSchedule(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn set_StartSchedule(&mut self, ftstartschedule: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn IntervalDuration(&mut self) -> ::windows::core::Result<u32>;
    fn set_IntervalDuration(&mut self, ulintervalduration: u32) -> ::windows::core::Result<()>;
    fn RunForever(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_RunForever(&mut self, frunforever: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MaxRunCount(&mut self) -> ::windows::core::Result<u32>;
    fn set_MaxRunCount(&mut self, ulmaxruncount: u32) -> ::windows::core::Result<()>;
    fn RunCount(&mut self) -> ::windows::core::Result<u32>;
    fn set_RunCount(&mut self, ulruncount: u32) -> ::windows::core::Result<()>;
    fn RecurrenceType(&mut self) -> ::windows::core::Result<u32>;
    fn set_RecurrenceType(&mut self, ulrecurrencetype: u32) -> ::windows::core::Result<()>;
    fn TileXML(&mut self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::Result<()>;
    fn set_TileXML(&mut self, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::Result<()>;
    fn UrlXML(&mut self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::Result<()>;
    fn set_UrlXML(&mut self, purlxml: *const u8, cburlxml: u32) -> ::windows::core::Result<()>;
    fn AttemptCount(&mut self) -> ::windows::core::Result<u32>;
    fn set_AttemptCount(&mut self, ulattemptcount: u32) -> ::windows::core::Result<()>;
    fn DownloadState(&mut self) -> ::windows::core::Result<u32>;
    fn set_DownloadState(&mut self, uldownloadstate: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMLiveTileJobInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMLiveTileJobInfo_Vtbl {
        unsafe extern "system" fn ProductID<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TileID(::core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn NextSchedule<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    *pnextschedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_NextSchedule<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_NextSchedule(::core::mem::transmute_copy(&ftnextschedule)).into()
        }
        unsafe extern "system" fn StartSchedule<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    *pstartschedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_StartSchedule<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_StartSchedule(::core::mem::transmute_copy(&ftstartschedule)).into()
        }
        unsafe extern "system" fn IntervalDuration<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintervalduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntervalDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pintervalduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IntervalDuration<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulintervalduration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_IntervalDuration(::core::mem::transmute_copy(&ulintervalduration)).into()
        }
        unsafe extern "system" fn RunForever<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunForever() {
                ::core::result::Result::Ok(ok__) => {
                    *isrunforever = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunForever<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_RunForever(::core::mem::transmute_copy(&frunforever)).into()
        }
        unsafe extern "system" fn MaxRunCount<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxRunCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxruncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_MaxRunCount<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_MaxRunCount(::core::mem::transmute_copy(&ulmaxruncount)).into()
        }
        unsafe extern "system" fn RunCount<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pruncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunCount<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_RunCount(::core::mem::transmute_copy(&ulruncount)).into()
        }
        unsafe extern "system" fn RecurrenceType<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precurrencetype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecurrenceType() {
                ::core::result::Result::Ok(ok__) => {
                    *precurrencetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RecurrenceType<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecurrencetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_RecurrenceType(::core::mem::transmute_copy(&ulrecurrencetype)).into()
        }
        unsafe extern "system" fn TileXML<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TileXML(::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&pcbtilexml)).into()
        }
        unsafe extern "system" fn set_TileXML<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_TileXML(::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&cbtilexml)).into()
        }
        unsafe extern "system" fn UrlXML<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UrlXML(::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&pcburlxml)).into()
        }
        unsafe extern "system" fn set_UrlXML<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_UrlXML(::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&cburlxml)).into()
        }
        unsafe extern "system" fn AttemptCount<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattemptcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttemptCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pattemptcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_AttemptCount<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulattemptcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_AttemptCount(::core::mem::transmute_copy(&ulattemptcount)).into()
        }
        unsafe extern "system" fn DownloadState<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdownloadstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdownloadstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_DownloadState<Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uldownloadstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_DownloadState(::core::mem::transmute_copy(&uldownloadstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProductID: ProductID::<Impl, IMPL_OFFSET>,
            TileID: TileID::<Impl, IMPL_OFFSET>,
            NextSchedule: NextSchedule::<Impl, IMPL_OFFSET>,
            set_NextSchedule: set_NextSchedule::<Impl, IMPL_OFFSET>,
            StartSchedule: StartSchedule::<Impl, IMPL_OFFSET>,
            set_StartSchedule: set_StartSchedule::<Impl, IMPL_OFFSET>,
            IntervalDuration: IntervalDuration::<Impl, IMPL_OFFSET>,
            set_IntervalDuration: set_IntervalDuration::<Impl, IMPL_OFFSET>,
            RunForever: RunForever::<Impl, IMPL_OFFSET>,
            set_RunForever: set_RunForever::<Impl, IMPL_OFFSET>,
            MaxRunCount: MaxRunCount::<Impl, IMPL_OFFSET>,
            set_MaxRunCount: set_MaxRunCount::<Impl, IMPL_OFFSET>,
            RunCount: RunCount::<Impl, IMPL_OFFSET>,
            set_RunCount: set_RunCount::<Impl, IMPL_OFFSET>,
            RecurrenceType: RecurrenceType::<Impl, IMPL_OFFSET>,
            set_RecurrenceType: set_RecurrenceType::<Impl, IMPL_OFFSET>,
            TileXML: TileXML::<Impl, IMPL_OFFSET>,
            set_TileXML: set_TileXML::<Impl, IMPL_OFFSET>,
            UrlXML: UrlXML::<Impl, IMPL_OFFSET>,
            set_UrlXML: set_UrlXML::<Impl, IMPL_OFFSET>,
            AttemptCount: AttemptCount::<Impl, IMPL_OFFSET>,
            set_AttemptCount: set_AttemptCount::<Impl, IMPL_OFFSET>,
            DownloadState: DownloadState::<Impl, IMPL_OFFSET>,
            set_DownloadState: set_DownloadState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMLiveTileJobInfoEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMLiveTileJobInfo>;
}
impl IPMLiveTileJobInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMLiveTileJobInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMLiveTileJobInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplivetilejobinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pplivetilejobinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTaskInfo_Impl: Sized {
    fn ProductID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&mut self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NavigationPage(&mut self, pnavigationpage: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskTransition(&mut self) -> ::windows::core::Result<PM_TASK_TRANSITION>;
    fn RuntimeType(&mut self) -> ::windows::core::Result<PACKMAN_RUNTIME>;
    fn ActivationPolicy(&mut self) -> ::windows::core::Result<PM_ACTIVATION_POLICY>;
    fn TaskType(&mut self) -> ::windows::core::Result<PM_TASK_TYPE>;
    fn InvocationInfo(&mut self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImagePath(&mut self, pimagepath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImageParams(&mut self, pimageparams: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InstallRootFolder(&mut self, pinstallrootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DataRootFolder(&mut self, pdatarootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsSingleInstanceHost(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsInteropEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ApplicationState(&mut self) -> ::windows::core::Result<PM_APPLICATION_STATE>;
    fn InstallType(&mut self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn Version(&mut self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::Result<()>;
    fn BitsPerPixel(&mut self) -> ::windows::core::Result<u16>;
    fn SuppressesDehydration(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BackgroundExecutionAbilities(&mut self, pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsOptedForExtendedMem(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMTaskInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMTaskInfo_Vtbl {
        unsafe extern "system" fn ProductID<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn NavigationPage<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnavigationpage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NavigationPage(::core::mem::transmute_copy(&pnavigationpage)).into()
        }
        unsafe extern "system" fn TaskTransition<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *ptasktransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuntimeType<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuntimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *pruntimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationPolicy<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pactivationpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskType<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptasktype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn ImagePath<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimagepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImagePath(::core::mem::transmute_copy(&pimagepath)).into()
        }
        unsafe extern "system" fn ImageParams<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageparams: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImageParams(::core::mem::transmute_copy(&pimageparams)).into()
        }
        unsafe extern "system" fn InstallRootFolder<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallrootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallRootFolder(::core::mem::transmute_copy(&pinstallrootfolder)).into()
        }
        unsafe extern "system" fn DataRootFolder<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatarootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DataRootFolder(::core::mem::transmute_copy(&pdatarootfolder)).into()
        }
        unsafe extern "system" fn IsSingleInstanceHost<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSingleInstanceHost() {
                ::core::result::Result::Ok(ok__) => {
                    *pissingleinstancehost = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInteropEnabled<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInteropEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pisinteropenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationState<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationState() {
                ::core::result::Result::Ok(ok__) => {
                    *papplicationstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallType<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallType() {
                ::core::result::Result::Ok(ok__) => {
                    *pinstalltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&ptargetmajorversion), ::core::mem::transmute_copy(&ptargetminorversion)).into()
        }
        unsafe extern "system" fn BitsPerPixel<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitsperpixel: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerPixel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbitsperpixel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressesDehydration<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressesDehydration() {
                ::core::result::Result::Ok(ok__) => {
                    *psuppressesdehydration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundExecutionAbilities<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackgroundExecutionAbilities(::core::mem::transmute_copy(&pbackgroundexecutionabilities)).into()
        }
        unsafe extern "system" fn IsOptedForExtendedMem<Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOptedForExtendedMem() {
                ::core::result::Result::Ok(ok__) => {
                    *pisoptedin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProductID: ProductID::<Impl, IMPL_OFFSET>,
            TaskID: TaskID::<Impl, IMPL_OFFSET>,
            NavigationPage: NavigationPage::<Impl, IMPL_OFFSET>,
            TaskTransition: TaskTransition::<Impl, IMPL_OFFSET>,
            RuntimeType: RuntimeType::<Impl, IMPL_OFFSET>,
            ActivationPolicy: ActivationPolicy::<Impl, IMPL_OFFSET>,
            TaskType: TaskType::<Impl, IMPL_OFFSET>,
            InvocationInfo: InvocationInfo::<Impl, IMPL_OFFSET>,
            ImagePath: ImagePath::<Impl, IMPL_OFFSET>,
            ImageParams: ImageParams::<Impl, IMPL_OFFSET>,
            InstallRootFolder: InstallRootFolder::<Impl, IMPL_OFFSET>,
            DataRootFolder: DataRootFolder::<Impl, IMPL_OFFSET>,
            IsSingleInstanceHost: IsSingleInstanceHost::<Impl, IMPL_OFFSET>,
            IsInteropEnabled: IsInteropEnabled::<Impl, IMPL_OFFSET>,
            ApplicationState: ApplicationState::<Impl, IMPL_OFFSET>,
            InstallType: InstallType::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            BitsPerPixel: BitsPerPixel::<Impl, IMPL_OFFSET>,
            SuppressesDehydration: SuppressesDehydration::<Impl, IMPL_OFFSET>,
            BackgroundExecutionAbilities: BackgroundExecutionAbilities::<Impl, IMPL_OFFSET>,
            IsOptedForExtendedMem: IsOptedForExtendedMem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTaskInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMTaskInfoEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMTaskInfo>;
}
impl IPMTaskInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTaskInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMTaskInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMTaskInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTaskInfoEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTileInfo_Impl: Sized {
    fn ProductID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TileID(&mut self, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TemplateType(&mut self) -> ::windows::core::Result<TILE_TEMPLATE_TYPE>;
    fn HubPinnedState(&mut self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn HubPosition(&mut self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<u32>;
    fn IsNotified(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn TaskID(&mut self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TileType(&mut self) -> ::windows::core::Result<PM_STARTTILE_TYPE>;
    fn IsThemable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PropertyById(&mut self, propid: u32) -> ::windows::core::Result<IPMTilePropertyInfo>;
    fn InvocationInfo(&mut self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PropertyEnum(&mut self) -> ::windows::core::Result<IPMTilePropertyEnumerator>;
    fn HubTileSize(&mut self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<PM_TILE_SIZE>;
    fn set_HubPosition(&mut self, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::Result<()>;
    fn set_NotifiedState(&mut self, notified: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_HubPinnedState(&mut self, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_HubTileSize(&mut self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::Result<()>;
    fn set_InvocationInfo(&mut self, taskname: &super::super::Foundation::BSTR, taskparameters: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartTileBlob(&mut self, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()>;
    fn IsRestoring(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsAutoRestoreDisabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsRestoring(&mut self, restoring: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IsAutoRestoreDisabled(&mut self, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMTileInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMTileInfo_Vtbl {
        unsafe extern "system" fn ProductID<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    *pproductid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TileID(::core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn TemplateType<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemplateType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptemplatetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubPinnedState<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HubPinnedState(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubPosition<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HubPosition(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotified<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNotified() {
                ::core::result::Result::Ok(ok__) => {
                    *pisnotified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *pisdefault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn TileType<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileType() {
                ::core::result::Result::Ok(ok__) => {
                    *pstarttiletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThemable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisthemable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyById<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, pppropinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyById(::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvocationInfo<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn PropertyEnum<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptilepropenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pptilepropenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HubTileSize<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HubTileSize(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_HubPosition<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_HubPosition(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn set_NotifiedState<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_NotifiedState(::core::mem::transmute_copy(&notified)).into()
        }
        unsafe extern "system" fn set_HubPinnedState<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_HubPinnedState(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&pinned)).into()
        }
        unsafe extern "system" fn set_HubTileSize<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_HubTileSize(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn set_InvocationInfo<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_InvocationInfo(::core::mem::transmute_copy(&taskname), ::core::mem::transmute_copy(&taskparameters)).into()
        }
        unsafe extern "system" fn StartTileBlob<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTileBlob(::core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsRestoring<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRestoring() {
                ::core::result::Result::Ok(ok__) => {
                    *pisrestoring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAutoRestoreDisabled<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAutoRestoreDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pisautorestoredisabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsRestoring<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_IsRestoring(::core::mem::transmute_copy(&restoring)).into()
        }
        unsafe extern "system" fn set_IsAutoRestoreDisabled<Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_IsAutoRestoreDisabled(::core::mem::transmute_copy(&autorestoredisabled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProductID: ProductID::<Impl, IMPL_OFFSET>,
            TileID: TileID::<Impl, IMPL_OFFSET>,
            TemplateType: TemplateType::<Impl, IMPL_OFFSET>,
            HubPinnedState: HubPinnedState::<Impl, IMPL_OFFSET>,
            HubPosition: HubPosition::<Impl, IMPL_OFFSET>,
            IsNotified: IsNotified::<Impl, IMPL_OFFSET>,
            IsDefault: IsDefault::<Impl, IMPL_OFFSET>,
            TaskID: TaskID::<Impl, IMPL_OFFSET>,
            TileType: TileType::<Impl, IMPL_OFFSET>,
            IsThemable: IsThemable::<Impl, IMPL_OFFSET>,
            PropertyById: PropertyById::<Impl, IMPL_OFFSET>,
            InvocationInfo: InvocationInfo::<Impl, IMPL_OFFSET>,
            PropertyEnum: PropertyEnum::<Impl, IMPL_OFFSET>,
            HubTileSize: HubTileSize::<Impl, IMPL_OFFSET>,
            set_HubPosition: set_HubPosition::<Impl, IMPL_OFFSET>,
            set_NotifiedState: set_NotifiedState::<Impl, IMPL_OFFSET>,
            set_HubPinnedState: set_HubPinnedState::<Impl, IMPL_OFFSET>,
            set_HubTileSize: set_HubTileSize::<Impl, IMPL_OFFSET>,
            set_InvocationInfo: set_InvocationInfo::<Impl, IMPL_OFFSET>,
            StartTileBlob: StartTileBlob::<Impl, IMPL_OFFSET>,
            IsRestoring: IsRestoring::<Impl, IMPL_OFFSET>,
            IsAutoRestoreDisabled: IsAutoRestoreDisabled::<Impl, IMPL_OFFSET>,
            set_IsRestoring: set_IsRestoring::<Impl, IMPL_OFFSET>,
            set_IsAutoRestoreDisabled: set_IsAutoRestoreDisabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTileInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPMTileInfoEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMTileInfo>;
}
impl IPMTileInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTileInfoEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMTileInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMTileInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptileinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pptileinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTileInfoEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IPMTilePropertyEnumerator_Impl: Sized {
    fn Next(&mut self) -> ::windows::core::Result<IPMTilePropertyInfo>;
}
impl IPMTilePropertyEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMTilePropertyEnumerator_Vtbl {
        unsafe extern "system" fn Next<Impl: IPMTilePropertyEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTilePropertyEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTilePropertyInfo_Impl: Sized {
    fn PropertyID(&mut self) -> ::windows::core::Result<u32>;
    fn PropertyValue(&mut self, ppropvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn set_Property(&mut self, propvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPMTilePropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPMTilePropertyInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPMTilePropertyInfo_Vtbl {
        unsafe extern "system" fn PropertyID<Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppropid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PropertyValue(::core::mem::transmute_copy(&ppropvalue)).into()
        }
        unsafe extern "system" fn set_Property<Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).set_Property(::core::mem::transmute_copy(&propvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PropertyID: PropertyID::<Impl, IMPL_OFFSET>,
            PropertyValue: PropertyValue::<Impl, IMPL_OFFSET>,
            set_Property: set_Property::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTilePropertyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IValidate_Impl: Sized {
    fn OpenDatabase(&mut self, szdatabase: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OpenCUB(&mut self, szcubfile: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CloseDatabase(&mut self) -> ::windows::core::Result<()>;
    fn CloseCUB(&mut self) -> ::windows::core::Result<()>;
    fn SetDisplay(&mut self, pdisplayfunction: &LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, pstatusfunction: &LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Validate(&mut self, wzices: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IValidate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValidate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValidate_Vtbl {
        unsafe extern "system" fn OpenDatabase<Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdatabase: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenDatabase(::core::mem::transmute_copy(&szdatabase)).into()
        }
        unsafe extern "system" fn OpenCUB<Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcubfile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenCUB(::core::mem::transmute_copy(&szcubfile)).into()
        }
        unsafe extern "system" fn CloseDatabase<Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseDatabase().into()
        }
        unsafe extern "system" fn CloseCUB<Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseCUB().into()
        }
        unsafe extern "system" fn SetDisplay<Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayfunction: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplay(::core::mem::transmute_copy(&pdisplayfunction), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetStatus<Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatusfunction: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&pstatusfunction), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Validate<Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzices: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Validate(::core::mem::transmute_copy(&wzices)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OpenDatabase: OpenDatabase::<Impl, IMPL_OFFSET>,
            OpenCUB: OpenCUB::<Impl, IMPL_OFFSET>,
            CloseDatabase: CloseDatabase::<Impl, IMPL_OFFSET>,
            CloseCUB: CloseCUB::<Impl, IMPL_OFFSET>,
            SetDisplay: SetDisplay::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            Validate: Validate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValidate as ::windows::core::Interface>::IID
    }
}
