#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IAssemblyCache_Impl: Sized {
    fn UninstallAssembly(&self, dwflags: u32, pszassemblyname: &::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::Result<()>;
    fn QueryAssemblyInfo(&self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: &::windows::core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::Result<()>;
    fn CreateAssemblyCacheItem(&self, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::core::option::Option<IAssemblyCacheItem>, pszassemblyname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Reserved(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn InstallAssembly(&self, dwflags: u32, pszmanifestfilepath: &::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAssemblyCache {}
impl IAssemblyCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: isize>() -> IAssemblyCache_Vtbl {
        unsafe extern "system" fn UninstallAssembly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszassemblyname: ::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UninstallAssembly(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszassemblyname), ::core::mem::transmute_copy(&prefdata), ::core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn QueryAssemblyInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: ::windows::core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryAssemblyInfo(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszassemblyname), ::core::mem::transmute_copy(&pasminfo)).into()
        }
        unsafe extern "system" fn CreateAssemblyCacheItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut *mut ::core::ffi::c_void, pszassemblyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAssemblyCacheItem(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&ppasmitem), ::core::mem::transmute(&pszassemblyname)).into()
        }
        unsafe extern "system" fn Reserved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Reserved() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallAssembly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszmanifestfilepath: ::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallAssembly(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszmanifestfilepath), ::core::mem::transmute_copy(&prefdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UninstallAssembly: UninstallAssembly::<Identity, Impl, OFFSET>,
            QueryAssemblyInfo: QueryAssemblyInfo::<Identity, Impl, OFFSET>,
            CreateAssemblyCacheItem: CreateAssemblyCacheItem::<Identity, Impl, OFFSET>,
            Reserved: Reserved::<Identity, Impl, OFFSET>,
            InstallAssembly: InstallAssembly::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyCache as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyCacheItem_Impl: Sized {
    fn CreateStream(&self, dwflags: u32, pszstreamname: &::windows::core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut ::core::option::Option<super::Com::IStream>, pulimaxsize: *mut u64) -> ::windows::core::Result<()>;
    fn Commit(&self, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::Result<()>;
    fn AbortItem(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAssemblyCacheItem {}
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyCacheItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>() -> IAssemblyCacheItem_Vtbl {
        unsafe extern "system" fn CreateStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszstreamname: ::windows::core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut *mut ::core::ffi::c_void, pulimaxsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateStream(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszstreamname), ::core::mem::transmute_copy(&dwformat), ::core::mem::transmute_copy(&dwformatflags), ::core::mem::transmute_copy(&ppistream), ::core::mem::transmute_copy(&pulimaxsize)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn AbortItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortItem().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            AbortItem: AbortItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyCacheItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IAssemblyName_Impl: Sized {
    fn SetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::Result<()>;
    fn Finalize(&self) -> ::windows::core::Result<()>;
    fn GetDisplayName(&self, szdisplayname: ::windows::core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::Result<()>;
    fn Reserved(&self, refiid: *const ::windows::core::GUID, punkreserved1: ::core::option::Option<&::windows::core::IUnknown>, punkreserved2: ::core::option::Option<&::windows::core::IUnknown>, szreserved: &::windows::core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetName(&self, lpcwbuffer: *mut u32, pwzname: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetVersion(&self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::Result<()>;
    fn IsEqual(&self, pname: ::core::option::Option<&IAssemblyName>, dwcmpflags: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IAssemblyName>;
}
impl ::windows::core::RuntimeName for IAssemblyName {}
impl IAssemblyName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>() -> IAssemblyName_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&cbproperty)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&pcbproperty)).into()
        }
        unsafe extern "system" fn Finalize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finalize().into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdisplayname: ::windows::core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayName(::core::mem::transmute_copy(&szdisplayname), ::core::mem::transmute_copy(&pccdisplayname), ::core::mem::transmute_copy(&dwdisplayflags)).into()
        }
        unsafe extern "system" fn Reserved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, punkreserved1: *mut ::core::ffi::c_void, punkreserved2: *mut ::core::ffi::c_void, szreserved: ::windows::core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved(::core::mem::transmute_copy(&refiid), ::windows::core::from_raw_borrowed(&punkreserved1), ::windows::core::from_raw_borrowed(&punkreserved2), ::core::mem::transmute(&szreserved), ::core::mem::transmute_copy(&llreserved), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&cbreserved), ::core::mem::transmute_copy(&ppreserved)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&lpcwbuffer), ::core::mem::transmute_copy(&pwzname)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersion(::core::mem::transmute_copy(&pdwversionhi), ::core::mem::transmute_copy(&pdwversionlow)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void, dwcmpflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsEqual(::windows::core::from_raw_borrowed(&pname), ::core::mem::transmute_copy(&dwcmpflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IAssemblyName as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmDependency_Impl: Sized {
    fn Next(&self, cfetch: u32, rgmsmdependencies: *mut ::core::option::Option<IMsmDependency>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMsmDependency>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IEnumMsmDependency {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>() -> IEnumMsmDependency_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmdependencies), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmdependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pemsmdependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmDependency as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmError_Impl: Sized {
    fn Next(&self, cfetch: u32, rgmsmerrors: *mut ::core::option::Option<IMsmError>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMsmError>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IEnumMsmError {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: isize>() -> IEnumMsmError_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmerrors), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pemsmerrors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmError as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IEnumMsmString_Impl: Sized {
    fn Next(&self, cfetch: u32, rgbstrstrings: *mut ::windows::core::BSTR, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMsmString>;
}
impl ::windows::core::RuntimeName for IEnumMsmString {}
impl IEnumMsmString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: isize>() -> IEnumMsmString_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgbstrstrings), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pemsmstrings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pemsmstrings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMsmString as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmDependencies_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32) -> ::windows::core::Result<IMsmDependency>;
    fn Count(&self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMsmDependencies {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmDependencies_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: isize>() -> IMsmDependencies_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#return, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmDependencies as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmDependency_Impl: Sized + super::Com::IDispatch_Impl {
    fn Module(&self, module: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Language(&self, language: *mut i16) -> ::windows::core::Result<()>;
    fn Version(&self, version: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMsmDependency {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmDependency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: isize>() -> IMsmDependency_Vtbl {
        unsafe extern "system" fn Module<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Module(::core::mem::transmute_copy(&module)).into()
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Language(::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Version(::core::mem::transmute_copy(&version)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Module: Module::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmDependency as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmError_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&self, errortype: *mut msmErrorType) -> ::windows::core::Result<()>;
    fn Path(&self, errorpath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Language(&self, errorlanguage: *mut i16) -> ::windows::core::Result<()>;
    fn DatabaseTable(&self, errortable: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DatabaseKeys(&self) -> ::windows::core::Result<IMsmStrings>;
    fn ModuleTable(&self, errortable: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ModuleKeys(&self) -> ::windows::core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMsmError {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>() -> IMsmError_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortype: *mut msmErrorType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Type(::core::mem::transmute_copy(&errortype)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Path(::core::mem::transmute_copy(&errorpath)).into()
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorlanguage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Language(::core::mem::transmute_copy(&errorlanguage)).into()
        }
        unsafe extern "system" fn DatabaseTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortable: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DatabaseTable(::core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn DatabaseKeys<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DatabaseKeys() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorkeys, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortable: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModuleTable(::core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn ModuleKeys<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModuleKeys() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorkeys, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMsmError as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmErrors_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32) -> ::windows::core::Result<IMsmError>;
    fn Count(&self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMsmErrors {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmErrors_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: isize>() -> IMsmErrors_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#return, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmErrors as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmGetFiles_Impl: Sized + super::Com::IDispatch_Impl {
    fn ModuleFiles(&self) -> ::windows::core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMsmGetFiles {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmGetFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmGetFiles_Impl, const OFFSET: isize>() -> IMsmGetFiles_Vtbl {
        unsafe extern "system" fn ModuleFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmGetFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModuleFiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(files, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ModuleFiles: ModuleFiles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmGetFiles as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmMerge_Impl: Sized + super::Com::IDispatch_Impl {
    fn OpenDatabase(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OpenModule(&self, path: &::windows::core::BSTR, language: i16) -> ::windows::core::Result<()>;
    fn CloseDatabase(&self, commit: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn CloseModule(&self) -> ::windows::core::Result<()>;
    fn OpenLog(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CloseLog(&self) -> ::windows::core::Result<()>;
    fn Log(&self, message: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Errors(&self) -> ::windows::core::Result<IMsmErrors>;
    fn Dependencies(&self) -> ::windows::core::Result<IMsmDependencies>;
    fn Merge(&self, feature: &::windows::core::BSTR, redirectdir: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Connect(&self, feature: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ExtractCAB(&self, filename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ExtractFiles(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMsmMerge {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmMerge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>() -> IMsmMerge_Vtbl {
        unsafe extern "system" fn OpenDatabase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenDatabase(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn OpenModule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, language: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenModule(::core::mem::transmute(&path), ::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn CloseDatabase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commit: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseDatabase(::core::mem::transmute_copy(&commit)).into()
        }
        unsafe extern "system" fn CloseModule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseModule().into()
        }
        unsafe extern "system" fn OpenLog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenLog(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn CloseLog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseLog().into()
        }
        unsafe extern "system" fn Log<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Log(::core::mem::transmute(&message)).into()
        }
        unsafe extern "system" fn Errors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Errors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dependencies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows::core::BSTR>, redirectdir: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Merge(::core::mem::transmute(&feature), ::core::mem::transmute(&redirectdir)).into()
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::core::mem::transmute(&feature)).into()
        }
        unsafe extern "system" fn ExtractCAB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExtractCAB(::core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn ExtractFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExtractFiles(::core::mem::transmute(&path)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMsmMerge as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMsmStrings_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32, r#return: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Count(&self, count: *mut i32) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMsmStrings {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMsmStrings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: isize>() -> IMsmStrings_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Item(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&r#return)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsmStrings as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMApplicationInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn InstanceID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OfferID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DefaultTask(&self, pdefaulttask: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AppTitle(&self, papptitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn IconPath(&self, pappiconpath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn NotificationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AppInstallType(&self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn State(&self) -> ::windows::core::Result<PM_APPLICATION_STATE>;
    fn IsRevoked(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn UpdateAvailable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InstallDate(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn IsUninstallable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsThemable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsTrial(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InstallPath(&self, pinstallpath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DataRoot(&self, pdataroot: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Genre(&self) -> ::windows::core::Result<PM_APP_GENRE>;
    fn Publisher(&self, ppublisher: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Author(&self, pauthor: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self, pdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Version(&self, pversion: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut ::windows::core::BSTR, pparameters: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AppPlatMajorVersion(&self) -> ::windows::core::Result<u8>;
    fn AppPlatMinorVersion(&self) -> ::windows::core::Result<u8>;
    fn PublisherID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsMultiCore(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SID(&self, psid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AppPlatMajorVersionLightUp(&self) -> ::windows::core::Result<u8>;
    fn AppPlatMinorVersionLightUp(&self) -> ::windows::core::Result<u8>;
    fn set_UpdateAvailable(&self, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_NotificationState(&self, isnotified: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IconPath(&self, appiconpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn set_Title(&self, apptitle: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMApplicationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMApplicationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>() -> IPMApplicationInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstanceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstanceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pofferid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OfferID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pofferid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaulttask: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DefaultTask(::core::mem::transmute_copy(&pdefaulttask)).into()
        }
        unsafe extern "system" fn AppTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papptitle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppTitle(::core::mem::transmute_copy(&papptitle)).into()
        }
        unsafe extern "system" fn IconPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappiconpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IconPath(::core::mem::transmute_copy(&pappiconpath)).into()
        }
        unsafe extern "system" fn NotificationState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NotificationState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisnotified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInstallType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppInstallType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappinstalltype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRevoked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRevoked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisrevoked, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UpdateAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisupdateavailable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstallDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstalldate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisuninstallable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsThemable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisthemable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrial<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTrial() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistrial, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallPath(::core::mem::transmute_copy(&pinstallpath)).into()
        }
        unsafe extern "system" fn DataRoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataroot: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DataRoot(::core::mem::transmute_copy(&pdataroot)).into()
        }
        unsafe extern "system" fn Genre<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Genre() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgenre, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publisher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublisher: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Publisher(::core::mem::transmute_copy(&ppublisher)).into()
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthor: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Author(::core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Version(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppPlatMajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmajorver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppPlatMinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pminorver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublisherid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppublisherid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMultiCore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMultiCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pismulticore, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersionLightUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppPlatMajorVersionLightUp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmajorver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersionLightUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppPlatMinorVersionLightUp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pminorver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_UpdateAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_UpdateAvailable(::core::mem::transmute_copy(&isupdateavailable)).into()
        }
        unsafe extern "system" fn set_NotificationState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_NotificationState(::core::mem::transmute_copy(&isnotified)).into()
        }
        unsafe extern "system" fn set_IconPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appiconpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_IconPath(::core::mem::transmute(&appiconpath)).into()
        }
        unsafe extern "system" fn set_UninstallableState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_UninstallableState(::core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn IsPinableOnKidZone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPinableOnKidZone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pispinable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyPreInstalled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOriginallyPreInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pispreinstalled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInstallOnSD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInstallOnSD() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisinstallonsd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutOnSD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOptoutOnSD() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisoptoutonsd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutBackupRestore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOptoutBackupRestore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisoptoutbackuprestore, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_EnterpriseDisabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_EnterpriseDisabled(::core::mem::transmute_copy(&isdisabled)).into()
        }
        unsafe extern "system" fn set_EnterpriseUninstallable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_EnterpriseUninstallable(::core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn EnterpriseDisabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnterpriseDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdisabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseUninstallable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnterpriseUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isuninstallable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisibleOnAppList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsVisibleOnAppList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisvisible, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInboxApp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInboxApp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisinboxapp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorageid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StorageID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstorageid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAppBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartAppBlob(::core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsMovable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMovable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pismovable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeploymentAppEnumerationHubFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeploymentAppEnumerationHubFilter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hubtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmodifieddate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyRestored<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOriginallyRestored() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisrestored, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldDeferMdilBind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShouldDeferMdilBind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdefermdilbind, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullyPreInstall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFullyPreInstall() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisfullypreinstall, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsMdilMaintenanceNeeded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_IsMdilMaintenanceNeeded(::core::mem::transmute_copy(&fismdilmaintenanceneeded)).into()
        }
        unsafe extern "system" fn set_Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apptitle: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_Title(::core::mem::transmute(&apptitle)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
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
        iid == &<IPMApplicationInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMApplicationInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMApplicationInfo>;
}
impl ::windows::core::RuntimeName for IPMApplicationInfoEnumerator {}
impl IPMApplicationInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfoEnumerator_Impl, const OFFSET: isize>() -> IPMApplicationInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMApplicationInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMApplicationInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundServiceAgentInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&self, ptaskid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn BSAID(&self) -> ::windows::core::Result<u32>;
    fn BGSpecifier(&self, pbgspecifier: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn BGName(&self, pbgname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn BGSource(&self, pbgsource: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn BGType(&self, pbgtype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn IsPeriodic(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsScheduled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsScheduleAllowed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Description(&self, pdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn IsLaunchOnBoot(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsScheduled(&self, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IsScheduleAllowed(&self, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMBackgroundServiceAgentInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMBackgroundServiceAgentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>() -> IPMBackgroundServiceAgentInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BSAID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsaid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BSAID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsaid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGSpecifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgspecifier: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BGSpecifier(::core::mem::transmute_copy(&pbgspecifier)).into()
        }
        unsafe extern "system" fn BGName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BGName(::core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn BGSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgsource: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BGSource(::core::mem::transmute_copy(&pbgsource)).into()
        }
        unsafe extern "system" fn BGType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BGType(::core::mem::transmute_copy(&pbgtype)).into()
        }
        unsafe extern "system" fn IsPeriodic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPeriodic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisperiodic, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsScheduled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisscheduled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduleAllowed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsScheduleAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisscheduleallowed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn IsLaunchOnBoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLaunchOnBoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plaunchonboot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsScheduled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_IsScheduled(::core::mem::transmute_copy(&isscheduled)).into()
        }
        unsafe extern "system" fn set_IsScheduleAllowed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_IsScheduleAllowed(::core::mem::transmute_copy(&isscheduleallowed)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IPMBackgroundServiceAgentInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMBackgroundServiceAgentInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo>;
}
impl ::windows::core::RuntimeName for IPMBackgroundServiceAgentInfoEnumerator {}
impl IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: isize>() -> IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbsainfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbsainfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundWorkerInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&self, ptaskid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn BGName(&self, pbgname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MaxStartupLatency(&self) -> ::windows::core::Result<u32>;
    fn ExpectedRuntime(&self) -> ::windows::core::Result<u32>;
    fn IsBootWorker(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMBackgroundWorkerInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMBackgroundWorkerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>() -> IPMBackgroundWorkerInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BGName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BGName(::core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn MaxStartupLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxstartuplatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxStartupLatency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxstartuplatency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpectedruntime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpectedRuntime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexpectedruntime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBootWorker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBootWorker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisbootworker, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            BGName: BGName::<Identity, Impl, OFFSET>,
            MaxStartupLatency: MaxStartupLatency::<Identity, Impl, OFFSET>,
            ExpectedRuntime: ExpectedRuntime::<Identity, Impl, OFFSET>,
            IsBootWorker: IsBootWorker::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMBackgroundWorkerInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMBackgroundWorkerInfo>;
}
impl ::windows::core::RuntimeName for IPMBackgroundWorkerInfoEnumerator {}
impl IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: isize>() -> IPMBackgroundWorkerInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbwinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbwinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
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
    fn GetLicenseChallenge(&self, packagepath: &::windows::core::BSTR, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallengeByProductID(&self, productid: &::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::Result<()>;
    fn GetLicenseChallengeByProductID2(&self, productid: &::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()>;
    fn RevokeLicense(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RebindMdilBinaries(&self, productid: &::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RebindAllMdilBinaries(&self, productid: &::windows::core::GUID, instanceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegenerateXbf(&self, productid: &::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GenerateXbfForCurrentLocale(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginProvision(&self, productid: &::windows::core::GUID, xmlpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn BeginDeprovision(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ReindexSQLCEDatabases(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetApplicationsNeedMaintenance(&self, requiredmaintenanceoperations: u32) -> ::windows::core::Result<u32>;
    fn UpdateChamberProfile(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnterprisePolicyIsApplicationAllowed(&self, productid: &::windows::core::GUID, publishername: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BeginUpdateDeployedPackage(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()>;
    fn ReportRestoreCancelled(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ResolveResourceString(&self, resourcestring: &::windows::core::PCWSTR, presolvedresourcestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn UpdateCapabilitiesForModernApps(&self) -> ::windows::core::Result<()>;
    fn ReportDownloadStatusUpdate(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn BeginUninstallWithOptions(&self, productid: &::windows::core::GUID, removaloptions: u32) -> ::windows::core::Result<()>;
    fn BindDeferredMdilBinaries(&self) -> ::windows::core::Result<()>;
    fn GenerateXamlLightupXbfForCurrentLocale(&self, packagefamilyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AddLicenseForAppx(&self, productid: &::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::Result<()>;
    fn FixJunctionsForAppsOnSDCard(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IPMDeploymentManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPMDeploymentManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>() -> IPMDeploymentManager_Vtbl {
        unsafe extern "system" fn ReportDownloadBegin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDownloadBegin(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ReportDownloadProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, usprogress: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDownloadProgress(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&usprogress)).into()
        }
        unsafe extern "system" fn ReportDownloadComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDownloadComplete(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn BeginInstall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginInstall(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUpdate(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginDeployPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDeployPackage(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateDeployedPackageLegacy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUpdateDeployedPackageLegacy(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUninstall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUninstall(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppInstall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginEnterpriseAppInstall(::core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppUpdate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginEnterpriseAppUpdate(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateLicense<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, offerid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUpdateLicense(::core::mem::transmute(&productid), ::core::mem::transmute(&offerid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallenge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLicenseChallenge(::core::mem::transmute(&packagepath), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcbchallenge), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue))
                .into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLicenseChallengeByProductID(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLicenseChallengeByProductID2(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue))
                .into()
        }
        unsafe extern "system" fn RevokeLicense<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevokeLicense(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn RebindMdilBinaries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RebindMdilBinaries(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&filenames)).into()
        }
        unsafe extern "system" fn RebindAllMdilBinaries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, instanceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RebindAllMdilBinaries(::core::mem::transmute(&productid), ::core::mem::transmute(&instanceid)).into()
        }
        unsafe extern "system" fn RegenerateXbf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegenerateXbf(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&assemblypaths)).into()
        }
        unsafe extern "system" fn GenerateXbfForCurrentLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateXbfForCurrentLocale(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginProvision<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, xmlpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginProvision(::core::mem::transmute(&productid), ::core::mem::transmute(&xmlpath)).into()
        }
        unsafe extern "system" fn BeginDeprovision<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDeprovision(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ReindexSQLCEDatabases<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReindexSQLCEDatabases(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn SetApplicationsNeedMaintenance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetApplicationsNeedMaintenance(::core::mem::transmute_copy(&requiredmaintenanceoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcapplications, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateChamberProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateChamberProfile(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn EnterprisePolicyIsApplicationAllowed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, publishername: ::windows::core::PCWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnterprisePolicyIsApplicationAllowed(::core::mem::transmute(&productid), ::core::mem::transmute(&publishername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisallowed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUpdateDeployedPackage(::core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn ReportRestoreCancelled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportRestoreCancelled(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ResolveResourceString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcestring: ::windows::core::PCWSTR, presolvedresourcestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResolveResourceString(::core::mem::transmute(&resourcestring), ::core::mem::transmute_copy(&presolvedresourcestring)).into()
        }
        unsafe extern "system" fn UpdateCapabilitiesForModernApps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateCapabilitiesForModernApps().into()
        }
        unsafe extern "system" fn ReportDownloadStatusUpdate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDownloadStatusUpdate(::core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginUninstallWithOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, removaloptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUninstallWithOptions(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&removaloptions)).into()
        }
        unsafe extern "system" fn BindDeferredMdilBinaries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindDeferredMdilBinaries().into()
        }
        unsafe extern "system" fn GenerateXamlLightupXbfForCurrentLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateXamlLightupXbfForCurrentLocale(::core::mem::transmute(&packagefamilyname)).into()
        }
        unsafe extern "system" fn AddLicenseForAppx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLicenseForAppx(::core::mem::transmute(&productid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense), ::core::mem::transmute_copy(&pbplayreadyheader), ::core::mem::transmute_copy(&cbplayreadyheader)).into()
        }
        unsafe extern "system" fn FixJunctionsForAppsOnSDCard<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FixJunctionsForAppsOnSDCard().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IPMDeploymentManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMEnumerationManager_Impl: Sized {
    fn get_AllApplications(&self, ppappenum: *mut ::core::option::Option<IPMApplicationInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn get_AllTiles(&self, pptileenum: *mut ::core::option::Option<IPMTileInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn get_AllTasks(&self, pptaskenum: *mut ::core::option::Option<IPMTaskInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn get_AllExtensions(&self, ppextensionenum: *mut ::core::option::Option<IPMExtensionInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn get_AllBackgroundServiceAgents(&self, ppbsaenum: *mut ::core::option::Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn get_AllBackgroundWorkers(&self, ppbswenum: *mut ::core::option::Option<IPMBackgroundWorkerInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows::core::Result<()>;
    fn get_ApplicationInfo(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo>;
    fn get_TileInfo(&self, productid: &::windows::core::GUID, tileid: &::windows::core::BSTR) -> ::windows::core::Result<IPMTileInfo>;
    fn get_TaskInfo(&self, productid: &::windows::core::GUID, taskid: &::windows::core::BSTR) -> ::windows::core::Result<IPMTaskInfo>;
    fn get_TaskInfoEx(&self, productid: &::windows::core::GUID, taskid: &::windows::core::PCWSTR) -> ::windows::core::Result<IPMTaskInfo>;
    fn get_BackgroundServiceAgentInfo(&self, bsaid: u32) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo>;
    fn AllLiveTileJobs(&self) -> ::windows::core::Result<IPMLiveTileJobInfoEnumerator>;
    fn get_LiveTileJob(&self, productid: &::windows::core::GUID, tileid: &::windows::core::BSTR, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> ::windows::core::Result<IPMLiveTileJobInfo>;
    fn get_ApplicationInfoExternal(&self, productid: &::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo>;
    fn get_FileHandlerGenericLogo(&self, filetype: &::windows::core::BSTR, logosize: PM_LOGO_SIZE, plogo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_ApplicationInfoFromAccessClaims(&self, sysappid0: &::windows::core::BSTR, sysappid1: &::windows::core::BSTR) -> ::windows::core::Result<IPMApplicationInfo>;
    fn get_StartTileEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()>;
    fn get_StartAppEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMEnumerationManager {}
#[cfg(feature = "Win32_Foundation")]
impl IPMEnumerationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>() -> IPMEnumerationManager_Vtbl {
        unsafe extern "system" fn get_AllApplications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllApplications(::core::mem::transmute_copy(&ppappenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllTiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptileenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllTiles(::core::mem::transmute_copy(&pptileenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllTasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllTasks(::core::mem::transmute_copy(&pptaskenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensionenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllExtensions(::core::mem::transmute_copy(&ppextensionenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllBackgroundServiceAgents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbsaenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllBackgroundServiceAgents(::core::mem::transmute_copy(&ppbsaenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllBackgroundWorkers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbswenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllBackgroundWorkers(::core::mem::transmute_copy(&ppbswenum), ::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_ApplicationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ApplicationInfo(::core::mem::transmute(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TileInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_TileInfo(::core::mem::transmute(&productid), ::core::mem::transmute(&tileid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptileinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TaskInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_TaskInfo(::core::mem::transmute(&productid), ::core::mem::transmute(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TaskInfoEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::windows::core::PCWSTR, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_TaskInfoEx(::core::mem::transmute(&productid), ::core::mem::transmute(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_BackgroundServiceAgentInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsaid: u32, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_BackgroundServiceAgentInfo(::core::mem::transmute_copy(&bsaid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllLiveTileJobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplivetilejobenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllLiveTileJobs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplivetilejobenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_LiveTileJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::std::mem::MaybeUninit<::windows::core::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_LiveTileJob(::core::mem::transmute(&productid), ::core::mem::transmute(&tileid), ::core::mem::transmute_copy(&recurrencetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplivetilejobinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ApplicationInfoExternal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ApplicationInfoExternal(::core::mem::transmute(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_FileHandlerGenericLogo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: ::std::mem::MaybeUninit<::windows::core::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_FileHandlerGenericLogo(::core::mem::transmute(&filetype), ::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn get_ApplicationInfoFromAccessClaims<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sysappid0: ::std::mem::MaybeUninit<::windows::core::BSTR>, sysappid1: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ApplicationInfoFromAccessClaims(::core::mem::transmute(&sysappid0), ::core::mem::transmute(&sysappid1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_StartTileEnumeratorBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_StartTileEnumeratorBlob(::core::mem::transmute(&filter), ::core::mem::transmute_copy(&pctiles), ::core::mem::transmute_copy(&pptileblobs)).into()
        }
        unsafe extern "system" fn get_StartAppEnumeratorBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_StartAppEnumeratorBlob(::core::mem::transmute(&filter), ::core::mem::transmute_copy(&pcapps), ::core::mem::transmute_copy(&ppappblobs)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllApplications: get_AllApplications::<Identity, Impl, OFFSET>,
            get_AllTiles: get_AllTiles::<Identity, Impl, OFFSET>,
            get_AllTasks: get_AllTasks::<Identity, Impl, OFFSET>,
            get_AllExtensions: get_AllExtensions::<Identity, Impl, OFFSET>,
            get_AllBackgroundServiceAgents: get_AllBackgroundServiceAgents::<Identity, Impl, OFFSET>,
            get_AllBackgroundWorkers: get_AllBackgroundWorkers::<Identity, Impl, OFFSET>,
            get_ApplicationInfo: get_ApplicationInfo::<Identity, Impl, OFFSET>,
            get_TileInfo: get_TileInfo::<Identity, Impl, OFFSET>,
            get_TaskInfo: get_TaskInfo::<Identity, Impl, OFFSET>,
            get_TaskInfoEx: get_TaskInfoEx::<Identity, Impl, OFFSET>,
            get_BackgroundServiceAgentInfo: get_BackgroundServiceAgentInfo::<Identity, Impl, OFFSET>,
            AllLiveTileJobs: AllLiveTileJobs::<Identity, Impl, OFFSET>,
            get_LiveTileJob: get_LiveTileJob::<Identity, Impl, OFFSET>,
            get_ApplicationInfoExternal: get_ApplicationInfoExternal::<Identity, Impl, OFFSET>,
            get_FileHandlerGenericLogo: get_FileHandlerGenericLogo::<Identity, Impl, OFFSET>,
            get_ApplicationInfoFromAccessClaims: get_ApplicationInfoFromAccessClaims::<Identity, Impl, OFFSET>,
            get_StartTileEnumeratorBlob: get_StartTileEnumeratorBlob::<Identity, Impl, OFFSET>,
            get_StartAppEnumeratorBlob: get_StartAppEnumeratorBlob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMEnumerationManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionCachedFileUpdaterInfo_Impl: Sized {
    fn SupportsUpdates(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMExtensionCachedFileUpdaterInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: isize>() -> IPMExtensionCachedFileUpdaterInfo_Vtbl {
        unsafe extern "system" fn SupportsUpdates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsupdates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SupportsUpdates: SupportsUpdates::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionCachedFileUpdaterInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMExtensionContractInfo_Impl: Sized {
    fn get_InvocationInfo(&self, paumid: *mut ::windows::core::BSTR, pargs: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPMExtensionContractInfo {}
impl IPMExtensionContractInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionContractInfo_Impl, const OFFSET: isize>() -> IPMExtensionContractInfo_Vtbl {
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionContractInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paumid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pargs: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_InvocationInfo(::core::mem::transmute_copy(&paumid), ::core::mem::transmute_copy(&pargs)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionContractInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMExtensionFileExtensionInfo_Impl: Sized {
    fn Name(&self, pname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DisplayName(&self, pdisplayname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_Logo(&self, logosize: PM_LOGO_SIZE, plogo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_ContentType(&self, filetype: &::windows::core::BSTR, pcontenttype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_FileType(&self, contenttype: &::windows::core::BSTR, pfiletype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut ::windows::core::BSTR, pparameters: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_AllFileTypes(&self, pcbtypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPMExtensionFileExtensionInfo {}
impl IPMExtensionFileExtensionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>() -> IPMExtensionFileExtensionInfo_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayName(::core::mem::transmute_copy(&pdisplayname)).into()
        }
        unsafe extern "system" fn get_Logo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Logo(::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn get_ContentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetype: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcontenttype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ContentType(::core::mem::transmute(&filetype), ::core::mem::transmute_copy(&pcontenttype)).into()
        }
        unsafe extern "system" fn get_FileType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfiletype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_FileType(::core::mem::transmute(&contenttype), ::core::mem::transmute_copy(&pfiletype)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllFileTypes(::core::mem::transmute_copy(&pcbtypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            get_Logo: get_Logo::<Identity, Impl, OFFSET>,
            get_ContentType: get_ContentType::<Identity, Impl, OFFSET>,
            get_FileType: get_FileType::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileExtensionInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileOpenPickerInfo_Impl: Sized {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMExtensionFileOpenPickerInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileOpenPickerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>() -> IPMExtensionFileOpenPickerInfo_Vtbl {
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsalltypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileOpenPickerInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileSavePickerInfo_Impl: Sized {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMExtensionFileSavePickerInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionFileSavePickerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>() -> IPMExtensionFileSavePickerInfo_Vtbl {
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsalltypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionFileSavePickerInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMExtensionInfo_Impl: Sized {
    fn SupplierPID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupplierTaskID(&self, psuppliertid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Title(&self, ptitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn IconPath(&self, piconpath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ExtraFile(&self, pfilepath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut ::windows::core::BSTR, pparameters: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPMExtensionInfo {}
impl IPMExtensionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>() -> IPMExtensionInfo_Vtbl {
        unsafe extern "system" fn SupplierPID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupplierpid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupplierPID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupplierpid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupplierTaskID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psuppliertid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SupplierTaskID(::core::mem::transmute_copy(&psuppliertid)).into()
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptitle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Title(::core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn IconPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piconpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IconPath(::core::mem::transmute_copy(&piconpath)).into()
        }
        unsafe extern "system" fn ExtraFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilepath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExtraFile(::core::mem::transmute_copy(&pfilepath)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupplierPID: SupplierPID::<Identity, Impl, OFFSET>,
            SupplierTaskID: SupplierTaskID::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            IconPath: IconPath::<Identity, Impl, OFFSET>,
            ExtraFile: ExtraFile::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMExtensionInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMExtensionInfo>;
}
impl ::windows::core::RuntimeName for IPMExtensionInfoEnumerator {}
impl IPMExtensionInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfoEnumerator_Impl, const OFFSET: isize>() -> IPMExtensionInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextensioninfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppextensioninfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMExtensionProtocolInfo_Impl: Sized {
    fn Protocol(&self, pprotocol: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut ::windows::core::BSTR, pparameters: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPMExtensionProtocolInfo {}
impl IPMExtensionProtocolInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>() -> IPMExtensionProtocolInfo_Vtbl {
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Protocol(::core::mem::transmute_copy(&pprotocol)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionProtocolInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionShareTargetInfo_Impl: Sized {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_AllDataFormats(&self, pcdataformats: *mut u32, ppdataformats: *mut *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMExtensionShareTargetInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMExtensionShareTargetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>() -> IPMExtensionShareTargetInfo_Vtbl {
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows::core::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllFileTypes(::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn get_AllDataFormats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut ::windows::core::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_AllDataFormats(::core::mem::transmute_copy(&pcdataformats), ::core::mem::transmute_copy(&ppdataformats)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsAllFileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsalltypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
            get_AllDataFormats: get_AllDataFormats::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMExtensionShareTargetInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMLiveTileJobInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TileID(&self, ptileid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn get_TileXML(&self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::Result<()>;
    fn set_TileXML(&self, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::Result<()>;
    fn get_UrlXML(&self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::Result<()>;
    fn set_UrlXML(&self, purlxml: *const u8, cburlxml: u32) -> ::windows::core::Result<()>;
    fn AttemptCount(&self) -> ::windows::core::Result<u32>;
    fn set_AttemptCount(&self, ulattemptcount: u32) -> ::windows::core::Result<()>;
    fn DownloadState(&self) -> ::windows::core::Result<u32>;
    fn set_DownloadState(&self, uldownloadstate: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMLiveTileJobInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMLiveTileJobInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>() -> IPMLiveTileJobInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptileid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TileID(::core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn NextSchedule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnextschedule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_NextSchedule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_NextSchedule(::core::mem::transmute(&ftnextschedule)).into()
        }
        unsafe extern "system" fn StartSchedule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstartschedule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_StartSchedule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_StartSchedule(::core::mem::transmute(&ftstartschedule)).into()
        }
        unsafe extern "system" fn IntervalDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintervalduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IntervalDuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pintervalduration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IntervalDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulintervalduration: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_IntervalDuration(::core::mem::transmute_copy(&ulintervalduration)).into()
        }
        unsafe extern "system" fn RunForever<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunForever() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isrunforever, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunForever<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_RunForever(::core::mem::transmute_copy(&frunforever)).into()
        }
        unsafe extern "system" fn MaxRunCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxRunCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxruncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_MaxRunCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_MaxRunCount(::core::mem::transmute_copy(&ulmaxruncount)).into()
        }
        unsafe extern "system" fn RunCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pruncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pruncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulruncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_RunCount(::core::mem::transmute_copy(&ulruncount)).into()
        }
        unsafe extern "system" fn RecurrenceType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precurrencetype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecurrenceType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(precurrencetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RecurrenceType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrecurrencetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_RecurrenceType(::core::mem::transmute_copy(&ulrecurrencetype)).into()
        }
        unsafe extern "system" fn get_TileXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TileXML(::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&pcbtilexml)).into()
        }
        unsafe extern "system" fn set_TileXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_TileXML(::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&cbtilexml)).into()
        }
        unsafe extern "system" fn get_UrlXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UrlXML(::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&pcburlxml)).into()
        }
        unsafe extern "system" fn set_UrlXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_UrlXML(::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&cburlxml)).into()
        }
        unsafe extern "system" fn AttemptCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattemptcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttemptCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pattemptcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_AttemptCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulattemptcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_AttemptCount(::core::mem::transmute_copy(&ulattemptcount)).into()
        }
        unsafe extern "system" fn DownloadState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdownloadstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdownloadstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_DownloadState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uldownloadstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_DownloadState(::core::mem::transmute_copy(&uldownloadstate)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
            get_TileXML: get_TileXML::<Identity, Impl, OFFSET>,
            set_TileXML: set_TileXML::<Identity, Impl, OFFSET>,
            get_UrlXML: get_UrlXML::<Identity, Impl, OFFSET>,
            set_UrlXML: set_UrlXML::<Identity, Impl, OFFSET>,
            AttemptCount: AttemptCount::<Identity, Impl, OFFSET>,
            set_AttemptCount: set_AttemptCount::<Identity, Impl, OFFSET>,
            DownloadState: DownloadState::<Identity, Impl, OFFSET>,
            set_DownloadState: set_DownloadState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMLiveTileJobInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMLiveTileJobInfo>;
}
impl ::windows::core::RuntimeName for IPMLiveTileJobInfoEnumerator {}
impl IPMLiveTileJobInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: isize>() -> IPMLiveTileJobInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplivetilejobinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTaskInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TaskID(&self, ptaskid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn NavigationPage(&self, pnavigationpage: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TaskTransition(&self) -> ::windows::core::Result<PM_TASK_TRANSITION>;
    fn RuntimeType(&self) -> ::windows::core::Result<PACKMAN_RUNTIME>;
    fn ActivationPolicy(&self) -> ::windows::core::Result<PM_ACTIVATION_POLICY>;
    fn TaskType(&self) -> ::windows::core::Result<PM_TASK_TYPE>;
    fn get_InvocationInfo(&self, pimageurn: *mut ::windows::core::BSTR, pparameters: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ImagePath(&self, pimagepath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ImageParams(&self, pimageparams: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn InstallRootFolder(&self, pinstallrootfolder: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DataRootFolder(&self, pdatarootfolder: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn IsSingleInstanceHost(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsInteropEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ApplicationState(&self) -> ::windows::core::Result<PM_APPLICATION_STATE>;
    fn InstallType(&self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn get_Version(&self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::Result<()>;
    fn BitsPerPixel(&self) -> ::windows::core::Result<u16>;
    fn SuppressesDehydration(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BackgroundExecutionAbilities(&self, pbackgroundexecutionabilities: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn IsOptedForExtendedMem(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMTaskInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMTaskInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>() -> IPMTaskInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn NavigationPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnavigationpage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NavigationPage(::core::mem::transmute_copy(&pnavigationpage)).into()
        }
        unsafe extern "system" fn TaskTransition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TaskTransition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptasktransition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuntimeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RuntimeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pruntimetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pactivationpolicy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TaskType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptasktype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn ImagePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimagepath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImagePath(::core::mem::transmute_copy(&pimagepath)).into()
        }
        unsafe extern "system" fn ImageParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageparams: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImageParams(::core::mem::transmute_copy(&pimageparams)).into()
        }
        unsafe extern "system" fn InstallRootFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstallrootfolder: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallRootFolder(::core::mem::transmute_copy(&pinstallrootfolder)).into()
        }
        unsafe extern "system" fn DataRootFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatarootfolder: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DataRootFolder(::core::mem::transmute_copy(&pdatarootfolder)).into()
        }
        unsafe extern "system" fn IsSingleInstanceHost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSingleInstanceHost() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pissingleinstancehost, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInteropEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInteropEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisinteropenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papplicationstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstallType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstalltype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Version(::core::mem::transmute_copy(&ptargetmajorversion), ::core::mem::transmute_copy(&ptargetminorversion)).into()
        }
        unsafe extern "system" fn BitsPerPixel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitsperpixel: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BitsPerPixel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbitsperpixel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressesDehydration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SuppressesDehydration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psuppressesdehydration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundExecutionAbilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbackgroundexecutionabilities: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackgroundExecutionAbilities(::core::mem::transmute_copy(&pbackgroundexecutionabilities)).into()
        }
        unsafe extern "system" fn IsOptedForExtendedMem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOptedForExtendedMem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisoptedin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            NavigationPage: NavigationPage::<Identity, Impl, OFFSET>,
            TaskTransition: TaskTransition::<Identity, Impl, OFFSET>,
            RuntimeType: RuntimeType::<Identity, Impl, OFFSET>,
            ActivationPolicy: ActivationPolicy::<Identity, Impl, OFFSET>,
            TaskType: TaskType::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
            ImagePath: ImagePath::<Identity, Impl, OFFSET>,
            ImageParams: ImageParams::<Identity, Impl, OFFSET>,
            InstallRootFolder: InstallRootFolder::<Identity, Impl, OFFSET>,
            DataRootFolder: DataRootFolder::<Identity, Impl, OFFSET>,
            IsSingleInstanceHost: IsSingleInstanceHost::<Identity, Impl, OFFSET>,
            IsInteropEnabled: IsInteropEnabled::<Identity, Impl, OFFSET>,
            ApplicationState: ApplicationState::<Identity, Impl, OFFSET>,
            InstallType: InstallType::<Identity, Impl, OFFSET>,
            get_Version: get_Version::<Identity, Impl, OFFSET>,
            BitsPerPixel: BitsPerPixel::<Identity, Impl, OFFSET>,
            SuppressesDehydration: SuppressesDehydration::<Identity, Impl, OFFSET>,
            BackgroundExecutionAbilities: BackgroundExecutionAbilities::<Identity, Impl, OFFSET>,
            IsOptedForExtendedMem: IsOptedForExtendedMem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTaskInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMTaskInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMTaskInfo>;
}
impl ::windows::core::RuntimeName for IPMTaskInfoEnumerator {}
impl IPMTaskInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfoEnumerator_Impl, const OFFSET: isize>() -> IPMTaskInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTaskInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTaskInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTileInfo_Impl: Sized {
    fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TileID(&self, ptileid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TemplateType(&self) -> ::windows::core::Result<TILE_TEMPLATE_TYPE>;
    fn get_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn get_HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<u32>;
    fn IsNotified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn TaskID(&self, ptaskid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TileType(&self) -> ::windows::core::Result<PM_STARTTILE_TYPE>;
    fn IsThemable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn get_PropertyById(&self, propid: u32) -> ::windows::core::Result<IPMTilePropertyInfo>;
    fn get_InvocationInfo(&self, pimageurn: *mut ::windows::core::BSTR, pparameters: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PropertyEnum(&self) -> ::windows::core::Result<IPMTilePropertyEnumerator>;
    fn get_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<PM_TILE_SIZE>;
    fn set_HubPosition(&self, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::Result<()>;
    fn set_NotifiedState(&self, notified: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::Result<()>;
    fn set_InvocationInfo(&self, taskname: &::windows::core::BSTR, taskparameters: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn StartTileBlob(&self, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()>;
    fn IsRestoring(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsAutoRestoreDisabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn set_IsRestoring(&self, restoring: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_IsAutoRestoreDisabled(&self, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPMTileInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IPMTileInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>() -> IPMTileInfo_Vtbl {
        unsafe extern "system" fn ProductID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptileid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TileID(::core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn TemplateType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TemplateType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptemplatetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HubPinnedState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_HubPinnedState(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinned, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HubPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_HubPosition(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pposition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsNotified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisnotified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisdefault, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TaskID(::core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn TileType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TileType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstarttiletype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsThemable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisthemable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PropertyById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: u32, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PropertyById(::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_InvocationInfo(::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn PropertyEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptilepropenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptilepropenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HubTileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_HubTileSize(::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_HubPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_HubPosition(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn set_NotifiedState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_NotifiedState(::core::mem::transmute_copy(&notified)).into()
        }
        unsafe extern "system" fn set_HubPinnedState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_HubPinnedState(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&pinned)).into()
        }
        unsafe extern "system" fn set_HubTileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_HubTileSize(::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn set_InvocationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, taskparameters: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_InvocationInfo(::core::mem::transmute(&taskname), ::core::mem::transmute(&taskparameters)).into()
        }
        unsafe extern "system" fn StartTileBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartTileBlob(::core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsRestoring<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRestoring() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisrestoring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAutoRestoreDisabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAutoRestoreDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisautorestoredisabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsRestoring<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_IsRestoring(::core::mem::transmute_copy(&restoring)).into()
        }
        unsafe extern "system" fn set_IsAutoRestoreDisabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_IsAutoRestoreDisabled(::core::mem::transmute_copy(&autorestoredisabled)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TileID: TileID::<Identity, Impl, OFFSET>,
            TemplateType: TemplateType::<Identity, Impl, OFFSET>,
            get_HubPinnedState: get_HubPinnedState::<Identity, Impl, OFFSET>,
            get_HubPosition: get_HubPosition::<Identity, Impl, OFFSET>,
            IsNotified: IsNotified::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            TileType: TileType::<Identity, Impl, OFFSET>,
            IsThemable: IsThemable::<Identity, Impl, OFFSET>,
            get_PropertyById: get_PropertyById::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
            PropertyEnum: PropertyEnum::<Identity, Impl, OFFSET>,
            get_HubTileSize: get_HubTileSize::<Identity, Impl, OFFSET>,
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
        iid == &<IPMTileInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMTileInfoEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMTileInfo>;
}
impl ::windows::core::RuntimeName for IPMTileInfoEnumerator {}
impl IPMTileInfoEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfoEnumerator_Impl, const OFFSET: isize>() -> IPMTileInfoEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTileInfoEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptileinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTileInfoEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMTilePropertyEnumerator_Impl: Sized {
    fn Next(&self) -> ::windows::core::Result<IPMTilePropertyInfo>;
}
impl ::windows::core::RuntimeName for IPMTilePropertyEnumerator {}
impl IPMTilePropertyEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTilePropertyEnumerator_Impl, const OFFSET: isize>() -> IPMTilePropertyEnumerator_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTilePropertyEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTilePropertyEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"implement\"`*"]
pub trait IPMTilePropertyInfo_Impl: Sized {
    fn PropertyID(&self) -> ::windows::core::Result<u32>;
    fn PropertyValue(&self, ppropvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn set_Property(&self, propvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPMTilePropertyInfo {}
impl IPMTilePropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>() -> IPMTilePropertyInfo_Vtbl {
        unsafe extern "system" fn PropertyID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PropertyValue(::core::mem::transmute_copy(&ppropvalue)).into()
        }
        unsafe extern "system" fn set_Property<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_Property(::core::mem::transmute(&propvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropertyID: PropertyID::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            set_Property: set_Property::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPMTilePropertyInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IValidate_Impl: Sized {
    fn OpenDatabase(&self, szdatabase: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OpenCUB(&self, szcubfile: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn CloseDatabase(&self) -> ::windows::core::Result<()>;
    fn CloseCUB(&self) -> ::windows::core::Result<()>;
    fn SetDisplay(&self, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetStatus(&self, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Validate(&self, wzices: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IValidate {}
#[cfg(feature = "Win32_Foundation")]
impl IValidate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>() -> IValidate_Vtbl {
        unsafe extern "system" fn OpenDatabase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdatabase: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenDatabase(::core::mem::transmute(&szdatabase)).into()
        }
        unsafe extern "system" fn OpenCUB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcubfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenCUB(::core::mem::transmute(&szcubfile)).into()
        }
        unsafe extern "system" fn CloseDatabase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseDatabase().into()
        }
        unsafe extern "system" fn CloseCUB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseCUB().into()
        }
        unsafe extern "system" fn SetDisplay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplay(::core::mem::transmute_copy(&pdisplayfunction), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&pstatusfunction), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzices: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Validate(::core::mem::transmute(&wzices)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IValidate as ::windows::core::ComInterface>::IID
    }
}
